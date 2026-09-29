//! Real 3D models (glTF) embedded in the executable: the horse, sofas,
//! armchairs, street lanterns, lamps, plants... plus the environment map
//! that gives paint, chrome, glass and puddles something to reflect.

use bevy::prelude::*;

pub const PREFIX: &str = "embedded://red_thread/models/";

#[derive(Resource, Clone)]
pub struct Models {
    pub horse: Handle<Scene>,
    pub horse_graph: Handle<AnimationGraph>,
    pub horse_node: AnimationNodeIndex,
    pub sofa_velvet: Handle<Scene>,
    pub sofa_leather: Handle<Scene>,
    pub chair_sheen: Handle<Scene>,
    pub chair_damask: Handle<Scene>,
    pub lantern: Handle<Scene>,
    pub barn_lamp: Handle<Scene>,
    pub plant: Handle<Scene>,
    pub candle: Handle<Scene>,
    pub env_diffuse: Handle<Image>,
    pub env_specular: Handle<Image>,
}

fn scene(a: &AssetServer, f: &str) -> Handle<Scene> {
    a.load(GltfAssetLabel::Scene(0).from_asset(format!("{}{}", PREFIX, f)))
}

pub fn load_models(mut c: Commands, a: Res<AssetServer>, mut graphs: ResMut<Assets<AnimationGraph>>) {
    let clip: Handle<AnimationClip> = a.load(GltfAssetLabel::Animation(0).from_asset(format!("{}Horse.glb", PREFIX)));
    let (graph, node) = AnimationGraph::from_clip(clip);
    c.insert_resource(Models {
        horse: scene(&a, "Horse.glb"),
        horse_graph: graphs.add(graph),
        horse_node: node,
        sofa_velvet: scene(&a, "GlamVelvetSofa.glb"),
        sofa_leather: scene(&a, "SheenWoodLeatherSofa.glb"),
        chair_sheen: scene(&a, "SheenChair.glb"),
        chair_damask: scene(&a, "ChairDamaskPurplegold.glb"),
        lantern: scene(&a, "Lantern.glb"),
        barn_lamp: scene(&a, "AnisotropyBarnLamp.glb"),
        plant: scene(&a, "DiffuseTransmissionPlant.glb"),
        candle: scene(&a, "GlassHurricaneCandleHolder.glb"),
        env_diffuse: a.load(format!("{}pisa_diffuse_rgb9e5_zstd.ktx2", PREFIX)),
        env_specular: a.load(format!("{}pisa_specular_rgb9e5_zstd.ktx2", PREFIX)),
    });
}

/// Marks a horse scene instance; `speed` drives the gallop animation.
#[derive(Component)]
pub struct HorseModel {
    pub car: u32,
}

/// Start the horse animation once its scene has spawned its AnimationPlayer.
pub fn start_horse_anims(
    mut c: Commands,
    models: Option<Res<Models>>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    parents: Query<&ChildOf>,
    horses: Query<&HorseModel>,
) {
    let Some(models) = models else { return };
    for (e, mut p) in players.iter_mut() {
        // only for players inside a horse scene
        let mut cur = e;
        let mut is_horse = false;
        for _ in 0..8 {
            if horses.get(cur).is_ok() {
                is_horse = true;
                break;
            }
            match parents.get(cur) {
                Ok(pa) => cur = pa.parent(),
                Err(_) => break,
            }
        }
        if !is_horse {
            continue;
        }
        p.play(models.horse_node).repeat();
        c.entity(e).insert(AnimationGraphHandle(models.horse_graph.clone()));
    }
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
