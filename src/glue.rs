//! Systems that tie modules together: dialogue side effects, evidence
//! markers, echoes (ghosts of the past), red sight threads, the sphere,
//! reactions to a pointed gun, venue music.

use crate::audio::{MusicState, Sfx, VenueGen};
use crate::camera::Cursor;
use crate::cases::defs::Ghost;
use crate::cases::run::{clue_mesh, clue_visible, current, echo_positions, CaseDb, CaseRt, ClueMarker, EchoRun};
use crate::city::map::*;
use crate::crime::CrimeEv;
use crate::keys::{Act, Action};
use crate::narrative::Flow;
use crate::player::PlayerRt;
use crate::render::character::{spawn_character, Garment, Hair, Hat, Look, Pose, Rig};
use crate::render::city3d::Mats;
use crate::sim::agents::{AState, Sim};
use crate::sim::speech::{plead, Situation};
use crate::state::*;
use crate::ui::board::BoardSel;
use crate::ui::dialogue::Dlg;
use crate::ui::hud::{Prompt, Toasts};
use crate::ui::screens::{shop_stock, Choices, Popup, PopStyle, Popups, Shop};
use crate::ui::{Mode, UiState};
use crate::world::CityMap;
use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;

// ------------------------------------------------------------------ dialogue side effects

#[allow(clippy::too_many_arguments)]
pub fn dialogue_effects(
    mut dlg: ResMut<Dlg>,
    mut ui: ResMut<UiState>,
    mut game: ResMut<Game>,
    mut sim: ResMut<Sim>,
    db: Res<CaseDb>,
    mut shop: ResMut<Shop>,
    mut sel: ResMut<BoardSel>,
    mut popups: ResMut<Popups>,
    mut choices: ResMut<Choices>,
    mut sfx: EventWriter<Sfx>,
    mut toasts: ResMut<Toasts>,
    mut crimes: EventWriter<CrimeEv>,
    mut cars: ResMut<crate::vehicles::Cars>,
    mut c: Commands,
    map: Option<Res<CityMap>>,
    mut rt: ResMut<PlayerRt>,
) {
    let pp = game.player.pos;
    let weapon = if game.player.weapon == crate::items::Weapon::Fists { game.player.weapons().get(1).copied() } else { Some(game.player.weapon) };
    for (kind, pid) in dlg.crimes.drain(..) {
        crimes.write(CrimeEv { kind, pos: pp, victim: Some(pid), noise: 4.0, weapon });
        rt.weapon_out = weapon.is_some();
        if let Some(w) = weapon {
            game.player.weapon = w;
        }
    }
    let now = game.abs_minute();
    for pid in dlg.hands_up.drain(..) {
        if let Some(a) = sim.agent_mut(pid) {
            a.state = AState::HandsUp { until: now + 25.0 };
        }
        // everyone else in the shop drops to the floor
        if let Some(map) = &map {
            if let Some(b) = map.0.building_at(pp) {
                for a in sim.agents.iter_mut() {
                    if a.pid != pid && a.active() && map.0.building_at(a.pos) == Some(b) {
                        a.state = AState::Cower { until: now + 20.0 };
                        a.chat = None;
                    }
                }
            }
        }
    }
    if dlg.open_shop && ui.mode == Mode::None {
        dlg.open_shop = false;
        let black = crate::ui::dialogue::BLACK.with(|b| std::mem::take(&mut *b.borrow_mut()));
        let keeper = dlg.agent.map(|i| sim.agents.get(i).map(|a| a.pid)).flatten();
        let job = keeper.map(|k| game.pop.get(k).job).unwrap_or(crate::sim::people::Job::Merchant);
        let kind = map.as_ref().and_then(|m| m.0.building_at(pp).map(|b| m.0.buildings[b].kind));
        shop.keeper = keeper;
        shop.kind = kind;
        shop.black = black;
        shop.stock = shop_stock(kind, job, game.year, black);
        ui.open(Mode::Shop);
    }
    if crate::ui::dialogue::SELLCAR.with(|b| std::mem::take(&mut *b.borrow_mut())) {
        match crate::vehicles::sell_nearest_car(&mut cars, &mut game, &mut c) {
            Some(v) => {
                toasts.push(format!("Carro vendido por {}.", crate::economy::money_str(&game, v)));
                sfx.write(Sfx::Cash);
            }
            None => toasts.push("\"Que carro? Traga ele aqui perto primeiro.\""),
        }
    }
    if let Some((_, direct)) = dlg.resolve_case.take() {
        if direct {
            if ui.mode == Mode::Dialogue {
                ui.close();
            }
            crate::narrative::resolve_case(&mut game, &db, &mut popups, &mut choices, &mut ui, &mut sfx, &mut toasts);
        } else {
            sel.accusing = true;
            ui.open(Mode::Board);
            ui.tab = 1;
        }
    }
    if let Some(pid) = dlg.start_date.take() {
        let n = game.pop.get(pid).first.clone();
        toasts.push(format!("{} acompanha você por algumas horas.", n));
    }
}

// ------------------------------------------------------------------ evidence markers

#[derive(Component)]
pub struct Glint;

#[allow(clippy::too_many_arguments)]
pub fn case_markers(
    mut c: Commands,
    game: Res<Game>,
    db: Res<CaseDb>,
    mut crt: ResMut<CaseRt>,
    map: Option<Res<CityMap>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mats: Option<Res<Mats>>,
    mut q: Query<(Entity, &ClueMarker, &mut Visibility, &mut Transform)>,
    time: Res<Time>,
) {
    let Some(map) = map else { return };
    let Some(mats) = mats else { return };
    let m = &map.0;
    let Some((def, pi)) = current(&game, &db) else {
        for (e, ..) in q.iter() {
            c.entity(e).despawn();
        }
        crt.spawned = None;
        crt.echo_pos.clear();
        return;
    };
    let key = (def.id, game.timeline, game.year);
    let prog = &game.cases[pi];
    if crt.spawned != Some(key) || q.is_empty() && !prog.clue_pos.is_empty() && crt.respawn {
        for (e, ..) in q.iter() {
            c.entity(e).despawn();
        }
        crt.spawned = Some(key);
        crt.respawn = false;
        for (i, cl) in def.clues.iter().enumerate() {
            let Some(Some((x, y))) = prog.clue_pos.get(i) else { continue };
            if cl.kind == crate::cases::defs::ClueKind::Temporal || cl.look == crate::cases::defs::Look3d::None {
                continue;
            }
            let p = Vec2::new(*x, *y);
            let gy = crate::world::ground_y(m, p);
            let mb = clue_mesh(cl.look);
            let mat = if cl.look == crate::cases::defs::Look3d::Glow { mats.red.clone() } else { mats.furn.clone() };
            c.spawn((Mesh3d(meshes.add(mb.build())), MeshMaterial3d(mat), Transform::from_xyz(p.x, gy, p.y), ClueMarker { idx: i as u8 }, Visibility::Hidden, crate::world::AgentRoot));
        }
        crt.echo_pos = echo_positions(def, &game, prog, m);
        return;
    }
    let t = time.elapsed_secs();
    for (_, cm, mut v, mut tr) in q.iter_mut() {
        let cl = &def.clues[cm.idx as usize];
        let found = prog.has(cm.idx);
        let visible = clue_visible(cl, &game, crt.red_sight) && !(found && matches!(cl.look, crate::cases::defs::Look3d::Paper | crate::cases::defs::Look3d::Weapon));
        *v = if visible { Visibility::Inherited } else { Visibility::Hidden };
        // unexamined evidence "breathes" a little so the eye catches it
        let s = if !found { 1.0 + (t * 3.0 + cm.idx as f32).sin() * 0.06 } else { 1.0 };
        tr.scale = Vec3::splat(s);
    }
}

// ------------------------------------------------------------------ echoes

#[derive(Component)]
pub struct GhostVis;

fn ghost_look() -> Look {
    Look {
        female: false,
        skin: [0.7, 0.8, 1.0],
        hair_col: [0.6, 0.7, 0.9],
        hair: Hair::Short,
        hat: Hat::Fedora,
        hat_col: [0.6, 0.7, 0.9],
        garment: Garment::LongCoat,
        top: [0.6, 0.7, 0.95],
        bottom: [0.5, 0.6, 0.85],
        accent: [0.8, 0.9, 1.0],
        height: 1.0,
        girth: 1.0,
        beard: false,
        age: 40.0,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn echo_system(
    time: Res<Time>,
    act: Res<Act>,
    mut game: ResMut<Game>,
    db: Res<CaseDb>,
    mut crt: ResMut<CaseRt>,
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mats: Option<Res<Mats>>,
    mut ghosts: Query<(&mut Transform, &mut Rig), With<GhostVis>>,
    mut sfx: EventWriter<Sfx>,
    mut toasts: ResMut<Toasts>,
    mut music: ResMut<MusicState>,
    ui: Res<UiState>,
) {
    let dt = time.delta_secs();
    if let Some((_, tt)) = &mut crt.caption {
        *tt -= dt;
        if *tt <= 0.0 {
            crt.caption = None;
        }
    }
    let Some(mats) = mats else { return };
    // start
    if crt.echo.is_none() && act.just(Action::Echo) && !ui.blocks_input() {
        let pp = game.player.pos;
        let near = crt.echo_pos.iter().enumerate().filter(|(_, p)| p.distance(pp) < 3.0).min_by(|a, b| a.1.distance(pp).partial_cmp(&b.1.distance(pp)).unwrap()).map(|(i, p)| (i, *p));
        match near {
            Some((i, p)) if current(&game, &db).is_some() => {
                let lvl = game.player.skill(Skill::Echo) as f32;
                let dur = 3.5 + lvl * 1.5 + if game.player.visionary > 0.0 { 5.0 } else { 0.0 };
                let (root, rig) = spawn_character(&mut c, &mut meshes, &mats.ghost, &ghost_look());
                c.entity(root).insert((GhostVis, rig, Transform::from_xyz(p.x, 0.1, p.y), crate::world::AgentRoot));
                crt.echo = Some(EchoRun { idx: i as u8, t: 0.0, dur, origin: p, ghost: Some(root), frame: 0 });
                sfx.write(Sfx::Echo);
                music.duck = 0.25;
                game.fatigue += 2.0;
                game.player.train(Skill::Echo, 8);
            }
            _ => {
                game.fatigue += 0.5;
                crt.caption = Some(("...o passado não fala aqui.".into(), 2.0));
                sfx.write(Sfx::Whisper);
            }
        }
    }
    let Some(run) = &mut crt.echo else { return };
    run.t += dt;
    let Some((def, pi)) = current(&game, &db) else {
        crt.echo = None;
        return;
    };
    let def = def.clone();
    let e = &def.echoes[run.idx as usize];
    let k = (run.t / run.dur).min(1.0);
    // ghost choreography
    if let Some(g) = run.ghost {
        if let Ok((mut tr, mut rig)) = ghosts.get_mut(g) {
            let o = run.origin;
            let (pos, pose, face) = match e.ghost {
                Ghost::Struggle => {
                    let p = o + Vec2::new((k * 6.0).sin() * 0.4, -1.5 + k * 1.5);
                    (p, if (k * 8.0) as i32 % 2 == 0 { Pose::Punch } else { Pose::Walk }, 1.57)
                }
                Ghost::Flee => (o + Vec2::new(k * 6.0, k * 2.0), Pose::Run, 0.3),
                Ghost::Drag => (o + Vec2::new(-k * 3.0, 0.0), Pose::Drag, 3.14),
                Ghost::Write => (o, if k < 0.8 { Pose::Work } else { Pose::Idle }, 1.57),
                Ghost::Hide => (o + Vec2::new(0.0, k * 1.2), Pose::Sneak, 1.57),
                Ghost::Argue => (o, Pose::Talk, (k * 3.0).sin()),
                Ghost::Vanish => (o, Pose::Idle, 0.0),
                Ghost::Walk => (o + Vec2::new(k * 4.0 - 2.0, 0.0), Pose::Walk, 0.0),
                Ghost::Wait => (o, Pose::Idle, (k * 2.0).sin() * 0.5),
            };
            tr.translation = Vec3::new(pos.x, 0.1, pos.y);
            tr.rotation = Quat::from_rotation_y(-face - std::f32::consts::FRAC_PI_2);
            rig.pose = pose;
            rig.speed = if matches!(pose, Pose::Run) { 4.0 } else { 1.5 };
            // flicker
            let vis = if e.ghost == Ghost::Vanish { 1.0 - k } else { 1.0 };
            tr.scale = Vec3::splat(if (run.t * 17.0).sin() > 0.93 { 0.0 } else { vis.max(0.05) });
        }
    }
    // captions, one fragment at a time (more with higher Echo skill)
    let lvl = game.player.skill(Skill::Echo) as usize;
    let n_frames = (1 + lvl / 2 + if game.player.visionary > 0.0 { 2 } else { 0 }).min(e.frames.len()).max(1);
    let fi = ((k * n_frames as f32) as usize).min(n_frames - 1);
    let no_caption = crt.caption.is_none();
    let run = crt.echo.as_mut().unwrap();
    let mut new_caption = None;
    if fi != run.frame || no_caption && run.t < 0.2 {
        run.frame = fi;
        new_caption = Some((e.frames[fi].to_string(), run.dur / n_frames as f32 + 0.3));
        sfx.write(Sfx::Whisper);
    }
    if new_caption.is_some() {
        crt.caption = new_caption;
    }
    let run = crt.echo.as_mut().unwrap();
    if run.t >= run.dur {
        if let Some(g) = run.ghost {
            c.entity(g).despawn();
        }
        let idx = run.idx;
        crt.echo = None;
        music.duck = 1.0;
        let unlock = e.unlocks;
        if !game.cases[pi].found.contains(&unlock) {
            game.cases[pi].found.push(unlock);
            toasts.push(format!("EVIDÊNCIA TEMPORAL — {}", def.clues[unlock as usize].name));
            sfx.write(Sfx::Evidence);
        }
        if !game.cases[pi].echoes.contains(&idx) {
            game.cases[pi].echoes.push(idx);
        }
    }
}

// ------------------------------------------------------------------ red sight

pub fn red_sight(
    act: Res<Act>,
    mut crt: ResMut<CaseRt>,
    game: Res<Game>,
    db: Res<CaseDb>,
    sim: Res<Sim>,
    mut gizmos: Gizmos,
    mut sfx: EventWriter<Sfx>,
    ui: Res<UiState>,
    time: Res<Time>,
    map: Option<Res<CityMap>>,
) {
    if act.just(Action::RedSight) && !ui.blocks_input() {
        crt.red_sight = !crt.red_sight;
        sfx.write(Sfx::RedSight);
    }
    if !crt.red_sight {
        return;
    }
    let t = time.elapsed_secs();
    let red = Color::srgb(4.0, 0.1, 0.2);
    let Some((def, pi)) = current(&game, &db) else {
        // outside a case: only the sphere hums
        return;
    };
    let prog = &game.cases[pi];
    let pos_of = |ci: u8| -> Option<Vec3> {
        let pid = *prog.cast.get(ci as usize)?;
        if let Some(a) = sim.agent(pid) {
            if a.state != AState::Carried {
                return Some(Vec3::new(a.pos.x, 1.4, a.pos.y));
            }
        }
        // the dead are tied to where they lived
        let h = game.pop.get(pid).home?;
        let m = &map.as_ref()?.0;
        let c = m.buildings.get(h)?.center_px();
        Some(Vec3::new(c.x, 1.4, c.y))
    };
    let pp = Vec3::new(game.player.pos.x, 1.4, game.player.pos.y);
    for (a, b) in def.threads {
        if let (Some(pa), Some(pb)) = (pos_of(*a), pos_of(*b)) {
            if pa.distance(pp) > 45.0 && pb.distance(pp) > 45.0 {
                continue;
            }
            // a sagging, trembling thread
            let n = 16;
            let mut prev = pa;
            for k in 1..=n {
                let f = k as f32 / n as f32;
                let mut p = pa.lerp(pb, f);
                p.y += -(f * (1.0 - f)) * 3.0 + (t * 3.0 + f * 9.0).sin() * 0.05;
                gizmos.line(prev, p, red);
                prev = p;
            }
        }
    }
    // cast members glow faintly: distortion rings
    for (i, _) in def.cast.iter().enumerate() {
        if let Some(p) = pos_of(i as u8) {
            if p.distance(pp) < 30.0 {
                let r = 0.5 + (t * 2.0 + i as f32).sin().abs() * 0.2;
                gizmos.circle(Isometry3d::new(p - Vec3::Y * 1.35, Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)), r, Color::srgb(2.0, 0.05, 0.1));
            }
        }
    }
    // temporal evidence pulses
    for (i, cl) in def.clues.iter().enumerate() {
        if cl.look == crate::cases::defs::Look3d::Glow || cl.kind == crate::cases::defs::ClueKind::Temporal {
            if let Some(Some((x, y))) = prog.clue_pos.get(i) {
                let p = Vec3::new(*x, 0.2, *y);
                if p.distance(pp) < 25.0 {
                    gizmos.line(p, p + Vec3::Y * (3.0 + (t * 4.0).sin()), red);
                }
            }
        }
    }
}

// ------------------------------------------------------------------ the sphere & prologue places

#[derive(Component)]
pub struct SphereVis;

pub fn spawn_sphere(mut c: Commands, map: Option<Res<CityMap>>, mut meshes: ResMut<Assets<Mesh>>, mut mats: ResMut<Assets<StandardMaterial>>, q: Query<Entity, With<SphereVis>>, mut last: Local<Option<String>>) {
    let Some(map) = map else { return };
    let key = format!("{}{}", map.0.name, map.0.props.len());
    if last.as_deref() == Some(&key) {
        return;
    }
    *last = Some(key);
    for e in q.iter() {
        c.entity(e).despawn();
    }
    for p in map.0.props.iter().filter(|p| p.kind == PKind::Sphere) {
        let pos = Vec3::new(p.x as f32 + 0.5, 1.6, p.y as f32 + 0.5);
        let core = mats.add(StandardMaterial { base_color: Color::srgb(0.12, 0.0, 0.18), emissive: LinearRgba::rgb(0.9, 0.05, 1.4), perceptual_roughness: 0.05, reflectance: 1.0, ..default() });
        let veins = mats.add(StandardMaterial { base_color: Color::srgb(0.5, 0.0, 0.05), emissive: LinearRgba::rgb(14.0, 0.2, 0.6), unlit: true, ..default() });
        c.spawn((Mesh3d(meshes.add(Sphere::new(0.62).mesh().uv(48, 24))), MeshMaterial3d(core), Transform::from_translation(pos), SphereVis, crate::world::AgentRoot))
            .with_children(|ch| {
                for k in 0..14 {
                    let a = k as f32 * 2.39;
                    let b = (k as f32 * 0.77).sin() * 1.2;
                    let dir = Vec3::new(a.cos() * b.cos(), b.sin(), a.sin() * b.cos());
                    ch.spawn((Mesh3d(meshes.add(Sphere::new(0.035).mesh().uv(8, 4))), MeshMaterial3d(veins.clone()), Transform::from_translation(dir * 0.63)));
                }
                ch.spawn((PointLight { color: Color::srgb(1.0, 0.1, 0.25), intensity: 180000.0, range: 12.0, shadows_enabled: true, ..default() }, Transform::from_xyz(0.0, 0.0, 0.0)));
            });
    }
}

pub fn animate_sphere(time: Res<Time>, mut q: Query<&mut Transform, With<SphereVis>>) {
    let t = time.elapsed_secs();
    for mut tr in q.iter_mut() {
        tr.rotation = Quat::from_rotation_y(t * 0.4) * Quat::from_rotation_x((t * 0.3).sin() * 0.3);
        tr.translation.y = 1.6 + (t * 1.1).sin() * 0.08;
    }
}

/// Interactions specific to the prologue apartment, the laboratory and the limbo.
#[allow(clippy::too_many_arguments)]
pub fn prologue_interact(
    act: Res<Act>,
    mut game: ResMut<Game>,
    map: Option<Res<CityMap>>,
    mut prompt: ResMut<Prompt>,
    mut flow: ResMut<Flow>,
    mut popups: ResMut<Popups>,
    ui: Res<UiState>,
    db: Res<CaseDb>,
    mut sfx: EventWriter<Sfx>,
    binds: Res<crate::keys::Bindings>,
    mut read: Local<Vec<usize>>,
) {
    if game.phase != Phase::Prologue || ui.blocks_input() {
        return;
    }
    let Some(map) = map else { return };
    let m = &map.0;
    let pp = game.player.pos;
    let e = crate::keys::key_name(binds.key(Action::Interact));
    let mut target: Option<(String, String)> = None;
    if game.flag("prologue_apartment") {
        for (i, p) in m.props.iter().enumerate() {
            if p.center().distance(pp) > 1.8 {
                continue;
            }
            let note = match p.kind {
                PKind::Board if p.x < 4 => Some(("board1", "Recortes: \"Homem do Machado: 6 ataques, nenhum culpado (1918-1919)\". \"Corpo de Somerton: um código que ninguém decifra (1948)\". \"D.B. Cooper salta com 200 mil dólares (1971)\". Fios vermelhos ligam tudo a um ponto no centro: um prédio sem nome.")),
                PKind::Board => Some(("board2", "Um mapa de Nova Orleans. Um círculo vermelho em volta de um terreno no fim da estrada: \"LABORATÓRIO DELTA — fechado em 1971. Não consta em nenhum registro.\" Embaixo, com a sua letra: \"E se todos os mistérios forem a mesma porta?\"")),
                PKind::Desk | PKind::Typewriter => Some(("desk", "Uma carta anônima, sem selo, chegou ontem: \"Você está perto. Desça as escadas. Não toque em nada vermelho. — E.\" A letra se parece com a sua.")),
                PKind::RadioSet => Some(("radio", "O rádio chia entre as estações. Por um segundo, entre um jazz antigo e a previsão do tempo, alguém diz seu nome.")),
                PKind::Shelf => Some(("shelf", "Livros sobre sociedades secretas, OVNIs, desaparecimentos, magia, experimentos militares. Fitas cassete etiquetadas por ano: 1920, 1948, 1969, 1986...")),
                _ => None,
            };
            if let Some((k, t)) = note {
                target = Some((format!("[{}] Examinar", e), format!("{}|{}|{}", i, k, t)));
            }
        }
        // the door out
        let door = m.buildings[0].doors.iter().find(|d| d.1 == m.buildings[0].y + m.buildings[0].h - 1).copied();
        if let Some((dx, dy)) = door {
            if tile_center(dx, dy).distance(pp) < 1.5 {
                target = Some((format!("[{}] Ir ao Laboratório Delta", e), "go_lab".into()));
            }
        }
    } else if game.flag("prologue_lab") {
        if let Some(sp) = m.props.iter().find(|p| p.kind == PKind::Sphere) {
            let d = sp.center().distance(pp);
            if d < 5.0 && crate::util::hashf(game.minute as i32, 1, 1) < 0.02 {
                sfx.write(Sfx::Whisper);
            }
            if d < 2.2 {
                target = Some((format!("[{}] Tocar a esfera", e), "touch".into()));
                if crate::save::global_flag("break_thread") {
                    target = Some((format!("[{}] Tocar a esfera   [H] Virar as costas e ir embora", e), "touch".into()));
                }
            }
        }
        for p in m.props.iter() {
            if p.center().distance(pp) < 1.6 && matches!(p.kind, PKind::Board | PKind::Desk) && target.is_none() {
                target = Some((format!("[{}] Examinar", e), format!("{}|lab|Relatórios mofados: \"Projeto VERMILION. Objeto encontrado em 1947 sob a Rua Royal. Emite datas. Pesquisador-chefe: E. V.\" As iniciais foram raspadas com lâmina.", p.x * 100 + p.y)));
            }
        }
    } else if game.flag("limbo") {
        let door = crate::narrative::limbo_door();
        if door.distance(pp) < 2.0 {
            let next = crate::narrative::next_case(&game, &db);
            if let Some(n) = next {
                let d = db.get(n).unwrap();
                target = Some((format!("[{}] Abrir a porta: {} — {}", e, d.city.upper(), d.year), format!("door:{}", n)));
            }
        }
        if let Some(sp) = m.props.iter().find(|p| p.kind == PKind::Sphere) {
            if sp.center().distance(pp) < 2.2 {
                target = Some((format!("[{}] Ouvir a esfera", e), "listen".into()));
            }
        }
    }
    prompt.0 = target.as_ref().map(|t| t.0.clone());
    if act.just(Action::Hide) && game.flag("prologue_lab") && crate::save::global_flag("break_thread") {
        if target.as_ref().map(|t| t.1 == "touch").unwrap_or(false) {
            popups.push(Popup::new(
                PopStyle::Limbo,
                "QUEBRAR O FIO",
                "FIM VERDADEIRO",
                "Você olha para a esfera por muito tempo. Ela espera. Todas as vozes esperam.\n\nVocê vira as costas.\n\nLá fora, a chuva parou. Em algum lugar de 1920, um machado nunca é levantado. Em 1948, um homem de terno chega em casa para jantar. Em 1971, um avião pousa.\n\nNinguém vai lembrar de você. É exatamente assim que deveria ser.",
            ));
            popups.then("title");
            crate::save::set_global_flag("broken");
            return;
        }
    }
    if !act.just(Action::Interact) {
        return;
    }
    let Some((_, what)) = target else { return };
    if what == "go_lab" {
        if read.len() >= 2 {
            flow.actions.push("to_lab".into());
        } else {
            popups.push(Popup::plain("Ainda não", "Você ainda não juntou as peças. O quadro, a carta na mesa... algo aponta para um lugar."));
        }
    } else if what == "touch" {
        flow.actions.push("touch_sphere".into());
    } else if let Some(n) = what.strip_prefix("door:") {
        flow.actions.push(format!("start_case:{}", n));
    } else if what == "listen" {
        let lines = [
            "...o machado nunca foi o problema. A hora é o problema...",
            "...você já esteve aqui. Você sempre esteve aqui...",
            "...cada porta que você fecha abre outra...",
            "...ela não escolhe investigadores. Ela escolhe você. Sempre você...",
            "...3:17...",
        ];
        let l = lines[(game.limbo_stage as usize + game.day as usize) % lines.len()];
        popups.push(Popup::new(PopStyle::Mystery, "", "A ESFERA", l));
        sfx.write(Sfx::Hum);
    } else {
        let mut it = what.splitn(3, '|');
        let i: usize = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
        let _k = it.next();
        let text = it.next().unwrap_or("");
        if !read.contains(&i) {
            read.push(i);
        }
        popups.push(Popup::new(PopStyle::Evidence, "Arquivo de Elias Vale", "NOTAS", text));
        sfx.write(Sfx::Paper);
        game.write(text.to_string(), false);
    }
}

// ------------------------------------------------------------------ pointing a gun at people

#[allow(clippy::too_many_arguments)]
pub fn aim_reactions(time: Res<Time>, rt: Res<PlayerRt>, cursor: Res<Cursor>, mut game: ResMut<Game>, mut sim: ResMut<Sim>, mut t: Local<f32>, ui: Res<UiState>) {
    if ui.pauses_world() {
        return;
    }
    *t -= time.delta_secs();
    let pp = game.player.pos;
    let now = game.abs_minute();
    let year = game.year;
    let range = game.player.weapon.stats().range;
    if rt.aiming && cursor.valid {
        for i in 0..sim.agents.len() {
            let a = &sim.agents[i];
            if !a.active() && !matches!(a.state, AState::Tied { .. } | AState::Hostage) || a.pos.distance(pp) > range || a.pos.distance(cursor.world) > 1.0 {
                continue;
            }
            let pid = a.pid;
            let p = game.pop.get(pid).clone();
            let st = a.state;
            let a = &mut sim.agents[i];
            match st {
                AState::Normal | AState::Investigate { .. } => {
                    if matches!(p.job, crate::sim::people::Job::Police | crate::sim::people::Job::Detective) || p.job.armed() && p.traits.courage > 60 {
                        a.state = AState::Hostile;
                        a.say("Abaixa essa arma!", 2.0);
                    } else {
                        a.state = AState::HandsUp { until: now + 20.0 };
                        a.chat = None;
                        a.bubble = None;
                    }
                }
                _ => {}
            }
            // desperate pleading, a new line every few seconds
            if *t <= 0.0 && a.bubble.is_none() && matches!(a.state, AState::HandsUp { .. } | AState::Tied { .. } | AState::Hostage | AState::Cower { .. }) {
                let mut r = crate::util::Rng::new(now as u64 * 31 + pid as u64);
                let sit = match a.state {
                    AState::Tied { .. } | AState::Hostage => Situation::Execution,
                    _ => Situation::Aimed,
                };
                let line = plead(&p, &game.pop, sit, &mut r, year);
                a.say(line, 4.5);
                *t = 2.5;
                if let AState::HandsUp { .. } = a.state {
                    a.state = AState::HandsUp { until: now + 20.0 };
                }
            }
        }
    }
    let _ = &mut game;
}

/// People in desperate situations speak on their own.
pub fn desperate_voices(time: Res<Time>, mut sim: ResMut<Sim>, game: Res<Game>, mut t: Local<f32>, ui: Res<UiState>) {
    if ui.pauses_world() {
        return;
    }
    *t -= time.delta_secs();
    if *t > 0.0 {
        return;
    }
    *t = 3.0;
    let pp = game.player.pos;
    let now = game.abs_minute();
    for a in sim.agents.iter_mut() {
        if a.bubble.is_some() || a.pos.distance(pp) > 14.0 {
            continue;
        }
        let sit = match a.state {
            AState::Tied { gagged: false, .. } => Situation::Tied,
            AState::Hostage => Situation::Hostage,
            AState::HandsUp { .. } => Situation::Robbed,
            _ => continue,
        };
        if crate::util::hashf(a.pid as i32, now as i32, 5) < 0.5 {
            let p = game.pop.get(a.pid);
            let mut r = crate::util::Rng::new(now as u64 ^ a.pid as u64);
            let l = plead(p, &game.pop, sit, &mut r, game.year);
            a.say(l, 4.0);
        }
    }
}

// ------------------------------------------------------------------ venue music

#[derive(Component)]
pub struct VenueMusic;

#[derive(Resource, Default)]
pub struct VenueState {
    pub building: Option<usize>,
    pub seed: u32,
    pub entity: Option<Entity>,
    pub visits: u32,
}

#[allow(clippy::too_many_arguments)]
pub fn venue_music(
    mut c: Commands,
    mut vs: ResMut<VenueState>,
    mut vg: ResMut<VenueGen>,
    mut assets: ResMut<Assets<AudioSource>>,
    game: Res<Game>,
    map: Option<Res<CityMap>>,
    mut music: ResMut<MusicState>,
    mut sinks: Query<&mut AudioSink, With<VenueMusic>>,
    settings: Res<crate::keys::Settings>,
    sim: Res<Sim>,
) {
    vg.poll(&mut assets);
    let Some(map) = map else { return };
    if game.phase != Phase::City {
        if let Some(e) = vs.entity.take() {
            c.entity(e).despawn();
        }
        vs.building = None;
        return;
    }
    let m = &map.0;
    let pp = game.player.pos;
    let hour = game.hour();
    // the nearest place playing music: bars, clubs, cabarets, hotel lobbies, and homes with a radio on
    let mut best: Option<(f32, usize)> = None;
    for (i, b) in m.buildings.iter().enumerate() {
        let plays = match b.kind {
            BKind::Bar | BKind::Club | BKind::Cabaret => b.kind.is_open(hour),
            BKind::Hotel | BKind::Restaurant => b.kind.is_open(hour) && (6.0..24.0).contains(&hour),
            BKind::House | BKind::Apartment => (i % 5 == 0) && (18.0..23.5).contains(&hour) && sim.agents.iter().any(|a| a.target_b == Some(i) && a.arrived && a.act != crate::sim::agents::Act::Sleep),
            _ => false,
        };
        if !plays || b.closed_forever {
            continue;
        }
        let d = b.center_px().distance(pp);
        if d < 16.0 && best.map(|x| d < x.0).unwrap_or(true) {
            best = Some((d, i));
        }
    }
    match best {
        Some((d, b)) => {
            if vs.building != Some(b) {
                // a new place: a new piece that nobody ever heard before
                vs.building = Some(b);
                vs.visits += 1;
                vs.seed = (b as u32).wrapping_mul(7919) ^ (game.day as u32).wrapping_mul(104729) ^ vs.visits.wrapping_mul(2654435761) ^ game.timeline;
                vg.request(game.year, vs.seed);
                if let Some(e) = vs.entity.take() {
                    c.entity(e).despawn();
                }
            }
            if vs.entity.is_none() {
                if let Some((_, h)) = vg.ready.iter().find(|(s, _)| *s == vs.seed) {
                    let e = c.spawn((AudioPlayer(h.clone()), PlaybackSettings { mode: PlaybackMode::Loop, volume: Volume::Linear(0.0), ..default() }, VenueMusic)).id();
                    vs.entity = Some(e);
                }
            }
            let inside = m.building_at(pp) == Some(b);
            let homey = matches!(m.buildings[b].kind, BKind::House | BKind::Apartment);
            let vol = if inside { 0.9 } else { (1.0 - d / 16.0).max(0.0) * 0.35 } * if homey { 0.4 } else { 1.0 };
            if let Ok(mut s) = sinks.single_mut() {
                s.set_volume(Volume::Linear(vol * settings.music * settings.master));
            }
            music.duck = if inside { 0.1 } else { 1.0 - vol * 0.7 };
        }
        None => {
            if let Some(e) = vs.entity.take() {
                c.entity(e).despawn();
            }
            vs.building = None;
            music.duck = 1.0;
        }
    }
    // forget old generated pieces so they are never reused
    if vg.ready.len() > 6 {
        vg.ready.remove(0);
    }
}

// ------------------------------------------------------------------ photography

/// [P] raises the camera (viewfinder: closer framing), [V] takes a picture.
/// Photos are kept in the journal and saved as PNG next to the case files.
#[derive(Resource, Default)]
pub struct PhotoMode {
    pub on: bool,
    pub prev_dist: f32,
}

#[allow(clippy::too_many_arguments)]
pub fn photo_system(
    mut c: Commands,
    act: Res<Act>,
    ui: Res<crate::ui::UiState>,
    mut game: ResMut<Game>,
    mut pm: ResMut<PhotoMode>,
    mut cam: ResMut<crate::camera::CamState>,
    mut toasts: ResMut<Toasts>,
    mut sfx: EventWriter<Sfx>,
    (map, sim, crt): (Option<Res<CityMap>>, Res<crate::sim::agents::Sim>, Res<CaseRt>),
) {
    if ui.blocks_input() {
        return;
    }
    let has_cam = game.player.inv.iter().any(|i| matches!(i, crate::items::Item::Tool(crate::items::Tool::Camera)));
    if act.just(Action::Camera) {
        if !has_cam {
            toasts.push("Você não tem uma câmera. Procure numa loja ou casa de penhores.");
        } else {
            pm.on = !pm.on;
            if pm.on {
                pm.prev_dist = cam.dist_target;
                cam.dist_target = 10.0;
                toasts.push("Câmera erguida — [V] fotografar, [P] baixar");
            } else {
                cam.dist_target = pm.prev_dist.max(12.0);
            }
        }
    }
    if !act.just(Action::Photo) {
        return;
    }
    if !has_cam {
        toasts.push("Você não tem uma câmera.");
        return;
    }
    let Some(map) = map else { return };
    let m = &map.0;
    let pp = game.player.pos;
    let place = m
        .building_at(pp)
        .map(|b| if m.buildings[b].name.is_empty() { m.buildings[b].kind.label().to_string() } else { m.buildings[b].name.clone() })
        .or_else(|| m.district_at(pp).map(|d| m.districts[d].name.clone()))
        .unwrap_or_else(|| game.city.name().to_string());
    let mut people = Vec::new();
    for a in sim.agents.iter() {
        if a.pos.distance(pp) < 9.0 && m.line_clear(pp, a.pos, true) {
            let p = game.pop.get(a.pid);
            let who = if p.elias.met || p.case_role.is_some() { p.name() } else { p.job.label(p.female).to_string() };
            let who = if matches!(a.state, crate::sim::agents::AState::Dead) { format!("{} (morto)", who) } else { who };
            people.push(who);
            if people.len() >= 6 {
                break;
            }
        }
    }
    let m317 = (game.minute as i32).rem_euclid(24 * 60);
    let anomaly = if crt.echo_pos.iter().any(|e| e.distance(pp) < 5.0) {
        Some("Na revelação aparece uma silhueta de sobretudo que não estava ali quando você fotografou.".to_string())
    } else if (196..=198).contains(&m317) {
        Some("O relógio no canto da foto marca 3:17. Todos os rostos saíram borrados.".to_string())
    } else if game.fatigue > 50.0 && crate::util::hashf(game.day, m317, 5) < 0.4 {
        Some("A foto mostra a mesma rua, mas com carros de outra década.".to_string())
    } else {
        None
    };
    let n = game.photos.len() + 1;
    let (year, day) = (game.year, game.day);
    game.photos.push(crate::state::Photo { year, day, place: place.clone(), people, anomaly: anomaly.clone() });
    sfx.write(Sfx::Shutter);
    toasts.push(format!("Foto #{} — {}{}", n, place, if anomaly.is_some() { " (há algo estranho nela)" } else { "" }));
    let dir = crate::save::dir().join("fotos");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join(format!("foto_{}_{}_{:03}.png", game.city.name().replace(' ', "_"), game.year, n));
    c.spawn(bevy::render::view::screenshot::Screenshot::primary_window()).observe(bevy::render::view::screenshot::save_to_disk(path));
}
