//! Time of day, weather, street lamps, lit windows, rain and lightning.

use crate::camera::{CamState, MainCam};
use crate::city::map::*;
use crate::render::city3d::{CityVis, Mats};
use crate::sim::agents::{Act, Sim};
use crate::state::Game;
use crate::world::CityMap;
use bevy::pbr::{CascadeShadowConfigBuilder, DistanceFog, FogFalloff, NotShadowCaster};
use bevy::prelude::*;

/// Game minutes per real second.
pub const TIME_SCALE: f32 = 1.0;

#[derive(Component)]
pub struct Sun;

#[derive(Component)]
pub struct PlayerFill;

pub fn player_fill(game: Res<Game>, env: Res<EnvState>, mut q: Query<(&mut Transform, &mut PointLight), With<PlayerFill>>) {
    if let Ok((mut t, mut l)) = q.single_mut() {
        t.translation = Vec3::new(game.player.pos.x, 2.6, game.player.pos.y + 0.8);
        l.intensity = 6000.0 + env.darkness * 20000.0;
    }
}

#[derive(Component)]
pub struct LampLight(pub usize);

#[derive(Component)]
pub struct InteriorLight(pub usize);

#[derive(Component)]
pub struct RainDrop {
    pub vel: f32,
}

#[derive(Resource, Default)]
pub struct EnvState {
    pub lightning: f32,
    pub next_lightning: f32,
    pub window_timer: f32,
    pub darkness: f32,
    pub light_level_player: f32,
    /// 0..1 fade of the red sight (the world darkens, only the threads glow)
    pub red_fx: f32,
    /// how wet the streets are (follows the rain slowly, dries slowly)
    pub wet: f32,
    pub wet_applied: f32,
}

pub fn setup_env(mut c: Commands, mut meshes: ResMut<Assets<Mesh>>, mut mats: ResMut<Assets<StandardMaterial>>) {
    c.spawn((
        DirectionalLight { illuminance: 3000.0, shadows_enabled: true, color: Color::srgb(0.7, 0.75, 1.0), ..default() },
        CascadeShadowConfigBuilder { num_cascades: 2, maximum_distance: 60.0, first_cascade_far_bound: 20.0, ..default() }.build(),
        Transform::from_xyz(10.0, 30.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        Sun,
    ));
    for i in 0..20 {
        c.spawn((
            PointLight { color: Color::srgb(1.0, 0.72, 0.42), intensity: 0.0, range: 16.0, radius: 0.15, shadows_enabled: i < 4, ..default() },
            Transform::from_xyz(0.0, -50.0, 0.0),
            LampLight(i),
        ));
    }
    for i in 0..6 {
        c.spawn((
            PointLight { color: Color::srgb(1.0, 0.78, 0.5), intensity: 0.0, range: 9.0, radius: 0.1, shadows_enabled: i < 2, ..default() },
            Transform::from_xyz(0.0, -50.0, 0.0),
            InteriorLight(i),
        ));
    }
    c.spawn((
        PointLight { color: Color::srgb(0.75, 0.75, 1.0), intensity: 20000.0, range: 5.0, radius: 0.3, shadows_enabled: false, ..default() },
        Transform::from_xyz(0.0, 2.5, 0.0),
        PlayerFill,
    ));
    // rain
    let drop = meshes.add(Cuboid::new(0.012, 0.55, 0.012));
    let mat = mats.add(StandardMaterial {
        base_color: Color::srgba(0.6, 0.65, 0.8, 0.18),
        emissive: LinearRgba::rgb(0.12, 0.13, 0.17),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    for i in 0..700 {
        let h = crate::util::hashf(i, 3, 9);
        c.spawn((
            Mesh3d(drop.clone()),
            MeshMaterial3d(mat.clone()),
            Transform::from_xyz(0.0, -100.0 - h, 0.0),
            RainDrop { vel: 16.0 + h * 6.0 },
            NotShadowCaster,
        ));
    }
}

/// Sky colour curves.
fn day_curve(hour: f32) -> (f32, Color, f32) {
    // returns (sun strength 0..1, sky colour, ambient)
    let h = hour;
    let s = if (6.0..19.0).contains(&h) {
        let t = ((h - 6.0) / 13.0 * std::f32::consts::PI).sin();
        t.max(0.0)
    } else {
        0.0
    };
    let dusk = if (17.5..20.0).contains(&h) {
        1.0 - ((h - 18.7).abs() / 1.3).min(1.0)
    } else if (5.0..7.5).contains(&h) {
        1.0 - ((h - 6.2).abs() / 1.2).min(1.0)
    } else {
        0.0
    };
    let night = Color::srgb(0.02, 0.02, 0.05);
    let day = Color::srgb(0.30, 0.32, 0.36);
    let dusk_c = Color::srgb(0.28, 0.12, 0.14);
    let base = night.mix(&day, s);
    let sky = base.mix(&dusk_c, dusk * 0.7);
    (s, sky, 0.15 + s * 0.85)
}

pub fn clock(time: Res<Time>, mut game: ResMut<Game>, ui: Res<crate::ui::UiState>) {
    if ui.pauses_world() {
        return;
    }
    let dt = time.delta_secs().min(0.1);
    game.played_secs += dt;
    game.minute += dt * TIME_SCALE;
    if game.minute >= 1440.0 {
        game.minute -= 1440.0;
        game.day += 1;
        // weather for the new day
        let st = crate::city::style::style(game.city, game.year);
        let r = crate::util::hashf(game.day, game.year, 17);
        game.rain_target = if r > st.rain { 0.0 } else if r > st.rain * 0.5 { 0.4 } else { 0.85 };
        game.fog = crate::util::hashf(game.day, game.year, 18) * st.fog;
    }
    // weather drifts, and changes a few times a day
    if (game.minute as i32) % 240 == 0 && crate::util::hashf(game.day, game.minute as i32, 5) < 0.02 {
        game.rain_target = crate::util::hashf(game.day, game.minute as i32, 6);
    }
    let rt = if game.phase == crate::state::Phase::Prologue && !game.flag("prologue_apartment") { 0.0 } else { game.rain_target };
    game.rain += (rt - game.rain) * (dt * 0.02).min(1.0);
    if game.phase == crate::state::Phase::Prologue && game.flag("limbo") {
        game.rain = 0.0;
    }
}

#[allow(clippy::too_many_arguments)]
pub fn update_env(
    time: Res<Time>,
    game: Res<Game>,
    cam: Res<CamState>,
    map: Option<Res<CityMap>>,
    mut vis: ResMut<CityVis>,
    sim: Res<Sim>,
    mats_h: Option<Res<Mats>>,
    mut env: ResMut<EnvState>,
    mut ambient: ResMut<AmbientLight>,
    mut clear: ResMut<ClearColor>,
    (mut sun, mut lamps, mut inter, mut fogq, mut glass, mut fx): (
        Query<(&mut DirectionalLight, &mut Transform), With<Sun>>,
        Query<(&LampLight, &mut PointLight, &mut Transform), (Without<Sun>, Without<InteriorLight>)>,
        Query<(&InteriorLight, &mut PointLight, &mut Transform), (Without<Sun>, Without<LampLight>)>,
        Query<&mut DistanceFog, With<MainCam>>,
        Query<&mut MeshMaterial3d<StandardMaterial>>,
        Query<(&mut Visibility, Has<crate::render::city3d::LampHalos>), Or<(With<crate::render::city3d::LampHalos>, With<crate::render::city3d::Puddles>)>>,
    ),
    mut mats: ResMut<Assets<StandardMaterial>>,
    settings: Res<crate::keys::Settings>,
    crt: Res<crate::cases::run::CaseRt>,
) {
    let dt = time.delta_secs();
    let want_red = if crt.red_sight { 1.0 } else { 0.0 };
    env.red_fx += (want_red - env.red_fx) * (dt * 4.0).min(1.0);
    let rf = env.red_fx;
    let hour = game.hour();
    let (s, sky, amb) = day_curve(hour);
    let rain = game.rain;
    let overcast = 1.0 - rain * 0.55;
    // lightning during heavy rain
    env.next_lightning -= dt;
    if rain > 0.75 && env.next_lightning <= 0.0 {
        env.lightning = 1.0;
        env.next_lightning = 8.0 + crate::util::hashf(game.minute as i32, game.day, 3) * 25.0;
    }
    env.lightning = (env.lightning - dt * 3.0).max(0.0);
    let flash = if env.lightning > 0.0 { (env.lightning * 40.0).sin().abs() * env.lightning } else { 0.0 };
    let redness = crate::cases::redness(&game);

    if let Ok((mut dl, mut tr)) = sun.single_mut() {
        let ang = (hour - 6.0) / 12.0 * std::f32::consts::PI;
        let dir = Vec3::new(ang.cos() * 0.6, ang.sin().max(0.25), 0.45);
        tr.translation = cam.focus + dir * 40.0;
        tr.look_at(cam.focus, Vec3::Y);
        let moon = 0.22;
        dl.illuminance = (s * 9000.0 * overcast + (1.0 - s) * 350.0 * moon * 8.0) + flash * 20000.0;
        dl.color = if s > 0.05 {
            Color::srgb(1.0, 0.9 - (1.0 - s) * 0.3, 0.8 - (1.0 - s) * 0.4)
        } else {
            Color::srgb(0.55, 0.6, 1.0)
        };
        dl.shadows_enabled = settings.shadows;
        dl.illuminance *= 1.0 - rf * 0.75;
        if rf > 0.01 {
            dl.color = dl.color.mix(&Color::srgb(1.0, 0.25, 0.25), rf * 0.8);
        }
    }
    ambient.brightness = (320.0 + amb * 600.0 * overcast + flash * 1500.0) * (1.0 - rf * 0.6);
    ambient.color = Color::srgb(0.5 + redness * 0.4, 0.5 - redness * 0.2, 0.75 - redness * 0.3).mix(&Color::srgb(0.9, 0.15, 0.2), rf * 0.7);
    let sky_c = sky.to_srgba();
    clear.0 = Color::srgb(sky_c.red * overcast + flash * 0.5, sky_c.green * overcast + flash * 0.5, sky_c.blue * overcast + flash * 0.6);
    env.darkness = 1.0 - s * overcast;

    if let Ok(mut fog) = fogq.single_mut() {
        let f = if settings.fog { game.fog.max(rain * 0.5) } else { 0.0 };
        let start = 22.0 - f * 14.0;
        let end = 70.0 - f * 40.0;
        fog.falloff = FogFalloff::Linear { start, end };
        let c = sky_c;
        fog.color = Color::srgb(c.red * 0.8 + 0.02 + redness * 0.08, c.green * 0.8 + 0.02, c.blue * 0.9 + 0.04).mix(&Color::srgb(0.12, 0.0, 0.02), rf);
        if rf > 0.01 {
            fog.falloff = FogFalloff::Linear { start: start * (1.0 - rf * 0.6), end: end * (1.0 - rf * 0.45) };
        }
    }

    // wet streets: shiny when it rains, puddles linger a while after
    let wet_target = if rain > 0.2 { 1.0 } else { 0.0 };
    env.wet += (wet_target - env.wet) * (dt * if wet_target > env.wet { 0.2 } else { 0.01 }).min(1.0);
    if (env.wet - env.wet_applied).abs() > 0.05 {
        env.wet_applied = env.wet;
        if let Some(mh) = &mats_h {
            let w = env.wet;
            for (h, dry) in [(&mh.road, 1.0), (&mh.asphalt, 1.0), (&mh.sidewalk, 1.0), (&mh.stone, 0.62)] {
                if let Some(mat) = mats.get_mut(h) {
                    mat.perceptual_roughness = (dry * (1.0 - w * 0.55)).max(0.2);
                    mat.reflectance = 0.35 + w * 0.3;
                }
            }
        }
    }
    let night_halo = env.darkness > 0.55;
    for (mut v, is_halo) in fx.iter_mut() {
        let on = if is_halo { night_halo } else { env.wet > 0.35 };
        let want = if on { Visibility::Inherited } else { Visibility::Hidden };
        if *v != want {
            *v = want;
        }
    }

    let Some(map) = map else { return };
    let m = &map.0;
    let night = env.darkness > 0.55;
    // street lamps: assign the pool to the nearest lamps
    let focus = cam.focus;
    let mut near: Vec<(f32, Vec3)> = vis.lamps.iter().map(|l| (l.distance_squared(focus), *l)).filter(|(d, _)| *d < 40.0 * 40.0).collect();
    near.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for (ll, mut pl, mut tr) in lamps.iter_mut() {
        if let (true, Some((_, p))) = (night, near.get(ll.0)) {
            tr.translation = *p;
            pl.intensity = 260000.0 * (0.95 + (time.elapsed_secs() * 7.0 + ll.0 as f32).sin() * 0.02);
            pl.color = Color::srgb(1.0, 0.7 - redness * 0.3, 0.42 - redness * 0.2);
            pl.shadows_enabled = settings.shadows && ll.0 < 4;
        } else {
            pl.intensity = 0.0;
            tr.translation = Vec3::new(0.0, -50.0, 0.0);
        }
    }
    // interior lights of the building Elias is in
    let inside = m.building_at(game.player.pos);
    let mut slots: Vec<Vec3> = Vec::new();
    if let Some(b) = inside {
        for l in &m.buildings[b].lights {
            slots.push(Vec3::new(l.x, 2.4, l.y));
        }
    }
    for (il, mut pl, mut tr) in inter.iter_mut() {
        if let Some(p) = slots.get(il.0) {
            tr.translation = *p;
            pl.intensity = if night { 120000.0 } else { 40000.0 };
        } else {
            pl.intensity = 0.0;
            tr.translation = Vec3::new(0.0, -50.0, 0.0);
        }
    }
    // lit windows (every second)
    env.window_timer -= dt;
    if env.window_timer <= 0.0 {
        env.window_timer = 1.0;
        if let Some(mh) = &mats_h {
            let mut occupied = vec![false; m.buildings.len()];
            for a in sim.agents.iter() {
                if let Some(b) = a.target_b {
                    if a.arrived && a.act != Act::Sleep && b < occupied.len() {
                        occupied[b] = true;
                    }
                }
            }
            for (i, b) in m.buildings.iter().enumerate() {
                if i >= vis.buildings.len() {
                    break;
                }
                let lit = night && !b.closed_forever && (occupied[i] || (b.kind.public() && b.kind.is_open(hour)));
                if vis.buildings[i].lit != lit {
                    vis.buildings[i].lit = lit;
                    if let Ok(mut mm) = glass.get_mut(vis.buildings[i].glass) {
                        mm.0 = if lit { mh.glass_lit.clone() } else { mh.glass_dark.clone() };
                    }
                }
            }
            // wet streets
            if let Some(road) = mats.get_mut(&mh.road) {
                road.perceptual_roughness = 0.55 - rain * 0.45;
            }
            if let Some(st) = mats.get_mut(&mh.stone) {
                st.perceptual_roughness = 0.6 - rain * 0.4;
            }
        }
    }
    // light level where the player stands (for stealth)
    let pp = Vec3::new(game.player.pos.x, 1.0, game.player.pos.y);
    let mut lvl = (1.0 - env.darkness) * 0.9;
    for l in vis.lamps.iter() {
        let d = l.distance(pp);
        if d < 7.0 && night {
            lvl += (1.0 - d / 7.0) * 0.8;
        }
    }
    if inside.is_some() && night {
        lvl += 0.35;
    }
    env.light_level_player = lvl.min(1.0);
}

pub fn update_rain(time: Res<Time>, game: Res<Game>, cam: Res<CamState>, mut q: Query<(&RainDrop, &mut Transform, &mut Visibility)>) {
    let dt = time.delta_secs();
    let n_active = (game.rain * 700.0) as usize;
    let f = cam.focus;
    for (i, (rd, mut tr, mut v)) in q.iter_mut().enumerate() {
        if i >= n_active {
            *v = Visibility::Hidden;
            continue;
        }
        *v = Visibility::Inherited;
        tr.translation.y -= rd.vel * dt;
        tr.translation.x -= rd.vel * dt * 0.12;
        let rel = tr.translation - f;
        if tr.translation.y < 0.0 || rel.x.abs() > 16.0 || rel.z.abs() > 14.0 {
            let h1 = crate::util::hashf(i as i32, (time.elapsed_secs() * 10.0) as i32, 1);
            let h2 = crate::util::hashf(i as i32, (time.elapsed_secs() * 10.0) as i32, 2);
            let h3 = crate::util::hashf(i as i32, (time.elapsed_secs() * 10.0) as i32, 3);
            tr.translation = Vec3::new(f.x + (h1 - 0.5) * 32.0, 3.0 + h3 * 12.0, f.z + (h2 - 0.5) * 28.0);
        }
    }
    let _ = TS;
}
