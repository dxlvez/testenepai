//! Period cars built from smooth lofted panels: 1920s tourers and sedans,
//! 30s/40s fastbacks, 50s finned cruisers, 60s/70s long sedans and wagons,
//! 80s/90s wedges; plus taxis, police cars, pickups and vans. Each car is
//! split by material (clear-coated paint, chrome, glass, rubber, lights).

use super::mesh::{c3, MB};
use crate::city::gen::CityId;
use bevy::prelude::*;

pub struct CarGeo {
    pub paint: MB,
    pub chrome: MB,
    pub glass: MB,
    pub dark: MB,
    pub lights: MB,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Body {
    Sedan,
    Tourer,
    Pickup,
    Van,
    Wagon,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Livery {
    Plain,
    Taxi,
    Police,
}

pub fn kind_of(year: i32, id: u32, city: CityId) -> (Body, Livery) {
    let big_city = matches!(city, CityId::NewYork | CityId::Chicago | CityId::SanFrancisco | CityId::LosAngeles | CityId::London | CityId::Berlin | CityId::NewOrleans);
    let livery = if id % 13 == 5 {
        Livery::Police
    } else if big_city && id % 6 == 2 && year >= 1922 {
        Livery::Taxi
    } else {
        Livery::Plain
    };
    let body = if livery != Livery::Plain {
        Body::Sedan
    } else if id % 7 == 3 {
        Body::Pickup
    } else if year >= 1962 && id % 9 == 4 {
        Body::Van
    } else if (1950..1992).contains(&year) && id % 8 == 1 {
        Body::Wagon
    } else if year < 1932 && id % 3 == 0 {
        Body::Tourer
    } else {
        Body::Sedan
    };
    (body, livery)
}

/// Period-correct paint, varied car to car (some repeat, colours always change).
pub fn paint_for(year: i32, id: u32, livery: Livery, city: CityId) -> ([f32; 3], Option<[f32; 3]>) {
    match livery {
        Livery::Taxi => {
            let c = match city {
                CityId::London => [0.06, 0.06, 0.07],
                CityId::Berlin => [0.85, 0.8, 0.6],
                _ => [0.95, 0.72, 0.08],
            };
            return (c, None);
        }
        Livery::Police => {
            return if year < 1950 {
                ([0.05, 0.05, 0.06], None)
            } else if year < 1975 {
                ([0.05, 0.05, 0.06], Some([0.92, 0.92, 0.9]))
            } else {
                ([0.92, 0.92, 0.9], Some([0.1, 0.15, 0.4]))
            };
        }
        Livery::Plain => {}
    }
    let pal: &[[f32; 3]] = if year < 1935 {
        &[[0.03, 0.03, 0.035], [0.03, 0.03, 0.035], [0.08, 0.15, 0.1], [0.25, 0.05, 0.06], [0.06, 0.08, 0.2], [0.2, 0.16, 0.1]]
    } else if year < 1950 {
        &[[0.04, 0.04, 0.045], [0.1, 0.18, 0.28], [0.25, 0.28, 0.22], [0.4, 0.1, 0.08], [0.55, 0.5, 0.4], [0.18, 0.22, 0.18]]
    } else if year < 1962 {
        &[[0.55, 0.75, 0.7], [0.85, 0.55, 0.6], [0.95, 0.92, 0.85], [0.7, 0.1, 0.1], [0.35, 0.55, 0.8], [0.95, 0.85, 0.45], [0.05, 0.05, 0.06]]
    } else if year < 1980 {
        &[[0.55, 0.35, 0.15], [0.9, 0.55, 0.1], [0.45, 0.5, 0.2], [0.75, 0.62, 0.3], [0.15, 0.25, 0.45], [0.6, 0.08, 0.1], [0.92, 0.9, 0.85], [0.05, 0.05, 0.06]]
    } else {
        &[[0.75, 0.76, 0.78], [0.92, 0.92, 0.9], [0.05, 0.05, 0.06], [0.55, 0.06, 0.08], [0.12, 0.18, 0.35], [0.2, 0.3, 0.22], [0.45, 0.45, 0.48]]
    };
    let h = crate::util::hash2(id as i32, year, 71);
    let c = pal[(h as usize) % pal.len()];
    // slight per-car shade so equal models still differ
    let k = 0.9 + ((h >> 8) % 20) as f32 * 0.01;
    let two = if (1950..1962).contains(&year) && h % 3 == 0 { Some([0.95, 0.94, 0.9]) } else { None };
    ([c[0] * k, c[1] * k, c[2] * k], two)
}

fn s(x: f32, y: f32, hw: f32, hh: f32) -> (Vec3, Vec3, f32, f32) {
    (Vec3::new(x, y, 0.0), Vec3::X, hw, hh)
}

/// Body loft from (x, bottom, top, half width) stations along the car (+x = front).
fn body(mb: &mut MB, st: &[(f32, f32, f32, f32)], n: f32, col: [f32; 4]) {
    let secs: Vec<_> = st.iter().map(|(x, b, t, hw)| s(*x, (b + t) / 2.0, *hw, (t - b) / 2.0)).collect();
    mb.loft_box(&secs, n, 20, col);
}

fn wheel(g: &mut CarGeo, x: f32, z: f32, r: f32, w: f32, spoked: bool, whitewall: bool) {
    let tyre = c3([0.05, 0.05, 0.05]);
    let side = z.signum();
    g.dark.cyl_z(Vec3::new(x, r, z), r, w, 18, tyre, tyre);
    if whitewall {
        g.chrome.cyl_z(Vec3::new(x, r, z + side * 0.005), r * 0.78, w, 18, c3([0.92, 0.92, 0.9]), c3([0.92, 0.92, 0.9]));
    }
    g.chrome.cyl_z(Vec3::new(x, r, z + side * 0.01), r * 0.55, w, 14, c3([0.8, 0.8, 0.82]), c3([0.8, 0.8, 0.82]));
    if spoked {
        for k in 0..10 {
            let a = k as f32 / 10.0 * std::f32::consts::TAU;
            g.paint.rod(Vec3::new(x, r, z + side * (w + 0.01)), Vec3::new(x + a.cos() * r * 0.8, r + a.sin() * r * 0.8, z + side * (w + 0.01)), 0.012, c3([0.85, 0.8, 0.7]));
        }
    }
}

pub fn build_car(year: i32, id: u32, city: CityId) -> CarGeo {
    let mut g = CarGeo { paint: MB::new(), chrome: MB::new(), glass: MB::new(), dark: MB::new(), lights: MB::new() };
    let (bodyk, livery) = kind_of(year, id, city);
    let (col, two) = paint_for(year, id, livery, city);
    let p = c3(col);
    let roofc = c3(two.unwrap_or(col));
    let chrome = c3([0.85, 0.85, 0.88]);
    let glass = [1.0, 1.0, 1.0, 1.0];
    let black = c3([0.03, 0.03, 0.03]);
    let lamp = c3([1.0, 0.95, 0.8]);
    let tail = c3([1.0, 0.1, 0.08]);
    if year < 1935 {
        // ---------------------------------------------------------------- 1920s: Model-T / sedans / tourers
        let (hl, hw) = (1.9, 0.8);
        // chassis + lower body
        body(&mut g.paint, &[(-hl, 0.55, 0.95, hw * 0.92), (-hl + 0.1, 0.52, 1.02, hw), (0.7, 0.52, 1.02, hw), (0.95, 0.56, 0.98, hw * 0.9)], 8.0, p);
        // long hood and radiator
        body(&mut g.paint, &[(0.9, 0.6, 1.06, 0.44), (1.75, 0.62, 1.02, 0.42), (1.8, 0.64, 1.0, 0.4)], 6.0, p);
        g.chrome.cuboid(Vec3::new(1.8, 0.58, -0.44), Vec3::new(1.9, 1.12, 0.44), chrome);
        g.dark.cuboid(Vec3::new(1.9, 0.63, -0.38), Vec3::new(1.905, 1.06, 0.38), c3([0.15, 0.15, 0.15]));
        if bodyk == Body::Tourer {
            // open car: windshield, folded top, leather seats
            g.glass.cuboid(Vec3::new(0.62, 1.02, -0.7), Vec3::new(0.66, 1.5, 0.7), glass);
            g.chrome.cuboid(Vec3::new(0.6, 1.5, -0.72), Vec3::new(0.68, 1.53, 0.72), chrome);
            g.dark.loft_box(&[s(-1.85, 1.1, 0.72, 0.1), s(-1.55, 1.12, 0.74, 0.12)], 3.0, 12, black);
            for x in [-1.2, 0.1] {
                g.dark.cuboid(Vec3::new(x, 1.0, -0.65), Vec3::new(x + 0.5, 1.12, 0.65), c3([0.3, 0.12, 0.06]));
                g.dark.cuboid(Vec3::new(x - 0.1, 1.0, -0.65), Vec3::new(x, 1.45, 0.65), c3([0.3, 0.12, 0.06]));
            }
        } else if bodyk == Body::Pickup {
            body(&mut g.paint, &[(-0.1, 1.0, 1.85, 0.72), (0.7, 1.0, 1.85, 0.72)], 10.0, p);
            g.glass.cuboid(Vec3::new(0.69, 1.25, -0.6), Vec3::new(0.72, 1.7, 0.6), glass);
            g.dark.cuboid(Vec3::new(-1.85, 1.0, -0.75), Vec3::new(-0.15, 1.4, -0.7), c3([0.35, 0.25, 0.15]));
            g.dark.cuboid(Vec3::new(-1.85, 1.0, 0.7), Vec3::new(-0.15, 1.4, 0.75), c3([0.35, 0.25, 0.15]));
        } else {
            // tall closed cabin with window band and pillars
            body(&mut g.paint, &[(-1.75, 1.0, 1.9, 0.76), (0.7, 1.0, 1.9, 0.76)], 12.0, roofc);
            g.glass.loft_box(&[s(-1.6, 1.45, 0.77, 0.24), s(0.62, 1.45, 0.77, 0.24)], 12.0, 12, glass);
            for x in [-1.62, -0.55, 0.62] {
                g.paint.cuboid(Vec3::new(x - 0.04, 1.2, -0.78), Vec3::new(x + 0.04, 1.72, 0.78), roofc);
            }
            g.dark.cuboid(Vec3::new(-1.78, 1.88, -0.78), Vec3::new(0.73, 1.93, 0.78), black);
            // spare wheel on the back
            g.dark.cyl_z(Vec3::new(-1.95, 1.0, 0.0), 0.36, 0.08, 16, black, black);
        }
        // separate arched fenders + running boards
        for sz in [-1.0f32, 1.0] {
            for wx in [1.25f32, -1.2] {
                let secs: Vec<_> = (0..9)
                    .map(|k| {
                        let a = std::f32::consts::PI * (k as f32 / 8.0);
                        let c = Vec3::new(wx + a.cos() * 0.5, 0.38 + a.sin() * 0.44, sz * 0.78);
                        (c, Vec3::new(-a.sin(), a.cos(), 0.0), 0.17, 0.035)
                    })
                    .collect();
                g.paint.loft_box(&secs, 4.0, 10, black);
            }
            g.dark.cuboid(Vec3::new(-0.7, 0.36, sz * 0.78 - 0.17), Vec3::new(0.75, 0.41, sz * 0.78 + 0.17), black);
        }
        // drum headlights on stalks
        for z in [-0.5f32, 0.5] {
            g.chrome.rod(Vec3::new(1.7, 0.85, z * 0.8), Vec3::new(1.75, 1.05, z), 0.02, chrome);
            g.chrome.cyl_z(Vec3::new(1.78, 1.1, z), 0.12, 0.0, 12, chrome, chrome);
            g.lights.sphere(Vec3::new(1.86, 1.1, z), Vec3::new(0.03, 0.1, 0.1), 10, lamp);
        }
        g.lights.sphere(Vec3::new(-1.92, 0.9, 0.55), Vec3::splat(0.05), 6, tail);
        for (x, z) in [(1.25f32, 0.8f32), (1.25, -0.8), (-1.2, 0.8), (-1.2, -0.8)] {
            wheel(&mut g, x, z, 0.38, 0.07, true, false);
        }
    } else if year < 1950 {
        // ---------------------------------------------------------------- late 30s / 40s fastback
        let hw = 0.88;
        body(&mut g.paint, &[(-2.3, 0.5, 0.82, 0.7), (-2.1, 0.42, 1.0, 0.84), (-1.3, 0.38, 1.08, hw), (0.3, 0.38, 1.05, hw), (1.3, 0.42, 0.98, 0.8), (2.1, 0.5, 0.86, 0.6), (2.3, 0.56, 0.8, 0.42)], 3.4, p);
        body(&mut g.paint, &[(-1.75, 0.98, 1.12, 0.7), (-1.25, 0.98, 1.5, 0.74), (-0.6, 0.98, 1.64, 0.76), (0.2, 0.98, 1.62, 0.76), (0.6, 0.98, 1.4, 0.74)], 4.0, roofc);
        g.glass.loft_box(&[s(-1.2, 1.27, 0.765, 0.16), s(-0.6, 1.3, 0.775, 0.2), s(0.2, 1.3, 0.775, 0.2), s(0.5, 1.22, 0.755, 0.14)], 5.0, 14, glass);
        // pontoon fender bulges
        for (x, z) in [(1.35f32, 1.0f32), (1.35, -1.0), (-1.35, 1.0), (-1.35, -1.0)] {
            g.paint.sphere(Vec3::new(x, 0.62, z * 0.72), Vec3::new(0.62, 0.34, 0.2), 14, p);
        }
        // tall waterfall grille + bumpers
        for k in 0..9 {
            let z = -0.3 + k as f32 * 0.075;
            g.chrome.cuboid(Vec3::new(2.22, 0.5, z - 0.012), Vec3::new(2.33, 0.95, z + 0.012), chrome);
        }
        for x in [2.38f32, -2.38] {
            g.chrome.loft_box(&[(Vec3::new(x, 0.45, -0.85), Vec3::Z, 0.06, 0.06), (Vec3::new(x, 0.45, 0.85), Vec3::Z, 0.06, 0.06)], 3.0, 10, chrome);
        }
        for z in [-0.62f32, 0.62] {
            g.lights.sphere(Vec3::new(1.95, 0.9, z), Vec3::new(0.1, 0.09, 0.09), 10, lamp);
            g.lights.sphere(Vec3::new(-2.25, 0.8, z * 0.9), Vec3::new(0.04, 0.06, 0.06), 6, tail);
        }
        for (x, z) in [(1.35f32, 0.78f32), (1.35, -0.78), (-1.35, 0.78), (-1.35, -0.78)] {
            wheel(&mut g, x, z, 0.36, 0.1, false, (id % 3) == 0);
        }
    } else if year < 1962 {
        // ---------------------------------------------------------------- 50s cruiser with fins and chrome
        let (hl, hw) = (2.6, 0.98);
        body(&mut g.paint, &[(-hl, 0.42, 0.88, 0.9), (-hl + 0.2, 0.36, 0.95, hw), (hl - 0.3, 0.36, 0.95, hw), (hl, 0.42, 0.85, 0.9)], 5.0, p);
        if year >= 1955 {
            for z in [-0.82f32, 0.82] {
                g.paint.loft_box(&[s(-2.55, 1.0, 0.06, 0.02), s(-1.6, 0.93, 0.05, 0.01)].map(|(c, f, a, b)| (c + Vec3::new(0.0, 0.0, z), f, a, b)), 3.0, 8, p);
                g.lights.sphere(Vec3::new(-2.6, 0.95, z), Vec3::new(0.05, 0.1, 0.05), 8, tail);
            }
        }
        let cab = if bodyk == Body::Wagon { [(-2.2, 0.95, 1.5, 0.86), (0.4, 0.95, 1.5, 0.86), (0.85, 0.95, 1.3, 0.84)] } else { [(-1.3, 0.95, 1.42, 0.84), (0.3, 0.95, 1.48, 0.86), (0.75, 0.95, 1.25, 0.84)] };
        body(&mut g.paint, &cab, 6.0, roofc);
        g.glass.loft_box(&[s(cab[0].0 + 0.1, 1.18, 0.87, 0.2), s(cab[1].0, 1.2, 0.875, 0.22), s(cab[2].0 - 0.05, 1.12, 0.855, 0.15)], 6.0, 14, glass);
        // chrome: side spear, big bumpers, grille, hubcaps
        for z in [-0.99f32, 0.99] {
            g.chrome.cuboid(Vec3::new(-2.3, 0.7, z - 0.01), Vec3::new(2.2, 0.73, z + 0.01), chrome);
        }
        for x in [2.65f32, -2.65] {
            g.chrome.loft_box(&[(Vec3::new(x, 0.45, -0.95), Vec3::Z, 0.08, 0.09), (Vec3::new(x, 0.45, 0.95), Vec3::Z, 0.08, 0.09)], 3.0, 10, chrome);
        }
        g.chrome.cuboid(Vec3::new(2.58, 0.55, -0.7), Vec3::new(2.62, 0.78, 0.7), chrome);
        for z in [-0.72f32, 0.72] {
            g.lights.sphere(Vec3::new(2.58, 0.8, z), Vec3::new(0.05, 0.1, 0.1), 10, lamp);
        }
        for (x, z) in [(1.6f32, 0.86f32), (1.6, -0.86), (-1.6, 0.86), (-1.6, -0.86)] {
            wheel(&mut g, x, z, 0.35, 0.11, false, true);
        }
    } else if year < 1981 {
        // ---------------------------------------------------------------- long 60s / 70s sedans, wagons, pickups, vans
        let (hl, hw) = (2.65, 1.0);
        if bodyk == Body::Van {
            body(&mut g.paint, &[(-2.3, 0.4, 2.1, 0.95), (1.9, 0.4, 2.1, 0.95), (2.3, 0.45, 1.3, 0.9)], 8.0, p);
            g.glass.cuboid(Vec3::new(2.05, 1.3, -0.8), Vec3::new(2.25, 1.85, 0.8), glass);
            g.glass.loft_box(&[s(1.2, 1.6, 0.96, 0.22), s(1.95, 1.6, 0.96, 0.22)], 8.0, 10, glass);
            if id % 2 == 0 {
                g.dark.cuboid(Vec3::new(-2.2, 0.9, -0.965), Vec3::new(1.0, 1.1, 0.965), c3([0.92, 0.9, 0.85]));
            }
        } else {
            body(&mut g.paint, &[(-hl, 0.38, 0.85, 0.94), (-hl + 0.15, 0.32, 0.9, hw), (hl - 0.1, 0.32, 0.88, hw), (hl, 0.36, 0.82, 0.95)], 7.0, p);
            let (c0, c1) = match bodyk {
                Body::Wagon => (-2.45, 0.55),
                Body::Pickup => (-0.35, 0.55),
                _ => (-1.45, 0.45),
            };
            let roof = if livery == Livery::Plain && id % 4 == 0 { c3([0.12, 0.1, 0.09]) } else { roofc };
            body(&mut g.paint, &[(c0, 0.88, 1.38, 0.9), (c1 - 0.3, 0.88, 1.38, 0.9), (c1, 0.88, 1.12, 0.88)], 8.0, roof);
            g.glass.loft_box(&[s(c0 + 0.1, 1.12, 0.905, 0.2), s(c1 - 0.3, 1.12, 0.905, 0.2), s(c1 - 0.05, 1.02, 0.89, 0.12)], 8.0, 12, glass);
            if bodyk == Body::Wagon && year >= 1966 {
                for z in [-1.005f32, 1.005] {
                    g.dark.cuboid(Vec3::new(-2.4, 0.45, z - 0.005), Vec3::new(1.9, 0.82, z + 0.005), c3([0.45, 0.28, 0.14]));
                }
            }
            if bodyk == Body::Pickup {
                g.dark.cuboid(Vec3::new(-2.55, 0.85, -0.95), Vec3::new(-0.45, 0.9, 0.95), c3([0.1, 0.1, 0.1]));
            }
        }
        for x in [2.7f32, -2.7] {
            g.chrome.cuboid(Vec3::new(x - 0.05, 0.36, -0.98), Vec3::new(x + 0.05, 0.52, 0.98), chrome);
        }
        g.dark.cuboid(Vec3::new(2.62, 0.52, -0.75), Vec3::new(2.67, 0.78, 0.75), c3([0.08, 0.08, 0.08]));
        for z in [-0.78f32, 0.78] {
            g.lights.cuboid(Vec3::new(2.62, 0.55, z - 0.14), Vec3::new(2.68, 0.75, z + 0.14), lamp);
            g.lights.cuboid(Vec3::new(-2.7, 0.55, z - 0.2), Vec3::new(-2.64, 0.72, z + 0.2), tail);
        }
        for (x, z) in [(1.65f32, 0.9f32), (1.65, -0.9), (-1.65, 0.9), (-1.65, -0.9)] {
            wheel(&mut g, x, z, 0.36, 0.12, false, id % 5 == 0);
        }
    } else {
        // ---------------------------------------------------------------- 80s / 90s wedges and rounded sedans
        let rounded = year >= 1990;
        let n = if rounded { 3.2 } else { 5.5 };
        if bodyk == Body::Van {
            body(&mut g.paint, &[(-2.3, 0.35, 1.95, 0.92), (1.6, 0.35, 1.95, 0.92), (2.3, 0.4, 1.0, 0.88)], if rounded { 4.0 } else { 7.0 }, p);
            g.glass.loft_box(&[s(-1.8, 1.55, 0.93, 0.25), s(1.7, 1.5, 0.93, 0.25)], 7.0, 10, glass);
        } else {
            body(&mut g.paint, &[(-2.3, 0.35, 0.86, 0.86), (-2.1, 0.3, 0.9, 0.9), (1.9, 0.3, 0.82, 0.9), (2.3, 0.35, 0.72, 0.86)], n, p);
            let (c0, c1) = if bodyk == Body::Pickup { (-0.2, 0.7) } else { (-1.55, 0.75) };
            body(&mut g.paint, &[(c0, 0.84, 1.32, 0.82), (c0 + 0.3, 0.84, 1.36, 0.84), (c1 - 0.4, 0.84, 1.36, 0.84), (c1, 0.84, 1.0, 0.86)], if rounded { 3.5 } else { 7.0 }, roofc);
            g.glass.loft_box(&[s(c0 + 0.1, 1.08, 0.83, 0.2), s(c1 - 0.45, 1.1, 0.855, 0.22), s(c1 - 0.05, 0.95, 0.87, 0.08)], 6.0, 12, glass);
            if bodyk == Body::Pickup {
                g.dark.cuboid(Vec3::new(-2.2, 0.8, -0.86), Vec3::new(-0.3, 0.85, 0.86), c3([0.1, 0.1, 0.1]));
            }
        }
        // black plastic bumpers, flush lights
        for x in [2.35f32, -2.35] {
            g.dark.loft_box(&[(Vec3::new(x, 0.42, -0.9), Vec3::Z, 0.07, 0.1), (Vec3::new(x, 0.42, 0.9), Vec3::Z, 0.07, 0.1)], 4.0, 10, c3([0.06, 0.06, 0.07]));
        }
        g.dark.cuboid(Vec3::new(2.28, 0.58, -0.45), Vec3::new(2.32, 0.7, 0.45), c3([0.05, 0.05, 0.05]));
        for z in [-0.7f32, 0.7] {
            g.lights.cuboid(Vec3::new(2.26, 0.58, z - 0.2), Vec3::new(2.32, 0.7, z + 0.2), lamp);
            g.lights.cuboid(Vec3::new(-2.32, 0.6, z - 0.22), Vec3::new(-2.26, 0.78, z + 0.22), tail);
        }
        for (x, z) in [(1.45f32, 0.82f32), (1.45, -0.82), (-1.45, 0.82), (-1.45, -0.82)] {
            wheel(&mut g, x, z, 0.32, 0.12, false, false);
        }
    }
    // taxi sign / police lights
    let roof_y = if year < 1935 { 1.93 } else if year < 1950 { 1.64 } else if year < 1981 { 1.4 } else { 1.37 };
    match livery {
        Livery::Taxi => {
            g.lights.cuboid(Vec3::new(-0.35, roof_y, -0.18), Vec3::new(-0.05, roof_y + 0.14, 0.18), c3([1.0, 0.85, 0.3]));
            if (1950..1985).contains(&year) && city == CityId::NewYork {
                // checker band
                for k in 0..20 {
                    if k % 2 == 0 {
                        for z in [-1.005f32, 1.005] {
                            g.dark.cuboid(Vec3::new(-2.0 + k as f32 * 0.2, 0.62, z - 0.004), Vec3::new(-1.9 + k as f32 * 0.2, 0.72, z + 0.004), black);
                        }
                    }
                }
            }
        }
        Livery::Police => {
            if year < 1960 {
                g.lights.sphere(Vec3::new(-0.2, roof_y + 0.08, 0.0), Vec3::new(0.08, 0.1, 0.08), 8, tail);
            } else {
                g.dark.cuboid(Vec3::new(-0.35, roof_y, -0.6), Vec3::new(-0.1, roof_y + 0.05, 0.6), black);
                g.lights.cuboid(Vec3::new(-0.33, roof_y + 0.05, -0.55), Vec3::new(-0.12, roof_y + 0.17, -0.1), tail);
                g.lights.cuboid(Vec3::new(-0.33, roof_y + 0.05, 0.1), Vec3::new(-0.12, roof_y + 0.17, 0.55), c3([0.2, 0.35, 1.0]));
            }
        }
        Livery::Plain => {}
    }
    g
}
