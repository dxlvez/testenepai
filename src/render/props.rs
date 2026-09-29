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
    if furniture(kind, &mut s, &mut g, w, h, tint, year, seed) {
        return PropGeo { solid: s, glow: g };
    }
    match kind {
        PKind::Counter => {
            // panelled wooden bar/shop counter, polished top, brass foot rail
            s.cuboid(v(0.0, 0.0, 0.15), v(w, 1.02, h - 0.15), dark_wood);
            for i in 0..((w / 0.5).max(1.0) as i32) {
                let x = i as f32 * 0.5;
                for z in [0.14, h - 0.16] {
                    s.cuboid(v(x + 0.06, 0.15, z - 0.005), v(x + 0.44, 0.85, z + 0.005), wood);
                }
            }
            s.cuboid(v(-0.02, 1.02, 0.08), v(w + 0.02, 1.1, h - 0.08), wood);
            for z in [0.02, h - 0.02] {
                s.rod(v(0.0, 0.18, z), v(w, 0.18, z), 0.022, c3([0.75, 0.6, 0.28]));
            }
            match seed % 5 {
                0 => {
                    // beer taps
                    for i in 0..3 {
                        let x = 0.2 + i as f32 * 0.25;
                        s.cylinder(v(x, 1.1, h * 0.5), 0.03, 0.12, 8, c3([0.75, 0.75, 0.78]));
                        s.cylinder(v(x, 1.22, h * 0.5), 0.015, 0.22, 6, c3(pick(&[[0.1, 0.1, 0.1], [0.6, 0.1, 0.1], [0.8, 0.7, 0.3]], seed, i + 3)));
                    }
                }
                1 => {
                    // cash register
                    let col = if year < 1950 { c3([0.72, 0.58, 0.28]) } else { c3([0.3, 0.3, 0.32]) };
                    s.cuboid(v(0.2, 1.1, 0.25), v(0.8, 1.35, h - 0.25), col);
                    s.cuboid(v(0.25, 1.35, 0.35), v(0.75, 1.5, 0.6), col);
                    for k in 0..4 {
                        s.cylinder(v(0.3 + k as f32 * 0.12, 1.35, 0.3), 0.02, 0.03, 6, c3([0.9, 0.88, 0.8]));
                    }
                }
                2 => {
                    // glasses and a bottle
                    for i in 0..3 {
                        s.frustum(v(0.2 + i as f32 * 0.2, 1.1, 0.45), 0.03, 0.04, 0.11, 8, c3([0.75, 0.8, 0.82]));
                    }
                    s.cylinder(v(0.8, 1.1, 0.55), 0.035, 0.22, 8, c3([0.3, 0.2, 0.08]));
                }
                3 => {
                    // ledger / newspapers
                    s.cuboid(v(0.25, 1.1, 0.3), v(0.75, 1.13, 0.7), c3([0.85, 0.82, 0.72]));
                }
                _ => {}
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
        PKind::RadioSet => {
            let col = if year < 1950 { t([0.4, 0.25, 0.14]) } else { c3([0.2, 0.2, 0.22]) };
            s.cuboid(v(0.2, 0.1, 0.3), v(0.8, 0.9, 0.75), col);
            s.cuboid(v(0.28, 0.4, 0.28), v(0.72, 0.8, 0.3), c3([0.5, 0.45, 0.35]));
            g.sphere(v(0.35, 0.3, 0.29), Vec3::splat(0.04), 6, c3([1.0, 0.6, 0.2]));
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
        _ => {}
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

// ---------------------------------------------------------------- home furniture with variety

const WOODS: [[f32; 3]; 7] = [[0.36, 0.24, 0.16], [0.22, 0.14, 0.1], [0.5, 0.36, 0.22], [0.28, 0.16, 0.1], [0.6, 0.5, 0.36], [0.16, 0.1, 0.08], [0.42, 0.3, 0.2]];
const FABRICS: [[f32; 3]; 12] = [
    [0.55, 0.14, 0.16],
    [0.18, 0.3, 0.45],
    [0.62, 0.55, 0.38],
    [0.25, 0.4, 0.28],
    [0.45, 0.3, 0.5],
    [0.75, 0.7, 0.62],
    [0.15, 0.15, 0.18],
    [0.7, 0.45, 0.2],
    [0.35, 0.5, 0.55],
    [0.6, 0.35, 0.4],
    [0.3, 0.22, 0.15],
    [0.8, 0.78, 0.72],
];
const SHEETS: [[f32; 3]; 4] = [[0.9, 0.88, 0.84], [0.85, 0.85, 0.8], [0.78, 0.82, 0.86], [0.88, 0.84, 0.76]];

fn pick<T: Copy>(a: &[T], seed: u32, salt: u32) -> T {
    a[(crate::util::hash2(seed as i32, salt as i32, 313) as usize) % a.len()]
}

fn roll(seed: u32, salt: u32) -> f32 {
    (crate::util::hash2(seed as i32, salt as i32, 717) & 0xFFFF) as f32 / 65535.0
}

fn k(c: [f32; 3], f: f32) -> [f32; 4] {
    c3([(c[0] * f).min(1.0), (c[1] * f).min(1.0), (c[2] * f).min(1.0)])
}

/// Small clutter on a surface at height y (books, cups, bottles, vases...).
fn clutter(s: &mut MB, g: &mut MB, x: f32, y: f32, z: f32, seed: u32, year: i32) {
    match seed % 7 {
        0 => {
            // stack of books
            for i in 0..3 {
                let c = pick(&FABRICS, seed, 40 + i);
                s.cuboid(v(x - 0.1, y + i as f32 * 0.04, z - 0.07), v(x + 0.08 - i as f32 * 0.01, y + 0.04 + i as f32 * 0.04, z + 0.07), k(c, 0.8));
            }
        }
        1 => {
            // cup and saucer
            s.cylinder(v(x, y, z), 0.07, 0.01, 10, c3([0.9, 0.9, 0.88]));
            s.frustum(v(x, y + 0.01, z), 0.035, 0.045, 0.07, 10, c3([0.92, 0.92, 0.9]));
        }
        2 => {
            // bottle
            let c = if roll(seed, 5) < 0.5 { [0.15, 0.3, 0.12] } else { [0.35, 0.2, 0.08] };
            s.cylinder(v(x, y, z), 0.04, 0.2, 8, c3(c));
            s.frustum(v(x, y + 0.2, z), 0.04, 0.015, 0.08, 8, c3(c));
        }
        3 => {
            // vase with flowers
            s.frustum(v(x, y, z), 0.05, 0.035, 0.18, 8, k(pick(&FABRICS, seed, 7), 0.9));
            for i in 0..4 {
                let a = i as f32 * 1.6;
                let fc = pick(&[[0.9, 0.2, 0.25], [0.95, 0.85, 0.3], [0.9, 0.9, 0.9], [0.7, 0.3, 0.7]], seed, 8 + i);
                s.sphere(v(x + a.cos() * 0.05, y + 0.26, z + a.sin() * 0.05), Vec3::splat(0.035), 6, c3(fc));
            }
        }
        4 => {
            // candle or small lamp
            if year < 1925 {
                s.cylinder(v(x, y, z), 0.05, 0.02, 8, c3([0.6, 0.5, 0.3]));
                s.cylinder(v(x, y + 0.02, z), 0.018, 0.12, 6, c3([0.92, 0.9, 0.82]));
                g.sphere(v(x, y + 0.16, z), Vec3::new(0.012, 0.025, 0.012), 6, c3([1.0, 0.75, 0.35]));
            } else {
                s.cylinder(v(x, y, z), 0.06, 0.02, 8, c3([0.3, 0.25, 0.2]));
                s.cylinder(v(x, y + 0.02, z), 0.012, 0.25, 6, c3([0.5, 0.45, 0.3]));
                g.frustum(v(x, y + 0.22, z), 0.11, 0.06, 0.12, 10, k(pick(&[[0.95, 0.85, 0.6], [0.9, 0.7, 0.5], [0.8, 0.85, 0.7]], seed, 9), 1.0));
            }
        }
        5 => {
            // newspaper
            s.cuboid(v(x - 0.14, y, z - 0.1), v(x + 0.14, y + 0.01, z + 0.1), c3([0.82, 0.8, 0.72]));
        }
        _ => {
            // plate with food
            s.cylinder(v(x, y, z), 0.11, 0.015, 12, c3([0.92, 0.92, 0.9]));
            s.sphere(v(x, y + 0.02, z), Vec3::new(0.06, 0.025, 0.05), 6, c3(pick(&[[0.6, 0.35, 0.15], [0.8, 0.7, 0.3], [0.4, 0.5, 0.2]], seed, 10)));
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn furniture(kind: PKind, s: &mut MB, g: &mut MB, w: f32, h: f32, tint: f32, year: i32, seed: u32) -> bool {
    let rich = tint > 1.08;
    let poor = tint < 0.9;
    let wood_c = pick(&WOODS, seed, 1);
    let wood = k(wood_c, tint.min(1.1));
    let wood_d = k(wood_c, 0.65);
    let metal = c3([0.25, 0.25, 0.27]);
    let brass = c3([0.72, 0.58, 0.28]);
    let fab = pick(&FABRICS, seed, 2);
    let style = seed % 4;
    match kind {
        PKind::Bed => {
            let (x0, x1, z0, z1) = (0.05, w - 0.05, 0.05, h - 0.05);
            let top = if year >= 1955 && style >= 2 { 0.42 } else { 0.55 };
            let st = if rich && style == 0 { 3 } else if year < 1950 && style == 1 { 1 } else if year >= 1955 && style >= 2 { 2 } else { 0 };
            match st {
                1 => {
                    // brass / iron bed
                    let col = if roll(seed, 3) < 0.5 { brass } else { c3([0.15, 0.15, 0.16]) };
                    for (x, z, hh) in [(x0, z0, 1.2), (x1, z0, 1.2), (x0, z1, 0.85), (x1, z1, 0.85)] {
                        s.cylinder(v(x, 0.0, z), 0.025, hh, 6, col);
                        s.sphere(v(x, hh, z), Vec3::splat(0.04), 6, col);
                    }
                    for (z, hh) in [(z0, 1.05), (z1, 0.75)] {
                        s.cuboid(v(x0, hh, z - 0.012), v(x1, hh + 0.025, z + 0.012), col);
                        s.cuboid(v(x0, 0.35, z - 0.012), v(x1, 0.38, z + 0.012), col);
                        let n = ((x1 - x0) / 0.12) as i32;
                        for i in 1..n {
                            let x = x0 + i as f32 * (x1 - x0) / n as f32;
                            s.cuboid(v(x - 0.008, 0.38, z - 0.008), v(x + 0.008, hh, z + 0.008), col);
                        }
                    }
                    s.cuboid(v(x0, 0.25, z0), v(x1, 0.32, z1), col);
                }
                2 => {
                    // modern platform bed with upholstered headboard
                    s.cuboid(v(x0 - 0.02, 0.05, z0), v(x1 + 0.02, 0.28, z1), wood);
                    s.cuboid(v(x0 - 0.05, 0.28, z0 - 0.03), v(x1 + 0.05, 1.05, z0 + 0.08), k(fab, 0.8));
                }
                3 => {
                    // four-poster with canopy
                    for (x, z) in [(x0, z0), (x1, z0), (x0, z1), (x1, z1)] {
                        s.cylinder(v(x, 0.0, z), 0.05, 2.0, 8, wood_d);
                        s.sphere(v(x, 0.5, z), Vec3::splat(0.065), 6, wood_d);
                    }
                    s.cuboid(v(x0, 1.95, z0), v(x1, 2.05, z1), wood_d);
                    s.quad(v(x0, 2.0, z0), v(x1, 2.0, z0), v(x1, 1.6, z0 - 0.01), v(x0, 1.6, z0 - 0.01), k(fab, 0.9));
                    s.cuboid(v(x0, 0.2, z0), v(x1, 0.34, z1), wood_d);
                    s.cuboid(v(x0 + 0.05, 0.34, z0 - 0.02), v(x1 - 0.05, 1.3, z0 + 0.04), wood);
                }
                _ => {
                    // turned-wood bed
                    for (x, z, hh) in [(x0, z0, 1.1), (x1, z0, 1.1), (x0, z1, 0.75), (x1, z1, 0.75)] {
                        s.cylinder(v(x, 0.0, z), 0.045, hh, 8, wood_d);
                        s.sphere(v(x, hh, z), Vec3::splat(0.055), 6, wood_d);
                    }
                    s.cuboid(v(x0, 0.2, z0), v(x1, 0.34, z1), wood_d);
                    s.cuboid(v(x0 + 0.04, 0.4, z0 - 0.02), v(x1 - 0.04, 1.0, z0 + 0.03), wood);
                    s.cuboid(v(x0 + 0.04, 0.4, z1 - 0.03), v(x1 - 0.04, 0.68, z1 + 0.02), wood);
                    if roll(seed, 4) < 0.5 {
                        // carved arch on the headboard
                        s.sphere(v(w * 0.5, 1.0, z0), Vec3::new((x1 - x0) * 0.4, 0.12, 0.03), 10, wood);
                    }
                }
            }
            // mattress, sheet, pillows, blanket (made or messy)
            let sheet = pick(&SHEETS, seed, 5);
            s.cuboid(v(x0 + 0.04, top - 0.2, z0 + 0.06), v(x1 - 0.04, top, z1 - 0.04), k(sheet, 0.95));
            let pillows = if w > 1.5 { 2 } else { 1 + (seed % 2) as i32 };
            for i in 0..pillows {
                let px = x0 + (x1 - x0) * (i as f32 + 0.5) / pillows as f32;
                s.sphere(v(px, top + 0.05, z0 + 0.25), Vec3::new((x1 - x0) * 0.4 / pillows as f32, 0.07, 0.15), 8, k(sheet, 1.0));
            }
            let blanket = pick(&FABRICS, seed, 6);
            let messy = roll(seed, 7) < 0.35;
            let bz = if messy { h * 0.55 } else { h * 0.38 };
            s.cuboid(v(x0 + 0.02, top - 0.12, bz), v(x1 - 0.02, top + 0.06, z1 - 0.02), k(blanket, 0.95));
            // folded sheet edge
            s.cuboid(v(x0 + 0.03, top + 0.06, bz), v(x1 - 0.03, top + 0.075, bz + 0.12), k(sheet, 1.0));
            if roll(seed, 8) < 0.5 {
                // quilt pattern squares
                let n = 4;
                for i in 0..n {
                    for j in 0..3 {
                        if (i + j) % 2 == 0 {
                            let (qx, qz) = (x0 + 0.05 + i as f32 * (x1 - x0 - 0.1) / n as f32, bz + 0.15 + j as f32 * (z1 - bz - 0.2) / 3.0);
                            s.floor(qx, qz, qx + (x1 - x0 - 0.1) / n as f32, qz + (z1 - bz - 0.2) / 3.0, top + 0.062, k(pick(&FABRICS, seed, 20 + i as u32), 0.9));
                        }
                    }
                }
            }
            if messy {
                // clothes thrown on the bed
                s.cuboid(v(x0 + 0.2, top + 0.06, z1 - 0.5), v(x0 + 0.55, top + 0.1, z1 - 0.15), k(pick(&FABRICS, seed, 30), 0.8));
            }
        }
        PKind::Nightstand => {
            s.cuboid(v(0.2, 0.0, 0.2), v(0.8, 0.55, 0.8), wood);
            s.cuboid(v(0.22, 0.32, 0.18), v(0.78, 0.48, 0.2), wood_d);
            s.sphere(v(0.5, 0.4, 0.175), Vec3::splat(0.02), 6, brass);
            clutter(s, g, 0.5, 0.55, 0.5, 4 + (seed % 2) * 3, year);
            if seed % 3 == 0 {
                clutter(s, g, 0.3, 0.55, 0.35, 0, year);
            }
        }
        PKind::Table => {
            let st = seed % 3;
            let (x0, x1, z0, z1) = (0.08, w - 0.08, 0.08, h - 0.08);
            if st == 1 && w <= 1.5 {
                // round pedestal table
                let r = (w.min(h) * 0.45).max(0.4);
                s.cylinder(v(w * 0.5, 0.72, h * 0.5), r, 0.05, 16, wood);
                s.frustum(v(w * 0.5, 0.05, h * 0.5), 0.06, 0.05, 0.67, 8, wood_d);
                s.frustum(v(w * 0.5, 0.0, h * 0.5), 0.25, 0.08, 0.08, 8, wood_d);
            } else {
                s.cuboid(v(x0, 0.72, z0), v(x1, 0.78, z1), wood);
                s.cuboid(v(x0 + 0.05, 0.62, z0 + 0.05), v(x1 - 0.05, 0.72, z1 - 0.05), wood_d);
                for (x, z) in [(x0 + 0.05, z0 + 0.05), (x1 - 0.1, z0 + 0.05), (x0 + 0.05, z1 - 0.1), (x1 - 0.1, z1 - 0.1)] {
                    if year < 1950 {
                        s.frustum(v(x + 0.025, 0.0, z + 0.025), 0.022, 0.032, 0.62, 6, wood_d);
                    } else {
                        s.cuboid(v(x, 0.0, z), v(x + 0.05, 0.62, z + 0.05), wood_d);
                    }
                }
                if st == 2 || rich {
                    // tablecloth
                    let cloth = if roll(seed, 11) < 0.5 { [0.9, 0.88, 0.84] } else { pick(&FABRICS, seed, 12) };
                    s.cuboid(v(x0 - 0.04, 0.6, z0 - 0.04), v(x1 + 0.04, 0.785, z1 + 0.04), k(cloth, 0.95));
                    if roll(seed, 13) < 0.5 {
                        // checkered
                        for i in 0..6 {
                            for j in 0..6 {
                                if (i + j) % 2 == 0 {
                                    let (cx, cz) = (x0 + i as f32 * (x1 - x0) / 6.0, z0 + j as f32 * (z1 - z0) / 6.0);
                                    s.floor(cx, cz, cx + (x1 - x0) / 6.0, cz + (z1 - z0) / 6.0, 0.787, c3([0.7, 0.15, 0.15]));
                                }
                            }
                        }
                    }
                }
            }
            for i in 0..(1 + seed % 3) {
                let (cx, cz) = (w * (0.3 + roll(seed, 14 + i) * 0.4), h * (0.3 + roll(seed, 17 + i) * 0.4));
                clutter(s, g, cx, 0.79, cz, seed.wrapping_add(i * 13), year);
            }
        }
        PKind::Chair | PKind::Stool => {
            let seat = if kind == PKind::Stool { 0.7 } else { 0.46 };
            let st = if year >= 1955 && style == 3 { 3 } else { style % 3 };
            if kind == PKind::Stool || st == 1 {
                // bentwood: round seat, splayed legs
                s.cylinder(v(0.5, seat - 0.04, 0.5), 0.2, 0.04, 12, wood);
                for a in 0..4 {
                    let ang = a as f32 * std::f32::consts::FRAC_PI_2 + 0.78;
                    s.cylinder(v(0.5 + ang.cos() * 0.15, 0.0, 0.5 + ang.sin() * 0.15), 0.018, seat - 0.04, 6, wood_d);
                }
                if kind == PKind::Chair {
                    s.frustum(v(0.5, seat, 0.32), 0.03, 0.03, 0.45, 6, wood_d);
                    s.sphere(v(0.5, seat + 0.45, 0.32), Vec3::new(0.18, 0.12, 0.02), 10, wood_d);
                }
            } else if st == 3 {
                // 1950s+ chrome and vinyl
                s.cuboid(v(0.3, seat - 0.06, 0.3), v(0.7, seat, 0.7), k(fab, 1.0));
                for (x, z) in [(0.32, 0.32), (0.66, 0.32), (0.32, 0.66), (0.66, 0.66)] {
                    s.cylinder(v(x, 0.0, z), 0.015, seat - 0.06, 6, c3([0.75, 0.75, 0.78]));
                }
                s.cuboid(v(0.3, seat + 0.1, 0.28), v(0.7, seat + 0.4, 0.33), k(fab, 1.0));
            } else {
                // ladder-back or upholstered
                let upholst = st == 2;
                s.cuboid(v(0.3, seat - 0.05, 0.3), v(0.7, seat, 0.7), wood);
                if upholst {
                    s.cuboid(v(0.31, seat, 0.31), v(0.69, seat + 0.05, 0.69), k(fab, 0.9));
                }
                for (x, z) in [(0.31, 0.31), (0.64, 0.31), (0.31, 0.64), (0.64, 0.64)] {
                    s.cuboid(v(x, 0.0, z), v(x + 0.05, seat - 0.05, z + 0.05), wood_d);
                }
                for x in [0.31, 0.64] {
                    s.cuboid(v(x, seat, 0.31), v(x + 0.05, seat + 0.5, 0.35), wood_d);
                }
                if upholst {
                    s.cuboid(v(0.34, seat + 0.12, 0.3), v(0.66, seat + 0.48, 0.34), k(fab, 0.9));
                } else {
                    for k2 in 0..3 {
                        let y = seat + 0.15 + k2 as f32 * 0.12;
                        s.cuboid(v(0.34, y, 0.32), v(0.66, y + 0.05, 0.34), wood);
                    }
                }
            }
        }
        PKind::Armchair => {
            let col = k(fab, 0.9);
            let wing = style % 2 == 0;
            s.cuboid(v(0.1, 0.12, 0.1), v(0.9, 0.42, 0.9), col);
            s.cuboid(v(0.14, 0.42, 0.14), v(0.86, 0.5, 0.86), k(fab, 1.0));
            s.cuboid(v(0.1, 0.42, 0.1), v(0.9, if wing { 1.15 } else { 0.95 }, 0.28), col);
            for x in [0.1, 0.78] {
                s.cuboid(v(x, 0.42, 0.1), v(x + 0.12, 0.66, 0.9), col);
                s.sphere(v(x + 0.06, 0.66, 0.5), Vec3::new(0.07, 0.04, 0.4), 8, col);
            }
            if wing {
                for x in [0.1, 0.84] {
                    s.cuboid(v(x, 0.66, 0.1), v(x + 0.06, 1.05, 0.4), col);
                }
            }
            for (x, z) in [(0.14, 0.14), (0.82, 0.14), (0.14, 0.82), (0.82, 0.82)] {
                s.frustum(v(x, 0.0, z), 0.03, 0.025, 0.12, 6, wood_d);
            }
        }
        PKind::Wardrobe => {
            let st = seed % 3;
            let (x0, x1, z0, z1) = (0.05, w - 0.05, 0.15, h - 0.1);
            let hh = 1.95 + roll(seed, 15) * 0.25;
            s.cuboid(v(x0, 0.08, z0), v(x1, hh, z1), wood);
            s.cuboid(v(x0 - 0.02, 0.0, z0 - 0.02), v(x1 + 0.02, 0.1, z1 + 0.02), wood_d);
            if st != 2 {
                // crown moulding
                s.cuboid(v(x0 - 0.04, hh, z0 - 0.04), v(x1 + 0.04, hh + 0.1, z1 + 0.04), wood_d);
            }
            let doors = if w > 1.3 { 3 } else { 2 };
            for d in 0..doors {
                let dx0 = x0 + 0.04 + d as f32 * (x1 - x0 - 0.08) / doors as f32;
                let dx1 = dx0 + (x1 - x0 - 0.08) / doors as f32 - 0.03;
                if st == 1 && d == doors / 2 {
                    // mirror door
                    s.cuboid(v(dx0 + 0.04, 0.3, z0 - 0.015), v(dx1 - 0.04, hh - 0.2, z0), c3([0.55, 0.6, 0.65]));
                } else {
                    s.cuboid(v(dx0 + 0.05, 0.3, z0 - 0.012), v(dx1 - 0.05, hh * 0.5, z0), wood_d);
                    s.cuboid(v(dx0 + 0.05, hh * 0.55, z0 - 0.012), v(dx1 - 0.05, hh - 0.15, z0), wood_d);
                }
                s.sphere(v(if d % 2 == 0 { dx1 - 0.06 } else { dx0 + 0.06 }, hh * 0.52, z0 - 0.02), Vec3::splat(0.022), 6, brass);
            }
            if roll(seed, 16) < 0.4 {
                // hat box / suitcase on top
                s.cuboid(v(x0 + 0.1, hh + 0.1, z0 + 0.1), v(x0 + 0.5, hh + 0.35, z1 - 0.1), k(pick(&FABRICS, seed, 17), 0.7));
            }
        }
        PKind::Stove => {
            if year < 1935 {
                // cast-iron wood/coal stove with pipe
                let iron = c3([0.1, 0.1, 0.11]);
                s.cuboid(v(0.12, 0.2, 0.15), v(w - 0.12, 0.85, h - 0.15), iron);
                s.cuboid(v(0.08, 0.85, 0.1), v(w - 0.08, 0.9, h - 0.1), iron);
                for (x, z) in [(0.15, 0.18), (w - 0.2, 0.18), (0.15, h - 0.22), (w - 0.2, h - 0.22)] {
                    s.frustum(v(x + 0.025, 0.0, z + 0.025), 0.035, 0.025, 0.2, 6, iron);
                }
                s.cylinder(v(w * 0.5, 0.9, h * 0.7), 0.07, 1.8, 10, iron);
                g.cuboid(v(0.3, 0.3, 0.13), v(w - 0.3, 0.55, 0.15), c3([1.0, 0.4, 0.1]));
                s.cylinder(v(w * 0.35, 0.9, h * 0.35), 0.12, 0.14, 10, c3([0.3, 0.3, 0.32]));
            } else {
                // enamel range: cream in the 40s-50s, white or avocado later
                let col = if year < 1960 { [0.9, 0.87, 0.78] } else if year < 1980 && roll(seed, 18) < 0.4 { [0.55, 0.6, 0.3] } else { [0.92, 0.92, 0.9] };
                s.cuboid(v(0.08, 0.0, 0.12), v(w - 0.08, 0.9, h - 0.08), c3(col));
                s.cuboid(v(0.08, 0.9, h - 0.2), v(w - 0.08, 1.1, h - 0.08), c3(col));
                s.cuboid(v(0.18, 0.2, 0.1), v(w - 0.18, 0.6, 0.12), c3([0.15, 0.15, 0.16]));
                for (x, z) in [(0.3, 0.35), (w - 0.3, 0.35), (0.3, 0.7), (w - 0.3, 0.7)] {
                    s.cylinder(v(x, 0.9, z), 0.1, 0.015, 10, c3([0.1, 0.1, 0.1]));
                }
                for i in 0..4 {
                    s.cylinder(v(0.2 + i as f32 * (w - 0.4) / 3.0, 0.72, 0.1), 0.025, 0.02, 8, c3([0.2, 0.2, 0.2]));
                }
                // a pot
                s.cylinder(v(0.3, 0.915, 0.35), 0.11, 0.14, 10, c3([0.35, 0.36, 0.4]));
            }
        }
        PKind::Fridge => {
            if year < 1935 {
                // wooden icebox
                s.cuboid(v(0.12, 0.0, 0.2), v(w - 0.12, 1.3, h - 0.15), wood);
                for y in [0.1, 0.7] {
                    s.cuboid(v(0.18, y, 0.18), v(w - 0.18, y + 0.5, 0.2), wood_d);
                    s.cuboid(v(w - 0.3, y + 0.25, 0.15), v(w - 0.22, y + 0.3, 0.18), brass);
                }
            } else {
                let rounded = year < 1965;
                let col = if year >= 1965 && year < 1982 && roll(seed, 19) < 0.5 { [0.75, 0.6, 0.25] } else { [0.92, 0.92, 0.9] };
                s.cuboid(v(0.1, 0.0, 0.2), v(w - 0.1, 1.55, h - 0.1), c3(col));
                if rounded {
                    s.sphere(v(w * 0.5, 1.55, h * 0.55), Vec3::new(w * 0.4, 0.1, h * 0.35), 10, c3(col));
                }
                s.cuboid(v(w - 0.28, 0.9, 0.16), v(w - 0.24, 1.25, 0.2), c3([0.7, 0.7, 0.72]));
                s.cuboid(v(0.12, 1.05, 0.19), v(w - 0.12, 1.06, 0.2), c3([0.6, 0.6, 0.6]));
            }
        }
        PKind::Sink => {
            // pedestal wash basin with mirror
            s.frustum(v(0.5, 0.0, 0.7), 0.1, 0.07, 0.72, 10, c3([0.92, 0.92, 0.9]));
            s.frustum(v(0.5, 0.72, 0.7), 0.16, 0.24, 0.12, 12, c3([0.94, 0.94, 0.92]));
            s.cylinder(v(0.5, 0.84, 0.85), 0.015, 0.12, 6, c3([0.75, 0.75, 0.78]));
            s.cuboid(v(0.25, 1.15, 0.9), v(0.75, 1.75, 0.95), wood_d);
            s.cuboid(v(0.28, 1.18, 0.89), v(0.72, 1.72, 0.9), c3([0.6, 0.66, 0.72]));
        }
        PKind::Toilet => {
            let porcelain = c3([0.93, 0.93, 0.91]);
            s.frustum(v(0.5, 0.0, 0.45), 0.13, 0.18, 0.38, 12, porcelain);
            s.cylinder(v(0.5, 0.38, 0.45), 0.2, 0.04, 12, if year < 1950 { wood } else { porcelain });
            if year < 1935 {
                // high tank with pull chain
                s.cuboid(v(0.3, 1.7, 0.8), v(0.7, 1.95, 0.95), wood);
                s.cylinder(v(0.62, 0.4, 0.85), 0.02, 1.3, 6, metal);
                s.cylinder(v(0.35, 1.25, 0.8), 0.004, 0.45, 4, metal);
                s.sphere(v(0.35, 1.23, 0.8), Vec3::splat(0.025), 6, wood_d);
            } else {
                s.cuboid(v(0.3, 0.4, 0.72), v(0.7, 0.85, 0.92), porcelain);
            }
        }
        PKind::Bathtub => {
            let enamel = c3([0.94, 0.94, 0.92]);
            if year < 1950 {
                // clawfoot tub
                s.sphere(v(w * 0.5, 0.45, h * 0.5), Vec3::new(w * 0.42, 0.25, h * 0.46), 14, enamel);
                s.cuboid(v(0.08, 0.45, 0.08), v(w - 0.08, 0.62, h - 0.08), enamel);
                for (x, z) in [(0.2, 0.25), (w - 0.2, 0.25), (0.2, h - 0.25), (w - 0.2, h - 0.25)] {
                    s.sphere(v(x, 0.08, z), Vec3::new(0.05, 0.08, 0.05), 6, brass);
                }
            } else {
                s.cuboid(v(0.05, 0.0, 0.05), v(w - 0.05, 0.58, h - 0.05), c3(pick(&[[0.93, 0.93, 0.91], [0.8, 0.85, 0.75], [0.85, 0.75, 0.7]], seed, 21)));
            }
            s.floor(0.2, 0.2, w - 0.2, h - 0.2, 0.55, c3([0.55, 0.65, 0.72]));
            s.cylinder(v(w * 0.5, 0.6, 0.12), 0.02, 0.2, 6, c3([0.75, 0.75, 0.78]));
        }
        PKind::Sofa => {
            let st = if year >= 1955 && style >= 2 { 2 } else { style % 2 };
            let col = k(fab, 0.9);
            let (x0, x1, z0, z1) = (0.05, w - 0.05, 0.12, h - 0.08);
            match st {
                0 => {
                    // chesterfield: rolled arms, tufted back
                    s.cuboid(v(x0, 0.1, z0), v(x1, 0.45, z1), col);
                    s.cuboid(v(x0, 0.45, z1 - 0.28), v(x1, 0.85, z1), col);
                    for x in [x0, x1 - 0.22] {
                        s.cuboid(v(x, 0.45, z0), v(x + 0.22, 0.7, z1), col);
                        s.cylinder(v(x + 0.11, 0.7, z0), 0.0, 0.0, 3, col);
                        s.sphere(v(x + 0.11, 0.7, (z0 + z1) / 2.0), Vec3::new(0.13, 0.08, (z1 - z0) / 2.0), 8, col);
                    }
                    let n = ((x1 - x0) / 0.25) as i32;
                    for i in 1..n {
                        for yy in [0.58, 0.72] {
                            s.sphere(v(x0 + i as f32 * (x1 - x0) / n as f32, yy, z1 - 0.285), Vec3::splat(0.018), 4, k(fab, 0.6));
                        }
                    }
                }
                1 => {
                    // camelback with wooden legs
                    s.cuboid(v(x0, 0.2, z0), v(x1, 0.45, z1), col);
                    s.sphere(v(w * 0.5, 0.75, z1 - 0.12), Vec3::new((x1 - x0) * 0.5, 0.3, 0.13), 12, col);
                    for x in [x0, x1 - 0.15] {
                        s.cuboid(v(x, 0.45, z0), v(x + 0.15, 0.65, z1), col);
                    }
                    for (x, z) in [(x0 + 0.05, z0 + 0.05), (x1 - 0.08, z0 + 0.05), (x0 + 0.05, z1 - 0.08), (x1 - 0.08, z1 - 0.08)] {
                        s.frustum(v(x, 0.0, z), 0.03, 0.02, 0.2, 6, wood_d);
                    }
                }
                _ => {
                    // low modern sofa with cushions
                    s.cuboid(v(x0, 0.12, z0), v(x1, 0.38, z1), col);
                    s.cuboid(v(x0, 0.38, z1 - 0.2), v(x1, 0.72, z1), col);
                    for x in [x0, x1 - 0.12] {
                        s.cuboid(v(x, 0.38, z0), v(x + 0.12, 0.55, z1), col);
                    }
                    for (x, z) in [(x0 + 0.05, z0 + 0.05), (x1 - 0.08, z0 + 0.05), (x0 + 0.05, z1 - 0.08), (x1 - 0.08, z1 - 0.08)] {
                        s.cylinder(v(x, 0.0, z), 0.02, 0.12, 6, c3([0.7, 0.7, 0.72]));
                    }
                }
            }
            // seat cushions and a couple of throw pillows
            let nc = ((x1 - x0) / 0.7).max(1.0) as i32;
            for i in 0..nc {
                let cx0 = x0 + 0.25 + i as f32 * (x1 - x0 - 0.5) / nc as f32;
                s.cuboid(v(cx0 + 0.01, 0.45, z0 + 0.03), v(cx0 + (x1 - x0 - 0.5) / nc as f32 - 0.01, 0.52, z1 - 0.3), k(fab, 1.0));
            }
            for i in 0..(seed % 3) {
                let px = x0 + 0.35 + i as f32 * 0.5;
                s.sphere(v(px, 0.65, z1 - 0.35), Vec3::new(0.15, 0.13, 0.05), 8, k(pick(&FABRICS, seed, 25 + i), 1.0));
            }
        }
        PKind::Tv => {
            let old = year < 1975;
            let cab = if old { wood } else { c3([0.12, 0.12, 0.13]) };
            if old {
                s.cuboid(v(0.15, 0.3, 0.3), v(0.85, 0.95, 0.85), cab);
                for (x, z) in [(0.2, 0.35), (0.75, 0.35), (0.2, 0.75), (0.75, 0.75)] {
                    s.frustum(v(x, 0.0, z), 0.025, 0.02, 0.3, 6, wood_d);
                }
                s.cylinder(v(0.45, 0.95, 0.6), 0.005, 0.35, 4, metal);
                s.cylinder(v(0.55, 0.95, 0.6), 0.005, 0.35, 4, metal);
            } else {
                s.cuboid(v(0.1, 0.0, 0.3), v(0.9, 0.45, 0.85), wood_d);
                s.cuboid(v(0.18, 0.45, 0.35), v(0.82, 0.98, 0.85), cab);
            }
            // glowing screen (blue-grey flicker handled by the glow material)
            let (y0, y1) = if old { (0.42, 0.85) } else { (0.52, 0.92) };
            g.cuboid(v(0.22, y0, 0.28), v(0.68, y1, 0.3), c3([0.35, 0.45, 0.6]));
        }
        PKind::BottleShelf => {
            // back bar: cabinet, mirror, three shelves packed with bottles of every shape
            let (x0, x1) = (0.02, w - 0.02);
            s.cuboid(v(x0, 0.0, 0.55), v(x1, 0.95, h - 0.02), wood_d);
            s.cuboid(v(x0, 0.95, 0.6), v(x1, 1.0, h - 0.02), wood);
            s.cuboid(v(x0, 1.0, h - 0.08), v(x1, 2.5, h - 0.02), wood_d);
            s.cuboid(v(x0 + 0.1, 1.05, h - 0.09), v(x1 - 0.1, 2.35, h - 0.08), c3([0.45, 0.5, 0.55]));
            s.cuboid(v(x0 - 0.03, 2.45, h - 0.2), v(x1 + 0.03, 2.55, h), wood);
            for (k, y) in [1.3f32, 1.72, 2.12].iter().enumerate() {
                s.cuboid(v(x0 + 0.05, *y - 0.03, h - 0.3), v(x1 - 0.05, *y, h - 0.08), wood);
                let n = ((x1 - x0) / 0.1) as i32;
                for i in 0..n {
                    let bs = seed.wrapping_add(i as u32 * 31 + k as u32 * 977);
                    let x = x0 + 0.08 + i as f32 * 0.1;
                    let z = h - 0.2;
                    let col = pick(&[[0.2, 0.35, 0.15], [0.45, 0.25, 0.08], [0.85, 0.85, 0.8], [0.6, 0.12, 0.1], [0.15, 0.15, 0.25], [0.7, 0.55, 0.2], [0.3, 0.2, 0.12]], bs, 1);
                    let tall = 0.2 + roll(bs, 2) * 0.12;
                    match bs % 3 {
                        0 => {
                            s.cylinder(v(x, *y, z), 0.035, tall, 8, c3(col));
                            s.frustum(v(x, *y + tall, z), 0.035, 0.012, 0.08, 8, c3(col));
                        }
                        1 => {
                            s.cuboid(v(x - 0.035, *y, z - 0.03), v(x + 0.035, *y + tall * 0.8, z + 0.03), c3(col));
                            s.cylinder(v(x, *y + tall * 0.8, z), 0.012, 0.05, 6, c3([0.2, 0.2, 0.2]));
                        }
                        _ => {
                            s.sphere(v(x, *y + 0.07, z), Vec3::new(0.045, 0.07, 0.045), 8, c3(col));
                            s.cylinder(v(x, *y + 0.12, z), 0.012, 0.1, 6, c3(col));
                        }
                    }
                    // labels
                    if bs % 2 == 0 {
                        s.cuboid(v(x - 0.03, *y + 0.05, z - 0.037), v(x + 0.03, *y + 0.11, z - 0.034), c3([0.9, 0.85, 0.7]));
                    }
                }
            }
            // glasses on the counter top
            for i in 0..((x1 - x0) / 0.25) as i32 {
                s.frustum(v(x0 + 0.12 + i as f32 * 0.25, 1.0, 0.75), 0.03, 0.04, 0.1, 8, c3([0.75, 0.8, 0.82]));
            }
        }
        PKind::PoolTable => {
            let felt = pick(&[[0.1, 0.4, 0.2], [0.1, 0.25, 0.45], [0.45, 0.1, 0.12]], seed, 1);
            s.cuboid(v(0.1, 0.62, 0.1), v(w - 0.1, 0.78, h - 0.1), wood_d);
            s.cuboid(v(0.18, 0.78, 0.18), v(w - 0.18, 0.8, h - 0.18), c3(felt));
            for (x, z) in [(0.2, 0.2), (w - 0.2, 0.2), (0.2, h - 0.2), (w - 0.2, h - 0.2)] {
                s.frustum(v(x, 0.0, z), 0.07, 0.09, 0.62, 8, wood_d);
            }
            for i in 0..8 {
                let col = [[0.9, 0.8, 0.2], [0.2, 0.3, 0.8], [0.8, 0.15, 0.1], [0.4, 0.2, 0.5], [0.95, 0.5, 0.1], [0.1, 0.45, 0.2], [0.5, 0.15, 0.1], [0.05, 0.05, 0.05]][i];
                s.sphere(v(w * 0.3 + (i % 3) as f32 * 0.07, 0.83, h * 0.5 + (i / 3) as f32 * 0.07 - 0.07), Vec3::splat(0.03), 8, c3(col));
            }
            s.sphere(v(w * 0.72, 0.83, h * 0.5), Vec3::splat(0.03), 8, c3([0.95, 0.95, 0.92]));
            s.rod(v(w * 0.75, 0.82, h * 0.45), v(w * 0.95, 0.9, 0.2), 0.012, c3([0.6, 0.45, 0.25]));
        }
        PKind::Jukebox => {
            let body = pick(&[[0.6, 0.1, 0.12], [0.75, 0.55, 0.3], [0.2, 0.3, 0.5]], seed, 1);
            s.cuboid(v(0.15, 0.0, 0.3), v(0.85, 1.2, 0.85), c3(body));
            s.sphere(v(0.5, 1.2, 0.58), Vec3::new(0.35, 0.3, 0.27), 12, c3(body));
            g.cuboid(v(0.22, 0.7, 0.28), v(0.78, 1.15, 0.3), c3([1.0, 0.7, 0.3]));
            g.cuboid(v(0.18, 0.2, 0.28), v(0.24, 1.3, 0.3), c3([1.0, 0.3, 0.6]));
            g.cuboid(v(0.76, 0.2, 0.28), v(0.82, 1.3, 0.3), c3([0.3, 0.8, 1.0]));
            s.cuboid(v(0.25, 0.25, 0.27), v(0.75, 0.6, 0.29), c3([0.7, 0.7, 0.72]));
        }
        PKind::Stage => {
            s.cuboid(v(0.0, 0.0, 0.0), v(w, 0.4, h), wood_d);
            s.cuboid(v(-0.02, 0.36, -0.02), v(w + 0.02, 0.42, h + 0.02), wood);
            // velvet curtain at the back and sides
            let red = c3(pick(&[[0.5, 0.05, 0.08], [0.35, 0.05, 0.3], [0.15, 0.1, 0.35]], seed, 2));
            for i in 0..((w / 0.18) as i32) {
                let x = i as f32 * 0.18;
                s.cuboid(v(x, 0.4, h - 0.12 - (i % 2) as f32 * 0.05), v(x + 0.18, 3.0, h - 0.02), red);
            }
            s.cuboid(v(-0.05, 2.8, h - 0.3), v(w + 0.05, 3.0, h), c3([0.7, 0.55, 0.2]));
            // microphone stand
            s.cylinder(v(w * 0.5, 0.42, h * 0.35), 0.1, 0.02, 10, c3([0.1, 0.1, 0.1]));
            s.cylinder(v(w * 0.5, 0.42, h * 0.35), 0.012, 1.45, 6, c3([0.7, 0.7, 0.72]));
            s.sphere(v(w * 0.5, 1.9, h * 0.35), Vec3::new(0.035, 0.05, 0.035), 8, c3([0.75, 0.75, 0.78]));
            // footlights
            for i in 0..((w / 0.5) as i32) {
                g.sphere(v(0.25 + i as f32 * 0.5, 0.44, 0.08), Vec3::splat(0.04), 6, c3([1.0, 0.85, 0.5]));
            }
        }
        PKind::DrumKit => {
            let shell = c3(pick(&[[0.6, 0.1, 0.1], [0.85, 0.85, 0.82], [0.1, 0.1, 0.12], [0.3, 0.15, 0.4]], seed, 1));
            s.cyl_z(v(0.5, 0.3, 0.5), 0.28, 0.2, 14, shell, c3([0.92, 0.9, 0.85]));
            for (x, z, r, y) in [(0.2, 0.25, 0.14, 0.55), (0.8, 0.25, 0.14, 0.55), (0.25, 0.8, 0.16, 0.45)] {
                s.cylinder(v(x, y, z), r, 0.15, 12, shell);
                s.cylinder(v(x, y + 0.15, z), r, 0.005, 12, c3([0.92, 0.9, 0.85]));
                s.cylinder(v(x, 0.0, z), 0.012, y, 6, c3([0.7, 0.7, 0.72]));
            }
            for (x, z) in [(0.05, 0.6), (0.9, 0.7)] {
                s.cylinder(v(x, 0.0, z), 0.012, 0.95, 6, c3([0.7, 0.7, 0.72]));
                s.frustum(v(x, 0.95, z), 0.2, 0.02, 0.04, 14, c3([0.75, 0.6, 0.25]));
            }
            s.cylinder(v(0.5, 0.0, 0.9), 0.15, 0.45, 10, c3([0.1, 0.1, 0.1]));
        }
        PKind::KeyRack => {
            // pigeonholes with keys and letters behind reception
            s.cuboid(v(0.05, 0.9, h - 0.12), v(w - 0.05, 2.2, h - 0.02), wood_d);
            let cols = ((w - 0.1) / 0.14) as i32;
            for r in 0..6 {
                for c in 0..cols {
                    let x = 0.08 + c as f32 * 0.14;
                    let y = 0.95 + r as f32 * 0.2;
                    s.cuboid(v(x, y, h - 0.13), v(x + 0.12, y + 0.17, h - 0.12), c3([0.18, 0.12, 0.08]));
                    let k = seed.wrapping_add((r * 17 + c) as u32);
                    if k % 3 != 0 {
                        s.cuboid(v(x + 0.05, y + 0.03, h - 0.135), v(x + 0.07, y + 0.1, h - 0.13), c3([0.75, 0.6, 0.25]));
                    }
                    if k % 5 == 0 {
                        s.cuboid(v(x + 0.01, y + 0.1, h - 0.14), v(x + 0.11, y + 0.14, h - 0.13), c3([0.92, 0.9, 0.82]));
                    }
                }
            }
        }
        PKind::CoatRack => {
            s.cylinder(v(0.5, 0.0, 0.5), 0.18, 0.03, 10, wood_d);
            s.cylinder(v(0.5, 0.0, 0.5), 0.025, 1.8, 8, wood_d);
            for k in 0..4 {
                let a = k as f32 * 1.57;
                s.rod(v(0.5, 1.65, 0.5), v(0.5 + a.cos() * 0.15, 1.75, 0.5 + a.sin() * 0.15), 0.012, wood_d);
            }
            // a coat and a hat hanging
            if seed % 2 == 0 {
                s.cuboid(v(0.35, 0.9, 0.42), v(0.55, 1.7, 0.52), c3(pick(&FABRICS, seed, 3)));
            }
            s.frustum(v(0.62, 1.72, 0.5), 0.1, 0.08, 0.1, 10, c3([0.12, 0.1, 0.1]));
        }
        PKind::ClothesRack => {
            s.rod(v(0.05, 1.5, 0.5), v(w - 0.05, 1.5, 0.5), 0.015, c3([0.7, 0.7, 0.72]));
            for x in [0.05, w - 0.05] {
                s.rod(v(x, 0.0, 0.5), v(x, 1.5, 0.5), 0.015, c3([0.7, 0.7, 0.72]));
            }
            let n = ((w - 0.2) / 0.09) as i32;
            for i in 0..n {
                let x = 0.1 + i as f32 * 0.09;
                let col = pick(&FABRICS, seed.wrapping_add(i as u32), 4);
                let long = seed.wrapping_add(i as u32) % 3 == 0;
                s.cuboid(v(x, if long { 0.4 } else { 0.85 }, 0.32), v(x + 0.05, 1.45, 0.68), c3(col));
            }
        }
        PKind::FilingCabinet => {
            let col = if year < 1950 { [0.35, 0.3, 0.22] } else { pick(&[[0.4, 0.45, 0.4], [0.55, 0.55, 0.52], [0.3, 0.32, 0.38]], seed, 1) };
            s.cuboid(v(0.2, 0.0, 0.2), v(0.8, 1.35, 0.9), c3(col));
            for k in 0..4 {
                let y = 0.1 + k as f32 * 0.32;
                s.cuboid(v(0.24, y, 0.18), v(0.76, y + 0.27, 0.2), c3([col[0] * 0.85, col[1] * 0.85, col[2] * 0.85]));
                s.cuboid(v(0.44, y + 0.18, 0.16), v(0.56, y + 0.21, 0.18), c3([0.75, 0.75, 0.78]));
            }
        }
        PKind::Stairs => {
            // a flight of stairs going up (out of the cut-away view)
            let steps = 10;
            for i in 0..steps {
                let y = i as f32 * 0.25;
                let z0 = i as f32 * (h / steps as f32);
                s.cuboid(v(0.05, 0.0, z0), v(w - 0.05, y + 0.25, z0 + h / steps as f32 + 0.02), wood);
                s.cuboid(v(0.05, y + 0.22, z0), v(w - 0.05, y + 0.27, z0 + 0.04), wood_d);
            }
            for x in [0.05, w - 0.05] {
                s.rod(v(x, 1.0, 0.0), v(x, 3.4, h), 0.03, wood_d);
            }
        }
        PKind::FloorLamp => {
            s.cylinder(v(0.5, 0.0, 0.5), 0.14, 0.03, 10, metal);
            s.cylinder(v(0.5, 0.03, 0.5), 0.015, 1.45, 6, if year < 1950 { brass } else { metal });
            g.frustum(v(0.5, 1.35, 0.5), 0.22, 0.13, 0.28, 12, k(pick(&[[0.95, 0.85, 0.6], [0.9, 0.75, 0.55], [0.85, 0.8, 0.7]], seed, 26), 1.0));
        }
        _ => return false,
    }
    let _ = poor;
    true
}
