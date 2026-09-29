//! Cheap screen-space mood: a vignette that darkens the corners and an
//! animated film grain, both drawn as full-screen UI images below the HUD.

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

#[derive(Component)]
pub struct Vignette;

#[derive(Component)]
pub struct Grain;

#[derive(Resource)]
pub struct GrainFrames(pub Vec<Handle<Image>>);

fn rgba(w: u32, h: u32, f: impl Fn(u32, u32) -> [u8; 4]) -> Image {
    let mut data = Vec::with_capacity((w * h * 4) as usize);
    for y in 0..h {
        for x in 0..w {
            data.extend_from_slice(&f(x, y));
        }
    }
    Image::new(Extent3d { width: w, height: h, depth_or_array_layers: 1 }, TextureDimension::D2, data, TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::RENDER_WORLD)
}

pub fn setup_post(mut c: Commands, mut images: ResMut<Assets<Image>>) {
    let vig = rgba(256, 144, |x, y| {
        let u = x as f32 / 255.0 * 2.0 - 1.0;
        let v = y as f32 / 143.0 * 2.0 - 1.0;
        let d = (u * u * 0.8 + v * v).sqrt();
        let a = ((d - 0.55) / 0.75).clamp(0.0, 1.0).powf(1.6) * 0.85;
        [4, 2, 6, (a * 255.0) as u8]
    });
    let mut frames = Vec::new();
    for k in 0..4u32 {
        frames.push(images.add(rgba(320, 180, |x, y| {
            let h = crate::util::hash2(x as i32, y as i32, 900 + k);
            let n = (h & 0xFF) as u8;
            [n, n, n, 22]
        })));
    }
    c.spawn((
        ImageNode::new(images.add(vig)),
        Node { position_type: PositionType::Absolute, width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() },
        GlobalZIndex(-10),
        Pickable::IGNORE,
        Vignette,
    ));
    c.spawn((
        ImageNode::new(frames[0].clone()),
        Node { position_type: PositionType::Absolute, width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() },
        GlobalZIndex(-9),
        Pickable::IGNORE,
        Grain,
    ));
    c.insert_resource(GrainFrames(frames));
}

pub fn animate_post(
    time: Res<Time>,
    settings: Res<crate::keys::Settings>,
    frames: Option<Res<GrainFrames>>,
    mut grain: Query<(&mut ImageNode, &mut Visibility), (With<Grain>, Without<Vignette>)>,
    mut vig: Query<&mut Visibility, (With<Vignette>, Without<Grain>)>,
    mut t: Local<f32>,
    mut i: Local<usize>,
) {
    let Some(frames) = frames else { return };
    for mut v in vig.iter_mut() {
        *v = if settings.grain { Visibility::Inherited } else { Visibility::Hidden };
    }
    *t += time.delta_secs();
    for (mut img, mut v) in grain.iter_mut() {
        *v = if settings.grain { Visibility::Inherited } else { Visibility::Hidden };
        if *t > 0.06 {
            *i = (*i + 1) % frames.0.len();
            img.image = frames.0[*i].clone();
            img.flip_x = *i % 2 == 0;
            img.flip_y = *i % 3 == 0;
        }
    }
    if *t > 0.06 {
        *t = 0.0;
    }
}
