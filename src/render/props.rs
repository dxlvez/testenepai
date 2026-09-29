//! Low-poly geometry for every prop kind. Coordinates: x/z in tiles with the
//! prop's minimum corner at the origin, y up (1 unit = 1 tile ≈ 1 metre).

use super::mesh::{c3, MB};
use crate::city::map::PKind;
use crate::city::style::TreeKind;
use bevy::prelude::*;

pub struct PropGeo {
    pub solid: MB,
    pub glow: MB,
}

fn v(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

pub fn build(kind: PKind, w: f32, h: f32, tint: f32, year: i32, seed: u32, tree: TreeKind, snow: bool) -> PropGeo {
    let mut s = MB::new();
    let mut g = MB::new();
    let t = |c: [f32; 3]| c3([c[0] * tint, c[1] * tint, c[2] * tint]);
    let wood = t([0.36, 0.24, 0.16]);
    let dark_wood = t([0.22, 0.14, 0.10]);
    let metal = c3([0.25, 0.25, 0.27]);
    let cloth = t([0.55, 0.52, 0.48]);
    match kind {
        PKind::Bed => {
            s.cuboid(v(0.08, 0.1, 0.05), v(w - 0.08, 0.42, h - 0.05), dark_wood);
            s.cuboid(v(0.12, 0.42, 0.1), v(w - 0.12, 0.55, h - 0.1), cloth);
            s.cuboid(v(0.15, 0.55, 0.1), v(w - 0.15, 0.65, 0.4), c3([0.8, 0.78, 0.74]));
            let blanket = t([0.35, 0.18, 0.22]);
            s.cuboid(v(0.1, 0.5, h * 0.45), v(w - 0.1, 0.6, h - 0.08), blanket);
            s.cuboid(v(0.05, 0.1, 0.0), v(w - 0.05, 0.9, 0.08), dark_wood);
        }
        PKind::Table => {
            s.cuboid(v(0.08, 0.72, 0.08), v(w - 0.08, 0.8, h - 0.08), wood);
            for (x, z) in [(0.15, 0.15), (w - 0.2, 0.15), (0.15, h - 0.2), (w - 0.2, h - 0.2)] {
                s.cuboid(v(x, 0.1, z), v(x + 0.06, 0.72, z + 0.06), dark_wood);
            }
            if seed % 3 == 0 {
                s.cylinder(v(w * 0.5, 0.8, h * 0.5), 0.05, 0.25, 8, c3([0.2, 0.35, 0.25]));
            }
            if seed % 4 == 1 {
                g.cylinder(v(w * 0.4, 0.8, h * 0.4), 0.04, 0.12, 6, c3([1.0, 0.8, 0.5]));
            }
        }
        PKind::Chair | PKind::Stool => {
            let seat = if kind == PKind::Stool { 0.7 } else { 0.45 };
            s.cuboid(v(0.3, seat - 0.05, 0.3), v(0.7, seat, 0.7), wood);
            for (x, z) in [(0.32, 0.32), (0.62, 0.32), (0.32, 0.62), (0.62, 0.62)] {
                s.cuboid(v(x, 0.1, z), v(x + 0.05, seat - 0.05, z + 0.05), dark_wood);
            }
            if kind == PKind::Chair {
                s.cuboid(v(0.3, seat, 0.3), v(0.7, seat + 0.45, 0.35), wood);
            }
        }
        PKind::Counter => {
            s.cuboid(v(0.0, 0.1, 0.15), v(w, 1.05, h - 0.15), dark_wood);
            s.cuboid(v(-0.02, 1.05, 0.1), v(w + 0.02, 1.12, h - 0.1), wood);
            if seed % 2 == 0 {
                for i in 0..3 {
                    s.cylinder(v(0.2 + i as f32 * 0.25, 1.12, 0.5), 0.04, 0.22, 6, c3([0.25, 0.4, 0.3]));
                }
            }
        }
        PKind::Shelf => {
            s.cuboid(v(0.05, 0.1, 0.2), v(w - 0.05, 2.0, h - 0.2), dark_wood);
            for i in 0..4 {
                let y = 0.4 + i as f32 * 0.42;
                for k in 0..((w * 5.0) as i32) {
                    let hh = 0.18 + ((seed + k as u32 * 7 + i) % 5) as f32 * 0.03;
                    let col = match (seed + k as u32 + i) % 4 {
                        0 => c3([0.5, 0.15, 0.12]),
                        1 => c3([0.18, 0.25, 0.4]),
                        2 => c3([0.6, 0.55, 0.4]),
                        _ => c3([0.2, 0.35, 0.22]),
                    };
                    let x = 0.1 + k as f32 * 0.19;
                    if x + 0.15 < w {
                        s.cuboid(v(x, y, 0.22), v(x + 0.14, y + hh, h - 0.25), col);
                    }
                }
            }
        }
        PKind::Wardrobe => {
            s.cuboid(v(0.05, 0.1, 0.15), v(w - 0.05, 2.1, h - 0.1), dark_wood);
            s.cuboid(v(w * 0.5 - 0.01, 0.2, 0.12), v(w * 0.5 + 0.01, 2.0, 0.15), c3([0.1, 0.07, 0.05]));
            s.cuboid(v(w * 0.5 - 0.1, 1.1, 0.1), v(w * 0.5 - 0.06, 1.2, 0.15), c3([0.7, 0.6, 0.3]));
        }
        PKind::Stove => {
            s.cuboid(v(0.1, 0.1, 0.1), v(w - 0.1, 0.9, h - 0.1), c3([0.12, 0.12, 0.13]));
            s.cylinder(v(w * 0.5, 0.9, h * 0.5), 0.08, 1.3, 8, c3([0.1, 0.1, 0.1]));
            g.cuboid(v(0.25, 0.25, 0.08), v(w - 0.25, 0.45, 0.1), c3([1.0, 0.35, 0.1]));
        }
        PKind::Piano => {
            s.cuboid(v(0.0, 0.1, 0.1), v(w, 1.2, 0.7), c3([0.05, 0.05, 0.06]));
            s.cuboid(v(0.05, 0.7, 0.7), v(w - 0.05, 0.8, 0.95), c3([0.9, 0.9, 0.85]));
            s.cuboid(v(0.4, 0.1, 1.0), v(w - 0.4, 0.5, 1.3), c3([0.08, 0.08, 0.09]));
        }
        PKind::Pew => {
            s.cuboid(v(0.0, 0.4, 0.2), v(w, 0.5, 0.8), wood);
            s.cuboid(v(0.0, 0.5, 0.75), v(w, 1.05, 0.85), wood);
            s.cuboid(v(0.0, 0.1, 0.2), v(0.08, 1.0, 0.85), dark_wood);
            s.cuboid(v(w - 0.08, 0.1, 0.2), v(w, 1.0, 0.85), dark_wood);
        }
        PKind::Altar => {
            s.cuboid(v(0.0, 0.1, 0.1), v(w, 1.0, h - 0.1), c3([0.75, 0.72, 0.66]));
            s.cuboid(v(w * 0.5 - 0.03, 1.0, 0.45), v(w * 0.5 + 0.03, 1.6, 0.55), c3([0.7, 0.6, 0.3]));
            s.cuboid(v(w * 0.5 - 0.18, 1.35, 0.45), v(w * 0.5 + 0.18, 1.41, 0.55), c3([0.7, 0.6, 0.3]));
            for x in [0.15, w - 0.15] {
                s.cylinder(v(x, 1.0, 0.5), 0.04, 0.3, 6, c3([0.9, 0.88, 0.8]));
                g.sphere(v(x, 1.36, 0.5), Vec3::splat(0.05), 6, c3([1.0, 0.7, 0.3]));
            }
        }
        PKind::Desk => {
            s.cuboid(v(0.05, 0.72, 0.1), v(w - 0.05, 0.8, h - 0.1), wood);
            s.cuboid(v(0.05, 0.1, 0.1), v(0.5, 0.72, h - 0.1), dark_wood);
            s.cuboid(v(w - 0.5, 0.1, 0.1), v(w - 0.05, 0.72, h - 0.1), dark_wood);
            s.cuboid(v(0.7, 0.8, 0.3), v(1.0, 0.82, 0.7), c3([0.85, 0.82, 0.75]));
            if year >= 1980 {
                s.cuboid(v(w - 0.8, 0.8, 0.3), v(w - 0.4, 1.2, 0.7), c3([0.75, 0.72, 0.66]));
                g.cuboid(v(w - 0.76, 0.86, 0.28), v(w - 0.44, 1.14, 0.3), c3([0.3, 0.9, 0.5]));
            } else {
                g.sphere(v(w - 0.3, 1.1, 0.5), Vec3::splat(0.09), 8, c3([1.0, 0.85, 0.5]));
                s.cylinder(v(w - 0.3, 0.8, 0.5), 0.02, 0.25, 6, metal);
            }
        }
        PKind::CellBars => {
            for i in 0..((w * 6.0) as i32) {
                let x = i as f32 / 6.0;
                s.cuboid(v(x, 0.1, 0.45), v(x + 0.04, 2.3, 0.5), metal);
            }
            s.cuboid(v(0.0, 2.2, 0.43), v(w, 2.3, 0.52), metal);
        }
        PKind::Crate => {
            s.cuboid(v(0.1, 0.1, 0.1), v(0.9, 0.85, 0.9), t([0.45, 0.33, 0.2]));
            s.cuboid(v(0.08, 0.4, 0.08), v(0.92, 0.5, 0.92), t([0.35, 0.25, 0.15]));
        }
        PKind::Barrel => {
            s.cylinder(v(0.5, 0.1, 0.5), 0.33, 0.9, 12, t([0.35, 0.22, 0.14]));
            s.cylinder(v(0.5, 0.3, 0.5), 0.345, 0.06, 12, metal);
            s.cylinder(v(0.5, 0.75, 0.5), 0.345, 0.06, 12, metal);
        }
        PKind::Stall => {
            s.cuboid(v(0.05, 0.1, 0.2), v(w - 0.05, 0.85, 0.8), wood);
            for (x, z) in [(0.05, 0.15), (w - 0.1, 0.15), (0.05, 0.8), (w - 0.1, 0.8)] {
                s.cuboid(v(x, 0.1, z), v(x + 0.05, 2.1, z + 0.05), dark_wood);
            }
            let awn = match seed % 3 {
                0 => c3([0.5, 0.12, 0.12]),
                1 => c3([0.2, 0.3, 0.4]),
                _ => c3([0.5, 0.45, 0.3]),
            };
            s.quad(v(-0.1, 2.0, 1.0), v(w + 0.1, 2.0, 1.0), v(w + 0.1, 2.3, 0.0), v(-0.1, 2.3, 0.0), awn);
            for i in 0..((w * 3.0) as i32) {
                let col = if (seed + i as u32) % 2 == 0 { c3([0.65, 0.3, 0.1]) } else { c3([0.3, 0.45, 0.15]) };
                s.sphere(v(0.2 + i as f32 * 0.3, 0.92, 0.5), Vec3::splat(0.1), 6, col);
            }
        }
        PKind::Sofa => {
            let col = t([0.35, 0.12, 0.15]);
            s.cuboid(v(0.05, 0.1, 0.15), v(w - 0.05, 0.5, h - 0.1), col);
            s.cuboid(v(0.05, 0.5, h - 0.35), v(w - 0.05, 0.95, h - 0.1), col);
            s.cuboid(v(0.05, 0.5, 0.15), v(0.25, 0.7, h - 0.1), col);
            s.cuboid(v(w - 0.25, 0.5, 0.15), v(w - 0.05, 0.7, h - 0.1), col);
        }
        PKind::RadioSet => {
            let col = if year < 1950 { t([0.4, 0.25, 0.14]) } else { c3([0.2, 0.2, 0.22]) };
            s.cuboid(v(0.2, 0.1, 0.3), v(0.8, 0.9, 0.75), col);
            s.cuboid(v(0.28, 0.4, 0.28), v(0.72, 0.8, 0.3), c3([0.5, 0.45, 0.35]));
            g.sphere(v(0.35, 0.3, 0.29), Vec3::splat(0.04), 6, c3([1.0, 0.6, 0.2]));
        }
        PKind::Bathtub => {
            s.cuboid(v(0.1, 0.1, 0.1), v(w - 0.1, 0.6, h - 0.1), c3([0.85, 0.85, 0.82]));
        }
        PKind::Slab => {
            s.cuboid(v(0.15, 0.85, 0.05), v(w - 0.15, 0.95, h - 0.05), c3([0.55, 0.57, 0.6]));
            s.cuboid(v(0.4, 0.1, 0.3), v(w - 0.4, 0.85, h - 0.3), metal);
            s.cuboid(v(0.2, 0.95, 0.2), v(w - 0.2, 1.1, h - 0.2), c3([0.85, 0.85, 0.85]));
        }
        PKind::LampPost => {
            let col = c3([0.08, 0.08, 0.09]);
            if year < 1950 {
                s.cylinder(v(0.5, 0.0, 0.5), 0.07, 3.2, 8, col);
                s.cylinder(v(0.5, 0.0, 0.5), 0.14, 0.3, 8, col);
                s.bx(v(0.5, 3.35, 0.5), v(0.16, 0.05, 0.16), col);
                g.bx(v(0.5, 3.15, 0.5), v(0.12, 0.17, 0.12), c3([1.0, 0.82, 0.55]));
                s.bx(v(0.5, 2.95, 0.5), v(0.14, 0.03, 0.14), col);
                s.cuboid(v(0.3, 2.6, 0.48), v(0.7, 2.64, 0.52), col);
            } else {
                // modern cobra-head street light
                let grey = c3([0.4, 0.4, 0.42]);
                s.cylinder(v(0.5, 0.0, 0.5), 0.08, 4.2, 8, grey);
                s.cuboid(v(0.46, 4.1, 0.46), v(1.4, 4.2, 0.54), grey);
                s.cuboid(v(1.2, 4.0, 0.38), v(1.8, 4.18, 0.62), grey);
                g.cuboid(v(1.25, 3.98, 0.42), v(1.75, 4.0, 0.58), c3([1.0, 0.75, 0.45]));
            }
        }
        PKind::Tree => {
            let trunk = c3([0.18, 0.13, 0.1]);
            let hgt = 1.8 + (seed % 5) as f32 * 0.3;
            let snowy = |c: [f32; 4]| if snow { c3([0.8, 0.82, 0.85]) } else { c };
            match tree {
                TreeKind::Pine => {
                    s.frustum(v(0.5, 0.0, 0.5), 0.12, 0.08, hgt * 0.6, 7, trunk);
                    let leaf = c3([0.06, 0.13, 0.08]);
                    for k in 0..4 {
                        let y = hgt * 0.4 + k as f32 * 0.8;
                        let r = 1.2 - k as f32 * 0.25;
                        s.frustum(v(0.5, y, 0.5), r, 0.05, 1.3, 9, if k == 3 { snowy(leaf) } else { leaf });
                    }
                }
                TreeKind::Palm => {
                    let h2 = hgt * 2.6;
                    for k in 0..6 {
                        let y = k as f32 * h2 / 6.0;
                        let dx = (k as f32 * 0.4).sin() * 0.15;
                        s.frustum(v(0.5 + dx, y, 0.5), 0.13, 0.1, h2 / 6.0 + 0.02, 7, c3([0.35, 0.28, 0.2]));
                    }
                    let leaf = c3([0.18, 0.3, 0.12]);
                    for k in 0..7 {
                        let a = k as f32 / 7.0 * std::f32::consts::TAU;
                        let (sa, ca) = a.sin_cos();
                        let tip = v(0.5 + ca * 1.8, h2 - 0.6, 0.5 + sa * 1.8);
                        let top = v(0.5, h2 + 0.1, 0.5);
                        let side = v(-sa * 0.25, 0.0, ca * 0.25);
                        s.quad(top - side, top + side, tip + side * 0.3, tip - side * 0.3, leaf);
                    }
                }
                TreeKind::Eucalyptus | TreeKind::Birch => {
                    let bark = if tree == TreeKind::Birch { c3([0.8, 0.78, 0.74]) } else { c3([0.6, 0.55, 0.48]) };
                    s.frustum(v(0.5, 0.0, 0.5), 0.12, 0.06, hgt * 1.5, 7, bark);
                    let leaf = if tree == TreeKind::Birch { snowy(c3([0.25, 0.32, 0.12])) } else { c3([0.28, 0.36, 0.28]) };
                    s.sphere(v(0.5, hgt * 1.5, 0.5), v(0.9, 1.1, 0.9), 8, leaf);
                    s.sphere(v(0.2, hgt * 1.2, 0.7), v(0.6, 0.7, 0.6), 7, leaf);
                }
                TreeKind::Oak => {
                    s.frustum(v(0.5, 0.0, 0.5), 0.14, 0.08, hgt, 7, trunk);
                    let leaf = snowy(match seed % 3 {
                        0 => c3([0.1, 0.16, 0.1]),
                        1 => c3([0.12, 0.14, 0.09]),
                        _ => c3([0.08, 0.13, 0.11]),
                    });
                    let r = 1.0 + (seed % 4) as f32 * 0.2;
                    s.sphere(v(0.5, hgt + 0.3, 0.5), v(r, r * 0.8, r), 8, leaf);
                    s.sphere(v(0.1, hgt, 0.8), v(r * 0.7, r * 0.6, r * 0.7), 7, leaf);
                    s.sphere(v(0.9, hgt - 0.1, 0.2), v(r * 0.6, r * 0.5, r * 0.6), 7, leaf);
                    if seed % 2 == 0 && !snow {
                        for i in 0..4 {
                            let a = i as f32 * 1.6;
                            s.cuboid(v(0.5 + a.cos() * r * 0.8, hgt - 0.6, 0.5 + a.sin() * r * 0.8), v(0.55 + a.cos() * r * 0.8, hgt, 0.55 + a.sin() * r * 0.8), c3([0.35, 0.38, 0.3]));
                        }
                    }
                }
            }
        }
        PKind::Bench => {
            s.cuboid(v(0.0, 0.42, 0.25), v(w, 0.48, 0.75), wood);
            s.cuboid(v(0.0, 0.5, 0.7), v(w, 0.9, 0.76), wood);
            for x in [0.1, w - 0.15] {
                s.cuboid(v(x, 0.0, 0.25), v(x + 0.06, 0.45, 0.75), metal);
            }
        }
        PKind::Grave => {
            let stone = t([0.45, 0.45, 0.46]);
            if seed % 3 == 0 {
                // raised New Orleans style tomb
                s.cuboid(v(0.05, 0.0, 0.1), v(0.95, 1.2, 1.9), stone);
                s.gable(0.0, 0.05, 1.0, 1.95, 1.2, 0.4, stone);
                s.cuboid(v(0.4, 1.6, 0.9), v(0.6, 2.0, 1.1), stone);
            } else {
                s.cuboid(v(0.2, 0.0, 0.2), v(0.8, 0.9, 0.35), stone);
                s.cuboid(v(0.15, 0.0, 0.4), v(0.85, 0.12, 1.8), c3([0.2, 0.18, 0.15]));
            }
        }
        PKind::Fountain => {
            let stone = c3([0.5, 0.49, 0.47]);
            s.frustum(v(w / 2.0, 0.0, h / 2.0), 1.4, 1.4, 0.5, 16, stone);
            s.frustum(v(w / 2.0, 0.0, h / 2.0), 0.25, 0.2, 1.5, 8, stone);
            s.frustum(v(w / 2.0, 1.5, h / 2.0), 0.6, 0.2, 0.2, 10, stone);
            g.frustum(v(w / 2.0, 0.45, h / 2.0), 1.25, 1.25, 0.02, 16, c3([0.05, 0.08, 0.12]));
        }
        PKind::Dumpster => {
            s.cuboid(v(0.05, 0.05, 0.1), v(0.95, 1.0, 0.9), c3([0.15, 0.22, 0.18]));
            s.cuboid(v(0.03, 1.0, 0.08), v(0.97, 1.08, 0.92), c3([0.1, 0.14, 0.12]));
        }
        PKind::Phone => {
            if year < 1940 {
                s.cuboid(v(0.3, 0.9, 0.6), v(0.7, 1.5, 0.8), t([0.35, 0.2, 0.12]));
                s.cylinder(v(0.5, 0.0, 0.7), 0.05, 0.9, 6, metal);
            } else {
                // phone booth
                let col = if year < 1980 { c3([0.45, 0.08, 0.08]) } else { c3([0.3, 0.32, 0.35]) };
                s.cuboid(v(0.1, 0.0, 0.1), v(0.9, 2.3, 0.2), col);
                s.cuboid(v(0.1, 0.0, 0.1), v(0.2, 2.3, 0.9), col);
                s.cuboid(v(0.8, 0.0, 0.1), v(0.9, 2.3, 0.9), col);
                s.cuboid(v(0.1, 2.3, 0.1), v(0.9, 2.45, 0.9), col);
                g.cuboid(v(0.2, 2.3, 0.85), v(0.8, 2.42, 0.9), c3([1.0, 0.95, 0.85]));
            }
        }
        PKind::NewsStand => {
            s.cuboid(v(0.05, 0.0, 0.2), v(0.95, 1.1, 0.9), t([0.3, 0.25, 0.2]));
            s.cuboid(v(0.0, 1.1, 0.1), v(1.0, 1.2, 1.0), t([0.25, 0.08, 0.08]));
            for i in 0..3 {
                s.cuboid(v(0.1 + i as f32 * 0.28, 0.7, 0.15), v(0.33 + i as f32 * 0.28, 1.0, 0.2), c3([0.85, 0.82, 0.75]));
            }
        }
        PKind::Hay => {
            s.cuboid(v(0.05, 0.0, 0.1), v(0.95, 0.7, 0.9), c3([0.6, 0.5, 0.25]));
        }
        PKind::Rug => {
            let col = t([0.4, 0.12, 0.14]);
            s.cuboid(v(0.1, 0.1, 0.1), v(w - 0.1, 0.115, h - 0.1), col);
            s.cuboid(v(0.25, 0.115, 0.25), v(w - 0.25, 0.12, h - 0.25), t([0.55, 0.4, 0.2]));
        }
        PKind::Plant => {
            s.frustum(v(0.5, 0.1, 0.5), 0.18, 0.24, 0.4, 8, t([0.45, 0.25, 0.18]));
            s.sphere(v(0.5, 0.75, 0.5), v(0.35, 0.4, 0.35), 7, c3([0.12, 0.25, 0.12]));
        }
        PKind::Safe => {
            s.cuboid(v(0.15, 0.1, 0.15), v(0.85, 1.0, 0.85), c3([0.15, 0.16, 0.18]));
            s.cylinder(v(0.5, 0.5, 0.13), 0.08, 0.03, 8, c3([0.6, 0.55, 0.3]));
        }
        PKind::Board => {
            s.cuboid(v(0.0, 0.9, 0.35), v(w, 2.1, 0.45), c3([0.5, 0.36, 0.22]));
            for i in 0..6 {
                let x = 0.15 + (i % 3) as f32 * 0.55;
                let y = 1.05 + (i / 3) as f32 * 0.5;
                s.cuboid(v(x, y, 0.33), v(x + 0.35, y + 0.35, 0.35), c3([0.85, 0.82, 0.72]));
            }
            // red thread
            g.cuboid(v(0.3, 1.2, 0.32), v(1.5, 1.22, 0.33), c3([1.0, 0.05, 0.1]));
            g.cuboid(v(0.8, 1.2, 0.32), v(0.82, 1.7, 0.33), c3([1.0, 0.05, 0.1]));
        }
        PKind::Sphere => {
            // handled specially by the sphere entity
        }
        PKind::Typewriter => {
            s.cuboid(v(0.25, 0.8, 0.3), v(0.75, 0.95, 0.7), c3([0.1, 0.1, 0.1]));
            s.cuboid(v(0.3, 0.95, 0.55), v(0.7, 1.05, 0.65), c3([0.12, 0.12, 0.12]));
            s.cuboid(v(0.35, 1.0, 0.6), v(0.65, 1.25, 0.61), c3([0.9, 0.88, 0.8]));
        }
        PKind::Mirror => {
            s.cuboid(v(0.2, 0.1, 0.4), v(0.8, 2.0, 0.5), dark_wood);
            s.cuboid(v(0.27, 0.2, 0.38), v(0.73, 1.9, 0.4), c3([0.55, 0.6, 0.65]));
        }
        PKind::Car => {}
        PKind::Rubble => {
            let c1 = t([0.35, 0.28, 0.24]);
            s.cuboid(v(0.05, 0.0, 0.1), v(0.7, 0.35, 0.6), c1);
            s.cuboid(v(0.4, 0.0, 0.4), v(0.95, 0.5, 0.9), t([0.3, 0.3, 0.3]));
            s.cuboid(v(0.2, 0.3, 0.3), v(0.5, 0.6, 0.45), t([0.25, 0.18, 0.12]));
        }
        PKind::Trough => {
            s.cuboid(v(0.05, 0.0, 0.2), v(w - 0.05, 0.6, 0.8), wood);
            g.floor(0.12, 0.26, w - 0.12, 0.74, 0.55, c3([0.06, 0.08, 0.1]));
            s.cuboid(v(0.1, 0.0, 0.95), v(0.18, 1.0, 1.0), dark_wood);
        }
        PKind::Pillar => {
            s.cuboid(v(0.3, 0.0, 0.3), v(0.7, 5.0, 0.7), c3([0.15, 0.16, 0.15]));
            s.cuboid(v(0.2, 4.6, 0.2), v(0.8, 5.0, 0.8), c3([0.12, 0.13, 0.12]));
        }
        PKind::Hydrant => {
            let red = c3([0.6, 0.08, 0.06]);
            s.cylinder(v(0.5, 0.0, 0.5), 0.12, 0.6, 8, red);
            s.sphere(v(0.5, 0.6, 0.5), v(0.12, 0.08, 0.12), 6, red);
            s.cuboid(v(0.35, 0.35, 0.45), v(0.65, 0.42, 0.55), red);
        }
        PKind::Barrier => {
            s.cuboid(v(0.4, 0.0, 0.4), v(0.6, 1.0, 0.6), c3([0.3, 0.3, 0.3]));
            for k in 0..6 {
                let col = if k % 2 == 0 { c3([0.8, 0.1, 0.1]) } else { c3([0.9, 0.9, 0.9]) };
                s.cuboid(v(0.5 + k as f32 * 0.5, 0.9, 0.45), v(1.0 + k as f32 * 0.5, 1.0, 0.55), col);
            }
            s.cuboid(v(-0.5, 0.0, 1.0), v(0.9, 2.2, 2.2), c3([0.5, 0.52, 0.45]));
            g.cuboid(v(-0.4, 1.4, 0.98), v(0.8, 1.9, 1.0), c3([1.0, 0.85, 0.5]));
        }
        PKind::Sandbags => {
            let sb = c3([0.55, 0.5, 0.38]);
            for k in 0..3 {
                s.sphere(v(0.5, 0.15 + k as f32 * 0.22, 0.5), v(0.45 - k as f32 * 0.05, 0.12, 0.3), 6, sb);
            }
        }
    }
    PropGeo { solid: s, glow: g }
}
