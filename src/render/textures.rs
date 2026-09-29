//! Procedural textures generated at startup (no external image files).

use crate::util::hash2;
use bevy::asset::RenderAssetUsages;
use bevy::image::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor};
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

fn img(size: u32, f: impl Fn(u32, u32) -> [u8; 4]) -> Image {
    let mut data = Vec::with_capacity((size * size * 4) as usize);
    for y in 0..size {
        for x in 0..size {
            data.extend_from_slice(&f(x, y));
        }
    }
    let mut im = Image::new(
        Extent3d { width: size, height: size, depth_or_array_layers: 1 },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    im.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..ImageSamplerDescriptor::linear()
    });
    im
}

/// Grey detail textures multiplied with vertex colours.
pub struct Tex {
    pub cobble: Handle<Image>,
    pub grain: Handle<Image>,
    pub planks: Handle<Image>,
    pub brick: Handle<Image>,
}

pub fn make(images: &mut Assets<Image>) -> Tex {
    let s = 256u32;
    // cobblestones: voronoi-ish rounded stones
    let cobble = img(s, |x, y| {
        let fx = x as f32 / s as f32 * 8.0;
        let fy = y as f32 / s as f32 * 8.0;
        let mut d1 = 9.0f32;
        let mut d2 = 9.0f32;
        let cx = fx.floor() as i32;
        let cy = fy.floor() as i32;
        for oy in -1..=1 {
            for ox in -1..=1 {
                let gx = cx + ox;
                let gy = cy + oy;
                let h = hash2(gx.rem_euclid(8), gy.rem_euclid(8), 7);
                let px = gx as f32 + 0.2 + (h & 0xFF) as f32 / 255.0 * 0.6;
                let py = gy as f32 + 0.2 + ((h >> 8) & 0xFF) as f32 / 255.0 * 0.6;
                let d = ((fx - px).powi(2) + (fy - py).powi(2)).sqrt();
                if d < d1 {
                    d2 = d1;
                    d1 = d;
                } else if d < d2 {
                    d2 = d;
                }
            }
        }
        let edge = ((d2 - d1) * 6.0).clamp(0.0, 1.0);
        let n = vnoise(fx * 4.0, fy * 4.0, 32, 3) * 0.25;
        let v = (0.45 + edge * 0.5 + n).clamp(0.0, 1.0);
        let c = (v * 255.0) as u8;
        [c, c, c, 255]
    });
    let grain = img(s, |x, y| {
        let fx = x as f32 / s as f32;
        let fy = y as f32 / s as f32;
        let n = vnoise(fx * 16.0, fy * 16.0, 16, 11) * 0.5 + vnoise(fx * 48.0, fy * 48.0, 48, 12) * 0.3 + vnoise(fx * 4.0, fy * 4.0, 4, 13) * 0.2;
        let c = (170.0 + n * 85.0) as u8;
        [c, c, c, 255]
    });
    let planks = img(s, |x, y| {
        let fy = y as f32 / s as f32 * 8.0;
        let row = fy.floor() as i32;
        let fx = x as f32 / s as f32 * 2.0 + (hash2(row, 0, 5) & 0xFF) as f32 / 255.0;
        let seam = if (fy.fract() < 0.05) || ((fx * 2.0).fract() < 0.01) { 0.55 } else { 1.0 };
        let wood = vnoise(fx * 20.0, fy * 1.5, 40, row as u32) * 0.25 + 0.75;
        let c = (220.0 * seam * wood) as u8;
        [c, c, c, 255]
    });
    let brick = img(s, |x, y| {
        let fy = y as f32 / s as f32 * 8.0;
        let row = fy.floor() as i32;
        let fx = x as f32 / s as f32 * 4.0 + if row % 2 == 0 { 0.5 } else { 0.0 };
        let mortar = fy.fract() < 0.1 || fx.fract() < 0.05;
        let tone = 0.8 + (hash2(fx.floor() as i32, row, 9) & 0xFF) as f32 / 255.0 * 0.2;
        let n = vnoise(x as f32 / 8.0, y as f32 / 8.0, 32, 4) * 0.15;
        let v = if mortar { 0.55 } else { tone - n };
        let c = (v * 240.0) as u8;
        [c, c, c, 255]
    });
    Tex {
        cobble: images.add(cobble),
        grain: images.add(grain),
        planks: images.add(planks),
        brick: images.add(brick),
    }
}
