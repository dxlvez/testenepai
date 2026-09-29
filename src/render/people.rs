//! Realistic people: skinned MakeHuman characters (CC0) sharing one skeleton,
//! driven by motion-capture clips (CMU) from `anims.glb`. Each citizen's
//! procedural `Rig` keeps deciding the pose; this module only swaps the look
//! for a real body and plays the matching clip with smooth cross-fades.

use super::character::{Garment, Hat, Look, Pose, Rig};
use bevy::animation::{AnimationTarget, AnimationTargetId};
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;

const ANIMS: &str = "embedded://red_thread/models/people/anims.glb";

#[derive(Deserialize)]
struct Manifest {
    clips: HashMap<String, ClipE>,
    characters: Vec<CharE>,
}

#[derive(Deserialize, Clone)]
struct ClipE {
    index: usize,
    #[serde(default)]
    speed: f32,
    #[serde(default, rename = "loop")]
    looping: bool,
}

#[derive(Deserialize, Clone)]
pub struct CharE {
    pub file: String,
    pub sex: String,
    pub age: String,
    pub min_year: i32,
    pub max_year: i32,
    pub role: String,
    pub skin: String,
}

#[derive(Clone, Copy)]
struct Clip {
    node: AnimationNodeIndex,
    speed: f32,
    looping: bool,
}

#[derive(Resource)]
pub struct People {
    chars: Vec<CharE>,
    scenes: Vec<Handle<Scene>>,
    graph: Handle<AnimationGraph>,
    clips: HashMap<&'static str, Clip>,
}

/// A body swapped in for the procedural one; bound to its animation player once the scene exists.
#[derive(Component)]
pub struct RealPerson {
    scene: Entity,
    player: Option<Entity>,
    cur: Option<Pose>,
    rate: f32,
}

fn clip_name(p: Pose) -> &'static str {
    match p {
        Pose::Idle => "idle",
        Pose::Walk => "walk",
        Pose::Run => "run",
        Pose::Sneak => "sneak",
        Pose::Sit => "sit",
        Pose::Lie => "lie",
        Pose::Dead => "dead",
        Pose::Aim => "aim",
        Pose::Punch => "punch",
        Pose::Carry => "carry",
        Pose::Drag => "drag",
        Pose::Talk => "talk",
        Pose::Pray => "pray",
        Pose::Work => "work",
        Pose::Dance => "dance",
        Pose::Cower => "cower",
        Pose::Play => "play",
    }
}

pub fn load_people(mut c: Commands, a: Res<AssetServer>, mut graphs: ResMut<Assets<AnimationGraph>>) {
    let Some((_, bytes)) = super::lib3d::MODEL_FILES.iter().find(|(p, _)| *p == "people/manifest.json") else { return };
    let Ok(m) = serde_json::from_slice::<Manifest>(bytes) else {
        warn!("manifest de pessoas inválido");
        return;
    };
    let chars: Vec<CharE> = m.characters.into_iter().filter(|ch| super::lib3d::MODEL_FILES.iter().any(|(p, _)| *p == format!("people/{}", ch.file))).collect();
    if chars.is_empty() {
        return;
    }
    let scenes = chars.iter().map(|ch| a.load(GltfAssetLabel::Scene(0).from_asset(format!("embedded://red_thread/models/people/{}", ch.file)))).collect();
    let names = ["idle", "walk", "run", "sneak", "sit", "lie", "dead", "aim", "punch", "carry", "drag", "talk", "pray", "work", "dance", "cower", "play", "drink", "hands_up"];
    let mut graph = AnimationGraph::new();
    let mut clips = HashMap::new();
    for n in names {
        let Some(ce) = m.clips.get(n) else { continue };
        let h: Handle<AnimationClip> = a.load(GltfAssetLabel::Animation(ce.index).from_asset(ANIMS));
        let node = graph.add_clip(h, 1.0, graph.root);
        clips.insert(n, Clip { node, speed: ce.speed, looping: ce.looping });
    }
    info!("pessoas realistas: {} modelos, {} animações", chars.len(), clips.len());
    c.insert_resource(People { chars, scenes, graph: graphs.add(graph), clips });
}

fn age_class(age: f32) -> &'static str {
    if age < 14.0 {
        "child"
    } else if age < 30.0 {
        "young"
    } else if age >= 60.0 {
        "old"
    } else {
        "adult"
    }
}

fn role_of(l: &Look) -> &'static str {
    let police_hat = matches!(l.hat, Hat::PoliceCap | Hat::Bobby | Hat::Shako | Hat::Helmet);
    match l.garment {
        Garment::Uniform if police_hat => "police",
        Garment::Overalls => "worker",
        Garment::Apron if l.female => "worker",
        Garment::Apron | Garment::Hoodie => "poor",
        Garment::Suit | Garment::LongCoat if matches!(l.hat, Hat::Bowler | Hat::Fedora) && l.age > 38.0 => "rich",
        _ => "civil",
    }
}

fn skin_class(l: &Look) -> &'static str {
    let lum = l.skin[0] * 0.3 + l.skin[1] * 0.59 + l.skin[2] * 0.11;
    if lum < 0.35 {
        "dark"
    } else if lum < 0.55 {
        "medium"
    } else {
        "light"
    }
}

fn look_seed(l: &Look) -> u32 {
    let mut h: u32 = 0x811c9dc5;
    for v in l.skin.iter().chain(l.top.iter()).chain(l.bottom.iter()).chain([l.height, l.age].iter()) {
        h ^= v.to_bits();
        h = h.wrapping_mul(16777619);
    }
    h
}

impl People {
    /// The body that best fits a citizen's sex, age, trade, era and complexion.
    fn pick(&self, l: &Look, year: i32) -> usize {
        let year = year.min(1999);
        let (ac, role, skin) = (age_class(l.age), role_of(l), skin_class(l));
        let sex = if l.female { "f" } else { "m" };
        let mut best: Vec<(i32, usize)> = Vec::new();
        for (i, ch) in self.chars.iter().enumerate() {
            if ch.role == "elias" || ch.sex != sex {
                continue;
            }
            let mut s = 0;
            if year >= ch.min_year && year <= ch.max_year {
                s += 6;
            }
            if ch.age == ac {
                s += 4;
            } else if (ch.age == "child") != (ac == "child") {
                s -= 10;
            }
            if ch.role == role {
                s += 3;
            } else if ch.role == "police" || role == "police" {
                s -= 6;
            }
            if ch.skin == skin {
                s += 1;
            }
            best.push((s, i));
        }
        let top = best.iter().map(|b| b.0).max().unwrap_or(0);
        let pool: Vec<usize> = best.iter().filter(|b| b.0 >= top - 1).map(|b| b.1).collect();
        if pool.is_empty() {
            return 0;
        }
        pool[(look_seed(l) as usize) % pool.len()]
    }

    fn elias(&self) -> usize {
        self.chars.iter().position(|c| c.role == "elias").unwrap_or(0)
    }
}

/// Swap a freshly spawned procedural citizen for a realistic body.
pub fn dress(c: &mut Commands, people: Option<&People>, root: Entity, rig: &Rig, look: &Look, year: i32, is_elias: bool) {
    let Some(p) = people else { return };
    let idx = if is_elias { p.elias() } else { p.pick(look, year) };
    for e in [rig.body, rig.leg_l, rig.leg_r] {
        c.entity(e).insert(Visibility::Hidden);
    }
    let scene = c.spawn((SceneRoot(p.scenes[idx].clone()), Transform::from_xyz(0.0, 0.02, 0.0), Visibility::Inherited)).id();
    c.entity(root).add_child(scene);
    c.entity(root).insert(RealPerson { scene, player: None, cur: None, rate: 1.0 });
}

fn find_named(e: Entity, name: &str, q: &Query<(&Name, Option<&Children>)>, kids: &Query<&Children>) -> Option<Entity> {
    if let Ok((n, _)) = q.get(e) {
        if n.as_str() == name {
            return Some(e);
        }
    }
    let ch = kids.get(e).ok()?;
    ch.iter().find_map(|k| find_named(k, name, q, kids))
}

fn tag_targets(c: &mut Commands, e: Entity, path: &mut Vec<Name>, player: Entity, q: &Query<(&Name, Option<&Children>)>) {
    let Ok((n, ch)) = q.get(e) else { return };
    path.push(n.clone());
    c.entity(e).insert(AnimationTarget { id: AnimationTargetId::from_names(path.iter()), player });
    if let Some(ch) = ch {
        for k in ch.iter() {
            tag_targets(c, k, path, player, q);
        }
    }
    path.pop();
}

/// Once a body's scene has spawned, give its skeleton an animation player.
pub fn bind_people(mut c: Commands, people: Option<Res<People>>, mut q: Query<&mut RealPerson>, names: Query<(&Name, Option<&Children>)>, kids: Query<&Children>) {
    let Some(p) = people else { return };
    for mut rp in q.iter_mut() {
        if rp.player.is_some() {
            continue;
        }
        let Some(rig) = find_named(rp.scene, "rig", &names, &kids) else { continue };
        let mut path = Vec::new();
        tag_targets(&mut c, rig, &mut path, rig, &names);
        c.entity(rig).insert((AnimationPlayer::default(), AnimationGraphHandle(p.graph.clone()), AnimationTransitions::new()));
        rp.player = Some(rig);
        rp.cur = None;
    }
}

/// Play the clip for each person's current pose, cross-fading and matching the gait to their speed.
pub fn animate_people(people: Option<Res<People>>, mut q: Query<(&Rig, &mut RealPerson)>, mut players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>) {
    let Some(p) = people else { return };
    for (rig, mut rp) in q.iter_mut() {
        let Some(pe) = rp.player else { continue };
        let Ok((mut player, mut tr)) = players.get_mut(pe) else { continue };
        let Some(clip) = p.clips.get(clip_name(rig.pose)).copied() else { continue };
        if rp.cur != Some(rig.pose) {
            let fade = if matches!(rig.pose, Pose::Dead | Pose::Punch) { 120 } else { 280 };
            let a = tr.play(&mut player, clip.node, Duration::from_millis(fade));
            if clip.looping {
                a.repeat();
            }
            // a crowd never walks in lock-step
            let off = (rig.quirk - 0.85) * 3.0;
            a.seek_to(off);
            rp.cur = Some(rig.pose);
        }
        let rate = if clip.speed > 0.0 && rig.speed > 0.05 { (rig.speed / clip.speed).clamp(0.6, 1.8) } else { rig.quirk.clamp(0.9, 1.1) };
        if (rate - rp.rate).abs() > 0.02 {
            rp.rate = rate;
            if let Some(a) = player.animation_mut(clip.node) {
                a.set_speed(rate);
            }
        }
    }
}
