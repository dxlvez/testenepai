//! Library of real 3D models (Poly Haven CC0 furniture, props, street
//! furniture, trees...) embedded in the executable. Picks a model for a
//! category by era, style and seed, and places it on a prop footprint.

use bevy::asset::io::embedded::EmbeddedAssetRegistry;
use bevy::prelude::*;
use serde::Deserialize;
use std::path::{Path, PathBuf};

include!(concat!(env!("OUT_DIR"), "/embedded_models.rs"));

#[derive(Deserialize, Clone, Debug)]
pub struct Entry {
    pub id: String,
    pub file: String,
    pub category: String,
    pub size: [f32; 3],
    #[serde(default)]
    pub wall_mount: bool,
    #[serde(default)]
    pub min_year: i32,
    #[serde(default = "far_future")]
    pub max_year: i32,
    #[serde(default)]
    pub style: String,
}

fn far_future() -> i32 {
    3000
}

#[derive(Resource, Default)]
pub struct Lib {
    pub entries: Vec<(Entry, Handle<Scene>)>,
}

impl Lib {
    /// A model of `category` plausible in `year`, chosen by `seed` (optionally preferring a style).
    pub fn pick(&self, category: &str, year: i32, seed: u32, style: Option<&str>) -> Option<&(Entry, Handle<Scene>)> {
        let ok: Vec<&(Entry, Handle<Scene>)> = self.entries.iter().filter(|(e, _)| e.category == category && year >= e.min_year && year <= e.max_year).collect();
        if ok.is_empty() {
            return None;
        }
        if let Some(st) = style {
            let styled: Vec<&&(Entry, Handle<Scene>)> = ok.iter().filter(|(e, _)| e.style.contains(st)).collect();
            if !styled.is_empty() {
                return Some(styled[(seed as usize) % styled.len()]);
            }
        }
        Some(ok[(seed as usize) % ok.len()])
    }
}

/// Registers the embedded files as `embedded://red_thread/models/...` assets.
pub fn register_embedded(app: &mut App) {
    let reg = app.world_mut().resource_mut::<EmbeddedAssetRegistry>();
    for (path, bytes) in MODEL_FILES {
        reg.insert_asset(PathBuf::from(format!("src/models/{}", path)), Path::new(&format!("red_thread/models/{}", path)), *bytes);
    }
}

pub fn load_lib(mut c: Commands, a: Res<AssetServer>) {
    let mut lib = Lib::default();
    if let Some((_, bytes)) = MODEL_FILES.iter().find(|(p, _)| *p == "props/manifest.json") {
        if let Ok(list) = serde_json::from_slice::<Vec<Entry>>(bytes) {
            for e in list {
                if !MODEL_FILES.iter().any(|(p, _)| *p == format!("props/{}", e.file)) {
                    continue;
                }
                let h = a.load(GltfAssetLabel::Scene(0).from_asset(format!("embedded://red_thread/models/props/{}", e.file)));
                lib.entries.push((e, h));
            }
        }
    }
    info!("modelos 3D reais: {}", lib.entries.len());
    c.insert_resource(lib);
}

/// Transform that sets a model (origin bottom-centre, back towards +z) on a
/// footprint of `fw` x `fd` tiles whose back faces `rot` (0 = +z, 1 = +x, 2 = -z, 3 = -x).
pub fn fit(e: &Entry, centre: Vec3, fw: f32, fd: f32, rot: u8, max_up: f32) -> Transform {
    let odd = rot % 2 == 1;
    let (w, d) = if odd { (fd, fw) } else { (fw, fd) };
    let sx = w / e.size[0].max(0.01);
    let sz = d / e.size[2].max(0.01);
    let s = sx.min(sz).min(max_up).max(0.2);
    let ang = [0.0, std::f32::consts::FRAC_PI_2, std::f32::consts::PI, -std::f32::consts::FRAC_PI_2][(rot % 4) as usize];
    Transform::from_translation(centre).with_rotation(Quat::from_rotation_y(ang)).with_scale(Vec3::splat(s))
}
