//! A proper horse: smooth lofted body, arched neck, long head, jointed legs
//! with hocks and hooves, mane and tail, many coats. Walks and trots with
//! the cart's speed (legs, head nod, tail sway).

use super::mesh::{c3, MB};
use bevy::prelude::*;

#[derive(Component)]
pub struct HorseRig {
    pub legs: [(Entity, Entity); 4],
    pub head: Entity,
    pub tail: Entity,
    pub phase: f32,
    pub speed: f32,
    pub car: u32,
}

fn v(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

fn shade(c: [f32; 3], k: f32) -> [f32; 3] {
    [(c[0] * k).min(1.0), (c[1] * k).min(1.0), (c[2] * k).min(1.0)]
}

/// Spawns a horse facing +x with its feet at y = 0. Returns the root entity.
pub fn spawn_horse(c: &mut Commands, meshes: &mut Assets<Mesh>, mat: &Handle<StandardMaterial>, seed: u32, car: u32) -> Entity {
    let coats: [([f32; 3], [f32; 3]); 7] = [
        ([0.42, 0.24, 0.12], [0.08, 0.06, 0.05]), // bay
        ([0.55, 0.28, 0.12], [0.5, 0.26, 0.12]),  // chestnut
        ([0.08, 0.07, 0.07], [0.04, 0.04, 0.04]), // black
        ([0.62, 0.6, 0.58], [0.4, 0.4, 0.4]),     // grey
        ([0.7, 0.58, 0.4], [0.15, 0.12, 0.1]),    // dun
        ([0.3, 0.18, 0.1], [0.1, 0.07, 0.05]),    // dark bay
        ([0.85, 0.83, 0.8], [0.75, 0.72, 0.7]),   // white
    ];
    let (coat, dark) = coats[(seed % 7) as usize];
    let pinto = seed % 11 == 3;
    let socks = seed % 3 == 0;
    let blaze = seed % 2 == 0;
    let col = c3(coat);
    let belly = c3(shade(coat, 0.85));
    let hair = c3(dark);
    let hoof = c3([0.12, 0.1, 0.09]);
    let white = c3([0.9, 0.88, 0.85]);

    // ---------------------------------------------------------------- body
    let mut body = MB::new();
    let spine: [(f32, f32, f32, f32); 9] = [
        (-0.98, 1.22, 0.12, 0.12),
        (-0.9, 1.22, 0.24, 0.26),
        (-0.72, 1.2, 0.29, 0.33),
        (-0.45, 1.15, 0.3, 0.36),
        (-0.1, 1.12, 0.31, 0.38),
        (0.25, 1.14, 0.3, 0.37),
        (0.55, 1.2, 0.27, 0.35),
        (0.78, 1.24, 0.2, 0.3),
        (0.9, 1.25, 0.08, 0.14),
    ];
    let secs: Vec<(Vec3, Vec3, f32, f32)> = spine.iter().map(|(x, y, rz, ry)| (v(*x, *y, 0.0), Vec3::X, *rz, *ry)).collect();
    body.loft_c(&secs, 18, &|i, t| {
        // lighter belly, pinto patches
        let under = (t * std::f32::consts::TAU).sin() < -0.55;
        if pinto && (i % 3 == 1) && (t * 5.0) as i32 % 2 == 0 {
            white
        } else if under {
            belly
        } else {
            col
        }
    });
    // shoulders and haunches bulge
    for (x, y, r) in [(0.5, 1.12, 0.2), (-0.7, 1.16, 0.24)] {
        for z in [-0.17, 0.17] {
            body.sphere(v(x, y, z), Vec3::new(r * 1.2, r * 1.4, r * 0.6), 10, col);
        }
    }
    // neck: arched, from the withers up to the poll
    let neck: Vec<(Vec3, Vec3, f32, f32)> = (0..7)
        .map(|k| {
            let t = k as f32 / 6.0;
            let p = v(0.62 + t * 0.45 + t * t * 0.05, 1.3 + t * 0.62 - t * t * 0.05, 0.0);
            let dir = v(0.55 + t * 0.2, 0.9 - t * 0.35, 0.0);
            (p, dir, 0.17 - t * 0.075, 0.27 - t * 0.12)
        })
        .collect();
    body.loft(&neck, 14, col);
    // mane along the neck crest
    for k in 0..9 {
        let t = k as f32 / 8.0;
        let p = v(0.56 + t * 0.47, 1.52 + t * 0.52, 0.0);
        body.sphere(p + v(-0.03, 0.03, 0.0), Vec3::new(0.07, 0.09, 0.035 + (k % 2) as f32 * 0.02), 6, hair);
    }
    // harness: collar + saddle pad + breeching straps
    body.loft(
        &[(v(0.66, 1.38, 0.0), v(0.6, 0.8, 0.0), 0.2, 0.3), (v(0.71, 1.45, 0.0), v(0.6, 0.8, 0.0), 0.21, 0.31), (v(0.76, 1.52, 0.0), v(0.6, 0.8, 0.0), 0.19, 0.29)],
        14,
        c3([0.28, 0.15, 0.08]),
    );
    body.loft(&[(v(-0.05, 1.12, 0.0), Vec3::X, 0.33, 0.4), (v(0.15, 1.13, 0.0), Vec3::X, 0.33, 0.4)], 16, c3([0.12, 0.08, 0.06]));
    let body_e = c.spawn((Mesh3d(meshes.add(body.build())), MeshMaterial3d(mat.clone()), Transform::default())).id();

    // ---------------------------------------------------------------- head (pivot at the poll)
    let mut head = MB::new();
    let hs: [(f32, f32, f32, f32); 6] = [(0.0, 0.0, 0.09, 0.12), (0.1, -0.06, 0.1, 0.13), (0.22, -0.15, 0.085, 0.115), (0.33, -0.24, 0.07, 0.095), (0.42, -0.32, 0.065, 0.085), (0.47, -0.36, 0.05, 0.06)];
    let hsecs: Vec<(Vec3, Vec3, f32, f32)> = hs.iter().map(|(x, y, rz, ry)| (v(*x, *y, 0.0), v(0.8, -0.65, 0.0), *rz, *ry)).collect();
    head.loft_c(&hsecs, 14, &|i, t| {
        let front = (t * std::f32::consts::TAU).cos().abs() < 0.3 && (t * std::f32::consts::TAU).sin() > 0.0;
        if blaze && front && i >= 1 {
            white
        } else if i >= 4 {
            c3(shade(coat, 0.6))
        } else {
            col
        }
    });
    for z in [-0.045f32, 0.045] {
        // ears
        head.frustum(v(0.0, 0.08, z), 0.03, 0.006, 0.12, 6, col);
        // eyes
        head.sphere(v(0.12, 0.0, z * 2.1), Vec3::new(0.022, 0.018, 0.012), 8, c3([0.03, 0.02, 0.02]));
        // nostrils
        head.sphere(v(0.47, -0.35, z * 0.8), Vec3::splat(0.014), 6, c3([0.05, 0.03, 0.03]));
    }
    // forelock + bridle
    head.sphere(v(0.02, 0.07, 0.0), Vec3::new(0.06, 0.05, 0.04), 6, hair);
    head.loft(&[(v(0.33, -0.24, 0.0), v(0.8, -0.65, 0.0), 0.075, 0.1), (v(0.345, -0.255, 0.0), v(0.8, -0.65, 0.0), 0.075, 0.1)], 12, c3([0.1, 0.06, 0.04]));
    let head_e = c.spawn((Mesh3d(meshes.add(head.build())), MeshMaterial3d(mat.clone()), Transform::from_xyz(1.12, 1.9, 0.0))).id();

    // ---------------------------------------------------------------- tail (pivot at the dock)
    let mut tail = MB::new();
    let ts: Vec<(Vec3, Vec3, f32, f32)> = (0..7)
        .map(|k| {
            let t = k as f32 / 6.0;
            (v(-0.06 - t * 0.12, -t * 0.72, 0.0), v(-0.2, -1.0, 0.0), 0.04 + t * 0.06 - t * t * 0.05, 0.05 + t * 0.05)
        })
        .collect();
    tail.loft(&ts, 10, hair);
    let tail_e = c.spawn((Mesh3d(meshes.add(tail.build())), MeshMaterial3d(mat.clone()), Transform::from_xyz(-0.96, 1.28, 0.0))).id();

    // ---------------------------------------------------------------- legs (upper pivot at the body, lower at the knee/hock)
    let mut legs = Vec::new();
    for (i, (lx, lz)) in [(0.55f32, -0.15f32), (0.55, 0.15), (-0.66, -0.16), (-0.66, 0.16)].iter().enumerate() {
        let hind = i >= 2;
        let sock = socks && (i == 0 || i == 3);
        let mut up = MB::new();
        let top_r = if hind { 0.14 } else { 0.11 };
        let joint_y = if hind { -0.5 } else { -0.48 };
        up.loft(&[(v(0.0, 0.1, 0.0), -Vec3::Y, top_r, top_r * 1.2), (v(if hind { -0.06 } else { 0.0 }, -0.2, 0.0), -Vec3::Y, top_r * 0.75, top_r * 0.8), (v(if hind { -0.05 } else { 0.0 }, joint_y, 0.0), -Vec3::Y, 0.055, 0.06)], 10, col);
        up.sphere(v(if hind { -0.05 } else { 0.0 }, joint_y, 0.0), Vec3::new(0.06, 0.065, 0.055), 8, col);
        let mut low = MB::new();
        let len = 0.62 + if hind { -0.02 } else { 0.0 } - 0.02;
        let leg_col = if sock { white } else { col };
        low.loft(&[(v(0.0, 0.0, 0.0), -Vec3::Y, 0.045, 0.05), (v(0.0, -len * 0.7, 0.0), -Vec3::Y, 0.035, 0.04), (v(0.01, -len + 0.08, 0.0), -Vec3::Y, 0.042, 0.045)], 8, leg_col);
        // fetlock feathering + hoof
        low.sphere(v(0.015, -len + 0.08, 0.0), Vec3::new(0.05, 0.045, 0.048), 8, if sock { white } else { c3(shade(dark, 1.4)) });
        low.frustum(v(0.03, -len - 0.02, 0.0), 0.065, 0.05, 0.1, 10, hoof);
        let up_e = c.spawn((Mesh3d(meshes.add(up.build())), MeshMaterial3d(mat.clone()), Transform::from_xyz(*lx, 1.1, *lz))).id();
        let low_e = c.spawn((Mesh3d(meshes.add(low.build())), MeshMaterial3d(mat.clone()), Transform::from_xyz(if hind { -0.05 } else { 0.0 }, joint_y, 0.0))).id();
        c.entity(up_e).add_child(low_e);
        legs.push((up_e, low_e));
    }
    let root = c.spawn((Transform::default(), Visibility::default())).id();
    c.entity(root).add_children(&[body_e, head_e, tail_e]);
    for (u, _) in &legs {
        c.entity(root).add_child(*u);
    }
    c.entity(root).insert(HorseRig { legs: [legs[0], legs[1], legs[2], legs[3]], head: head_e, tail: tail_e, phase: (seed % 100) as f32 * 0.063, speed: 0.0, car });
    root
}

/// Walk / trot cycle driven by speed (m/s); idle horses shift weight and swish the tail.
pub fn animate_horses(time: Res<Time>, cars: Res<crate::vehicles::Cars>, mut rigs: Query<&mut HorseRig>, mut parts: Query<&mut Transform, Without<HorseRig>>) {
    let dt = time.delta_secs();
    let t = time.elapsed_secs();
    for mut r in rigs.iter_mut() {
        let sp = cars.list.iter().find(|c| c.id == r.car).map(|c| c.speed.abs()).unwrap_or(0.0);
        r.speed += (sp - r.speed) * (dt * 3.0).min(1.0);
        let s = r.speed;
        let freq = if s < 0.05 { 0.0 } else { 3.0 + s * 1.3 };
        r.phase += dt * freq;
        let ph = r.phase;
        let amp = (s / 3.0).clamp(0.0, 1.0) * 0.45;
        // diagonal pairs move together (trot): front-left + hind-right
        let offs = [0.0, std::f32::consts::PI, std::f32::consts::PI, 0.0];
        for (i, (up, low)) in r.legs.iter().enumerate() {
            let a = (ph + offs[i]).sin();
            let lift = (ph + offs[i]).cos().max(0.0);
            let hind = i >= 2;
            if let Ok(mut tr) = parts.get_mut(*up) {
                tr.rotation = Quat::from_rotation_z(a * amp);
            }
            if let Ok(mut tr) = parts.get_mut(*low) {
                // knees fold backwards on the front legs, hocks forwards on the hind legs
                let bend = lift * amp * 2.2 + if s < 0.05 && i == 3 { (t * 0.5).sin().max(0.0) * 0.4 } else { 0.0 };
                tr.rotation = Quat::from_rotation_z(if hind { bend } else { -bend });
            }
        }
        if let Ok(mut tr) = parts.get_mut(r.head) {
            let nod = if s > 0.05 { (ph * 2.0).sin() * 0.08 * amp * 2.0 } else { (t * 0.4 + ph).sin() * 0.06 };
            tr.rotation = Quat::from_rotation_z(nod) * Quat::from_rotation_y((t * 0.23 + ph).sin() * if s > 0.05 { 0.05 } else { 0.25 });
        }
        if let Ok(mut tr) = parts.get_mut(r.tail) {
            tr.rotation = Quat::from_rotation_x((t * 1.7 + ph).sin() * 0.25) * Quat::from_rotation_z(-0.15 - amp * 0.4);
        }
    }
}

/// A real, skeletally animated horse model (hitched to cart `car`).
#[derive(Component)]
pub struct RealHorse {
    pub car: u32,
    pub file: String,
    pub idle: usize,
    pub walk: usize,
    pub trot: usize,
    pub walk_speed: f32,
    pub trot_speed: f32,
    player: Option<Entity>,
    nodes: [AnimationNodeIndex; 3],
    cur: usize,
}

impl RealHorse {
    pub fn new(car: u32, e: &super::lib3d::Entry) -> Self {
        let g = |k: &str, d: usize| e.clips.get(k).copied().unwrap_or(d);
        RealHorse {
            car,
            file: e.file.clone(),
            idle: g("idle", 0),
            walk: g("walk", 0),
            trot: g("trot", 0),
            walk_speed: if e.walk_speed > 0.0 { e.walk_speed } else { 1.2 },
            trot_speed: if e.trot_speed > 0.0 { e.trot_speed } else { 3.0 },
            player: None,
            nodes: [AnimationNodeIndex::new(0); 3],
            cur: 99,
        }
    }
}

fn find_player(e: Entity, players: &Query<(), With<AnimationPlayer>>, kids: &Query<&Children>) -> Option<Entity> {
    if players.get(e).is_ok() {
        return Some(e);
    }
    kids.get(e).ok()?.iter().find_map(|k| find_player(k, players, kids))
}

/// Hook up each real horse's animation player, then walk / trot / stand with its cart's speed.
#[allow(clippy::type_complexity)]
pub fn animate_real_horses(
    mut c: Commands,
    a: Res<AssetServer>,
    cars: Res<crate::vehicles::Cars>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut horses: Query<(Entity, &mut RealHorse)>,
    has_player: Query<(), With<AnimationPlayer>>,
    kids: Query<&Children>,
    mut players: Query<(&mut AnimationPlayer, Option<&mut AnimationTransitions>)>,
) {
    for (e, mut h) in horses.iter_mut() {
        let Some(pe) = h.player else {
            let Some(pe) = find_player(e, &has_player, &kids) else { continue };
            let path = super::lib3d::Lib::path(&h.file);
            let clips = [h.idle, h.walk, h.trot].map(|i| a.load_with_settings(GltfAssetLabel::Animation(i).from_asset(path.clone()), super::lib3d::gpu_only_textures));
            let (graph, nodes) = AnimationGraph::from_clips(clips);
            h.nodes = [nodes[0], nodes[1], nodes[2]];
            c.entity(pe).insert((AnimationGraphHandle(graphs.add(graph)), AnimationTransitions::new()));
            h.player = Some(pe);
            continue;
        };
        let Ok((mut player, Some(mut tr))) = players.get_mut(pe) else { continue };
        let sp = cars.list.iter().find(|c| c.id == h.car).map(|c| c.speed.abs()).unwrap_or(0.0);
        let want = if sp < 0.15 { 0 } else if sp < (h.walk_speed + h.trot_speed) * 0.5 { 1 } else { 2 };
        if want != h.cur {
            tr.play(&mut player, h.nodes[want], std::time::Duration::from_millis(350)).repeat();
            h.cur = want;
        }
        let rate = match want {
            1 => (sp / h.walk_speed).clamp(0.6, 1.6),
            2 => (sp / h.trot_speed).clamp(0.7, 1.6),
            _ => 1.0,
        };
        if let Some(anim) = player.animation_mut(h.nodes[want]) {
            anim.set_speed(rate);
        }
    }
}
