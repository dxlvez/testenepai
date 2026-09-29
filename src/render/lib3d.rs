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
    /// animated models (horses): clip name -> animation index in the file
    #[serde(default)]
    pub clips: std::collections::HashMap<String, usize>,
    #[serde(default)]
    pub walk_speed: f32,
    #[serde(default)]
    pub trot_speed: f32,
    /// horse: where its breast is; vehicle: where the horse's breast goes
    #[serde(default)]
    pub chest: Option<[f32; 3]>,
    #[serde(default)]
    pub hitch: Option<[f32; 3]>,
}

fn far_future() -> i32 {
    3000
}

#[derive(Resource)]
pub struct Lib {
    pub entries: Vec<Entry>,
    server: AssetServer,
    cache: std::sync::Mutex<std::collections::HashMap<String, Handle<Scene>>>,
}

impl Lib {
    /// A model of `category` plausible in `year`, chosen by `seed` (optionally preferring a style).
    /// The model is loaded the first time it is used.
    pub fn pick(&self, category: &str, year: i32, seed: u32, style: Option<&str>) -> Option<(Entry, Handle<Scene>)> {
        let want_chinese = style == Some("chinese");
        let ok: Vec<&Entry> = self
            .entries
            .iter()
            .filter(|e| e.category == category && year >= e.min_year && year <= e.max_year)
            // regional / luxury pieces only where they belong
            .filter(|e| want_chinese == e.style.contains("chinese") || category == "tree" || category == "grass")
            .filter(|e| !(e.style.contains("gothic") && !matches!(style, Some("gothic") | Some("rich"))))
            // painted farmhouse / school furniture only belongs in homes and offices
            .filter(|e| !((e.id.contains("painted") || e.id.contains("School")) && matches!(style, Some("bar") | Some("rich") | Some("gothic"))))
            .collect();
        if ok.is_empty() {
            return None;
        }
        let pref = match style {
            Some(st) => Some(st),
            None => Some("home"),
        };
        let chosen = match pref {
            Some(st) => {
                let styled: Vec<&&Entry> = ok.iter().filter(|e| e.style.contains(st)).collect();
                if styled.is_empty() || (seed >> 5) % 5 == 0 {
                    ok[(seed as usize) % ok.len()]
                } else {
                    styled[(seed as usize) % styled.len()]
                }
            }
            None => ok[(seed as usize) % ok.len()],
        };
        let mut cache = self.cache.lock().ok()?;
        let h = cache
            .entry(chosen.file.clone())
            .or_insert_with(|| self.server.load(GltfAssetLabel::Scene(0).from_asset(format!("embedded://red_thread/models/props/{}", chosen.file))))
            .clone();
        Some((chosen.clone(), h))
    }

    /// Asset path of an embedded model file, for loading its animation clips.
    pub fn path(file: &str) -> String {
        format!("embedded://red_thread/models/props/{}", file)
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
    let mut entries = Vec::new();
    if let Some((_, bytes)) = MODEL_FILES.iter().find(|(p, _)| *p == "props/manifest.json") {
        if let Ok(list) = serde_json::from_slice::<Vec<Entry>>(bytes) {
            for e in list {
                if MODEL_FILES.iter().any(|(p, _)| *p == format!("props/{}", e.file)) {
                    entries.push(e);
                }
            }
        }
    }
    info!("modelos 3D reais disponíveis: {}", entries.len());
    c.insert_resource(Lib { entries, server: a.clone(), cache: Default::default() });
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
