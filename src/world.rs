//! Loading a city into the running game: map, population, agents, visuals;
//! and per-frame sync of the simulation with the 3D scene.

use crate::city::gen;
use crate::city::map::*;
use crate::render::character::{spawn_character, Pose, Rig};
use crate::render::city3d::{spawn_city, CityRoot, CityVis, DoorPanel, Mats};
use crate::sim::agents::{AState, Agent, Sim};
use crate::sim::path::Pather;
use crate::state::Game;
use crate::util::Rng;
use bevy::prelude::*;

#[derive(Resource)]
pub struct CityMap(pub Map);

#[derive(Component)]
pub struct AgentVis {
    pub idx: usize,
}

#[derive(Component)]
pub struct AgentRoot;

/// Request to (re)load the current city of `Game`.
#[derive(Resource, Default)]
pub struct LoadCity(pub bool);

/// Agents to spawn mid-game (police reinforcements, etc.): (count, near)
#[derive(Resource, Default)]
pub struct SpawnAgents {
    pub police: Vec<(usize, Vec2)>,
}

pub fn build_map(game: &Game) -> Map {
    if game.phase == crate::state::Phase::Prologue {
        if game.flag("prologue_apartment") {
            return crate::narrative::apartment_map();
        }
        if game.flag("prologue_lab") {
            return crate::narrative::lab_map();
        }
        return crate::narrative::limbo_map(game.limbo_stage);
    }
    let mut m = gen::generate(game.city, game.year);
    // buildings closed by timeline alterations
    for (c, b, y) in &game.closed {
        if *c == game.city && *y <= game.year && *b < m.buildings.len() {
            m.buildings[*b].closed_forever = true;
            m.buildings[*b].kind = BKind::Abandoned;
            m.buildings[*b].name = "Prédio Fechado".into();
        }
    }
    m
}

/// Make sure the city has people for this year; spawn agents at plausible places.
pub fn populate(game: &mut Game, m: &mut Map, sim: &mut Sim) {
    let mut rng = Rng::new(game.seed ^ (game.year as u64 * 7919) ^ game.city as u64);
    if !game.populated.iter().any(|(c, _)| *c == game.city) {
        game.pop.generate_city(&mut rng, m, game.city, game.year, 190);
        game.populated.push((game.city, game.year));
    } else {
        // time has passed since the last visit: let lives unfold
        let last = game.populated.iter().filter(|(c, _)| *c == game.city).map(|(_, y)| *y).max().unwrap_or(game.year);
        if last < game.year {
            let mut log = Vec::new();
            let (city, year) = (game.city, game.year);
            game.pop.advance_years(&mut rng, city, last, year, &mut log);
            for l in log {
                game.news_ticker.push(l);
            }
            // keep the city lively: new families moved in
            let alive = game.pop.in_city(city, year).len();
            if alive < 170 {
                game.pop.generate_city(&mut rng, m, city, year, 190);
            }
            game.populated.push((city, year));
        }
        game.pop.assign_to_map(&mut rng, m, game.city, game.year);
    }
    sim.agents.clear();
    let ids = game.pop.in_city(game.city, game.year);
    for id in ids {
        let p = game.pop.get(id);
        let pos = p.home.map(|h| crate::sim::agents::random_floor(m, h, &mut rng)).unwrap_or(m.spawn);
        let mut a = Agent::new(id, pos, p.job.armed());
        a.replan_at = -1.0;
        sim.agents.push(a);
    }
    sim.rebuild_index();
    sim.pather = Some(Pather::new(m));
    sim.rng = Some(Rng::new(rng.next()));
}

#[allow(clippy::too_many_arguments)]
pub fn load_city_system(
    mut req: ResMut<LoadCity>,
    mut c: Commands,
    mut game: ResMut<Game>,
    mut sim: ResMut<Sim>,
    mut meshes: ResMut<Assets<Mesh>>,
    mats: Res<Mats>,
    mut vis: ResMut<CityVis>,
    db: Res<crate::cases::run::CaseDb>,
    roots: Query<Entity, With<CityRoot>>,
    agents: Query<Entity, With<AgentRoot>>,
) {
    if !req.0 {
        return;
    }
    req.0 = false;
    for e in roots.iter() {
        c.entity(e).despawn();
    }
    for e in agents.iter() {
        c.entity(e).despawn();
    }
    let mut m = build_map(&game);
    if game.phase == crate::state::Phase::City {
        crate::cases::run::prepare_case(&mut game, &mut m, &db);
        populate(&mut game, &mut m, &mut sim);
        // Elias rents a room the first time he is in a city
        if game.player.safehouse.map(|s| s >= m.buildings.len()).unwrap_or(true) {
            let rooms = m.buildings_of(BKind::Apartment);
            if !rooms.is_empty() {
                let pick = rooms[(game.case_idx * 7 + game.year as usize) % rooms.len()];
                game.player.safehouse = Some(pick);
                if !m.buildings[pick].spots.iter().any(|s| s.kind == SpotKind::Bed) {
                    let c = m.buildings[pick].center_px();
                    let (x, y) = to_tile(c);
                    m.add_prop(PKind::Bed, x, y, 1, 2, Some(pick));
                    m.rebuild_prop_index();
                }
            }
        }
    } else {
        sim.agents.clear();
        sim.rebuild_index();
        sim.pather = Some(Pather::new(&m));
    }
    spawn_city(&mut c, &mut meshes, &mats, &m, game.year, &mut vis);
    // agent visuals
    for (i, a) in sim.agents.iter().enumerate() {
        let look = game.pop.get(a.pid).look.clone();
        let (root, rig) = spawn_character(&mut c, &mut meshes, &mats.plain, &look);
        c.entity(root).insert((AgentVis { idx: i }, AgentRoot, rig, Transform::from_xyz(a.pos.x, 0.0, a.pos.y)));
    }
    if game.player.pos == Vec2::ZERO || !m.inb(game.player.pos.x as i32, game.player.pos.y as i32) || m.blocked(game.player.pos.x as i32, game.player.pos.y as i32) {
        game.player.pos = m.spawn;
        // start next to the crime scene when arriving for a case
        if game.phase == crate::state::Phase::City {
            if let Some(prog) = game.cases.iter().find(|c| c.id == game.case_idx as u8) {
                if let Some(Some((x, y))) = prog.clue_pos.first() {
                    let near = Vec2::new(*x, *y);
                    if let Some(b) = m.building_at(near) {
                        game.player.pos = m.buildings[b].outside_px();
                    }
                }
            }
        }
    }
    c.insert_resource(CityMap(m));
}

/// Floor height under a position (sidewalks and floors are raised).
pub fn ground_y(m: &Map, p: Vec2) -> f32 {
    let (x, y) = to_tile(p);
    match m.get(x, y) {
        Tile::Sidewalk | Tile::Plaza => 0.12,
        Tile::Floor | Tile::Door => 0.1,
        Tile::Dock => 0.15,
        _ => 0.02,
    }
}

pub fn sync_agents(
    sim: Res<Sim>,
    map: Option<Res<CityMap>>,
    vis: Res<CityVis>,
    mut q: Query<(&AgentVis, &mut Transform, &mut Rig, &mut Visibility)>,
    time: Res<Time>,
) {
    let Some(map) = map else { return };
    let m = &map.0;
    let dt = time.delta_secs();
    for (av, mut tr, mut rig, mut v) in q.iter_mut() {
        let Some(a) = sim.agents.get(av.idx) else { continue };
        let gy = ground_y(m, a.pos) + a.y;
        let target = Vec3::new(a.pos.x, gy, a.pos.y);
        tr.translation = target;
        let want = Quat::from_rotation_y(-a.facing - std::f32::consts::FRAC_PI_2);
        tr.rotation = tr.rotation.slerp(want, (dt * 10.0).min(1.0));
        rig.pose = a.pose;
        rig.speed = a.speed;
        // hidden when inside a building whose roof is on
        let inside = m.building_at(a.pos);
        let hidden = match inside {
            Some(b) => vis.buildings.get(b).map(|bv| !bv.cut).unwrap_or(false),
            None => false,
        } || matches!(a.state, AState::Carried) && false
            || a.in_car;
        *v = if hidden { Visibility::Hidden } else { Visibility::Inherited };
        if matches!(a.state, AState::Dead) {
            rig.pose = Pose::Dead;
        }
    }
}

pub fn animate_doors(sim: Res<Sim>, game: Res<Game>, mut q: Query<(&mut DoorPanel, &mut Transform)>, time: Res<Time>) {
    let dt = time.delta_secs();
    let pp = game.player.pos;
    for (mut d, mut tr) in q.iter_mut() {
        let c = tile_center(d.tile.0, d.tile.1);
        let near = pp.distance_squared(c) < 1.4 || sim.agents.iter().any(|a| a.pos.distance_squared(c) < 1.2);
        let target = if near { 1.0 } else { 0.0 };
        d.open += (target - d.open) * (dt * 6.0).min(1.0);
        let ang = d.open * 1.5;
        tr.rotation = if d.axis_x { Quat::from_rotation_y(ang) } else { Quat::from_rotation_y(-ang) };
    }
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_agents_system(
    mut req: ResMut<SpawnAgents>,
    mut c: Commands,
    mut game: ResMut<Game>,
    mut sim: ResMut<Sim>,
    map: Option<Res<CityMap>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mats: Res<Mats>,
) {
    let Some(map) = map else { return };
    let m = &map.0;
    let reqs: Vec<(usize, Vec2)> = req.police.drain(..).collect();
    for (n, near) in reqs {
        let mut rng = Rng::new(game.abs_minute() as u64 ^ 0xC0B);
        let start = m.find_building(BKind::Police).map(|b| m.buildings[b].outside_px()).unwrap_or(near);
        for k in 0..n {
            let (city, year) = (game.city, game.year);
            let birth = year - rng.range(24, 45);
            let pid = game.pop.new_person(&mut rng, city, false, birth, None, year);
            let p = game.pop.get_mut(pid);
            p.job = crate::sim::people::Job::Police;
            p.traits.courage = 80;
            let age = p.age(year);
            p.look = crate::sim::people::era_look(&mut rng, false, age, crate::sim::people::Job::Police, year, city);
            let pos = m.nearest_open(start + Vec2::new(k as f32 * 0.8, 0.5));
            let mut a = Agent::new(pid, pos, true);
            a.state = AState::Investigate { at: near, until: game.abs_minute() + 20.0 };
            let idx = sim.agents.len();
            sim.agents.push(a);
            sim.by_pid.insert(pid, idx);
            let look = game.pop.get(pid).look.clone();
            let (root, rig) = spawn_character(&mut c, &mut meshes, &mats.plain, &look);
            c.entity(root).insert((AgentVis { idx }, AgentRoot, rig, Transform::from_xyz(pos.x, 0.0, pos.y)));
        }
    }
}
