//! Street pigeons: small flocks pecking on sidewalks and squares by day.
//! They scatter, flapping, when Elias (or anyone running) gets close, and
//! land again somewhere else a little later.

use super::mesh::{c3, MB};
use crate::city::map::*;
use crate::world::CityMap;
use bevy::prelude::*;

#[derive(Component)]
pub struct Pigeon {
    home: Vec2,
    vel: Vec3,
    flying: f32,
    phase: f32,
    wing_l: Entity,
    wing_r: Entity,
    head: Entity,
}

#[derive(Resource, Default)]
pub struct Birds {
    spawned_for: Option<(crate::city::gen::CityId, i32, u32)>,
}

fn pigeon_mesh(seed: u32) -> (MB, MB, MB) {
    let tone = [[0.55, 0.56, 0.6], [0.4, 0.4, 0.45], [0.62, 0.6, 0.58], [0.3, 0.3, 0.32]][(seed % 4) as usize];
    let mut body = MB::new();
    body.sphere(Vec3::new(0.0, 0.12, 0.0), Vec3::new(0.1, 0.07, 0.06), 10, c3(tone));
    body.sphere(Vec3::new(-0.1, 0.13, 0.0), Vec3::new(0.06, 0.02, 0.04), 8, c3([tone[0] * 0.7, tone[1] * 0.7, tone[2] * 0.7]));
    body.rod(Vec3::new(0.02, 0.06, 0.02), Vec3::new(0.02, 0.0, 0.02), 0.006, c3([0.8, 0.3, 0.3]));
    body.rod(Vec3::new(0.02, 0.06, -0.02), Vec3::new(0.02, 0.0, -0.02), 0.006, c3([0.8, 0.3, 0.3]));
    let mut head = MB::new();
    head.sphere(Vec3::ZERO, Vec3::splat(0.035), 8, c3([0.35, 0.4, 0.45]));
    head.sphere(Vec3::new(0.0, -0.03, 0.0), Vec3::new(0.03, 0.02, 0.03), 6, c3([0.3, 0.45, 0.4]));
    head.frustum(Vec3::new(0.03, 0.0, 0.0), 0.008, 0.001, 0.02, 4, c3([0.2, 0.18, 0.15]));
    let mut wing = MB::new();
    wing.sphere(Vec3::new(-0.02, 0.0, 0.07), Vec3::new(0.07, 0.01, 0.08), 8, c3([tone[0] * 0.85, tone[1] * 0.85, tone[2] * 0.85]));
    (body, head, wing)
}

pub fn spawn_birds(
    mut c: Commands,
    game: Res<crate::state::Game>,
    map: Option<Res<CityMap>>,
    mut birds: ResMut<Birds>,
    old: Query<Entity, With<Pigeon>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mats: Option<Res<super::city3d::Mats>>,
) {
    let (Some(map), Some(mats)) = (map, mats) else { return };
    let key = (game.city, game.year, game.timeline);
    if birds.spawned_for == Some(key) {
        return;
    }
    birds.spawned_for = Some(key);
    for e in old.iter() {
        c.entity(e).despawn();
    }
    if game.phase != crate::state::Phase::City {
        return;
    }
    let m = &map.0;
    let mut rng = crate::util::Rng::new(game.year as u64 * 31 + game.city as u64);
    let mut flocks = 0;
    for _ in 0..4000 {
        if flocks >= 14 {
            break;
        }
        let (x, y) = (rng.range(0, m.w), rng.range(0, m.h));
        if !matches!(m.get(x, y), Tile::Plaza | Tile::Sidewalk | Tile::Grass) || m.blocked(x, y) {
            continue;
        }
        flocks += 1;
        let n = 3 + rng.range(0, 5);
        for k in 0..n {
            let seed = (x * 131 + y * 17 + k) as u32;
            let (b, h, w) = pigeon_mesh(seed);
            let home = tile_center(x, y) + Vec2::new(rng.rangef(-0.9, 0.9), rng.rangef(-0.9, 0.9));
            let hm = meshes.add(h.build());
            let wm = meshes.add(w.build());
            let head = c.spawn((Mesh3d(hm), MeshMaterial3d(mats.plain.clone()), Transform::from_xyz(0.08, 0.2, 0.0))).id();
            let wing_l = c.spawn((Mesh3d(wm.clone()), MeshMaterial3d(mats.plain.clone()), Transform::from_xyz(0.0, 0.15, 0.02))).id();
            let wing_r = c.spawn((Mesh3d(wm), MeshMaterial3d(mats.plain.clone()), Transform::from_xyz(0.0, 0.15, -0.02).with_scale(Vec3::new(1.0, 1.0, -1.0)))).id();
            let root = c
                .spawn((
                    Mesh3d(meshes.add(b.build())),
                    MeshMaterial3d(mats.plain.clone()),
                    Transform::from_xyz(home.x, 0.13, home.y).with_rotation(Quat::from_rotation_y(rng.rangef(0.0, 6.28))),
                    Pigeon { home, vel: Vec3::ZERO, flying: 0.0, phase: rng.rangef(0.0, 6.0), wing_l, wing_r, head },
                ))
                .id();
            c.entity(root).add_children(&[head, wing_l, wing_r]);
        }
    }
}

pub fn animate_birds(
    time: Res<Time>,
    game: Res<crate::state::Game>,
    rt: Res<crate::player::PlayerRt>,
    sim: Res<crate::sim::agents::Sim>,
    env: Res<crate::env::EnvState>,
    mut q: Query<(&mut Pigeon, &mut Transform, &mut Visibility)>,
    mut parts: Query<&mut Transform, Without<Pigeon>>,
) {
    let dt = time.delta_secs().min(0.05);
    let t = time.elapsed_secs();
    let night = env.darkness > 0.6 || game.rain > 0.6;
    let pp = game.player.pos;
    for (mut p, mut tr, mut vis) in q.iter_mut() {
        let want = if night && p.flying <= 0.0 { Visibility::Hidden } else { Visibility::Inherited };
        if *vis != want {
            *vis = want;
        }
        if night && p.flying <= 0.0 {
            continue;
        }
        p.phase += dt;
        let pos2 = Vec2::new(tr.translation.x, tr.translation.z);
        if p.flying > 0.0 {
            p.flying -= dt;
            let v = p.vel;
            tr.translation += v * dt;
            p.vel.y -= dt * 0.8;
            let flap = (t * 28.0 + p.phase).sin();
            for (w, s) in [(p.wing_l, 1.0f32), (p.wing_r, -1.0)] {
                if let Ok(mut wt) = parts.get_mut(w) {
                    wt.rotation = Quat::from_rotation_x(s * flap * 0.9);
                }
            }
            if p.flying <= 0.0 {
                // land somewhere near home again
                tr.translation = Vec3::new(p.home.x + (p.phase * 3.1).sin() * 0.8, 0.13, p.home.y + (p.phase * 2.3).cos() * 0.8);
                for w in [p.wing_l, p.wing_r] {
                    if let Ok(mut wt) = parts.get_mut(w) {
                        wt.rotation = Quat::IDENTITY;
                    }
                }
            }
            continue;
        }
        // startled by Elias moving close, or by anyone running past
        let near_elias = pos2.distance(pp) < if rt.running { 4.5 } else if rt.moving > 0.0 && !rt.sneaking { 2.2 } else { 1.0 };
        let runner = sim.agents.iter().any(|a| a.speed > 2.5 && a.pos.distance_squared(pos2) < 4.0);
        if near_elias || runner {
            let away = (pos2 - pp).normalize_or(Vec2::X);
            p.vel = Vec3::new(away.x * 3.5, 3.2, away.y * 3.5);
            p.flying = 2.2 + (p.phase % 1.0);
            tr.rotation = Quat::from_rotation_y(-away.y.atan2(away.x));
            continue;
        }
        // peck, bob, take a few steps
        let peck = ((t * 3.0 + p.phase).sin() > 0.7) as i32 as f32;
        if let Ok(mut ht) = parts.get_mut(p.head) {
            ht.translation = Vec3::new(0.08 + peck * 0.03, 0.2 - peck * 0.12, 0.0);
        }
        if (t * 0.7 + p.phase).sin() > 0.95 {
            let dir = tr.rotation * Vec3::X;
            tr.translation += dir * dt * 0.25;
        }
        if (t * 0.3 + p.phase * 2.0).sin() > 0.99 {
            tr.rotate_y(dt * 3.0);
        }
    }
}
