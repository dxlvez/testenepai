//! Cars of every era: parked and driven by NPCs, stealable (pull the driver
//! out, break the glass, pick the lock), drivable, sellable.

use crate::audio::Sfx;
use crate::city::map::*;
use crate::crime::{CrimeEv, DecalKind, SpawnDecal};
use crate::items::{Item, Tool};
use crate::keys::{Act, Action};
use crate::player::PlayerRt;
use crate::render::city3d::Mats;
use crate::render::mesh::{c3, MB};
use crate::sim::agents::{AState, Sim};
use crate::sim::people::Pid;
use crate::state::*;
use crate::ui::hud::Toasts;
use crate::ui::screens::{Choice, Choices, Lockpick};
use crate::ui::{Mode, UiState};
use crate::world::CityMap;
use bevy::prelude::*;
use std::sync::Mutex;

pub struct Car {
    pub id: u32,
    pub pos: Vec2,
    pub heading: f32,
    pub speed: f32,
    pub driver: Option<Pid>,
    pub owner: Option<Pid>,
    pub owner_elias: bool,
    pub locked: bool,
    pub stolen: bool,
    pub glass_broken: bool,
    pub color: [f32; 3],
    pub value: i32,
    pub entity: Option<Entity>,
    pub route: Vec<Vec2>,
    pub ri: usize,
    pub player: bool,
    pub wait: f32,
    /// horse-drawn carriage / wagon
    pub horse: bool,
}

#[derive(Resource, Default)]
pub struct Cars {
    pub list: Vec<Car>,
    pub spawned_for: Option<(crate::city::gen::CityId, i32, u32)>,
}

#[derive(Component)]
pub struct CarVis(pub u32);

static UNLOCK: Mutex<Vec<u32>> = Mutex::new(Vec::new());

pub fn unlock_car(id: u32) {
    if let Ok(mut v) = UNLOCK.lock() {
        v.push(id);
    }
}

fn car_mesh(year: i32, color: [f32; 3], id: u32) -> MB {
    let mut m = MB::new();
    let body = c3(color);
    let body_d = c3([color[0] * 0.7, color[1] * 0.7, color[2] * 0.7]);
    let dark = c3([0.04, 0.04, 0.045]);
    let tyre = c3([0.06, 0.06, 0.06]);
    let glass = c3([0.1, 0.13, 0.18]);
    let chrome = c3([0.7, 0.7, 0.72]);
    let variant = id % 3;
    // car points along +x
    let (l, w) = if year < 1935 { (3.6, 1.55) } else if year < 1960 { (4.5, 1.8) } else if year < 1980 { (4.8, 1.85) } else { (4.4, 1.75) };
    let (hl, hw) = (l / 2.0, w / 2.0);
    let wr = if year < 1935 { 0.38 } else { 0.33 };
    let wheel_x = hl * 0.64;
    if year < 1935 {
        // 1920s tourer / sedan: tall cabin, long hood, separate round fenders
        m.cuboid(Vec3::new(-hl * 0.95, 0.45, -hw * 0.78), Vec3::new(hl * 0.35, 0.95, hw * 0.78), body);
        m.cuboid(Vec3::new(hl * 0.3, 0.55, -hw * 0.45), Vec3::new(hl * 0.95, 0.95, hw * 0.45), body);
        m.cuboid(Vec3::new(hl * 0.93, 0.5, -hw * 0.42), Vec3::new(hl + 0.03, 1.0, hw * 0.42), chrome);
        if variant == 0 {
            // closed sedan cabin with windows
            m.cuboid(Vec3::new(-hl * 0.9, 0.95, -hw * 0.76), Vec3::new(hl * 0.25, 1.65, hw * 0.76), body_d);
            m.cuboid(Vec3::new(-hl * 0.85, 1.1, -hw * 0.77), Vec3::new(hl * 0.2, 1.5, hw * 0.77), glass);
            m.cuboid(Vec3::new(-hl * 0.92, 1.65, -hw * 0.78), Vec3::new(hl * 0.27, 1.7, hw * 0.78), dark);
        } else {
            // open tourer with folded roof and windshield
            m.cuboid(Vec3::new(hl * 0.22, 0.95, -hw * 0.7), Vec3::new(hl * 0.25, 1.4, hw * 0.7), glass);
            m.cuboid(Vec3::new(-hl * 0.95, 0.95, -hw * 0.75), Vec3::new(-hl * 0.7, 1.2, hw * 0.75), dark);
            m.cuboid(Vec3::new(-hl * 0.6, 0.8, -hw * 0.7), Vec3::new(hl * 0.1, 1.05, hw * 0.7), c3([0.3, 0.12, 0.08]));
        }
        for sx in [-1.0f32, 1.0] {
            // fenders + running board
            for x in [wheel_x, -wheel_x] {
                m.sphere(Vec3::new(x, wr + 0.12, sx * hw * 0.88), Vec3::new(wr * 1.25, wr * 0.55, 0.18), 10, dark);
            }
            m.cuboid(Vec3::new(-wheel_x, 0.38, sx * hw * 0.8 - 0.1), Vec3::new(wheel_x, 0.43, sx * hw * 0.8 + 0.1), dark);
        }
        m.cyl_z(Vec3::new(-hl * 0.98, 0.85, 0.0), 0.3, 0.1, 12, tyre, dark);
    } else if year < 1960 {
        // rounded 40s/50s sedan with fender curves and chrome grille
        m.sphere(Vec3::new(0.0, 0.68, 0.0), Vec3::new(hl, 0.36, hw), 16, body);
        m.cuboid(Vec3::new(-hl * 0.95, 0.4, -hw * 0.97), Vec3::new(hl * 0.95, 0.8, hw * 0.97), body);
        m.sphere(Vec3::new(-hl * 0.15, 1.0, 0.0), Vec3::new(hl * 0.48, 0.42, hw * 0.88), 14, body);
        m.sphere(Vec3::new(-hl * 0.15, 1.05, 0.0), Vec3::new(hl * 0.44, 0.34, hw * 0.9), 14, glass);
        m.sphere(Vec3::new(-hl * 0.15, 1.12, 0.0), Vec3::new(hl * 0.4, 0.3, hw * 0.8), 14, body);
        m.cuboid(Vec3::new(hl * 0.96, 0.4, -hw * 0.85), Vec3::new(hl + 0.06, 0.52, hw * 0.85), chrome);
        m.cuboid(Vec3::new(-hl - 0.06, 0.4, -hw * 0.85), Vec3::new(-hl * 0.96, 0.52, hw * 0.85), chrome);
        for k in 0..5 {
            m.cuboid(Vec3::new(hl * 0.97, 0.55, -0.35 + k as f32 * 0.17), Vec3::new(hl + 0.02, 0.75, -0.3 + k as f32 * 0.17), chrome);
        }
        if variant == 2 && year >= 1950 {
            // two-tone roof and tail fins
            m.sphere(Vec3::new(-hl * 0.15, 1.18, 0.0), Vec3::new(hl * 0.36, 0.26, hw * 0.75), 12, c3([0.92, 0.9, 0.86]));
            for sx in [-1.0f32, 1.0] {
                m.cuboid(Vec3::new(-hl * 0.98, 0.8, sx * hw * 0.8 - 0.05), Vec3::new(-hl * 0.6, 0.98, sx * hw * 0.8 + 0.05), body);
            }
        }
    } else if year < 1980 {
        // long boxy 60s/70s sedan
        m.cuboid(Vec3::new(-hl, 0.32, -hw), Vec3::new(hl, 0.82, hw), body);
        m.cuboid(Vec3::new(-hl * 0.5, 0.82, -hw * 0.9), Vec3::new(hl * 0.25, 1.3, hw * 0.9), if variant == 1 { dark } else { body });
        m.cuboid(Vec3::new(-hl * 0.48, 0.86, -hw * 0.92), Vec3::new(hl * 0.23, 1.25, hw * 0.92), glass);
        m.cuboid(Vec3::new(hl * 0.98, 0.35, -hw), Vec3::new(hl + 0.06, 0.5, hw), chrome);
        m.cuboid(Vec3::new(-hl - 0.06, 0.35, -hw), Vec3::new(-hl * 0.98, 0.5, hw), chrome);
        m.cuboid(Vec3::new(-hl, 0.6, -hw - 0.01), Vec3::new(hl, 0.63, hw + 0.01), chrome);
    } else {
        // 80s/90s: wedge-shaped with bumpers
        m.cuboid(Vec3::new(-hl, 0.3, -hw), Vec3::new(hl, 0.78, hw), body);
        m.quad(Vec3::new(hl * 0.2, 1.25, -hw * 0.85), Vec3::new(hl * 0.2, 1.25, hw * 0.85), Vec3::new(hl * 0.62, 0.78, hw * 0.95), Vec3::new(hl * 0.62, 0.78, -hw * 0.95), glass);
        m.cuboid(Vec3::new(-hl * 0.55, 0.78, -hw * 0.88), Vec3::new(hl * 0.2, 1.28, hw * 0.88), body);
        m.cuboid(Vec3::new(-hl * 0.53, 0.84, -hw * 0.9), Vec3::new(hl * 0.18, 1.2, hw * 0.9), glass);
        m.cuboid(Vec3::new(hl * 0.96, 0.28, -hw), Vec3::new(hl + 0.08, 0.45, hw), dark);
        m.cuboid(Vec3::new(-hl - 0.08, 0.28, -hw), Vec3::new(-hl * 0.96, 0.45, hw), dark);
    }
    // wheels: tyre + rim/hubcap
    for (x, z) in [(wheel_x, hw * 0.88), (wheel_x, -hw * 0.88), (-wheel_x, hw * 0.88), (-wheel_x, -hw * 0.88)] {
        m.cyl_z(Vec3::new(x, wr, z), wr, 0.12, 14, tyre, tyre);
        let rim = if year < 1935 { c3([0.75, 0.2, 0.15]) } else { chrome };
        m.cyl_z(Vec3::new(x, wr, z + z.signum() * 0.02), wr * 0.55, 0.115, 12, rim, rim);
        if year < 1935 {
            for k in 0..8 {
                let a = k as f32 / 8.0 * std::f32::consts::TAU;
                m.rod(Vec3::new(x, wr, z + z.signum() * 0.12), Vec3::new(x + a.cos() * wr * 0.9, wr + a.sin() * wr * 0.9, z + z.signum() * 0.12), 0.012, c3([0.85, 0.8, 0.7]));
            }
        }
    }
    m
}

fn headlights(year: i32) -> MB {
    let mut g = MB::new();
    let hl = if year < 1935 { 1.8 } else { 2.2 };
    for z in [-0.55, 0.55] {
        g.sphere(Vec3::new(hl, 0.7, z), Vec3::splat(0.12), 6, c3([1.0, 0.95, 0.8]));
    }
    g.cuboid(Vec3::new(-hl - 0.03, 0.55, -0.6), Vec3::new(-hl, 0.7, -0.4), c3([1.0, 0.1, 0.1]));
    g.cuboid(Vec3::new(-hl - 0.03, 0.55, 0.4), Vec3::new(-hl, 0.7, 0.6), c3([1.0, 0.1, 0.1]));
    g
}

/// Horse-drawn wagon: a horse in front, the cart behind (points along +x).
fn wagon_mesh(color: [f32; 3], id: u32) -> MB {
    let mut m = MB::new();
    let wood = c3([0.35, 0.24, 0.15]);
    let wood_d = c3([0.22, 0.14, 0.09]);
    let dark = c3([0.08, 0.06, 0.05]);
    let iron = c3([0.15, 0.15, 0.16]);
    let horse_col = [[0.32, 0.2, 0.12], [0.12, 0.1, 0.09], [0.55, 0.45, 0.35], [0.72, 0.7, 0.66], [0.45, 0.25, 0.12]][(id % 5) as usize];
    let hc = c3(horse_col);
    let mane = c3([horse_col[0] * 0.4, horse_col[1] * 0.4, horse_col[2] * 0.4]);
    let closed = id % 3 == 0;
    // cart bed with plank sides
    m.cuboid(Vec3::new(-1.9, 0.75, -0.78), Vec3::new(0.3, 0.85, 0.78), wood_d);
    for sz in [-0.78f32, 0.72] {
        for k in 0..3 {
            let y = 0.88 + k as f32 * 0.14;
            m.cuboid(Vec3::new(-1.9, y, sz), Vec3::new(0.3, y + 0.1, sz + 0.06), wood);
        }
    }
    m.cuboid(Vec3::new(-1.9, 0.85, -0.78), Vec3::new(-1.84, 1.28, 0.78), wood);
    if closed {
        // covered wagon / delivery van body
        m.cuboid(Vec3::new(-1.85, 0.85, -0.74), Vec3::new(0.2, 2.0, 0.74), c3(color));
        m.cuboid(Vec3::new(-1.9, 2.0, -0.8), Vec3::new(0.3, 2.08, 0.8), wood_d);
    } else {
        // load: crates, sacks or barrels
        for i in 0..3 {
            let x = -1.6 + i as f32 * 0.6;
            match (id + i) % 3 {
                0 => m.cuboid(Vec3::new(x, 0.85, -0.5), Vec3::new(x + 0.5, 1.3, 0.1), c3([0.5, 0.38, 0.22])),
                1 => m.sphere(Vec3::new(x + 0.25, 1.05, 0.25), Vec3::new(0.25, 0.2, 0.3), 8, c3([0.75, 0.68, 0.5])),
                _ => m.cylinder(Vec3::new(x + 0.25, 0.85, -0.2), 0.22, 0.5, 10, c3([0.35, 0.22, 0.14])),
            }
        }
    }
    // driver's bench
    m.cuboid(Vec3::new(0.0, 1.25, -0.6), Vec3::new(0.3, 1.32, 0.6), wood);
    // spoked wheels
    for (x, z, r) in [(-1.3f32, 0.86f32, 0.5f32), (-1.3, -0.86, 0.5), (0.05, 0.86, 0.42), (0.05, -0.86, 0.42)] {
        m.cyl_z(Vec3::new(x, r, z), r, 0.04, 16, iron, wood);
        m.cyl_z(Vec3::new(x, r, z), r * 0.88, 0.035, 16, wood_d, wood_d);
        m.cyl_z(Vec3::new(x, r, z), 0.08, 0.08, 8, wood, wood);
        for k in 0..10 {
            let a = k as f32 / 10.0 * std::f32::consts::TAU;
            m.rod(Vec3::new(x, r, z), Vec3::new(x + a.cos() * r * 0.88, r + a.sin() * r * 0.88, z), 0.02, wood);
        }
    }
    // shafts
    m.rod(Vec3::new(0.3, 0.9, -0.42), Vec3::new(2.1, 1.25, -0.36), 0.03, wood);
    m.rod(Vec3::new(0.3, 0.9, 0.42), Vec3::new(2.1, 1.25, 0.36), 0.03, wood);
    let _ = (hc, mane);
    m
}

fn lantern_mesh() -> MB {
    let mut g = MB::new();
    g.cuboid(Vec3::new(0.2, 1.3, 0.7), Vec3::new(0.35, 1.5, 0.85), c3([1.0, 0.8, 0.5]));
    g
}

pub fn car_value(year: i32) -> i32 {
    if year < 1935 {
        120
    } else if year < 1960 {
        180
    } else {
        260
    }
}

/// Spawn parked and moving cars for the current city.
pub fn spawn_cars(mut cars: ResMut<Cars>, game: Res<Game>, map: Option<Res<CityMap>>, mut c: Commands, mut meshes: ResMut<Assets<Mesh>>, mats: Option<Res<Mats>>, old: Query<Entity, With<CarVis>>, models: Option<Res<crate::render::models::Models>>) {
    let Some(map) = map else { return };
    let Some(mats) = mats else { return };
    let key = (game.city, game.year, game.timeline);
    if cars.spawned_for == Some(key) {
        return;
    }
    for e in old.iter() {
        c.entity(e).despawn();
    }
    cars.list.clear();
    cars.spawned_for = Some(key);
    if game.phase != crate::state::Phase::City {
        // no traffic in the prologue apartment, the lab or the limbo
        return;
    }
    if game.year < 1915 {
        return;
    }
    let m = &map.0;
    let mut rng = crate::util::Rng::new(game.year as u64 * 13 + game.timeline as u64);
    let horse_p = crate::city::style::style(game.city, game.year).horses;
    let palette: &[[f32; 3]] = if game.year < 1950 {
        &[[0.05, 0.05, 0.06], [0.12, 0.1, 0.09], [0.2, 0.08, 0.07], [0.08, 0.12, 0.1], [0.15, 0.15, 0.2]]
    } else if game.year < 1976 {
        &[[0.55, 0.1, 0.08], [0.15, 0.3, 0.5], [0.7, 0.6, 0.3], [0.2, 0.4, 0.25], [0.8, 0.78, 0.7], [0.1, 0.1, 0.1]]
    } else {
        &[[0.6, 0.6, 0.62], [0.1, 0.1, 0.12], [0.5, 0.08, 0.08], [0.2, 0.25, 0.4], [0.85, 0.85, 0.8]]
    };
    let n_parked = if game.year < 1935 { 14 } else { 26 };
    let mut id = 0u32;
    // parked along the road next to sidewalks
    let mut tries = 0;
    let people = game.pop.in_city(game.city, game.year);
    while (cars.list.len() as u32) < n_parked && tries < 4000 {
        tries += 1;
        let x = rng.range(crate::city::gen::OUT, m.w - 4);
        let y = rng.range(crate::city::gen::RAIL, m.h - 4);
        if m.get(x, y) != Tile::Road {
            continue;
        }
        let horiz = m.get(x, y - 1) == Tile::Sidewalk || m.get(x, y + 1) == Tile::Sidewalk;
        let vert = m.get(x - 1, y) == Tile::Sidewalk || m.get(x + 1, y) == Tile::Sidewalk;
        if !(horiz ^ vert) {
            continue;
        }
        let pos = tile_center(x, y);
        if cars.list.iter().any(|c| c.pos.distance(pos) < 5.0) {
            continue;
        }
        let owner = if people.is_empty() { None } else { Some(people[rng.idx(people.len())]) };
        cars.list.push(Car {
            id,
            pos,
            heading: if horiz { 0.0 } else { std::f32::consts::FRAC_PI_2 },
            speed: 0.0,
            driver: None,
            owner,
            owner_elias: false,
            locked: rng.chance(0.75),
            stolen: false,
            glass_broken: false,
            color: *rng.pick(palette),
            value: car_value(game.year),
            entity: None,
            route: Vec::new(),
            ri: 0,
            player: false,
            wait: 0.0,
            horse: rng.chance(horse_p),
        });
        id += 1;
    }
    // a few cars driving around the block grid
    let n_moving = if game.year < 1935 { 4 } else { 9 };
    let drivers: Vec<Pid> = people.iter().copied().filter(|p| game.pop.get(*p).job == crate::sim::people::Job::Driver || game.pop.get(*p).job.income() >= 7).collect();
    for k in 0..n_moving {
        let g = crate::city::gen::grid_of(game.city, game.year);
        let bx = rng.range(0, g.bx);
        let by = rng.range(0, g.by);
        let x0 = (g.x0 + bx * g.px) as f32 + 1.5;
        let y0 = (g.y0 + by * g.py) as f32 + 1.5;
        let x1 = x0 + g.px as f32;
        let y1 = y0 + g.py as f32;
        let route = vec![Vec2::new(x0, y0), Vec2::new(x1, y0), Vec2::new(x1, y1), Vec2::new(x0, y1)];
        let driver = drivers.get(k as usize).copied();
        cars.list.push(Car {
            id,
            pos: route[0],
            heading: 0.0,
            speed: 0.0,
            driver,
            owner: driver,
            owner_elias: false,
            locked: true,
            stolen: false,
            glass_broken: false,
            color: *rng.pick(palette),
            value: car_value(game.year),
            entity: None,
            route,
            ri: 1,
            player: false,
            wait: 0.0,
            horse: rng.chance(horse_p),
        });
        id += 1;
    }
    for car in cars.list.iter_mut() {
        if car.horse {
            car.value = car.value / 3;
        }
        let horse = car.horse;
        let cid = car.id;
        let e = if horse {
            let mb = wagon_mesh(car.color, car.id);
            let g = lantern_mesh();
            c.spawn((Mesh3d(meshes.add(mb.build())), MeshMaterial3d(mats.plain.clone()), Transform::from_xyz(car.pos.x, 0.0, car.pos.y), CarVis(car.id)))
                .with_children(|p| {
                    p.spawn((Mesh3d(meshes.add(g.build())), MeshMaterial3d(mats.glow.clone()), Transform::default()));
                })
                .id()
        } else {
            // period car split by material: clear-coated paint, chrome, glass, rubber, lamps
            let geo = crate::render::cars::build_car(game.year, car.id, game.city);
            let parts = [
                (geo.paint, mats.car_paint.clone()),
                (geo.chrome, mats.chrome.clone()),
                (geo.glass, mats.glass_dark.clone()),
                (geo.dark, mats.plain.clone()),
                (geo.lights, mats.lamp_unlit.clone()),
            ];
            let root = c.spawn((Transform::from_xyz(car.pos.x, 0.0, car.pos.y), Visibility::default(), CarVis(car.id))).id();
            for (mb, mat) in parts {
                if mb.is_empty() {
                    continue;
                }
                let ch = c.spawn((Mesh3d(meshes.add(mb.build())), MeshMaterial3d(mat), Transform::default())).id();
                c.entity(root).add_child(ch);
            }
            root
        };
        if horse {
            let h = crate::render::horse::spawn_horse(&mut c, &mut meshes, &mats.skin, cid.wrapping_mul(2654435761) >> 7, cid);
            c.entity(h).insert(Transform::from_xyz(1.55, 0.0, 0.0));
            c.entity(e).add_child(h);
        }
        car.entity = Some(e);
    }
    let _ = &models;
}

#[allow(clippy::too_many_arguments)]
pub fn drive_cars(
    time: Res<Time>,
    mut cars: ResMut<Cars>,
    mut game: ResMut<Game>,
    act: Res<Act>,
    map: Option<Res<CityMap>>,
    mut rt: ResMut<PlayerRt>,
    mut q: Query<&mut Transform, With<CarVis>>,
    mut sim: ResMut<Sim>,
    ui: Res<UiState>,
    mut crimes: EventWriter<CrimeEv>,
    mut sfx: EventWriter<Sfx>,
    cam: Res<crate::camera::CamState>,
) {
    let Some(map) = map else { return };
    let m = &map.0;
    let dt = time.delta_secs().min(0.1);
    rt.car_grace = (rt.car_grace - dt).max(0.0);
    // never stay "inside" a car that no longer exists (city change, load...)
    if rt.in_car && !cars.list.iter().any(|c| c.player) {
        rt.in_car = false;
    }
    if let Ok(mut v) = UNLOCK.lock() {
        for id in v.drain(..) {
            if let Some(c) = cars.list.iter_mut().find(|c| c.id == id) {
                c.locked = false;
                c.stolen = true;
            }
        }
    }
    if ui.pauses_world() {
        return;
    }
    let pp = game.player.pos;
    for car in cars.list.iter_mut() {
        if car.player {
            // Elias drives
            if !ui.blocks_input() {
                let mut thr = 0.0;
                let mut steer = 0.0;
                // screen-relative controls rotated by camera yaw
                if act.held(Action::Up) {
                    thr += 1.0;
                }
                if act.held(Action::Down) {
                    thr -= 0.6;
                }
                if act.held(Action::Left) {
                    steer -= 1.0;
                }
                if act.held(Action::Right) {
                    steer += 1.0;
                }
                let max = if car.horse { 6.0 } else if game.year < 1935 { 9.0 } else { 15.0 } * if act.held(Action::Run) { 1.3 } else { 1.0 };
                car.speed += thr * 7.0 * dt;
                car.speed *= 1.0 - dt * 0.9;
                car.speed = car.speed.clamp(-4.0, max);
                car.heading += steer * dt * 2.2 * (car.speed / 6.0).clamp(-1.0, 1.0);
                let _ = cam;
                if act.just(Action::Interact) && rt.car_grace <= 0.0 {
                    // get out
                    car.player = false;
                    car.speed = 0.0;
                    rt.in_car = false;
                    let side = Vec2::new(-car.heading.sin(), car.heading.cos()) * 1.4;
                    game.player.pos = m.nearest_open(car.pos + side);
                    continue;
                }
            }
            let dir = Vec2::new(car.heading.cos(), car.heading.sin());
            let np = car.pos + dir * car.speed * dt;
            let (tx, ty) = to_tile(np + dir * 1.8 * car.speed.signum());
            if m.blocked(tx, ty) || matches!(m.get(tx, ty), Tile::Water | Tile::Floor) {
                if car.speed.abs() > 6.0 {
                    sfx.write(Sfx::Punch);
                }
                car.speed = -car.speed * 0.2;
            } else {
                car.pos = np;
            }
            game.player.pos = car.pos;
            // run people over
            if car.speed.abs() > 4.0 {
                for a in sim.agents.iter_mut() {
                    if a.active() && a.pos.distance(car.pos) < 1.1 {
                        a.health -= car.speed.abs() * 8.0;
                        let pid = a.pid;
                        a.pos += dir * 1.5;
                        if a.health <= 0.0 {
                            a.state = AState::Dead;
                            crimes.write(CrimeEv { kind: CrimeKind::Murder, pos: a.pos, victim: Some(pid), noise: 12.0, weapon: None });
                        } else {
                            a.state = AState::Flee { from: car.pos, until: game.abs_minute() + 30.0 };
                            crimes.write(CrimeEv { kind: CrimeKind::Assault, pos: a.pos, victim: Some(pid), noise: 10.0, weapon: None });
                        }
                        sfx.write(Sfx::Punch);
                    }
                }
            }
        } else if car.driver.is_some() && !car.route.is_empty() {
            // NPC traffic: follow the block loop, stop for pedestrians and Elias
            let target = car.route[car.ri % car.route.len()];
            let d = target - car.pos;
            if d.length() < 0.6 {
                car.ri = (car.ri + 1) % car.route.len();
            }
            let want = d.y.atan2(d.x);
            let mut da = want - car.heading;
            while da > std::f32::consts::PI {
                da -= std::f32::consts::TAU;
            }
            while da < -std::f32::consts::PI {
                da += std::f32::consts::TAU;
            }
            car.heading += da.clamp(-dt * 3.0, dt * 3.0);
            let dir = Vec2::new(car.heading.cos(), car.heading.sin());
            let ahead = car.pos + dir * 2.6;
            let blocked = pp.distance(ahead) < 1.6 || sim.agents.iter().any(|a| a.active() && a.pos.distance(ahead) < 1.3);
            let target_speed = if blocked { 0.0 } else if da.abs() > 0.4 { 2.0 } else if car.horse { 3.2 } else if game.year < 1935 { 5.0 } else { 7.0 };
            car.speed += (target_speed - car.speed) * (dt * 2.0).min(1.0);
            car.pos += dir * car.speed * dt;
        }
        if let Some(d) = car.driver {
            if let Some(a) = sim.agent_mut(d) {
                a.pos = car.pos;
                a.in_car = true;
                a.pinned = Some(car.pos);
            }
        }
        if let Some(e) = car.entity {
            if let Ok(mut tr) = q.get_mut(e) {
                tr.translation = Vec3::new(car.pos.x, 0.0, car.pos.y);
                tr.rotation = Quat::from_rotation_y(-car.heading);
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn car_interact(id: u32, game: &mut Game, sim: &mut Sim, rt: &mut PlayerRt, choices: &mut Choices, ui: &mut UiState, toasts: &mut Toasts, crimes: &mut EventWriter<CrimeEv>, sfx: &mut EventWriter<Sfx>) {
    let _ = (sim, crimes, sfx);
    CAR_CTX.with(|c| *c.borrow_mut() = Some(id));
    let armed = game.player.weapons().len() > 1;
    let has_pick = game.player.has(&Item::Tool(Tool::Lockpick));
    let info = CAR_INFO.with(|c| c.borrow().iter().find(|x| x.0 == id).copied());
    let (has_driver, locked, mine) = info.map(|i| (i.1, i.2, i.3)).unwrap_or((false, true, false));
    if mine || (!locked && !has_driver) {
        CAR_ENTER.with(|c| *c.borrow_mut() = Some(id));
        toasts.push(format!("[{}] para sair. WASD dirige, Shift acelera.", "E"));
        return;
    }
    let mut opts = vec![];
    if has_driver {
        if armed {
            opts.push(("car_jack_gun".into(), "Apontar a arma e mandar sair".into()));
        }
        opts.push(("car_jack".into(), "Arrancar o motorista à força".into()));
    } else {
        if has_pick {
            opts.push(("car_pick".into(), "Abrir a fechadura com gazuas".into()));
        }
        opts.push(("car_glass".into(), "Quebrar o vidro (barulho)".into()));
    }
    opts.push(("cancel".into(), "Deixar".into()));
    choices.cur = Some(Choice { title: "Carro".into(), body: if has_driver { "O motorista olha para você pelo vidro.".into() } else { "Trancado.".into() }, opts, ctx: id.to_string() });
    ui.open(Mode::Choice);
}

static CAR_CTX: crate::util::Shared<Option<u32>> = crate::util::Shared::new(None);
static CAR_ENTER: crate::util::Shared<Option<u32>> = crate::util::Shared::new(None);
static CAR_ACTION: crate::util::Shared<Vec<(u32, String)>> = crate::util::Shared::new(Vec::new());
/// (id, has_driver, locked, mine)
static CAR_INFO: crate::util::Shared<Vec<(u32, bool, bool, bool)>> = crate::util::Shared::new(Vec::new());

#[allow(clippy::too_many_arguments)]
pub fn handle_car_choice(id: &str, ctx: &str, game: &mut Game, sim: &mut Sim, rt: &mut PlayerRt, toasts: &mut Toasts, crimes: &mut EventWriter<CrimeEv>, sfx: &mut EventWriter<Sfx>, decal: &mut EventWriter<SpawnDecal>, lock: &mut Lockpick, ui: &mut UiState) -> bool {
    if !id.starts_with("car_") {
        return false;
    }
    let cid: u32 = ctx.parse().unwrap_or(0);
    let pp = game.player.pos;
    match id {
        "car_pick" => {
            *lock = Lockpick { active: true, pins: 3, done: 0, pos: 0.0, dir: 1.0, zone: (0.45, 0.6), fails: 0, ctx: format!("car:{}", cid), result: None, era_label: "Fechadura do carro".into() };
            ui.open(Mode::Minigame);
            return true;
        }
        "car_glass" => {
            sfx.write(Sfx::Glass);
            decal.write(SpawnDecal { kind: DecalKind::Glass, pos: pp, crime: None });
            crimes.write(CrimeEv { kind: CrimeKind::CarTheft, pos: pp, victim: None, noise: 14.0, weapon: None });
            CAR_ACTION.with(|c| c.borrow_mut().push((cid, "glass".into())));
            toasts.push("Vidro quebrado. O carro é seu — por enquanto.");
        }
        "car_jack_gun" | "car_jack" => {
            CAR_ACTION.with(|c| c.borrow_mut().push((cid, id.to_string())));
            let _ = (sim, rt);
            crimes.write(CrimeEv { kind: CrimeKind::CarTheft, pos: pp, victim: None, noise: 8.0, weapon: if id == "car_jack_gun" { Some(game.player.weapon) } else { None } });
        }
        _ => {}
    }
    game.stat("car_thefts", 1);
    ui.close();
    true
}

/// Apply thread-local car actions coming from the interaction/choice systems.
pub fn car_actions(mut cars: ResMut<Cars>, mut sim: ResMut<Sim>, mut game: ResMut<Game>, mut rt: ResMut<PlayerRt>, map: Option<Res<CityMap>>, mut toasts: ResMut<Toasts>) {
    // publish info for the interaction code
    let info: Vec<(u32, bool, bool, bool)> = cars.list.iter().map(|c| (c.id, c.driver.is_some(), c.locked && !c.glass_broken, c.owner_elias)).collect();
    CAR_INFO.with(|c| *c.borrow_mut() = info);
    if let Some(id) = CAR_ENTER.with(|c| c.borrow_mut().take()) {
        for c in cars.list.iter_mut() {
            c.player = c.id == id;
        }
        rt.in_car = true;
        rt.car_grace = 0.35;
    }
    let actions: Vec<(u32, String)> = CAR_ACTION.with(|c| c.borrow_mut().drain(..).collect());
    let Some(map) = map else { return };
    let now = game.abs_minute();
    for (id, a) in actions {
        let Some(car) = cars.list.iter_mut().find(|c| c.id == id) else { continue };
        match a.as_str() {
            "glass" => {
                car.glass_broken = true;
                car.locked = false;
                car.stolen = true;
            }
            "car_jack_gun" | "car_jack" => {
                if let Some(d) = car.driver.take() {
                    let p = game.pop.get(d).clone();
                    let out = map.0.nearest_open(car.pos + Vec2::new(0.0, 1.5));
                    if let Some(ag) = sim.agent_mut(d) {
                        ag.pos = out;
                        ag.in_car = false;
                        ag.pinned = None;
                        let brave = p.traits.courage as i32 - p.traits.fear as i32 + if a == "car_jack_gun" { -60 } else { 0 };
                        if brave > 20 {
                            ag.state = AState::Hostile;
                            ag.say("Tira a mão do meu carro!", 2.5);
                        } else if p.traits.morality > 40 {
                            ag.state = AState::Flee { from: car.pos, until: now + 25.0 };
                            ag.say("Ladrão! Ladrão!", 2.5);
                            // the CarTheft crime gets the next id when it is processed
                            ag.knows_bodies.push(u32::MAX - game.police.next_id);
                        } else {
                            ag.state = AState::Cower { until: now + 10.0 };
                            ag.say("Leva! Leva, não atira!", 2.5);
                        }
                    }
                    car.route.clear();
                    car.speed = 0.0;
                    car.stolen = true;
                    car.locked = false;
                    toasts.push("O carro está livre.");
                }
            }
            _ => {}
        }
    }
    let _ = &mut game;
}

/// Sell the nearest stolen car at a chop shop (smuggler at the docks/warehouse).
pub fn sell_nearest_car(cars: &mut Cars, game: &mut Game, c: &mut Commands) -> Option<i32> {
    let pp = game.player.pos;
    let idx = cars.list.iter().position(|car| car.pos.distance(pp) < 14.0 && (car.stolen || car.owner_elias) && !car.player)?;
    let car = cars.list.remove(idx);
    if let Some(e) = car.entity {
        c.entity(e).despawn();
    }
    let v = crate::economy::price(game, (car.value as f32 * if car.glass_broken { 0.5 } else { 0.7 }) as i32);
    game.player.money += v;
    Some(v)
}
