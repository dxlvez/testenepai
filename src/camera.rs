//! Path-of-Exile style camera: angled top-down perspective that follows
//! Elias, with zoom, 90° rotation, cursor ground picking and cut-away of
//! buildings that would hide him.

use crate::city::map::*;
use crate::keys::{Act, Action, Script};
use crate::render::city3d::CityVis;
use crate::state::Game;
use crate::world::CityMap;
use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct MainCam;

#[derive(Resource)]
pub struct CamState {
    pub yaw: f32,
    pub yaw_target: f32,
    pub dist: f32,
    pub dist_target: f32,
    pub focus: Vec3,
    pub shake: f32,
    pub pitch: f32,
}

impl Default for CamState {
    fn default() -> Self {
        CamState { yaw: 0.0, yaw_target: 0.0, dist: 17.0, dist_target: 17.0, focus: Vec3::ZERO, shake: 0.0, pitch: 0.95 }
    }
}

/// Where the mouse cursor hits the ground plane (world XZ).
#[derive(Resource, Default)]
pub struct Cursor {
    pub world: Vec2,
    pub screen: Vec2,
    pub valid: bool,
    pub clicked: bool,
    pub held: bool,
    pub rclicked: bool,
}

pub fn spawn_camera(mut c: Commands) {
    c.spawn((
        Camera3d::default(),
        Camera { hdr: true, ..default() },
        Projection::Perspective(PerspectiveProjection { fov: 0.62, near: 0.5, far: 200.0, ..default() }),
        Tonemapping::TonyMcMapface,
        Bloom { intensity: 0.22, ..Bloom::NATURAL },
        DistanceFog {
            color: Color::srgb(0.03, 0.025, 0.05),
            falloff: FogFalloff::Linear { start: 20.0, end: 60.0 },
            ..default()
        },
        Msaa::Sample4,
        MainCam,
        Transform::from_xyz(0.0, 15.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn camera_follow(
    time: Res<Time>,
    act: Res<Act>,
    game: Res<Game>,
    mut st: ResMut<CamState>,
    mut q: Query<&mut Transform, With<MainCam>>,
    mut wheel: EventReader<bevy::input::mouse::MouseWheel>,
    ui: Res<crate::ui::UiState>,
) {
    let dt = time.delta_secs();
    if !ui.blocks_input() {
        for w in wheel.read() {
            st.dist_target = (st.dist_target - w.y * 1.5).clamp(8.0, 30.0);
        }
        if act.just(Action::CamLeft) {
            st.yaw_target -= std::f32::consts::FRAC_PI_2;
        }
        if act.just(Action::CamRight) {
            st.yaw_target += std::f32::consts::FRAC_PI_2;
        }
    } else {
        wheel.clear();
    }
    st.yaw += (st.yaw_target - st.yaw) * (dt * 8.0).min(1.0);
    st.dist += (st.dist_target - st.dist) * (dt * 6.0).min(1.0);
    let p = game.player.pos;
    let target = Vec3::new(p.x, 0.8, p.y);
    let f = st.focus;
    st.focus = f + (target - f) * (dt * 7.0).min(1.0);
    if st.focus.distance(target) > 15.0 {
        st.focus = target;
    }
    st.shake = (st.shake - dt * 2.0).max(0.0);
    let Ok(mut tr) = q.single_mut() else { return };
    let back = Vec3::new(st.yaw.sin(), 0.0, st.yaw.cos());
    let off = back * st.dist * st.pitch.cos() + Vec3::Y * st.dist * st.pitch.sin();
    let shake = if st.shake > 0.0 {
        Vec3::new((time.elapsed_secs() * 71.0).sin(), (time.elapsed_secs() * 53.0).cos(), 0.0) * st.shake * 0.15
    } else {
        Vec3::ZERO
    };
    tr.translation = st.focus + off + shake;
    tr.look_at(st.focus, Vec3::Y);
}

pub fn update_cursor(
    windows: Query<&Window, With<PrimaryWindow>>,
    cams: Query<(&Camera, &GlobalTransform), With<MainCam>>,
    mut cur: ResMut<Cursor>,
    buttons: Res<ButtonInput<MouseButton>>,
    script: Res<Script>,
) {
    let Ok(win) = windows.single() else { return };
    let Ok((cam, gt)) = cams.single() else { return };
    let mut pos = win.cursor_position();
    if let Some(m) = script.mouse {
        pos = Some(m);
    }
    cur.clicked = buttons.just_pressed(MouseButton::Left) || script.click.is_some();
    cur.held = buttons.pressed(MouseButton::Left) || script.mouse_held;
    cur.rclicked = buttons.just_pressed(MouseButton::Right);
    cur.valid = false;
    let Some(sp) = pos else { return };
    cur.screen = sp;
    let Ok(ray) = cam.viewport_to_world(gt, sp) else { return };
    // intersect with plane y = 1.0 (chest height feels right for aiming)
    let plane_y = 1.0;
    if ray.direction.y.abs() < 1e-4 {
        return;
    }
    let t = (plane_y - ray.origin.y) / ray.direction.y;
    if t < 0.0 {
        return;
    }
    let hit = ray.origin + ray.direction * t;
    cur.world = Vec2::new(hit.x, hit.z);
    cur.valid = true;
}

/// Hide roofs / upper walls of the building Elias is in and of buildings
/// standing between him and the camera.
pub fn cutaway(game: Res<Game>, map: Option<Res<CityMap>>, st: Res<CamState>, mut vis: ResMut<CityVis>, mut q: Query<&mut Visibility>) {
    let Some(map) = map else { return };
    let m = &map.0;
    let p = game.player.pos;
    let inside = m.building_at(p);
    let cam_dir = Vec2::new(st.yaw.sin(), st.yaw.cos());
    for (i, b) in m.buildings.iter().enumerate() {
        if i >= vis.buildings.len() {
            break;
        }
        let mut cut = inside == Some(i);
        if !cut {
            // sample points between player and camera direction
            for k in 1..8 {
                let s = p + cam_dir * (k as f32 * 0.9);
                let (x, y) = to_tile(s);
                if b.contains_tile(x, y) {
                    cut = true;
                    break;
                }
            }
        }
        let bv = &mut vis.buildings[i];
        if bv.cut != cut {
            bv.cut = cut;
            if let Ok(mut v) = q.get_mut(bv.roof) {
                *v = if cut { Visibility::Hidden } else { Visibility::Inherited };
            }
            if let Ok(mut v) = q.get_mut(bv.full) {
                *v = if cut { Visibility::Hidden } else { Visibility::Inherited };
            }
            if let Ok(mut v) = q.get_mut(bv.low) {
                *v = if cut { Visibility::Inherited } else { Visibility::Hidden };
            }
        }
    }
}
