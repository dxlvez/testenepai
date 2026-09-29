//! Procedural textures generated at startup (no external image files).
//! Every surface gets a grey detail map (multiplied with the vertex colour)
//! and a matching normal map derived from the same height field, so bricks,
//! cobbles, planks and roof tiles catch the light of the street lamps.

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

/// A surface sampled at `size`²: returns (albedo 0..1, height 0..1).
struct Field {
    size: u32,
    albedo: Vec<f32>,
    height: Vec<f32>,
}

fn field(size: u32, f: impl Fn(f32, f32) -> (f32, f32)) -> Field {
    let mut albedo = Vec::with_capacity((size * size) as usize);
    let mut height = Vec::with_capacity((size * size) as usize);
    for y in 0..size {
        for x in 0..size {
            let (a, h) = f(x as f32 / size as f32, y as f32 / size as f32);
            albedo.push(a.clamp(0.0, 1.0));
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
            let a = (f.albedo[(y * s + x) as usize] * 255.0) as u8;
            col.extend_from_slice(&[a, a, a, 255]);
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

/// Grey detail textures multiplied with vertex colours, with normal maps.
pub struct Tex {
    pub cobble: (Handle<Image>, Handle<Image>),
    pub asphalt: (Handle<Image>, Handle<Image>),
    pub grain: (Handle<Image>, Handle<Image>),
    pub plaster: (Handle<Image>, Handle<Image>),
    pub planks: (Handle<Image>, Handle<Image>),
    pub brick: (Handle<Image>, Handle<Image>),
    pub tiles: (Handle<Image>, Handle<Image>),
    pub slabs: (Handle<Image>, Handle<Image>),
    pub fabric: (Handle<Image>, Handle<Image>),
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

pub fn make(images: &mut Assets<Image>) -> Tex {
    let mut add = |f: Field, k: f32| {
        let (c, n) = to_images(&f, k);
        (images.add(c), images.add(n))
    };
    // rounded cobblestones with wet dark gaps
    let cobble = add(
        field(512, |u, v| {
            let (d1, d2, id) = voronoi(u * 14.0, v * 14.0, 14, 7);
            let edge = ((d2 - d1) * 4.0).clamp(0.0, 1.0);
            let dome = edge.sqrt();
            let tone = 0.62 + (id & 0xFF) as f32 / 255.0 * 0.3;
            let n = fbm(u * 32.0, v * 32.0, 32, 3);
            (if edge < 0.12 { 0.25 } else { tone - n * 0.25 }, dome * 0.8 + n * 0.2)
        }),
        3.0,
    );
    let asphalt = add(
        field(256, |u, v| {
            let n = fbm(u * 64.0, v * 64.0, 64, 21);
            let crack = (vnoise(u * 6.0, v * 6.0, 6, 22) - 0.5).abs() < 0.012;
            (if crack { 0.45 } else { 0.72 + n * 0.28 }, if crack { 0.2 } else { 0.5 + n * 0.5 })
        }),
        2.5,
    );
    let grain = add(
        field(256, |u, v| {
            let n = fbm(u * 16.0, v * 16.0, 16, 11);
            (0.68 + n * 0.32, n)
        }),
        1.5,
    );
    // plaster: soft stains, rain streaks and fine grit
    let plaster = add(
        field(256, |u, v| {
            let n = fbm(u * 6.0, v * 6.0, 6, 31);
            let fine = vnoise(u * 90.0, v * 90.0, 90, 32);
            let streak = vnoise(u * 40.0, v * 3.0, 40, 34) * vnoise(u * 3.0, v * 2.0, 3, 35);
            (0.8 + n * 0.18 - fine * 0.06 - streak * 0.06, 0.5 + fine * 0.35 + n * 0.15)
        }),
        1.6,
    );
    // floor boards (2 repeats = 4m: 16 boards)
    let planks = add(
        field(256, |u, v| {
            let fy = v * 8.0;
            let row = fy.floor() as i32;
            let fx = u * 2.0 + (hash2(row, 0, 5) & 0xFF) as f32 / 255.0;
            let seam = fy.fract() < 0.05 || (fx * 2.0).fract() < 0.01;
            let wood = vnoise(fx * 24.0, fy * 1.2, 48, row as u32) * 0.3 + (hash2(row, (fx * 2.0) as i32, 6) & 0xFF) as f32 / 255.0 * 0.15;
            (if seam { 0.4 } else { 0.72 + wood }, if seam { 0.0 } else { 0.6 + wood * 0.4 })
        }),
        3.0,
    );
    // bricks: 8 rows per repeat (2m) -> 25 cm courses
    let brick = add(
        field(512, |u, v| {
            let fy = v * 8.0;
            let row = fy.floor() as i32;
            let fx = u * 4.0 + if row % 2 == 0 { 0.5 } else { 0.0 };
            let (bx, by) = (fx.fract(), fy.fract());
            let mortar = by < 0.1 || bx < 0.04;
            let tone = 0.72 + (hash2(fx.floor() as i32, row, 9) & 0xFF) as f32 / 255.0 * 0.28;
            let n = fbm(u * 64.0, v * 64.0, 64, 4);
            let bevel = (by - 0.1).min(1.0 - by).min(bx - 0.04).min(1.0 - bx) * 12.0;
            (if mortar { 0.55 + n * 0.1 } else { tone - n * 0.2 }, if mortar { 0.1 } else { 0.5 + bevel.clamp(0.0, 0.5) + n * 0.15 })
        }),
        2.5,
    );
    // roof tiles / shingles: overlapping rows
    let tiles = add(
        field(256, |u, v| {
            let fy = v * 10.0;
            let row = fy.floor() as i32;
            let fx = u * 6.0 + if row % 2 == 0 { 0.5 } else { 0.0 };
            let (bx, by) = (fx.fract(), fy.fract());
            let tone = 0.7 + (hash2(fx.floor() as i32, row, 19) & 0xFF) as f32 / 255.0 * 0.3;
            let gap = bx < 0.05;
            (if gap { 0.35 } else { tone * (0.65 + by * 0.35) }, if gap { 0.0 } else { by })
        }),
        4.0,
    );
    // paving slabs for sidewalks
    let slabs = add(
        field(256, |u, v| {
            let (fx, fy) = (u * 4.0, v * 4.0);
            let joint = fx.fract() < 0.03 || fy.fract() < 0.03;
            let tone = 0.75 + (hash2(fx as i32, fy as i32, 41) & 0xFF) as f32 / 255.0 * 0.2;
            let n = fbm(u * 32.0, v * 32.0, 32, 42);
            (if joint { 0.4 } else { tone - n * 0.2 }, if joint { 0.0 } else { 0.6 + n * 0.3 })
        }),
        3.0,
    );
    // woven fabric (clothes, bedding, curtains)
    let fabric = add(
        field(128, |u, v| {
            let a = ((u * 128.0).sin() * (v * 128.0).cos()).abs();
            let n = vnoise(u * 16.0, v * 16.0, 16, 51);
            (0.82 + a * 0.12 + n * 0.06, a * 0.6 + n * 0.4)
        }),
        1.2,
    );
    Tex { cobble, asphalt, grain, plaster, planks, brick, tiles, slabs, fabric }
}
