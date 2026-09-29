//! Procedural textures generated at startup (no external image files).
//! Full-colour material maps (brick, stone, wood siding, roof tiles, grass,
//! cobbles, plaster...) with normal maps derived from the same height field.
//! Vertex colours still tint painted surfaces per building.

use crate::util::hash2;
use bevy::asset::RenderAssetUsages;
use bevy::image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

fn vnoise(x: f32, y: f32, period: i32, salt: u32) -> f32 {
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    let fx = x - xi as f32;
    let fy = y - yi as f32;
    let h = |a: i32, b: i32| (hash2(a.rem_euclid(period), b.rem_euclid(period), salt) & 0xFFFF) as f32 / 65535.0;
    let sx = fx * fx * (3.0 - 2.0 * fx);
    let sy = fy * fy * (3.0 - 2.0 * fy);
    let a = h(xi, yi) + (h(xi + 1, yi) - h(xi, yi)) * sx;
    let b = h(xi, yi + 1) + (h(xi + 1, yi + 1) - h(xi, yi + 1)) * sx;
    a + (b - a) * sy
}

/// Fractal noise, tileable on `period` cells.
fn fbm(x: f32, y: f32, period: i32, salt: u32) -> f32 {
    let mut v = 0.0;
    let mut amp = 0.5;
    let mut f = 1.0;
    for o in 0..4 {
        v += vnoise(x * f, y * f, period * f as i32, salt + o) * amp;
        amp *= 0.5;
        f *= 2.0;
    }
    v
}

fn sampler() -> ImageSampler {
    ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        anisotropy_clamp: 8,
        ..default()
    })
}

/// A surface sampled at `size`²: colour (linear-ish 0..1 sRGB values) + height.
struct Field {
    size: u32,
    albedo: Vec<[f32; 3]>,
    height: Vec<f32>,
}

fn field(size: u32, f: impl Fn(f32, f32) -> ([f32; 3], f32)) -> Field {
    let mut albedo = Vec::with_capacity((size * size) as usize);
    let mut height = Vec::with_capacity((size * size) as usize);
    for y in 0..size {
        for x in 0..size {
            let (a, h) = f(x as f32 / size as f32, y as f32 / size as f32);
            albedo.push([a[0].clamp(0.0, 1.0), a[1].clamp(0.0, 1.0), a[2].clamp(0.0, 1.0)]);
            height.push(h.clamp(0.0, 1.0));
        }
    }
    Field { size, albedo, height }
}

fn to_images(f: &Field, strength: f32) -> (Image, Image) {
    let s = f.size as i32;
    let at = |x: i32, y: i32| f.height[(y.rem_euclid(s) * s + x.rem_euclid(s)) as usize];
    let mut col = Vec::with_capacity((s * s * 4) as usize);
    let mut nrm = Vec::with_capacity((s * s * 4) as usize);
    for y in 0..s {
        for x in 0..s {
            let a = f.albedo[(y * s + x) as usize];
            col.extend_from_slice(&[(a[0] * 255.0) as u8, (a[1] * 255.0) as u8, (a[2] * 255.0) as u8, 255]);
            let dx = (at(x + 1, y) - at(x - 1, y)) * strength;
            let dy = (at(x, y + 1) - at(x, y - 1)) * strength;
            let n = Vec3::new(-dx, -dy, 1.0).normalize();
            nrm.extend_from_slice(&[((n.x * 0.5 + 0.5) * 255.0) as u8, ((n.y * 0.5 + 0.5) * 255.0) as u8, ((n.z * 0.5 + 0.5) * 255.0) as u8, 255]);
        }
    }
    let ci = with_mips(f.size, col, TextureFormat::Rgba8UnormSrgb, false);
    let ni = with_mips(f.size, nrm, TextureFormat::Rgba8Unorm, true);
    (ci, ni)
}

/// Builds the whole mip chain on the CPU (box filter), so distant cobbles
/// and bricks don't shimmer.
fn with_mips(size: u32, base: Vec<u8>, format: TextureFormat, normal: bool) -> Image {
    let mut data = base.clone();
    let mut prev = base;
    let mut sz = size;
    let mut levels = 1;
    while sz > 1 {
        let ns = sz / 2;
        let mut next = Vec::with_capacity((ns * ns * 4) as usize);
        for y in 0..ns {
            for x in 0..ns {
                let mut acc = [0f32; 4];
                for (ox, oy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                    let i = (((y * 2 + oy) * sz + x * 2 + ox) * 4) as usize;
                    for k in 0..4 {
                        acc[k] += prev[i + k] as f32;
                    }
                }
                let mut px = [acc[0] / 4.0, acc[1] / 4.0, acc[2] / 4.0, acc[3] / 4.0];
                if normal {
                    // renormalise and flatten a little with distance
                    let v = Vec3::new(px[0] / 127.5 - 1.0, px[1] / 127.5 - 1.0, px[2] / 127.5 - 1.0).normalize_or(Vec3::Z);
                    px = [(v.x * 0.5 + 0.5) * 255.0, (v.y * 0.5 + 0.5) * 255.0, (v.z * 0.5 + 0.5) * 255.0, 255.0];
                }
                next.extend(px.iter().map(|v| *v as u8));
            }
        }
        data.extend_from_slice(&next);
        prev = next;
        sz = ns;
        levels += 1;
    }
    let ext = Extent3d { width: size, height: size, depth_or_array_layers: 1 };
    let mut im = Image::new_uninit(ext, TextureDimension::D2, format, RenderAssetUsages::RENDER_WORLD);
    im.texture_descriptor.mip_level_count = levels;
    im.data = Some(data);
    im.sampler = sampler();
    im
}

pub type TexPair = (Handle<Image>, Handle<Image>);

/// Every surface material of the game.
pub struct Tex {
    pub cobble: TexPair,
    pub asphalt: TexPair,
    pub grain: TexPair,
    pub plaster: TexPair,
    pub planks: TexPair,
    pub brick: TexPair,
    pub brick_yellow: TexPair,
    pub stone: TexPair,
    pub siding: TexPair,
    pub tiles: TexPair,
    pub slate: TexPair,
    pub slabs: TexPair,
    pub fabric: TexPair,
    pub grass: TexPair,
    pub dirt: TexPair,
    pub sand: TexPair,
    pub gravel: TexPair,
    pub floor_tiles: TexPair,
    pub wallpaper: TexPair,
    pub leather: TexPair,
}

/// Voronoi distances (nearest, second nearest) and the cell id, tileable on `n` cells.
fn voronoi(fx: f32, fy: f32, n: i32, salt: u32) -> (f32, f32, u32) {
    let (cx, cy) = (fx.floor() as i32, fy.floor() as i32);
    let (mut d1, mut d2, mut id) = (9.0f32, 9.0f32, 0u32);
    for oy in -1..=1 {
        for ox in -1..=1 {
            let (gx, gy) = (cx + ox, cy + oy);
            let h = hash2(gx.rem_euclid(n), gy.rem_euclid(n), salt);
            let px = gx as f32 + 0.15 + (h & 0xFF) as f32 / 255.0 * 0.7;
            let py = gy as f32 + 0.15 + ((h >> 8) & 0xFF) as f32 / 255.0 * 0.7;
            let d = ((fx - px).powi(2) + (fy - py).powi(2)).sqrt();
            if d < d1 {
                d2 = d1;
                d1 = d;
                id = h;
            } else if d < d2 {
                d2 = d;
            }
        }
    }
    (d1, d2, id)
}

fn lerp3(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

fn mul3(a: [f32; 3], k: f32) -> [f32; 3] {
    [a[0] * k, a[1] * k, a[2] * k]
}

fn h01(x: i32, y: i32, s: u32) -> f32 {
    (hash2(x, y, s) & 0xFFFF) as f32 / 65535.0
}

/// Running-bond bricks: per-brick tone, chipped corners, soot and moss at the base.
fn bricks(u: f32, v: f32, palette: &[[f32; 3]], mortar: [f32; 3], salt: u32) -> ([f32; 3], f32) {
    let rows = 16.0;
    let fy = v * rows;
    let row = fy.floor() as i32;
    let fx = u * 8.0 + if row % 2 == 0 { 0.5 } else { 0.0 };
    let bi = fx.floor() as i32;
    let (bx, by) = (fx.fract(), fy.fract());
    let n = fbm(u * 64.0, v * 64.0, 64, salt);
    let chip = h01(bi, row, salt + 1) < 0.2 && (bx < 0.12 || bx > 0.88) && (by < 0.25 || by > 0.75);
    let gap = by < 0.12 || bx < 0.035 || chip;
    let tone = palette[(hash2(bi.rem_euclid(8), row.rem_euclid(16), salt) as usize) % palette.len()];
    let var = 0.85 + h01(bi, row, salt + 2) * 0.3;
    let burnt = if h01(bi, row, salt + 3) < 0.1 { 0.6 } else { 1.0 };
    let soot = fbm(u * 3.0, v * 3.0, 3, salt + 4);
    if gap {
        (mul3(mortar, 0.85 + n * 0.3), 0.15 + n * 0.1)
    } else {
        let bevel = (by - 0.12).min(1.0 - by).min(bx - 0.035).min(1.0 - bx).clamp(0.0, 0.08) / 0.08;
        let c = mul3(tone, var * burnt * (0.85 + n * 0.3) * (1.0 - soot * 0.18));
        (c, 0.55 + bevel * 0.35 + n * 0.1)
    }
}

pub fn make(images: &mut Assets<Image>) -> Tex {
    let mut add = |f: Field, k: f32| {
        let (c, n) = to_images(&f, k);
        (images.add(c), images.add(n))
    };
    // rounded cobblestones (granite setts) with dark wet gaps
    let cobble = add(
        field(512, |u, v| {
            let (d1, d2, id) = voronoi(u * 14.0, v * 14.0, 14, 7);
            let edge = ((d2 - d1) * 4.0).clamp(0.0, 1.0);
            let dome = edge.sqrt();
            let base = [[0.42, 0.4, 0.38], [0.36, 0.35, 0.34], [0.46, 0.42, 0.37], [0.33, 0.31, 0.3], [0.5, 0.48, 0.45]][(id % 5) as usize];
            let n = fbm(u * 48.0, v * 48.0, 48, 3);
            let c = if edge < 0.12 { [0.1, 0.09, 0.08] } else { mul3(base, 0.8 + n * 0.45) };
            (c, dome * 0.8 + n * 0.2)
        }),
        3.0,
    );
    let asphalt = add(
        field(256, |u, v| {
            let n = fbm(u * 64.0, v * 64.0, 64, 21);
            let patch = vnoise(u * 5.0, v * 5.0, 5, 22);
            let grit = vnoise(u * 180.0, v * 180.0, 180, 23);
            let g = 0.2 + n * 0.08 - (patch - 0.5).max(0.0) * 0.06 + grit * 0.05;
            ([g, g, g * 1.03], 0.5 + n * 0.3 + grit * 0.2)
        }),
        2.0,
    );
    let grain = add(
        field(256, |u, v| {
            let n = fbm(u * 16.0, v * 16.0, 16, 11);
            let g = 0.68 + n * 0.32;
            ([g, g, g], n)
        }),
        1.5,
    );
    // plaster / stucco (tinted per building): soft stains, rain streaks, flaking patches
    let plaster = add(
        field(256, |u, v| {
            let n = fbm(u * 6.0, v * 6.0, 6, 31);
            let fine = vnoise(u * 90.0, v * 90.0, 90, 32);
            let streak = vnoise(u * 40.0, v * 3.0, 40, 34) * vnoise(u * 3.0, v * 2.0, 3, 35);
            let flake = fbm(u * 5.0, v * 5.0, 5, 36) > 0.68;
            let g = if flake { 0.72 - fine * 0.1 } else { 0.9 + n * 0.1 - fine * 0.05 - streak * 0.08 };
            let c = if flake { [g * 0.95, g * 0.85, g * 0.75] } else { [g, g, g] };
            (c, if flake { 0.3 } else { 0.55 + fine * 0.3 + n * 0.15 })
        }),
        2.0,
    );
    // oak floorboards
    let planks = add(
        field(512, |u, v| {
            let fy = v * 16.0;
            let row = fy.floor() as i32;
            let fx = u * 2.0 + h01(row, 0, 5);
            let board = (fx * 2.0).floor() as i32;
            let seam = fy.fract() < 0.04 || (fx * 2.0).fract() < 0.006;
            let tone = [[0.52, 0.34, 0.2], [0.46, 0.3, 0.17], [0.58, 0.4, 0.24], [0.42, 0.27, 0.16]][(hash2(row, board, 6) % 4) as usize];
            let grain = (vnoise(fx * 30.0, fy * 0.8 + row as f32 * 3.1, 60, row as u32) * 6.0).sin() * 0.5 + 0.5;
            let c = if seam { [0.12, 0.08, 0.05] } else { mul3(tone, 0.82 + grain * 0.22) };
            (c, if seam { 0.0 } else { 0.6 + grain * 0.2 })
        }),
        3.0,
    );
    let brick = add(field(512, |u, v| bricks(u, v, &[[0.55, 0.22, 0.15], [0.5, 0.2, 0.14], [0.62, 0.28, 0.18], [0.45, 0.18, 0.13], [0.58, 0.3, 0.22]], [0.62, 0.6, 0.55], 40)), 4.0);
    let brick_yellow = add(field(512, |u, v| bricks(u, v, &[[0.66, 0.55, 0.36], [0.58, 0.48, 0.32], [0.7, 0.6, 0.42], [0.5, 0.4, 0.28], [0.62, 0.5, 0.3]], [0.55, 0.53, 0.5], 41)), 4.0);
    // dressed stone blocks (ashlar) for civic buildings and churches
    let stone = add(
        field(512, |u, v| {
            let fy = v * 6.0;
            let row = fy.floor() as i32;
            let fx = u * 3.0 + if row % 2 == 0 { 0.5 } else { 0.0 };
            let bi = fx.floor() as i32;
            let joint = fy.fract() < 0.03 || fx.fract() < 0.015;
            let tone = 0.75 + h01(bi, row, 50) * 0.18;
            let n = fbm(u * 40.0, v * 40.0, 40, 51);
            let pits = vnoise(u * 200.0, v * 200.0, 200, 52) > 0.82;
            let c = if joint { [0.35, 0.33, 0.3] } else { [tone * (0.9 + n * 0.2) - if pits { 0.12 } else { 0.0 }, tone * (0.88 + n * 0.2) - if pits { 0.12 } else { 0.0 }, tone * (0.8 + n * 0.2) - if pits { 0.12 } else { 0.0 }] };
            (c, if joint { 0.1 } else { 0.6 + n * 0.3 })
        }),
        3.0,
    );
    // painted clapboard siding (tinted per house)
    let siding = add(
        field(256, |u, v| {
            let fy = v * 12.0;
            let b = fy.fract();
            let n = fbm(u * 20.0, v * 20.0, 20, 60);
            let peel = fbm(u * 8.0, v * 8.0, 8, 61) > 0.72;
            let shadow = if b > 0.9 { 0.55 } else { 0.9 + b * 0.1 };
            let g = shadow * (0.95 + n * 0.1);
            (if peel { [0.5 * g, 0.4 * g, 0.3 * g] } else { [g, g, g] }, b * 0.8)
        }),
        3.0,
    );
    // terracotta / clay roof tiles (tinted by roof colour)
    let tiles = add(
        field(256, |u, v| {
            let fy = v * 10.0;
            let row = fy.floor() as i32;
            let fx = u * 6.0 + if row % 2 == 0 { 0.5 } else { 0.0 };
            let (bx, by) = (fx.fract(), fy.fract());
            let tone = 0.75 + h01(fx.floor() as i32, row, 19) * 0.25;
            let round = (bx * std::f32::consts::PI).sin();
            let lichen = fbm(u * 6.0, v * 6.0, 6, 70) > 0.7;
            let g = tone * (0.55 + by * 0.45) * (0.75 + round * 0.25);
            (if lichen { [g * 0.8, g * 0.85, g * 0.6] } else { [g, g, g] }, by * 0.7 + round * 0.3)
        }),
        4.0,
    );
    // slate shingles (London, Bergen)
    let slate = add(
        field(256, |u, v| {
            let fy = v * 12.0;
            let row = fy.floor() as i32;
            let fx = u * 8.0 + if row % 2 == 0 { 0.5 } else { 0.0 };
            let (bx, by) = (fx.fract(), fy.fract());
            let tone = 0.3 + h01(fx.floor() as i32, row, 80) * 0.12;
            let gap = bx < 0.03;
            ([tone * 0.95, tone, tone * 1.08], if gap { 0.0 } else { by })
        }),
        3.0,
    );
    // sidewalk paving slabs
    let slabs = add(
        field(256, |u, v| {
            let (fx, fy) = (u * 4.0, v * 4.0);
            let joint = fx.fract() < 0.03 || fy.fract() < 0.03;
            let tone = 0.55 + h01(fx as i32, fy as i32, 41) * 0.15;
            let n = fbm(u * 32.0, v * 32.0, 32, 42);
            let stain = fbm(u * 4.0, v * 4.0, 4, 43) > 0.65;
            let g = tone - n * 0.12 - if stain { 0.08 } else { 0.0 };
            (if joint { [0.2, 0.19, 0.18] } else { [g, g * 0.98, g * 0.95] }, if joint { 0.0 } else { 0.6 + n * 0.3 })
        }),
        3.0,
    );
    let fabric = add(
        field(128, |u, v| {
            let a = ((u * 128.0).sin() * (v * 128.0).cos()).abs();
            let n = vnoise(u * 16.0, v * 16.0, 16, 51);
            let g = 0.82 + a * 0.12 + n * 0.06;
            ([g, g, g], a * 0.6 + n * 0.4)
        }),
        1.2,
    );
    // lawn: many blade strokes, clover, bare patches
    let grass = add(
        field(512, |u, v| {
            let n = fbm(u * 8.0, v * 8.0, 8, 90);
            let blades = vnoise(u * 400.0, v * 90.0, 400, 91) * 0.6 + vnoise(u * 90.0, v * 380.0, 90, 92) * 0.4;
            let dry = fbm(u * 3.0, v * 3.0, 3, 93);
            let bare = fbm(u * 6.0, v * 6.0, 6, 94) > 0.74;
            let green = lerp3([0.16, 0.28, 0.08], [0.32, 0.42, 0.14], blades);
            let c = lerp3(green, [0.42, 0.4, 0.18], (dry - 0.45).max(0.0) * 1.6);
            let c = if bare { lerp3(c, [0.3, 0.23, 0.15], 0.7) } else { mul3(c, 0.8 + n * 0.35) };
            (c, blades * 0.8 + n * 0.2)
        }),
        2.5,
    );
    let dirt = add(
        field(256, |u, v| {
            let n = fbm(u * 12.0, v * 12.0, 12, 100);
            let pebbles = vnoise(u * 120.0, v * 120.0, 120, 101) > 0.8;
            let c = if pebbles { [0.45, 0.42, 0.38] } else { mul3([0.36, 0.27, 0.18], 0.75 + n * 0.45) };
            (c, n * 0.7 + if pebbles { 0.3 } else { 0.0 })
        }),
        2.5,
    );
    let sand = add(
        field(256, |u, v| {
            let ripple = ((u * 40.0 + fbm(u * 4.0, v * 4.0, 4, 110) * 6.0).sin() * 0.5 + 0.5) * 0.3;
            let grit = vnoise(u * 200.0, v * 200.0, 200, 111);
            let g = 0.78 + grit * 0.12 - ripple * 0.1;
            ([g, g * 0.9, g * 0.7], ripple + grit * 0.2)
        }),
        2.0,
    );
    let gravel = add(
        field(256, |u, v| {
            let (d1, d2, id) = voronoi(u * 40.0, v * 40.0, 40, 120);
            let edge = ((d2 - d1) * 3.0).clamp(0.0, 1.0);
            let t = 0.4 + (id % 7) as f32 * 0.05;
            (if edge < 0.15 { [0.15, 0.14, 0.12] } else { [t, t * 0.97, t * 0.92] }, edge.sqrt())
        }),
        3.0,
    );
    // checkerboard / hex tiles for kitchens, bathrooms and bars
    let floor_tiles = add(
        field(256, |u, v| {
            let (fx, fy) = (u * 8.0, v * 8.0);
            let joint = fx.fract() < 0.04 || fy.fract() < 0.04;
            let black = ((fx as i32) + (fy as i32)) % 2 == 0;
            let n = vnoise(u * 60.0, v * 60.0, 60, 130);
            let g = if black { 0.08 + n * 0.04 } else { 0.85 + n * 0.08 };
            (if joint { [0.5, 0.48, 0.45] } else { [g, g, g * 0.97] }, if joint { 0.2 } else { 0.8 })
        }),
        1.5,
    );
    // damask wallpaper (tinted per room)
    let wallpaper = add(
        field(256, |u, v| {
            let (fx, fy) = ((u * 6.0).fract() - 0.5, (v * 4.0).fract() - 0.5);
            let motif = ((fx * 9.0).sin() * (fy * 7.0).cos() + (fx * fx + fy * fy) * 4.0).sin().abs() < 0.2;
            let stripe = (u * 24.0).fract() < 0.08;
            let n = vnoise(u * 50.0, v * 50.0, 50, 140);
            let g = if motif { 0.78 } else if stripe { 0.86 } else { 0.95 } - n * 0.04;
            ([g, g, g], if motif { 0.6 } else { 0.5 })
        }),
        0.8,
    );
    let leather = add(
        field(128, |u, v| {
            let (d1, d2, _) = voronoi(u * 30.0, v * 30.0, 30, 150);
            let crease = ((d2 - d1) * 6.0).clamp(0.0, 1.0);
            let g = 0.75 + crease * 0.25;
            ([g, g, g], crease)
        }),
        1.5,
    );
    Tex { cobble, asphalt, grain, plaster, planks, brick, brick_yellow, stone, siding, tiles, slate, slabs, fabric, grass, dirt, sand, gravel, floor_tiles, wallpaper, leather }
}

// ---------------------------------------------------------------- photographic PBR materials (Poly Haven, CC0)

/// Colour, normal and ARM (occlusion / roughness / metal) maps of a scanned material.
#[derive(Clone)]
pub struct Pbr {
    pub diff: Handle<Image>,
    pub nor: Handle<Image>,
    pub arm: Handle<Image>,
}

fn decode(bytes: &[u8], srgb: bool, normal: bool) -> Image {
    use bevy::image::{CompressedImageFormats, ImageType};
    let im = Image::from_buffer(bytes, ImageType::Extension("jpg"), CompressedImageFormats::NONE, srgb, ImageSampler::Default, RenderAssetUsages::RENDER_WORLD).expect("textura embutida");
    let mut size = im.texture_descriptor.size.width;
    let mut data = im.data.clone().unwrap_or_default();
    // 1k is plenty from the gameplay camera and keeps video memory low on family PCs
    while size > 1024 {
        let h = size / 2;
        let mut out = vec![0u8; (h * h * 4) as usize];
        for y in 0..h {
            for x in 0..h {
                for ch in 0..4 {
                    let at = |xx: u32, yy: u32| data[((yy * size + xx) * 4 + ch) as usize] as u32;
                    let s = at(x * 2, y * 2) + at(x * 2 + 1, y * 2) + at(x * 2, y * 2 + 1) + at(x * 2 + 1, y * 2 + 1);
                    out[((y * h + x) * 4 + ch) as usize] = (s / 4) as u8;
                }
            }
        }
        data = out;
        size = h;
    }
    with_mips(size, data, if srgb { TextureFormat::Rgba8UnormSrgb } else { TextureFormat::Rgba8Unorm }, normal)
}

fn pbr(images: &mut Assets<Image>, d: &[u8], n: &[u8], a: &[u8]) -> Pbr {
    Pbr { diff: images.add(decode(d, true, false)), nor: images.add(decode(n, false, true)), arm: images.add(decode(a, false, false)) }
}

macro_rules! real {
    ($images:expr, $slot:literal) => {
        pbr(
            $images,
            include_bytes!(concat!("../tex/", $slot, "_diff.jpg")),
            include_bytes!(concat!("../tex/", $slot, "_nor.jpg")),
            include_bytes!(concat!("../tex/", $slot, "_arm.jpg")),
        )
    };
}

pub struct Real {
    pub cobble: Pbr,
    pub cobble2: Pbr,
    pub asphalt: Pbr,
    pub sidewalk: Pbr,
    pub brick: Pbr,
    pub brick_yellow: Pbr,
    pub stone: Pbr,
    pub plaster: Pbr,
    pub plaster_old: Pbr,
    pub siding: Pbr,
    pub roof_tiles: Pbr,
    pub slate: Pbr,
    pub grass: Pbr,
    pub grass_dry: Pbr,
    pub dirt: Pbr,
    pub sand: Pbr,
    pub gravel: Pbr,
    pub wood_floor: Pbr,
    pub parquet: Pbr,
    pub floor_tiles: Pbr,
    pub terracotta: Pbr,
    pub velvet: Pbr,
    pub planks_dark: Pbr,
    pub wood_light: Pbr,
}

pub fn make_real(images: &mut Assets<Image>) -> Real {
    Real {
        cobble: real!(images, "cobble"),
        cobble2: real!(images, "cobble2"),
        asphalt: real!(images, "asphalt"),
        sidewalk: real!(images, "sidewalk"),
        brick: real!(images, "brick"),
        brick_yellow: real!(images, "brick_yellow"),
        stone: real!(images, "stone"),
        plaster: real!(images, "plaster"),
        plaster_old: real!(images, "plaster_old"),
        siding: real!(images, "siding"),
        roof_tiles: real!(images, "roof_tiles"),
        slate: real!(images, "slate"),
        grass: real!(images, "grass"),
        grass_dry: real!(images, "grass_dry"),
        dirt: real!(images, "dirt"),
        sand: real!(images, "sand"),
        gravel: real!(images, "gravel"),
        wood_floor: real!(images, "wood_floor"),
        parquet: real!(images, "parquet"),
        floor_tiles: real!(images, "floor_tiles"),
        terracotta: real!(images, "terracotta"),
        velvet: real!(images, "velvet"),
        planks_dark: real!(images, "planks_dark"),
        wood_light: real!(images, "wood_light"),
    }
}
