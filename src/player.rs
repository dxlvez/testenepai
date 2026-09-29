//! Elias: movement (walk / run / slow walk), collision, facing the mouse
//! while aiming, stamina.

use crate::camera::Cursor;
use crate::keys::{Act, Action};
use crate::render::character::{spawn_character, Garment, Hair, Hat, Look, Pose, Rig};
use crate::render::city3d::Mats;
use crate::state::Game;
use crate::world::{ground_y, CityMap};
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerVis;

#[derive(Resource, Default)]
pub struct PlayerRt {
    pub facing: f32,
    pub moving: f32,
    pub pose: Pose,
    pub aiming: bool,
    pub noise: f32,
    pub running: bool,
    pub sneaking: bool,
    pub action_lock: f32,
    pub look_dirty: bool,
    pub entity: Option<Entity>,
    pub carrying: Option<u32>,
    pub dragging: bool,
    pub hidden_in: Option<usize>,
    pub in_car: bool,
    pub punch_cd: f32,
    pub hurt_flash: f32,
    pub weapon_out: bool,
    pub hold_t: f32,
}

pub fn elias_look(outfit: crate::items::Outfit, age: f32) -> Look {
    use crate::items::Outfit::*;
    let (garment, top, bottom, accent, hat, hat_col) = match outfit {
        Modern => (Garment::Hoodie, [0.16, 0.17, 0.2], [0.18, 0.24, 0.36], [0.5, 0.1, 0.12], Hat::None, [0.1, 0.1, 0.1]),
        Suit => (Garment::Suit, [0.22, 0.2, 0.19], [0.2, 0.19, 0.18], [0.45, 0.08, 0.1], Hat::Fedora, [0.15, 0.13, 0.12]),
        Worker => (Garment::Overalls, [0.55, 0.52, 0.45], [0.2, 0.26, 0.38], [0.3, 0.2, 0.1], Hat::FlatCap, [0.25, 0.22, 0.2]),
        Police => (Garment::Uniform, [0.1, 0.12, 0.2], [0.1, 0.12, 0.2], [0.1, 0.1, 0.15], Hat::PoliceCap, [0.1, 0.12, 0.2]),
        Doctor => (Garment::LabCoat, [0.88, 0.88, 0.86], [0.2, 0.2, 0.22], [0.3, 0.3, 0.35], Hat::None, [0.1, 0.1, 0.1]),
        Journalist => (Garment::Suit, [0.4, 0.34, 0.26], [0.3, 0.26, 0.2], [0.2, 0.3, 0.45], Hat::Fedora, [0.35, 0.3, 0.22]),
        Gangster => (Garment::LongCoat, [0.08, 0.08, 0.09], [0.1, 0.1, 0.1], [0.6, 0.05, 0.08], Hat::Fedora, [0.06, 0.06, 0.07]),
        Aristocrat => (Garment::Suit, [0.05, 0.05, 0.06], [0.05, 0.05, 0.06], [0.9, 0.9, 0.88], Hat::Bowler, [0.05, 0.05, 0.05]),
        Priest => (Garment::Robe, [0.04, 0.04, 0.05], [0.04, 0.04, 0.05], [0.9, 0.9, 0.9], Hat::None, [0.1, 0.1, 0.1]),
    };
    Look {
        female: false,
        skin: [0.86, 0.7, 0.58],
        hair_col: [0.12, 0.09, 0.07],
        hair: Hair::Short,
        hat,
        hat_col,
        garment,
        top,
        bottom,
        accent,
        height: 1.03,
        girth: 1.0,
        beard: age > 45.0,
        age,
    }
}

pub fn spawn_player_vis(mut c: Commands, mut meshes: ResMut<Assets<Mesh>>, mats: Option<Res<Mats>>, game: Res<Game>, mut rt: ResMut<PlayerRt>, q: Query<Entity, With<PlayerVis>>) {
    if !rt.look_dirty && rt.entity.is_some() {
        return;
    }
    let Some(mats) = mats else { return };
    for e in q.iter() {
        c.entity(e).despawn();
    }
    let look = elias_look(game.player.outfit, game.player.body_age);
    let (root, rig) = spawn_character(&mut c, &mut meshes, &mats.plain, &look);
    c.entity(root).insert((PlayerVis, rig, Transform::from_xyz(game.player.pos.x, 0.0, game.player.pos.y)));
    rt.entity = Some(root);
    rt.look_dirty = false;
}

#[allow(clippy::too_many_arguments)]
pub fn player_move(
    time: Res<Time>,
    act: Res<Act>,
    mut game: ResMut<Game>,
    map: Option<Res<CityMap>>,
    mut rt: ResMut<PlayerRt>,
    cursor: Res<Cursor>,
    cam: Res<crate::camera::CamState>,
    ui: Res<crate::ui::UiState>,
) {
    let Some(map) = map else { return };
    let m = &map.0;
    let dt = time.delta_secs().min(0.1);
    rt.action_lock = (rt.action_lock - dt).max(0.0);
    rt.punch_cd = (rt.punch_cd - dt).max(0.0);
    rt.hurt_flash = (rt.hurt_flash - dt).max(0.0);
    if ui.blocks_input() || rt.in_car || rt.hidden_in.is_some() {
        rt.moving = 0.0;
        if !rt.in_car {
            rt.pose = if rt.hidden_in.is_some() { Pose::Cower } else { Pose::Idle };
        }
        return;
    }
    let mut dir = Vec2::ZERO;
    if act.held(Action::Up) {
        dir.y -= 1.0;
    }
    if act.held(Action::Down) {
        dir.y += 1.0;
    }
    if act.held(Action::Left) {
        dir.x -= 1.0;
    }
    if act.held(Action::Right) {
        dir.x += 1.0;
    }
    // rotate input by camera yaw so "up" is always up on screen
    let yaw = cam.yaw;
    let (s, c) = yaw.sin_cos();
    let dir = Vec2::new(dir.x * c + dir.y * s, -dir.x * s + dir.y * c);
    let p = &mut game.player;
    let carrying = rt.carrying.is_some();
    let running = act.held(Action::Run) && p.stamina > 0.05 && !carrying && !rt.aiming;
    let sneaking = act.held(Action::Walk);
    let mut speed: f32 = if running {
        4.6 + if p.stim > 0.0 { 1.2 } else { 0.0 }
    } else if sneaking {
        1.0
    } else {
        2.1
    };
    if carrying {
        speed *= if rt.dragging { 0.45 } else { 0.6 };
    }
    if p.health < 35.0 {
        speed *= 0.75;
    }
    if rt.aiming {
        speed = speed.min(1.6);
    }
    rt.running = running && dir != Vec2::ZERO;
    rt.sneaking = sneaking && dir != Vec2::ZERO;
    if dir != Vec2::ZERO {
        let d = dir.normalize();
        let np = p.pos + d * speed * dt;
        p.pos = m.collide(np, 0.26);
        if !rt.aiming {
            rt.facing = d.y.atan2(d.x);
        }
        rt.moving = speed;
        if running {
            p.stamina = (p.stamina - dt * if p.stim > 0.0 { 0.04 } else { 0.11 }).max(0.0);
        }
    } else {
        rt.moving = 0.0;
    }
    if !running {
        p.stamina = (p.stamina + dt * 0.08).min(1.0);
    }
    // noise emitted (used by stealth / NPC hearing)
    let stealth = p.skill(crate::state::Skill::Stealth) as f32;
    rt.noise = if rt.moving == 0.0 {
        0.0
    } else if running {
        7.0
    } else if sneaking {
        (1.2 - stealth * 0.1).max(0.3)
    } else {
        3.0 - stealth * 0.15
    };
    if rt.aiming && cursor.valid {
        let d = cursor.world - p.pos;
        rt.facing = d.y.atan2(d.x);
    }
    rt.pose = if carrying {
        if rt.dragging {
            Pose::Drag
        } else {
            Pose::Carry
        }
    } else if rt.aiming {
        Pose::Aim
    } else if rt.action_lock > 0.0 {
        rt.pose
    } else if rt.moving > 0.0 {
        if running {
            Pose::Run
        } else if sneaking {
            Pose::Sneak
        } else {
            Pose::Walk
        }
    } else {
        Pose::Idle
    };
}

pub fn sync_player(game: Res<Game>, map: Option<Res<CityMap>>, rt: Res<PlayerRt>, mut q: Query<(&mut Transform, &mut Rig, &mut Visibility), With<PlayerVis>>, time: Res<Time>) {
    let Some(map) = map else { return };
    let Ok((mut tr, mut rig, mut v)) = q.single_mut() else { return };
    let p = game.player.pos;
    tr.translation = Vec3::new(p.x, ground_y(&map.0, p), p.y);
    let want = Quat::from_rotation_y(-rt.facing - std::f32::consts::FRAC_PI_2);
    tr.rotation = tr.rotation.slerp(want, (time.delta_secs() * 14.0).min(1.0));
    rig.pose = rt.pose;
    rig.speed = rt.moving;
    *v = if rt.in_car { Visibility::Hidden } else { Visibility::Inherited };
}
