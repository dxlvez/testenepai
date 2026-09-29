//! Real 3D models (glTF) embedded in the executable: the horse, sofas,
//! armchairs, street lanterns, lamps, plants... plus the environment map
//! that gives paint, chrome, glass and puddles something to reflect.

use bevy::prelude::*;

pub const PREFIX: &str = "embedded://red_thread/models/";

#[derive(Resource, Clone)]
pub struct Models {
    pub env_diffuse: Handle<Image>,
    pub env_specular: Handle<Image>,
}

pub fn load_models(mut c: Commands, a: Res<AssetServer>) {
    c.insert_resource(Models {
        env_diffuse: a.load(format!("{}pisa_diffuse_rgb9e5_zstd.ktx2", PREFIX)),
        env_specular: a.load(format!("{}pisa_specular_rgb9e5_zstd.ktx2", PREFIX)),
    });
}

/// Adds the environment map (reflections) to the main camera once loaded.
pub fn attach_env_map(mut c: Commands, models: Option<Res<Models>>, cams: Query<Entity, (With<crate::camera::MainCam>, Without<EnvironmentMapLight>)>) {
    let Some(models) = models else { return };
    for e in cams.iter() {
        c.entity(e).insert(EnvironmentMapLight {
            diffuse_map: models.env_diffuse.clone(),
            specular_map: models.env_specular.clone(),
            intensity: 600.0,
            ..default()
        });
    }
}
