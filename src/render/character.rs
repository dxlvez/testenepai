//! Procedural low-poly people: every citizen gets a rig built from their
//! `Look` (era clothing, hat, hair, build) and a code-driven animation.

use super::mesh::{c3, MB};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Hat {
    None,
    Fedora,
    FlatCap,
    Bowler,
    Cloche,
    PoliceCap,
    Boater,
    Beanie,
    NurseCap,
    Headscarf,
    Bobby,
    Shako,
    Alpine,
    Helmet,
    Cap,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Garment {
    Suit,
    LongCoat,
    Dress,
    Apron,
    Uniform,
    Overalls,
    Shirt,
    Hoodie,
    Robe,
    LabCoat,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Hair {
    Short,
    Long,
    Bun,
    Bald,
    Bob,
    Afro,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Look {
    pub female: bool,
    pub skin: [f32; 3],
    pub hair_col: [f32; 3],
    pub hair: Hair,
    pub hat: Hat,
    pub hat_col: [f32; 3],
    pub garment: Garment,
    pub top: [f32; 3],
    pub bottom: [f32; 3],
    pub accent: [f32; 3],
    pub height: f32,
    pub girth: f32,
    pub beard: bool,
    pub age: f32,
}

#[derive(Clone, Copy, PartialEq, Debug, Default, Serialize, Deserialize)]
pub enum Pose {
    #[default]
    Idle,
    Walk,
    Run,
    Sneak,
    Sit,
    Lie,
    Dead,
    Aim,
    Punch,
    Carry,
    Drag,
    Talk,
    Pray,
    Work,
    Dance,
    Cower,
    Play,
}

#[derive(Component)]
pub struct Rig {
    pub body: Entity,
    pub arm_l: Entity,
    pub arm_r: Entity,
    pub leg_l: Entity,
    pub leg_r: Entity,
    pub head: Entity,
    pub hand_r: Entity,
    pub phase: f32,
    pub pose: Pose,
    pub speed: f32,
    pub punch_t: f32,
    pub hip: f32,
}

/// Build the per-person meshes.
pub fn spawn_character(c: &mut Commands, meshes: &mut Assets<Mesh>, mat: &Handle<StandardMaterial>, look: &Look) -> (Entity, Rig) {
    let s = look.height;
    let g = look.girth;
    let hip = 0.88 * s;
    let skin = c3(look.skin);
    let top = c3(look.top);
    let bottom = c3(look.bottom);
    let accent = c3(look.accent);
    let shoe = c3([0.06, 0.05, 0.05]);
    let grey = |c: [f32; 3], a: f32| {
        let k = (a - 45.0).max(0.0) / 40.0;
        let l = (c[0] + c[1] + c[2]) / 3.0;
        [c[0] + (0.7 - l) * k, c[1] + (0.7 - l) * k, c[2] + (0.7 - l) * k]
    };
    let hair_c = c3(grey(look.hair_col, look.age));

    // torso (relative to hip pivot)
    let mut torso = MB::new();
    let sw = 0.21 * g; // half shoulder width
    let tw = 0.13 * g; // half depth
    let torso_h = 0.55 * s;
    match look.garment {
        Garment::Dress => {
            torso.cuboid(Vec3::new(-sw * 0.85, 0.0, -tw), Vec3::new(sw * 0.85, torso_h, tw), top);
            torso.frustum(Vec3::new(0.0, -0.55 * s, 0.0), 0.3 * g, 0.18 * g, 0.62 * s, 10, top);
            torso.cuboid(Vec3::new(-sw * 0.86, 0.02, -tw - 0.01), Vec3::new(sw * 0.86, 0.08, tw + 0.01), accent);
        }
        Garment::LongCoat | Garment::LabCoat | Garment::Robe => {
            torso.cuboid(Vec3::new(-sw, 0.0, -tw), Vec3::new(sw, torso_h, tw), top);
            torso.frustum(Vec3::new(0.0, -0.45 * s, 0.0), 0.27 * g, 0.22 * g, 0.47 * s, 8, top);
            torso.cuboid(Vec3::new(-0.03, 0.05, -tw - 0.015), Vec3::new(0.03, torso_h - 0.05, -tw), accent);
        }
        Garment::Suit | Garment::Uniform => {
            torso.cuboid(Vec3::new(-sw, 0.0, -tw), Vec3::new(sw, torso_h, tw), top);
            // shirt + tie triangle at the front (-z is the character's front)
            torso.cuboid(Vec3::new(-0.05, torso_h * 0.55, -tw - 0.01), Vec3::new(0.05, torso_h - 0.02, -tw), c3([0.85, 0.83, 0.78]));
            torso.cuboid(Vec3::new(-0.018, torso_h * 0.45, -tw - 0.02), Vec3::new(0.018, torso_h - 0.04, -tw - 0.01), accent);
            if look.garment == Garment::Uniform {
                torso.cuboid(Vec3::new(-sw - 0.005, 0.02, -tw - 0.005), Vec3::new(sw + 0.005, 0.07, tw + 0.005), c3([0.08, 0.07, 0.06]));
                torso.cuboid(Vec3::new(0.07, torso_h * 0.7, -tw - 0.02), Vec3::new(0.12, torso_h * 0.78, -tw), c3([0.8, 0.7, 0.3]));
            }
        }
        Garment::Apron => {
            torso.cuboid(Vec3::new(-sw, 0.0, -tw), Vec3::new(sw, torso_h, tw), top);
            torso.cuboid(Vec3::new(-sw * 0.8, -0.35 * s, -tw - 0.02), Vec3::new(sw * 0.8, torso_h * 0.8, -tw), accent);
        }
        Garment::Overalls => {
            torso.cuboid(Vec3::new(-sw, 0.0, -tw), Vec3::new(sw, torso_h, tw), top);
            torso.cuboid(Vec3::new(-sw * 0.7, 0.0, -tw - 0.015), Vec3::new(sw * 0.7, torso_h * 0.6, -tw), bottom);
            for x in [-sw * 0.55, sw * 0.4] {
                torso.cuboid(Vec3::new(x, torso_h * 0.6, -tw - 0.015), Vec3::new(x + 0.05, torso_h, -tw), bottom);
            }
        }
        Garment::Shirt | Garment::Hoodie => {
            torso.cuboid(Vec3::new(-sw * 0.95, 0.0, -tw), Vec3::new(sw * 0.95, torso_h, tw), top);
            if look.garment == Garment::Hoodie {
                torso.cuboid(Vec3::new(-0.13, torso_h - 0.05, tw - 0.02), Vec3::new(0.13, torso_h + 0.08, tw + 0.08), top);
                torso.cuboid(Vec3::new(-0.1, 0.05, -tw - 0.02), Vec3::new(0.1, 0.2, -tw), c3([look.top[0] * 0.8, look.top[1] * 0.8, look.top[2] * 0.8]));
            }
        }
    }
    if look.girth > 1.15 {
        torso.sphere(Vec3::new(0.0, torso_h * 0.35, -0.02), Vec3::new(sw * 0.9, 0.18, tw * 1.3), 8, top);
    }
    // neck
    torso.cylinder(Vec3::new(0.0, torso_h - 0.02, 0.0), 0.05, 0.08, 6, skin);

    // head (relative to neck top)
    let mut head = MB::new();
    let hr = 0.115;
    head.sphere(Vec3::new(0.0, hr, 0.0), Vec3::new(hr * 0.92, hr * 1.08, hr), 10, skin);
    // eyes (tiny dark dots looking forward)
    for x in [-0.04, 0.04] {
        head.bx(Vec3::new(x, hr * 1.1, -hr * 0.93), Vec3::new(0.012, 0.01, 0.005), c3([0.05, 0.04, 0.04]));
    }
    // nose
    head.bx(Vec3::new(0.0, hr * 0.95, -hr * 1.0), Vec3::new(0.012, 0.025, 0.02), c3([look.skin[0] * 0.9, look.skin[1] * 0.85, look.skin[2] * 0.85]));
    if look.beard {
        head.sphere(Vec3::new(0.0, hr * 0.55, -hr * 0.45), Vec3::new(hr * 0.8, hr * 0.55, hr * 0.6), 8, hair_c);
    }
    match look.hair {
        Hair::Bald => {}
        Hair::Short => head.sphere(Vec3::new(0.0, hr * 1.25, hr * 0.12), Vec3::new(hr * 0.98, hr * 0.8, hr * 0.95), 10, hair_c),
        Hair::Bob => {
            head.sphere(Vec3::new(0.0, hr * 1.2, hr * 0.1), Vec3::new(hr * 1.08, hr * 0.95, hr * 1.05), 10, hair_c);
        }
        Hair::Long => {
            head.sphere(Vec3::new(0.0, hr * 1.2, hr * 0.1), Vec3::new(hr * 1.05, hr * 0.9, hr * 1.02), 10, hair_c);
            head.cuboid(Vec3::new(-hr * 0.9, -hr * 1.6, hr * 0.2), Vec3::new(hr * 0.9, hr * 1.0, hr * 1.0), hair_c);
        }
        Hair::Bun => {
            head.sphere(Vec3::new(0.0, hr * 1.2, hr * 0.1), Vec3::new(hr * 1.0, hr * 0.85, hr * 1.0), 10, hair_c);
            head.sphere(Vec3::new(0.0, hr * 1.5, hr * 0.9), Vec3::splat(hr * 0.45), 8, hair_c);
        }
        Hair::Afro => head.sphere(Vec3::new(0.0, hr * 1.35, hr * 0.1), Vec3::splat(hr * 1.35), 10, hair_c),
    }
    let hc = c3(look.hat_col);
    let top_y = hr * 1.85;
    match look.hat {
        Hat::None => {}
        Hat::Fedora => {
            head.frustum(Vec3::new(0.0, top_y - 0.03, 0.0), 0.2, 0.2, 0.015, 12, hc);
            head.frustum(Vec3::new(0.0, top_y - 0.02, 0.0), 0.11, 0.095, 0.13, 10, hc);
            head.frustum(Vec3::new(0.0, top_y - 0.015, 0.0), 0.112, 0.112, 0.03, 10, c3([0.1, 0.08, 0.07]));
        }
        Hat::FlatCap => {
            head.sphere(Vec3::new(0.0, top_y - 0.03, 0.0), Vec3::new(0.13, 0.05, 0.13), 10, hc);
            head.bx(Vec3::new(0.0, top_y - 0.04, -0.13), Vec3::new(0.09, 0.012, 0.05), hc);
        }
        Hat::Bowler => {
            head.frustum(Vec3::new(0.0, top_y - 0.03, 0.0), 0.16, 0.16, 0.015, 12, hc);
            head.sphere(Vec3::new(0.0, top_y, 0.0), Vec3::new(0.11, 0.1, 0.11), 10, hc);
        }
        Hat::Cloche => {
            head.sphere(Vec3::new(0.0, top_y - 0.05, 0.02), Vec3::new(0.135, 0.1, 0.135), 10, hc);
            head.frustum(Vec3::new(0.0, top_y - 0.09, 0.0), 0.15, 0.14, 0.02, 10, hc);
        }
        Hat::PoliceCap => {
            head.frustum(Vec3::new(0.0, top_y - 0.05, 0.0), 0.12, 0.14, 0.1, 10, hc);
            head.bx(Vec3::new(0.0, top_y - 0.05, -0.12), Vec3::new(0.08, 0.01, 0.05), c3([0.05, 0.05, 0.05]));
            head.bx(Vec3::new(0.0, top_y, -0.135), Vec3::new(0.02, 0.02, 0.005), c3([0.85, 0.75, 0.3]));
        }
        Hat::Boater => {
            head.frustum(Vec3::new(0.0, top_y - 0.03, 0.0), 0.19, 0.19, 0.012, 12, hc);
            head.frustum(Vec3::new(0.0, top_y - 0.02, 0.0), 0.11, 0.11, 0.08, 10, hc);
            head.frustum(Vec3::new(0.0, top_y, 0.0), 0.112, 0.112, 0.025, 10, c3([0.1, 0.1, 0.15]));
        }
        Hat::Beanie => head.sphere(Vec3::new(0.0, top_y - 0.04, 0.0), Vec3::new(0.125, 0.11, 0.125), 10, hc),
        Hat::NurseCap => head.bx(Vec3::new(0.0, top_y, 0.0), Vec3::new(0.08, 0.04, 0.06), c3([0.95, 0.95, 0.95])),
        Hat::Headscarf => head.sphere(Vec3::new(0.0, top_y - 0.06, 0.03), Vec3::new(0.135, 0.11, 0.135), 10, hc),
        Hat::Bobby => {
            // custodian helmet
            head.frustum(Vec3::new(0.0, top_y - 0.07, 0.0), 0.14, 0.1, 0.16, 10, hc);
            head.sphere(Vec3::new(0.0, top_y + 0.09, 0.0), Vec3::new(0.1, 0.08, 0.1), 8, hc);
            head.bx(Vec3::new(0.0, top_y + 0.03, -0.12), Vec3::new(0.025, 0.03, 0.005), c3([0.8, 0.75, 0.5]));
        }
        Hat::Shako => {
            head.frustum(Vec3::new(0.0, top_y - 0.05, 0.0), 0.12, 0.13, 0.18, 10, hc);
            head.bx(Vec3::new(0.0, top_y - 0.04, -0.12), Vec3::new(0.08, 0.01, 0.05), c3([0.05, 0.05, 0.05]));
            head.sphere(Vec3::new(0.0, top_y + 0.15, -0.05), Vec3::splat(0.03), 6, c3([0.8, 0.8, 0.8]));
        }
        Hat::Alpine => {
            head.frustum(Vec3::new(0.0, top_y - 0.03, 0.0), 0.17, 0.17, 0.012, 12, hc);
            head.frustum(Vec3::new(0.0, top_y - 0.02, 0.0), 0.11, 0.07, 0.14, 10, hc);
            head.bx(Vec3::new(0.08, top_y + 0.08, 0.05), Vec3::new(0.005, 0.06, 0.015), c3([0.3, 0.35, 0.25]));
        }
        Hat::Helmet => {
            head.sphere(Vec3::new(0.0, top_y - 0.03, 0.0), Vec3::new(0.14, 0.08, 0.14), 10, hc);
            head.frustum(Vec3::new(0.0, top_y - 0.07, 0.0), 0.2, 0.2, 0.01, 12, hc);
        }
        Hat::Cap => {
            head.sphere(Vec3::new(0.0, top_y - 0.04, 0.0), Vec3::new(0.125, 0.08, 0.125), 10, hc);
            head.bx(Vec3::new(0.0, top_y - 0.06, -0.15), Vec3::new(0.08, 0.008, 0.06), hc);
        }
    }

    // limbs (pivot at the top)
    let arm_col = match look.garment {
        Garment::Shirt if !look.female => skin,
        Garment::Dress => skin,
        _ => top,
    };
    let mut arm = MB::new();
    let al = 0.52 * s;
    arm.cuboid(Vec3::new(-0.045, -al, -0.045), Vec3::new(0.045, 0.0, 0.045), arm_col);
    arm.sphere(Vec3::new(0.0, -al - 0.03, 0.0), Vec3::splat(0.045), 6, skin);
    let mut arm_r_mb = arm.clone();
    let hand_r_offset = Vec3::new(0.0, -al - 0.03, 0.0);
    let _ = &mut arm_r_mb;
    let mut leg = MB::new();
    let ll = hip - 0.05;
    let pants = if look.garment == Garment::Dress { skin } else { bottom };
    leg.cuboid(Vec3::new(-0.06 * g, -ll, -0.06), Vec3::new(0.06 * g, 0.0, 0.06), pants);
    leg.cuboid(Vec3::new(-0.065, -hip, -0.12), Vec3::new(0.065, -ll, 0.07), shoe);

    let body_mesh = meshes.add(torso.build());
    let head_mesh = meshes.add(head.build());
    let arm_mesh = meshes.add(arm.build());
    let arm_r_mesh = meshes.add(arm_r_mb.build());
    let leg_mesh = meshes.add(leg.build());

    let root = c.spawn((Transform::default(), Visibility::default())).id();
    let body = c.spawn((Mesh3d(body_mesh), MeshMaterial3d(mat.clone()), Transform::from_xyz(0.0, hip, 0.0))).id();
    let head_e = c
        .spawn((Mesh3d(head_mesh), MeshMaterial3d(mat.clone()), Transform::from_xyz(0.0, torso_h + 0.04, 0.0)))
        .id();
    let arm_l = c
        .spawn((Mesh3d(arm_mesh.clone()), MeshMaterial3d(mat.clone()), Transform::from_xyz(-sw - 0.045, torso_h - 0.04, 0.0)))
        .id();
    let arm_r = c
        .spawn((Mesh3d(arm_r_mesh), MeshMaterial3d(mat.clone()), Transform::from_xyz(sw + 0.045, torso_h - 0.04, 0.0)))
        .id();
    let hand_r = c.spawn((Transform::from_translation(hand_r_offset), Visibility::default())).id();
    let leg_l = c
        .spawn((Mesh3d(leg_mesh.clone()), MeshMaterial3d(mat.clone()), Transform::from_xyz(-0.09 * g, hip, 0.0)))
        .id();
    let leg_r = c.spawn((Mesh3d(leg_mesh), MeshMaterial3d(mat.clone()), Transform::from_xyz(0.09 * g, hip, 0.0))).id();
    c.entity(arm_r).add_child(hand_r);
    c.entity(body).add_children(&[head_e, arm_l, arm_r]);
    c.entity(root).add_children(&[body, leg_l, leg_r]);
    let rig = Rig {
        body,
        arm_l,
        arm_r,
        leg_l,
        leg_r,
        head: head_e,
        hand_r,
        phase: 0.0,
        pose: Pose::Idle,
        speed: 0.0,
        punch_t: 0.0,
        hip,
    };
    (root, rig)
}

/// Animate every rig from its pose and speed.
pub fn animate_rigs(time: Res<Time>, mut rigs: Query<(&mut Rig, &Transform)>, mut parts: Query<&mut Transform, Without<Rig>>) {
    let dt = time.delta_secs();
    let t = time.elapsed_secs();
    for (mut r, _) in rigs.iter_mut() {
        let sp = r.speed;
        r.phase += dt * (2.0 + sp * 3.4);
        r.punch_t = (r.punch_t - dt).max(0.0);
        let ph = r.phase;
        let hip = r.hip;
        let (mut body_y, mut body_rx, mut body_rz) = (hip, 0.0f32, 0.0f32);
        let (mut al, mut ar, mut ll, mut lr) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
        let (mut al_z, mut ar_z) = (0.0f32, 0.0f32);
        let mut head_rx = 0.0f32;
        let mut leg_y = hip;
        match r.pose {
            Pose::Idle | Pose::Talk => {
                let b = (t * 1.3 + ph * 0.1).sin() * 0.01;
                body_y += b;
                al = 0.05;
                ar = -0.05;
                if r.pose == Pose::Talk {
                    ar = -0.4 + (t * 3.0).sin() * 0.25;
                    head_rx = (t * 2.0).sin() * 0.06;
                }
            }
            Pose::Walk | Pose::Run | Pose::Sneak | Pose::Carry | Pose::Drag => {
                let amp = match r.pose {
                    Pose::Run => 0.9,
                    Pose::Sneak => 0.35,
                    _ => 0.55,
                };
                let s = ph.sin();
                ll = s * amp;
                lr = -s * amp;
                al = -s * amp * 0.8;
                ar = s * amp * 0.8;
                body_y += (ph * 2.0).sin().abs() * 0.03 * amp;
                if r.pose == Pose::Run {
                    body_rx = -0.18;
                }
                if r.pose == Pose::Sneak {
                    body_y -= 0.12;
                    body_rx = -0.25;
                    leg_y -= 0.1;
                }
                if r.pose == Pose::Carry {
                    al = -1.3;
                    ar = -1.3;
                }
                if r.pose == Pose::Drag {
                    al = 0.5;
                    ar = 0.5;
                    body_rx = 0.2;
                }
            }
            Pose::Sit => {
                body_y = hip * 0.55;
                leg_y = hip * 0.55;
                ll = -1.45;
                lr = -1.45;
                al = -0.3;
                ar = -0.3;
            }
            Pose::Play => {
                body_y = hip * 0.55;
                leg_y = hip * 0.55;
                ll = -1.45;
                lr = -1.45;
                al = -1.1 + (t * 7.0).sin() * 0.15;
                ar = -1.1 + (t * 6.3 + 1.0).sin() * 0.15;
            }
            Pose::Lie | Pose::Dead => {
                body_y = 0.25;
                leg_y = 0.25;
                body_rx = std::f32::consts::FRAC_PI_2;
                ll = -std::f32::consts::FRAC_PI_2;
                lr = -std::f32::consts::FRAC_PI_2;
                al = if r.pose == Pose::Dead { 2.2 } else { 0.3 };
                ar = if r.pose == Pose::Dead { 1.2 } else { 0.3 };
                al_z = if r.pose == Pose::Dead { 0.6 } else { 0.0 };
            }
            Pose::Aim => {
                ar = -std::f32::consts::FRAC_PI_2;
                al = -1.2;
                al_z = -0.5;
            }
            Pose::Punch => {
                let k = (r.punch_t / 0.3).clamp(0.0, 1.0);
                ar = -std::f32::consts::FRAC_PI_2 * (1.0 - (k - 0.5).abs() * 2.0).max(0.2);
                al = -0.8;
            }
            Pose::Pray => {
                body_y = hip * 0.7;
                leg_y = hip * 0.7;
                ll = -1.5;
                lr = -1.5;
                al = -1.0;
                ar = -1.0;
                al_z = -0.4;
                ar_z = 0.4;
                head_rx = -0.3;
            }
            Pose::Work => {
                al = -0.7 + (t * 4.0).sin() * 0.3;
                ar = -0.7 + (t * 4.0 + 1.5).sin() * 0.3;
                body_rx = -0.15;
            }
            Pose::Dance => {
                let s = (t * 5.0).sin();
                body_y += (t * 10.0).sin().abs() * 0.05;
                ll = s * 0.4;
                lr = -s * 0.4;
                al = -2.3 + s * 0.4;
                ar = -2.3 - s * 0.4;
                body_rz = s * 0.1;
            }
            Pose::Cower => {
                body_y = hip * 0.6;
                leg_y = hip * 0.6;
                ll = -1.2;
                lr = -1.2;
                al = -2.4;
                ar = -2.4;
                body_rx = -0.4;
                head_rx = -0.5;
            }
        }
        if let Ok(mut tr) = parts.get_mut(r.body) {
            tr.translation.y = body_y;
            tr.rotation = Quat::from_euler(EulerRot::XYZ, body_rx, 0.0, body_rz);
        }
        if let Ok(mut tr) = parts.get_mut(r.head) {
            tr.rotation = Quat::from_rotation_x(head_rx);
        }
        if let Ok(mut tr) = parts.get_mut(r.arm_l) {
            tr.rotation = Quat::from_euler(EulerRot::XYZ, -al, 0.0, al_z);
        }
        if let Ok(mut tr) = parts.get_mut(r.arm_r) {
            tr.rotation = Quat::from_euler(EulerRot::XYZ, -ar, 0.0, ar_z);
        }
        if let Ok(mut tr) = parts.get_mut(r.leg_l) {
            tr.translation.y = leg_y;
            tr.rotation = Quat::from_rotation_x(-ll);
        }
        if let Ok(mut tr) = parts.get_mut(r.leg_r) {
            tr.translation.y = leg_y;
            tr.rotation = Quat::from_rotation_x(-lr);
        }
    }
}
