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
    pub head: Entity,
    pub arm_l: Entity,
    pub arm_r: Entity,
    pub fore_l: Entity,
    pub fore_r: Entity,
    pub leg_l: Entity,
    pub leg_r: Entity,
    pub shin_l: Entity,
    pub shin_r: Entity,
    pub hand_r: Entity,
    pub phase: f32,
    pub pose: Pose,
    pub speed: f32,
    pub punch_t: f32,
    pub hip: f32,
    /// personal rhythm so crowds don't move in sync
    pub quirk: f32,
}

fn fhash(look: &Look) -> u32 {
    let mut h: u32 = 2166136261;
    for v in look.skin.iter().chain(look.hair_col.iter()).chain(look.top.iter()).chain(look.bottom.iter()).chain([look.height, look.girth, look.age].iter()) {
        h ^= v.to_bits();
        h = h.wrapping_mul(16777619);
    }
    h
}

fn shade(c: [f32; 3], k: f32) -> [f32; 3] {
    [(c[0] * k).min(1.0), (c[1] * k).min(1.0), (c[2] * k).min(1.0)]
}

/// Build the per-person meshes: a rounded low-poly body with jointed limbs,
/// a real face and era clothing. Small features vary per person.
pub fn spawn_character(c: &mut Commands, meshes: &mut Assets<Mesh>, mat: &Handle<StandardMaterial>, look: &Look) -> (Entity, Rig) {
    let seed = fhash(look);
    let r = |k: u32| ((seed.rotate_left(k * 5) ^ (seed >> (k % 13))) % 1000) as f32 / 1000.0;
    let s = look.height * 1.04;
    let g = look.girth;
    let fem = look.female;
    let hip = 0.9 * s;
    let skin = c3(look.skin);
    let skin_d = c3(shade(look.skin, 0.82));
    let top = c3(look.top);
    let top_d = c3(shade(look.top, 0.7));
    let bottom = c3(look.bottom);
    let accent = c3(look.accent);
    let shoe_c = if r(1) < 0.5 { [0.06, 0.05, 0.05] } else { [0.22, 0.13, 0.07] };
    let shoe = c3(shoe_c);
    let grey = |c: [f32; 3], a: f32| {
        let k = ((a - 42.0).max(0.0) / 35.0).min(1.0);
        let l = (c[0] + c[1] + c[2]) / 3.0;
        [c[0] + (0.72 - l) * k, c[1] + (0.72 - l) * k, c[2] + (0.72 - l) * k]
    };
    let hair_c = c3(grey(look.hair_col, look.age));
    let hair_d = c3(shade(grey(look.hair_col, look.age), 0.7));
    let seg = 10;

    // ---------------------------------------------------------------- torso (pivot at the hip)
    let mut torso = MB::new();
    let sw = (if fem { 0.175 } else { 0.2 }) * g; // shoulder half width
    let tw = 0.12 * g; // half depth
    let th = 0.56 * s; // hip -> neck
    let waist = (if fem { 0.12 } else { 0.145 }) * g;
    let hips_r = (if fem { 0.17 } else { 0.15 }) * g;
    // pelvis + waist + rib cage as stacked rounded shapes
    let body_col = top;
    torso.sphere(Vec3::new(0.0, 0.02, 0.0), Vec3::new(hips_r, 0.1 * s, tw * 0.95), seg, bottom);
    torso.frustum(Vec3::new(0.0, 0.02, 0.0), hips_r * 0.95, waist, 0.2 * s, seg, body_col);
    torso.frustum(Vec3::new(0.0, 0.22 * s, 0.0), waist, sw * 0.92, 0.22 * s, seg, body_col);
    torso.sphere(Vec3::new(0.0, th - 0.1 * s, 0.0), Vec3::new(sw * 0.98, 0.12 * s, tw * 1.05), seg, body_col);
    // shoulders
    for x in [-sw, sw] {
        torso.sphere(Vec3::new(x * 0.95, th - 0.08 * s, 0.0), Vec3::new(0.07 * g, 0.065, 0.075 * g), 8, body_col);
    }
    if fem {
        torso.sphere(Vec3::new(-0.065, th - 0.2 * s, -tw * 0.75), Vec3::new(0.07, 0.065, 0.06), 8, body_col);
        torso.sphere(Vec3::new(0.065, th - 0.2 * s, -tw * 0.75), Vec3::new(0.07, 0.065, 0.06), 8, body_col);
    }
    if g > 1.12 {
        torso.sphere(Vec3::new(0.0, 0.2 * s, -0.03), Vec3::new(waist * 1.15, 0.16 * s, tw * 1.25), seg, body_col);
    }
    // belt
    if !matches!(look.garment, Garment::Dress | Garment::Robe | Garment::LongCoat | Garment::LabCoat) {
        torso.frustum(Vec3::new(0.0, 0.08 * s, 0.0), hips_r * 0.99, hips_r * 0.97, 0.035, seg, c3([0.08, 0.06, 0.05]));
        torso.bx(Vec3::new(0.0, 0.095 * s, -hips_r * 0.97), Vec3::new(0.025, 0.018, 0.008), c3([0.75, 0.65, 0.35]));
    }
    let front = -tw * 1.02;
    match look.garment {
        Garment::Dress => {
            // flared skirt + waist ribbon + collar
            let flare = 0.26 + r(3) * 0.08;
            torso.frustum(Vec3::new(0.0, -0.5 * s, 0.0), flare * g, hips_r * 1.02, 0.56 * s, 14, top);
            torso.frustum(Vec3::new(0.0, -0.5 * s, 0.0), flare * g * 1.01, flare * g * 1.0, 0.03, 14, top_d);
            torso.frustum(Vec3::new(0.0, 0.2 * s, 0.0), waist * 1.03, waist * 1.03, 0.035, seg, accent);
            torso.bx(Vec3::new(0.0, th - 0.04, front * 0.9), Vec3::new(0.06, 0.02, 0.01), c3([0.92, 0.9, 0.85]));
        }
        Garment::LongCoat | Garment::LabCoat | Garment::Robe => {
            // coat tails down to the knees, lapels, buttons
            let len = if look.garment == Garment::Robe { 0.82 } else { 0.5 };
            torso.frustum(Vec3::new(0.0, -len * s, 0.0), hips_r * 1.25, hips_r * 1.05, len * s + 0.02, 12, top);
            torso.frustum(Vec3::new(0.0, -len * s, 0.0), hips_r * 1.26, hips_r * 1.25, 0.02, 12, top_d);
            if look.garment != Garment::Robe {
                for sx in [-1.0f32, 1.0] {
                    torso.quad(
                        Vec3::new(sx * 0.02, 0.3 * s, front - 0.012),
                        Vec3::new(sx * 0.02, th - 0.03, front - 0.012),
                        Vec3::new(sx * 0.12, th - 0.05, front - 0.02),
                        Vec3::new(sx * 0.03, 0.3 * s, front - 0.012),
                        top_d,
                    );
                }
                for k in 0..3 {
                    torso.bx(Vec3::new(0.035, 0.12 * s + k as f32 * 0.1 * s, front - 0.012), Vec3::splat(0.012), c3([0.1, 0.08, 0.06]));
                }
                // scarf or collar
                torso.frustum(Vec3::new(0.0, th - 0.05, 0.0), 0.085, 0.075, 0.06, 8, if r(4) < 0.5 { accent } else { top_d });
            } else {
                torso.bx(Vec3::new(0.0, th - 0.03, front), Vec3::new(0.03, 0.02, 0.01), c3([0.95, 0.95, 0.95]));
            }
        }
        Garment::Suit | Garment::Uniform => {
            // shirt V, tie, lapels, jacket hem
            torso.quad(
                Vec3::new(-0.07, th - 0.02, front - 0.004),
                Vec3::new(0.0, th - 0.2 * s, front - 0.004),
                Vec3::new(0.0, th - 0.2 * s, front - 0.004),
                Vec3::new(0.07, th - 0.02, front - 0.004),
                c3([0.9, 0.88, 0.84]),
            );
            torso.tri(Vec3::new(-0.07, th - 0.02, front - 0.006), Vec3::new(0.07, th - 0.02, front - 0.006), Vec3::new(0.0, th - 0.22 * s, front - 0.006), c3([0.9, 0.88, 0.84]));
            if look.garment == Garment::Suit {
                torso.bx(Vec3::new(0.0, th - 0.15 * s, front - 0.012), Vec3::new(0.018, 0.1 * s, 0.006), accent);
                torso.bx(Vec3::new(0.0, th - 0.035, front - 0.014), Vec3::new(0.022, 0.018, 0.008), accent);
                // pocket square + buttons
                torso.bx(Vec3::new(-0.11, th - 0.17 * s, front - 0.008), Vec3::new(0.025, 0.01, 0.005), c3([0.92, 0.9, 0.86]));
            } else {
                // badge, shoulder straps, brass buttons
                torso.bx(Vec3::new(-0.1, th - 0.17 * s, front - 0.012), Vec3::new(0.022, 0.028, 0.006), c3([0.85, 0.72, 0.3]));
                for x in [-sw, sw] {
                    torso.bx(Vec3::new(x * 0.9, th - 0.02, 0.0), Vec3::new(0.05, 0.012, 0.05), top_d);
                }
            }
            for k in 0..3 {
                torso.bx(Vec3::new(0.0, 0.14 * s + k as f32 * 0.08 * s, front - 0.01), Vec3::splat(0.011), if look.garment == Garment::Uniform { c3([0.85, 0.72, 0.3]) } else { c3([0.08, 0.07, 0.06]) });
            }
            torso.frustum(Vec3::new(0.0, -0.1 * s, 0.0), hips_r * 1.08, hips_r * 1.02, 0.12 * s, 12, top);
        }
        Garment::Apron => {
            torso.quad(Vec3::new(-0.13, -0.4 * s, front - 0.02), Vec3::new(0.13, -0.4 * s, front - 0.02), Vec3::new(0.12, th - 0.12 * s, front - 0.02), Vec3::new(-0.12, th - 0.12 * s, front - 0.02), accent);
            torso.frustum(Vec3::new(0.0, 0.2 * s, 0.0), waist * 1.04, waist * 1.04, 0.02, seg, accent);
        }
        Garment::Overalls => {
            torso.quad(Vec3::new(-0.12, 0.0, front - 0.012), Vec3::new(0.12, 0.0, front - 0.012), Vec3::new(0.11, th * 0.62, front - 0.012), Vec3::new(-0.11, th * 0.62, front - 0.012), bottom);
            for x in [-0.09, 0.06] {
                torso.bx(Vec3::new(x + 0.015, th * 0.82, front - 0.012), Vec3::new(0.018, th * 0.2, 0.006), bottom);
            }
            torso.bx(Vec3::new(0.0, th * 0.45, front - 0.016), Vec3::new(0.05, 0.04, 0.004), c3(shade(look.bottom, 0.8)));
        }
        Garment::Shirt => {
            torso.bx(Vec3::new(0.0, th - 0.04, front * 0.95), Vec3::new(0.07, 0.025, 0.012), top_d);
            for k in 0..4 {
                torso.bx(Vec3::new(0.0, 0.14 * s + k as f32 * 0.08 * s, front - 0.004), Vec3::splat(0.008), c3([0.9, 0.9, 0.88]));
            }
        }
        Garment::Hoodie => {
            torso.sphere(Vec3::new(0.0, th - 0.02, tw * 0.9), Vec3::new(0.13, 0.08, 0.09), 8, top_d);
            torso.bx(Vec3::new(0.0, 0.12 * s, front - 0.01), Vec3::new(0.1, 0.06, 0.01), top_d);
            for x in [-0.03, 0.03] {
                torso.bx(Vec3::new(x, th - 0.12, front - 0.01), Vec3::new(0.004, 0.06, 0.004), c3([0.9, 0.9, 0.9]));
            }
        }
    }
    // neck
    torso.frustum(Vec3::new(0.0, th - 0.06, 0.0), 0.052, 0.046, 0.12, 8, skin);

    // ---------------------------------------------------------------- head (pivot at the neck top)
    let mut head = MB::new();
    let hr = 0.112;
    let jaw_w = 0.8 + r(5) * 0.2 - if fem { 0.08 } else { 0.0 };
    let nose_l = 0.018 + r(6) * 0.018;
    head.sphere(Vec3::new(0.0, hr * 1.05, 0.005), Vec3::new(hr * 0.9, hr * 1.02, hr * 0.98), 12, skin);
    head.sphere(Vec3::new(0.0, hr * 0.62, -hr * 0.18), Vec3::new(hr * 0.78 * jaw_w, hr * 0.55, hr * 0.78), 10, skin);
    // ears
    for x in [-hr * 0.9, hr * 0.9] {
        head.sphere(Vec3::new(x, hr * 1.0, 0.01), Vec3::new(0.018, 0.03, 0.022), 6, skin_d);
    }
    // eyes: white, iris, lid shadow
    let eye_y = hr * 1.1;
    let iris = if r(7) < 0.55 { [0.2, 0.12, 0.06] } else if r(7) < 0.8 { [0.2, 0.35, 0.5] } else { [0.25, 0.35, 0.2] };
    for x in [-0.04, 0.04] {
        head.sphere(Vec3::new(x, eye_y, -hr * 0.86), Vec3::new(0.02, 0.013, 0.01), 6, c3([0.92, 0.9, 0.86]));
        head.sphere(Vec3::new(x, eye_y, -hr * 0.95), Vec3::new(0.009, 0.009, 0.004), 6, c3(iris));
        // eyebrows (hair colour), thicker on men
        head.bx(Vec3::new(x, eye_y + 0.028, -hr * 0.9), Vec3::new(0.024, if fem { 0.004 } else { 0.007 }, 0.006), hair_d);
    }
    // nose + mouth
    head.frustum(Vec3::new(0.0, hr * 0.78, -hr * 0.95), 0.016, 0.009, nose_l, 6, skin_d);
    head.bx(Vec3::new(0.0, hr * 0.78, -hr * 0.95 - nose_l * 0.5), Vec3::new(0.012, 0.028, nose_l * 0.5), skin);
    let lips = if fem { c3([0.62, 0.22, 0.25]) } else { c3(shade(look.skin, 0.7)) };
    head.bx(Vec3::new(0.0, hr * 0.5, -hr * 0.9), Vec3::new(0.026, 0.006, 0.008), lips);
    // wrinkles / age
    if look.age > 55.0 {
        for x in [-0.07, 0.07] {
            head.bx(Vec3::new(x, hr * 1.12, -hr * 0.8), Vec3::new(0.01, 0.002, 0.004), skin_d);
        }
    }
    // facial hair (men)
    if !fem {
        let style = (seed >> 3) % 4;
        if look.beard {
            match style {
                0 => head.sphere(Vec3::new(0.0, hr * 0.45, -hr * 0.35), Vec3::new(hr * 0.72, hr * 0.5, hr * 0.62), 10, hair_c),
                1 => head.bx(Vec3::new(0.0, hr * 0.3, -hr * 0.72), Vec3::new(0.025, 0.035, 0.025), hair_c),
                _ => {
                    head.sphere(Vec3::new(0.0, hr * 0.42, -hr * 0.4), Vec3::new(hr * 0.7, hr * 0.42, hr * 0.58), 10, hair_c);
                }
            }
        }
        if look.beard || r(8) < 0.3 {
            head.bx(Vec3::new(0.0, hr * 0.6, -hr * 0.93), Vec3::new(0.035, 0.009, 0.01), hair_c);
        }
    }
    // hair
    let part = r(9) < 0.5;
    match look.hair {
        Hair::Bald => {
            if look.age > 45.0 {
                for x in [-hr * 0.85, hr * 0.85] {
                    head.sphere(Vec3::new(x, hr * 1.0, hr * 0.3), Vec3::new(0.03, 0.05, 0.08), 6, hair_c);
                }
            }
        }
        Hair::Short => {
            head.sphere(Vec3::new(0.0, hr * 1.28, hr * 0.12), Vec3::new(hr * 0.96, hr * 0.72, hr * 0.94), 12, hair_c);
            // side part or slicked-back quiff
            if part {
                head.bx(Vec3::new(-0.04, hr * 1.83, -hr * 0.2), Vec3::new(0.05, 0.01, 0.06), hair_d);
            } else {
                head.sphere(Vec3::new(0.02, hr * 1.7, -hr * 0.45), Vec3::new(0.08, 0.035, 0.05), 8, hair_c);
            }
            head.sphere(Vec3::new(0.0, hr * 0.95, hr * 0.62), Vec3::new(hr * 0.85, hr * 0.5, hr * 0.4), 8, hair_c);
        }
        Hair::Bob => {
            head.sphere(Vec3::new(0.0, hr * 1.2, hr * 0.12), Vec3::new(hr * 1.08, hr * 0.98, hr * 1.06), 12, hair_c);
            head.sphere(Vec3::new(if part { -0.05 } else { 0.05 }, hr * 1.6, -hr * 0.6), Vec3::new(0.08, 0.03, 0.04), 8, hair_c);
            // finger waves (1920s)
            if r(10) < 0.5 {
                for k in 0..3 {
                    head.bx(Vec3::new(-hr * 0.98, hr * (0.9 + k as f32 * 0.25), 0.0), Vec3::new(0.008, 0.01, 0.08), hair_d);
                }
            }
        }
        Hair::Long => {
            head.sphere(Vec3::new(0.0, hr * 1.2, hr * 0.1), Vec3::new(hr * 1.05, hr * 0.92, hr * 1.02), 12, hair_c);
            // waves falling down the back and over the shoulders
            head.frustum(Vec3::new(0.0, -hr * 1.9, hr * 0.45), hr * 1.0, hr * 0.95, hr * 2.9, 10, hair_c);
            if r(11) < 0.5 {
                for x in [-hr * 0.85, hr * 0.85] {
                    head.frustum(Vec3::new(x, -hr * 1.2, -hr * 0.1), 0.04, 0.035, hr * 2.2, 6, hair_c);
                }
            }
        }
        Hair::Bun => {
            head.sphere(Vec3::new(0.0, hr * 1.22, hr * 0.1), Vec3::new(hr * 1.0, hr * 0.86, hr * 1.0), 12, hair_c);
            let (by, bz) = if r(12) < 0.5 { (hr * 1.55, hr * 0.95) } else { (hr * 2.0, hr * 0.4) };
            head.sphere(Vec3::new(0.0, by, bz), Vec3::splat(hr * 0.45), 8, hair_c);
            head.frustum(Vec3::new(0.0, by - hr * 0.1, bz - hr * 0.4), 0.05, 0.05, 0.015, 8, accent);
        }
        Hair::Afro => {
            head.sphere(Vec3::new(0.0, hr * 1.35, hr * 0.1), Vec3::splat(hr * 1.35), 12, hair_c);
            for k in 0..8 {
                let a = k as f32 / 8.0 * std::f32::consts::TAU;
                head.sphere(Vec3::new(a.cos() * hr * 1.2, hr * (1.3 + (k % 3) as f32 * 0.2), a.sin() * hr * 1.2 + hr * 0.1), Vec3::splat(hr * 0.42), 6, hair_c);
            }
        }
    }
    let hc = c3(look.hat_col);
    let band = c3(shade(look.hat_col, 0.45));
    let top_y = hr * 1.85;
    match look.hat {
        Hat::None => {}
        Hat::Fedora => {
            head.frustum(Vec3::new(0.0, top_y - 0.035, 0.0), 0.2, 0.19, 0.018, 16, hc);
            head.frustum(Vec3::new(0.0, top_y - 0.02, 0.0), 0.115, 0.1, 0.14, 12, hc);
            head.frustum(Vec3::new(0.0, top_y - 0.018, 0.0), 0.117, 0.115, 0.032, 12, band);
            head.bx(Vec3::new(0.0, top_y + 0.115, 0.0), Vec3::new(0.02, 0.006, 0.08), c3(shade(look.hat_col, 0.8)));
        }
        Hat::FlatCap => {
            head.sphere(Vec3::new(0.0, top_y - 0.03, 0.01), Vec3::new(0.135, 0.055, 0.14), 12, hc);
            head.sphere(Vec3::new(0.0, top_y - 0.045, -0.13), Vec3::new(0.1, 0.014, 0.06), 8, hc);
        }
        Hat::Bowler => {
            head.frustum(Vec3::new(0.0, top_y - 0.03, 0.0), 0.165, 0.16, 0.018, 16, hc);
            head.sphere(Vec3::new(0.0, top_y + 0.005, 0.0), Vec3::new(0.112, 0.105, 0.112), 12, hc);
            head.frustum(Vec3::new(0.0, top_y - 0.015, 0.0), 0.114, 0.114, 0.025, 12, band);
        }
        Hat::Cloche => {
            head.sphere(Vec3::new(0.0, top_y - 0.05, 0.02), Vec3::new(0.138, 0.105, 0.138), 12, hc);
            head.frustum(Vec3::new(0.0, top_y - 0.1, 0.0), 0.155, 0.145, 0.025, 12, hc);
            head.sphere(Vec3::new(0.09, top_y - 0.04, -0.08), Vec3::splat(0.025), 6, accent);
        }
        Hat::PoliceCap => {
            head.frustum(Vec3::new(0.0, top_y - 0.05, 0.0), 0.12, 0.145, 0.1, 12, hc);
            head.sphere(Vec3::new(0.0, top_y - 0.06, -0.12), Vec3::new(0.09, 0.012, 0.06), 8, c3([0.04, 0.04, 0.04]));
            head.bx(Vec3::new(0.0, top_y + 0.0, -0.135), Vec3::new(0.022, 0.022, 0.005), c3([0.85, 0.75, 0.3]));
        }
        Hat::Boater => {
            head.frustum(Vec3::new(0.0, top_y - 0.03, 0.0), 0.19, 0.19, 0.012, 16, hc);
            head.frustum(Vec3::new(0.0, top_y - 0.02, 0.0), 0.11, 0.11, 0.08, 12, hc);
            head.frustum(Vec3::new(0.0, top_y, 0.0), 0.112, 0.112, 0.025, 12, c3([0.1, 0.1, 0.15]));
        }
        Hat::Beanie => {
            head.sphere(Vec3::new(0.0, top_y - 0.04, 0.0), Vec3::new(0.128, 0.115, 0.128), 12, hc);
            head.frustum(Vec3::new(0.0, top_y - 0.1, 0.0), 0.13, 0.13, 0.035, 12, band);
        }
        Hat::NurseCap => head.bx(Vec3::new(0.0, top_y, 0.0), Vec3::new(0.08, 0.04, 0.06), c3([0.95, 0.95, 0.95])),
        Hat::Headscarf => {
            head.sphere(Vec3::new(0.0, top_y - 0.06, 0.03), Vec3::new(0.14, 0.115, 0.14), 12, hc);
            head.sphere(Vec3::new(0.0, hr * 0.6, hr * 0.6), Vec3::new(0.05, 0.05, 0.04), 6, hc);
        }
        Hat::Bobby => {
            head.frustum(Vec3::new(0.0, top_y - 0.07, 0.0), 0.14, 0.1, 0.16, 12, hc);
            head.sphere(Vec3::new(0.0, top_y + 0.09, 0.0), Vec3::new(0.1, 0.08, 0.1), 8, hc);
            head.bx(Vec3::new(0.0, top_y + 0.03, -0.12), Vec3::new(0.025, 0.03, 0.005), c3([0.8, 0.75, 0.5]));
        }
        Hat::Shako => {
            head.frustum(Vec3::new(0.0, top_y - 0.05, 0.0), 0.12, 0.13, 0.18, 12, hc);
            head.sphere(Vec3::new(0.0, top_y - 0.06, -0.12), Vec3::new(0.09, 0.012, 0.06), 8, c3([0.04, 0.04, 0.04]));
            head.sphere(Vec3::new(0.0, top_y + 0.15, -0.05), Vec3::splat(0.03), 6, c3([0.8, 0.8, 0.8]));
        }
        Hat::Alpine => {
            head.frustum(Vec3::new(0.0, top_y - 0.03, 0.0), 0.17, 0.17, 0.012, 16, hc);
            head.frustum(Vec3::new(0.0, top_y - 0.02, 0.0), 0.11, 0.07, 0.14, 12, hc);
            head.frustum(Vec3::new(0.0, top_y - 0.015, 0.0), 0.112, 0.105, 0.025, 12, c3([0.3, 0.15, 0.08]));
            head.bx(Vec3::new(0.08, top_y + 0.08, 0.05), Vec3::new(0.005, 0.06, 0.015), c3([0.3, 0.35, 0.25]));
        }
        Hat::Helmet => {
            head.sphere(Vec3::new(0.0, top_y - 0.03, 0.0), Vec3::new(0.145, 0.085, 0.145), 12, hc);
            head.frustum(Vec3::new(0.0, top_y - 0.07, 0.0), 0.2, 0.2, 0.01, 16, hc);
        }
        Hat::Cap => {
            head.sphere(Vec3::new(0.0, top_y - 0.04, 0.0), Vec3::new(0.128, 0.085, 0.128), 12, hc);
            head.sphere(Vec3::new(0.0, top_y - 0.07, -0.15), Vec3::new(0.085, 0.01, 0.065), 8, hc);
        }
    }

    // ---------------------------------------------------------------- limbs (pivot at the top joint)
    let short_sleeves = matches!(look.garment, Garment::Shirt if !fem && r(13) < 0.35) || look.garment == Garment::Dress && r(13) < 0.6;
    let sleeve = if look.garment == Garment::Dress && short_sleeves { skin } else { top };
    let ua = 0.29 * s;
    let fa = 0.26 * s;
    let mut upper = MB::new();
    upper.sphere(Vec3::ZERO, Vec3::splat(0.056 * g), 8, sleeve);
    upper.frustum(Vec3::new(0.0, -ua, 0.0), 0.046 * g, 0.054 * g, ua, 8, sleeve);
    let mut fore = MB::new();
    fore.sphere(Vec3::ZERO, Vec3::splat(0.045 * g), 8, if short_sleeves { skin } else { sleeve });
    fore.frustum(Vec3::new(0.0, -fa, 0.0), 0.034, 0.044 * g, fa, 8, if short_sleeves { skin } else { sleeve });
    if !short_sleeves {
        // cuff
        fore.frustum(Vec3::new(0.0, -fa, 0.0), 0.038, 0.038, 0.03, 8, if matches!(look.garment, Garment::Suit | Garment::Uniform) { c3([0.9, 0.88, 0.84]) } else { top_d });
    }
    // hand: palm + fingers block + thumb
    fore.sphere(Vec3::new(0.0, -fa - 0.045, 0.0), Vec3::new(0.03, 0.045, 0.042), 8, skin);
    fore.sphere(Vec3::new(0.0, -fa - 0.085, -0.005), Vec3::new(0.026, 0.03, 0.036), 6, skin);
    fore.sphere(Vec3::new(0.0, -fa - 0.04, -0.04), Vec3::new(0.012, 0.028, 0.012), 6, skin);
    let thigh = 0.45 * s;
    let shin_l = hip - thigh - 0.02;
    let legwear = match look.garment {
        Garment::Dress => c3(if r(14) < 0.5 { shade(look.skin, 0.95) } else { [0.2, 0.17, 0.16] }),
        _ => bottom,
    };
    let mut thigh_mb = MB::new();
    thigh_mb.sphere(Vec3::ZERO, Vec3::splat(0.075 * g), 8, legwear);
    thigh_mb.frustum(Vec3::new(0.0, -thigh, 0.0), 0.055 * g, 0.075 * g, thigh, 8, legwear);
    let mut shin = MB::new();
    shin.sphere(Vec3::ZERO, Vec3::splat(0.056 * g), 8, legwear);
    shin.frustum(Vec3::new(0.0, -shin_l + 0.06, 0.0), 0.043, 0.056 * g, shin_l - 0.06, 8, legwear);
    // trouser cuff / boots
    if !matches!(look.garment, Garment::Dress) {
        shin.frustum(Vec3::new(0.0, -shin_l + 0.06, 0.0), 0.05, 0.05, 0.03, 8, c3(shade(look.bottom, 0.8)));
    }
    let heel = fem && look.garment == Garment::Dress;
    let boot = matches!(look.garment, Garment::Uniform | Garment::Overalls);
    if boot {
        shin.frustum(Vec3::new(0.0, -shin_l + 0.02, 0.0), 0.052, 0.05, 0.14, 8, shoe);
    }
    shin.sphere(Vec3::new(0.0, -shin_l + 0.035, -0.035), Vec3::new(0.05, 0.035, if heel { 0.08 } else { 0.1 }), 8, shoe);
    shin.bx(Vec3::new(0.0, -shin_l + 0.012, -0.02), Vec3::new(0.048, 0.012, if heel { 0.085 } else { 0.11 }), c3(shade(shoe_c, 0.6)));
    if heel {
        shin.bx(Vec3::new(0.0, -shin_l + 0.03, 0.045), Vec3::new(0.012, 0.03, 0.012), shoe);
    }

    let body_mesh = meshes.add(torso.build());
    let head_mesh = meshes.add(head.build());
    let upper_mesh = meshes.add(upper.build());
    let fore_mesh = meshes.add(fore.build());
    let thigh_mesh = meshes.add(thigh_mb.build());
    let shin_mesh = meshes.add(shin.build());

    let m = |c: &mut Commands, mesh: &Handle<Mesh>, t: Transform| c.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(mat.clone()), t)).id();
    let root = c.spawn((Transform::default(), Visibility::default())).id();
    let body = m(c, &body_mesh, Transform::from_xyz(0.0, hip, 0.0));
    let head_e = m(c, &head_mesh, Transform::from_xyz(0.0, th + 0.05, 0.0));
    let arm_l = m(c, &upper_mesh, Transform::from_xyz(-sw - 0.05 * g, th - 0.07 * s, 0.0));
    let arm_r = m(c, &upper_mesh, Transform::from_xyz(sw + 0.05 * g, th - 0.07 * s, 0.0));
    let fore_l = m(c, &fore_mesh, Transform::from_xyz(0.0, -ua, 0.0));
    let fore_r = m(c, &fore_mesh, Transform::from_xyz(0.0, -ua, 0.0));
    let hand_r = c.spawn((Transform::from_xyz(0.0, -fa - 0.06, 0.0), Visibility::default())).id();
    let leg_l = m(c, &thigh_mesh, Transform::from_xyz(-0.085 * g, hip, 0.0));
    let leg_r = m(c, &thigh_mesh, Transform::from_xyz(0.085 * g, hip, 0.0));
    let shin_le = m(c, &shin_mesh, Transform::from_xyz(0.0, -thigh, 0.0));
    let shin_re = m(c, &shin_mesh, Transform::from_xyz(0.0, -thigh, 0.0));
    c.entity(fore_r).add_child(hand_r);
    c.entity(arm_l).add_child(fore_l);
    c.entity(arm_r).add_child(fore_r);
    c.entity(leg_l).add_child(shin_le);
    c.entity(leg_r).add_child(shin_re);
    c.entity(body).add_children(&[head_e, arm_l, arm_r]);
    c.entity(root).add_children(&[body, leg_l, leg_r]);
    let rig = Rig {
        body,
        head: head_e,
        arm_l,
        arm_r,
        fore_l,
        fore_r,
        leg_l,
        leg_r,
        shin_l: shin_le,
        shin_r: shin_re,
        hand_r,
        phase: r(15) * 6.28,
        pose: Pose::Idle,
        speed: 0.0,
        punch_t: 0.0,
        hip,
        quirk: 0.85 + r(16) * 0.3,
    };
    (root, rig)
}

/// Animate every rig from its pose and speed (with knees, elbows, breathing
/// and a personal rhythm so nobody moves exactly like anybody else).
pub fn animate_rigs(time: Res<Time>, mut rigs: Query<(&mut Rig, &Transform)>, mut parts: Query<&mut Transform, Without<Rig>>) {
    use std::f32::consts::FRAC_PI_2;
    let dt = time.delta_secs();
    let t = time.elapsed_secs();
    for (mut r, _) in rigs.iter_mut() {
        let sp = r.speed;
        let q = r.quirk;
        r.phase += dt * (2.0 + sp * 3.3) * q;
        r.punch_t = (r.punch_t - dt).max(0.0);
        let ph = r.phase;
        let hip = r.hip;
        let (mut body_y, mut body_rx, mut body_rz, mut body_ry) = (hip, 0.0f32, 0.0f32, 0.0f32);
        // shoulder / hip swing (x) and outward (z), elbows and knees (bend >= 0)
        let (mut al, mut ar, mut ll, mut lr) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
        let (mut al_z, mut ar_z) = (0.06f32, -0.06f32);
        let (mut el, mut er, mut kl, mut kr) = (0.15f32, 0.15f32, 0.0f32, 0.0f32);
        let (mut head_rx, mut head_ry) = (0.0f32, 0.0f32);
        let mut leg_y = hip;
        let breathe = (t * 1.6 * q + ph * 0.1).sin();
        match r.pose {
            Pose::Idle | Pose::Talk => {
                body_y += breathe * 0.006;
                // weight shift and looking around
                body_rz = (t * 0.35 * q + ph).sin() * 0.025;
                head_ry = (t * 0.27 * q + ph * 2.0).sin() * 0.35;
                al = 0.04 + breathe * 0.02;
                ar = -0.04 - breathe * 0.02;
                el = 0.2;
                er = 0.2;
                kl = 0.05 + (t * 0.35 * q + ph).sin().max(0.0) * 0.1;
                if r.pose == Pose::Talk {
                    ar = -0.45 + (t * 3.1 * q).sin() * 0.25;
                    er = 1.1 + (t * 2.4).sin() * 0.3;
                    al = -0.2 + (t * 2.2 * q + 1.0).sin() * 0.15;
                    el = 0.7;
                    head_rx = (t * 2.0).sin() * 0.06;
                    head_ry *= 0.3;
                }
            }
            Pose::Walk | Pose::Run | Pose::Sneak | Pose::Carry | Pose::Drag => {
                let amp = match r.pose {
                    Pose::Run => 0.85,
                    Pose::Sneak => 0.4,
                    _ => 0.5,
                } * if r.speed < 0.05 { 0.0 } else { 1.0 };
                let s = ph.sin();
                let c = ph.cos();
                ll = s * amp;
                lr = -s * amp;
                // knee bends while the leg swings forward, straight on contact
                kl = (-c).max(0.0) * amp * 1.6 + 0.05;
                kr = c.max(0.0) * amp * 1.6 + 0.05;
                al = -s * amp * 0.75;
                ar = s * amp * 0.75;
                el = 0.25 + (s.max(0.0)) * amp * 0.9;
                er = 0.25 + ((-s).max(0.0)) * amp * 0.9;
                body_y += (ph * 2.0).cos().abs() * 0.035 * amp - 0.015;
                body_ry = s * 0.08 * amp;
                if r.pose == Pose::Run {
                    body_rx = -0.2;
                    el = 1.3;
                    er = 1.3;
                }
                if r.pose == Pose::Sneak {
                    body_y -= 0.16;
                    leg_y -= 0.1;
                    body_rx = -0.35;
                    kl += 0.5;
                    kr += 0.5;
                    el = 1.2;
                    er = 1.2;
                    al = -0.5 + al * 0.4;
                    ar = -0.5 + ar * 0.4;
                    head_rx = 0.25;
                }
                if r.pose == Pose::Carry {
                    al = -2.6;
                    ar = -2.6;
                    el = 0.6;
                    er = 0.6;
                }
                if r.pose == Pose::Drag {
                    al = 0.7;
                    ar = 0.7;
                    el = 0.1;
                    er = 0.1;
                    body_rx = 0.3;
                }
            }
            Pose::Sit => {
                body_y = hip * 0.52;
                leg_y = hip * 0.52;
                ll = -FRAC_PI_2;
                lr = -FRAC_PI_2 + 0.1;
                kl = FRAC_PI_2;
                kr = FRAC_PI_2 - 0.1;
                al = -0.35;
                ar = -0.3 + breathe * 0.02;
                el = 0.9;
                er = 1.0;
                body_rx = 0.08;
                head_ry = (t * 0.2 * q + ph).sin() * 0.4;
            }
            Pose::Play => {
                body_y = hip * 0.52;
                leg_y = hip * 0.52;
                ll = -FRAC_PI_2;
                lr = -FRAC_PI_2;
                kl = FRAC_PI_2;
                kr = FRAC_PI_2;
                al = -1.0 + (t * 7.0).sin() * 0.12;
                ar = -1.0 + (t * 6.3 + 1.0).sin() * 0.12;
                el = 0.9;
                er = 0.9;
                head_rx = 0.2 + (t * 3.0).sin() * 0.05;
            }
            Pose::Lie | Pose::Dead => {
                body_y = 0.2;
                leg_y = 0.2;
                body_rx = FRAC_PI_2;
                ll = -FRAC_PI_2;
                lr = -FRAC_PI_2 + if r.pose == Pose::Dead { 0.4 } else { 0.0 };
                kl = if r.pose == Pose::Dead { 0.6 } else { 0.05 };
                al = if r.pose == Pose::Dead { 2.4 } else { 0.25 };
                ar = if r.pose == Pose::Dead { 1.0 } else { 0.25 + breathe * 0.02 };
                al_z = if r.pose == Pose::Dead { 0.7 } else { 0.1 };
                el = if r.pose == Pose::Dead { 1.2 } else { 0.4 };
                head_ry = if r.pose == Pose::Dead { 0.8 } else { 0.0 };
                if r.pose == Pose::Lie {
                    body_y += breathe * 0.008;
                }
            }
            Pose::Aim => {
                ar = -FRAC_PI_2;
                er = 0.05;
                al = -1.35;
                al_z = -0.55;
                el = 0.5;
                body_ry = 0.15;
                head_ry = 0.1;
            }
            Pose::Punch => {
                let k = (r.punch_t / 0.3).clamp(0.0, 1.0);
                let ext = (1.0 - (k - 0.5).abs() * 2.0).max(0.0);
                ar = -FRAC_PI_2 * (0.3 + ext * 0.7);
                er = 1.6 * (1.0 - ext);
                al = -0.9;
                el = 1.8;
                body_ry = -0.25 * ext;
                kl = 0.2;
                kr = 0.2;
            }
            Pose::Pray => {
                body_y = hip * 0.66;
                leg_y = hip * 0.66;
                ll = -0.1;
                lr = -0.1;
                kl = FRAC_PI_2 + 0.3;
                kr = FRAC_PI_2 + 0.3;
                al = -0.6;
                ar = -0.6;
                el = 1.5;
                er = 1.5;
                al_z = -0.3;
                ar_z = 0.3;
                head_rx = 0.35;
            }
            Pose::Work => {
                al = -0.6 + (t * 4.0 * q).sin() * 0.3;
                ar = -0.6 + (t * 4.0 * q + 1.5).sin() * 0.3;
                el = 1.1 + (t * 4.0 * q).cos() * 0.3;
                er = 1.1 + (t * 4.0 * q + 1.5).cos() * 0.3;
                body_rx = -0.18;
                head_rx = 0.3;
                kl = 0.1;
                kr = 0.1;
            }
            Pose::Dance => {
                let s = (t * 5.0 * q).sin();
                body_y += (t * 10.0 * q).sin().abs() * 0.05;
                ll = s * 0.4;
                lr = -s * 0.4;
                kl = (s).max(0.0) * 0.7;
                kr = (-s).max(0.0) * 0.7;
                al = -2.3 + s * 0.4;
                ar = -2.3 - s * 0.4;
                el = 0.4;
                er = 0.4;
                body_rz = s * 0.12;
                body_ry = s * 0.3;
                head_ry = -s * 0.2;
            }
            Pose::Cower => {
                body_y = hip * 0.55;
                leg_y = hip * 0.55;
                ll = -1.2;
                lr = -1.3;
                kl = 2.2;
                kr = 2.3;
                al = -2.6;
                ar = -2.5;
                el = 1.9;
                er = 1.9;
                body_rx = -0.45;
                head_rx = 0.5;
                body_rz = (t * 30.0).sin() * 0.01;
            }
        }
        let set = |parts: &mut Query<&mut Transform, Without<Rig>>, e: Entity, f: &dyn Fn(&mut Transform)| {
            if let Ok(mut tr) = parts.get_mut(e) {
                f(&mut tr);
            }
        };
        set(&mut parts, r.body, &|tr| {
            tr.translation.y = body_y;
            tr.rotation = Quat::from_euler(EulerRot::YXZ, body_ry, body_rx, body_rz);
        });
        set(&mut parts, r.head, &|tr| tr.rotation = Quat::from_euler(EulerRot::YXZ, head_ry, head_rx, 0.0));
        set(&mut parts, r.arm_l, &|tr| tr.rotation = Quat::from_euler(EulerRot::XYZ, -al, 0.0, al_z));
        set(&mut parts, r.arm_r, &|tr| tr.rotation = Quat::from_euler(EulerRot::XYZ, -ar, 0.0, ar_z));
        // elbows bend forward, knees bend backward
        set(&mut parts, r.fore_l, &|tr| tr.rotation = Quat::from_rotation_x(el));
        set(&mut parts, r.fore_r, &|tr| tr.rotation = Quat::from_rotation_x(er));
        set(&mut parts, r.leg_l, &|tr| {
            tr.translation.y = leg_y;
            tr.rotation = Quat::from_rotation_x(-ll);
        });
        set(&mut parts, r.leg_r, &|tr| {
            tr.translation.y = leg_y;
            tr.rotation = Quat::from_rotation_x(-lr);
        });
        set(&mut parts, r.shin_l, &|tr| tr.rotation = Quat::from_rotation_x(-kl));
        set(&mut parts, r.shin_r, &|tr| tr.rotation = Quat::from_rotation_x(-kr));
    }
}
