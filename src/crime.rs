//! Crime and consequence: combat and aiming, bodies, blood and casings,
//! witnesses, the police profile, hold-ups, tying people up, kidnapping,
//! missing persons, break-ins and cleaning a crime scene.

use crate::audio::Sfx;
use crate::camera::{CamState, Cursor};
use crate::city::map::*;
use crate::env::EnvState;
use crate::items::*;
use crate::keys::{Act, Action};
use crate::player::PlayerRt;
use crate::render::mesh::{c3, MB};
use crate::render::city3d::Mats;
use crate::sim::agents::{AState, Act as NAct, Sim};
use crate::sim::people::*;
use crate::sim::update::SimEvents;
use crate::state::*;
use crate::ui::hud::Toasts;
use crate::ui::UiState;
use crate::world::CityMap;
use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DecalKind {
    Blood,
    Casing,
    Glass,
    Prints,
}

#[derive(Component)]
pub struct Decal {
    pub kind: DecalKind,
    pub pos: Vec2,
    pub crime: Option<u32>,
}

#[derive(Component)]
pub struct Tracer {
    pub life: f32,
}

#[derive(Component)]
pub struct MuzzleLight {
    pub life: f32,
}

#[derive(Event, Clone)]
pub struct CrimeEv {
    pub kind: CrimeKind,
    pub pos: Vec2,
    pub victim: Option<Pid>,
    pub noise: f32,
    pub weapon: Option<Weapon>,
}

#[derive(Event, Clone)]
pub struct SpawnDecal {
    pub kind: DecalKind,
    pub pos: Vec2,
    pub crime: Option<u32>,
}

#[derive(Resource, Default)]
pub struct CrimeRt {
    pub fire_cd: f32,
    pub missing_check: f32,
    pub police_tick: f32,
    pub dead_time: Option<f32>,
    pub last_crime: Option<u32>,
    pub cleaning: Option<(Entity, f32)>,
    pub arrest_pending: bool,
    /// (building, seconds seen inside by a resident, already reported)
    pub trespass: Option<(usize, f32, bool)>,
    pub greeted: Option<usize>,
}

// ------------------------------------------------------------------ combat

#[allow(clippy::too_many_arguments)]
pub fn player_combat(
    time: Res<Time>,
    act: Res<Act>,
    cursor: Res<Cursor>,
    mut rt: ResMut<PlayerRt>,
    mut game: ResMut<Game>,
    mut sim: ResMut<Sim>,
    map: Option<Res<CityMap>>,
    mut crt: ResMut<CrimeRt>,
    mut sfx: EventWriter<Sfx>,
    mut crimes: EventWriter<CrimeEv>,
    mut decals: EventWriter<SpawnDecal>,
    mut cam: ResMut<CamState>,
    ui: Res<UiState>,
    mut toasts: ResMut<Toasts>,
    (mut c, mut meshes, mats): (Commands, ResMut<Assets<Mesh>>, Option<Res<Mats>>),
) {
    let Some(map) = map else { return };
    let m = &map.0;
    let dt = time.delta_secs();
    crt.fire_cd = (crt.fire_cd - dt).max(0.0);
    if ui.blocks_input() || rt.in_car {
        rt.aiming = false;
        return;
    }
    // draw / holster, cycling through owned weapons
    if act.just(Action::Weapon) {
        let ws = game.player.weapons();
        if !rt.weapon_out {
            if game.player.weapon == Weapon::Fists && ws.len() > 1 {
                game.player.weapon = ws[1];
            }
            rt.weapon_out = true;
        } else {
            let i = ws.iter().position(|w| *w == game.player.weapon).unwrap_or(0);
            let next = ws[(i + 1) % ws.len()];
            if next == Weapon::Fists {
                rt.weapon_out = false;
            }
            game.player.weapon = next;
        }
        sfx.write(Sfx::Click);
        if game.player.weapon != Weapon::Fists {
            // drawing a weapon in public alarms people nearby
            let pp = game.player.pos;
            for a in sim.agents.iter_mut() {
                if a.active() && a.pos.distance_squared(pp) < 36.0 && a.state == AState::Normal && m.line_clear(a.pos, pp, true) {
                    a.alert += 0.6;
                    if !game.player.weapon.stats().melee {
                        a.say("Ele está armado!", 2.5);
                    }
                }
            }
        }
    }
    let w = game.player.weapon;
    let st = w.stats();
    rt.aiming = rt.weapon_out && !st.melee && !rt.carrying.is_some();
    // reload
    if act.just(Action::Reload) && !st.melee {
        let need = st.mag - game.player.mag;
        let got = game.player.use_ammo(need);
        if got > 0 {
            game.player.mag += got;
            sfx.write(Sfx::Lockpick);
            rt.action_lock = 0.8;
        } else {
            toasts.push("Sem munição.");
        }
    }
    let attack = cursor.clicked && rt.weapon_out || act.just(Action::Attack);
    if !attack || crt.fire_cd > 0.0 || rt.carrying.is_some() {
        return;
    }
    let pp = game.player.pos;
    let dir = if cursor.valid && rt.weapon_out {
        (cursor.world - pp).normalize_or_zero()
    } else {
        Vec2::new(rt.facing.cos(), rt.facing.sin())
    };
    if dir == Vec2::ZERO {
        return;
    }
    rt.facing = dir.y.atan2(dir.x);
    let melee = st.melee || !rt.weapon_out || act.just(Action::Attack);
    let combat = game.player.skill(Skill::Combat) as f32;
    if melee {
        let (wpn, stt) = if rt.weapon_out && st.melee { (w, st) } else { (Weapon::Fists, Weapon::Fists.stats()) };
        crt.fire_cd = stt.cooldown;
        rt.pose = crate::render::character::Pose::Punch;
        rt.action_lock = 0.3;
        // find target in a cone
        let mut best: Option<(usize, f32)> = None;
        for (i, a) in sim.agents.iter().enumerate() {
            if matches!(a.state, AState::Dead | AState::Carried) {
                continue;
            }
            let d = a.pos - pp;
            let dist = d.length();
            if dist < stt.range + 0.3 && d.normalize_or_zero().dot(dir) > 0.5 && best.map(|b| dist < b.1).unwrap_or(true) {
                best = Some((i, dist));
            }
        }
        sfx.write(Sfx::Punch);
        if let Some((i, _)) = best {
            let dmg = stt.damage * (1.0 + combat * 0.08);
            let pid = sim.agents[i].pid;
            hurt_agent(&mut sim, &mut game, i, dmg, wpn == Weapon::Fists, dir, &mut crimes, &mut decals, Some(wpn));
            game.player.train(Skill::Combat, 3);
            cam.shake = cam.shake.max(0.25);
            let _ = pid;
        }
        return;
    }
    // firearm
    if game.player.mag <= 0 {
        sfx.write(Sfx::Click);
        toasts.push(format!("Arma vazia. [{}] recarrega.", crate::keys::key_name(KeyCode::KeyT)));
        crt.fire_cd = 0.3;
        return;
    }
    game.player.mag -= 1;
    crt.fire_cd = st.cooldown;
    sfx.write(if w == Weapon::Shotgun { Sfx::Shotgun } else { Sfx::Gunshot });
    cam.shake = cam.shake.max(0.35);
    let range = st.range;
    // aim distance is clamped to the weapon range
    let aim_len = if cursor.valid { (cursor.world - pp).length().min(range) } else { range };
    let _ = aim_len;
    let spread = st.spread * (1.2 - combat * 0.05).max(0.4) * if rt.moving > 0.0 { 1.6 } else { 1.0 };
    let mut rng = crate::util::Rng::new((time.elapsed_secs() * 1000.0) as u64);
    for _ in 0..st.pellets {
        let a = dir.y.atan2(dir.x) + rng.rangef(-spread, spread);
        let d = Vec2::new(a.cos(), a.sin());
        // find first hit along the ray
        let mut hit: Option<(usize, f32)> = None;
        for (i, ag) in sim.agents.iter().enumerate() {
            if matches!(ag.state, AState::Dead | AState::Carried) {
                continue;
            }
            let rel = ag.pos - pp;
            let along = rel.dot(d);
            if along <= 0.2 || along > range {
                continue;
            }
            let perp = (rel - d * along).length();
            if perp < 0.32 && hit.map(|h| along < h.1).unwrap_or(true) && m.line_clear(pp, ag.pos, false) {
                hit = Some((i, along));
            }
        }
        let end_len = hit.map(|h| h.1).unwrap_or_else(|| {
            // stop at walls
            let mut l = range;
            let steps = (range / 0.25) as i32;
            for k in 1..steps {
                let p = pp + d * (k as f32 * 0.25);
                let (tx, ty) = to_tile(p);
                if m.blocked(tx, ty) {
                    l = k as f32 * 0.25;
                    break;
                }
            }
            l
        });
        // tracer
        if let Some(mats) = &mats {
            let mut mb = MB::new();
            let from = Vec3::new(pp.x, 1.25, pp.y);
            let to = Vec3::new(pp.x + d.x * end_len, 1.2, pp.y + d.y * end_len);
            let mid = (from + to) / 2.0;
            let len = from.distance(to);
            mb.bx(Vec3::ZERO, Vec3::new(len / 2.0, 0.012, 0.012), c3([1.0, 0.85, 0.5]));
            let rot = Quat::from_rotation_y(-d.y.atan2(d.x));
            c.spawn((Mesh3d(meshes.add(mb.build())), MeshMaterial3d(mats.glow.clone()), Transform::from_translation(mid).with_rotation(rot), Tracer { life: 0.06 }));
            c.spawn((
                PointLight { color: Color::srgb(1.0, 0.8, 0.5), intensity: 90000.0, range: 7.0, ..default() },
                Transform::from_translation(from + Vec3::new(d.x, 0.0, d.y) * 0.5),
                MuzzleLight { life: 0.05 },
            ));
        }
        if let Some((i, _)) = hit {
            let dmg = st.damage * rng.rangef(0.8, 1.2);
            hurt_agent(&mut sim, &mut game, i, dmg, false, d, &mut crimes, &mut decals, Some(w));
            game.player.train(Skill::Combat, 2);
        }
    }
    decals.write(SpawnDecal { kind: DecalKind::Casing, pos: pp + Vec2::new(dir.y, -dir.x) * 0.5, crime: crt.last_crime });
    crimes.write(CrimeEv { kind: CrimeKind::Shooting, pos: pp, victim: None, noise: st.noise, weapon: Some(w) });
}

#[allow(clippy::too_many_arguments)]
pub fn hurt_agent(sim: &mut Sim, game: &mut Game, i: usize, dmg: f32, nonlethal: bool, dir: Vec2, crimes: &mut EventWriter<CrimeEv>, decals: &mut EventWriter<SpawnDecal>, weapon: Option<Weapon>) {
    let a = &mut sim.agents[i];
    let pid = a.pid;
    let was_alive = !matches!(a.state, AState::Dead);
    a.health -= dmg;
    a.pos += dir * 0.25;
    a.alert = 1.0;
    let pos = a.pos;
    if !nonlethal {
        decals.write(SpawnDecal { kind: DecalKind::Blood, pos, crime: None });
    }
    let year = game.year;
    let day = game.day;
    if a.health <= 0.0 && was_alive {
        if nonlethal && a.health > -40.0 {
            a.state = AState::Unconscious { until: game.abs_minute() + 90.0 };
            a.health = 15.0;
            crimes.write(CrimeEv { kind: CrimeKind::Assault, pos, victim: Some(pid), noise: 6.0, weapon });
        } else {
            a.state = AState::Dead;
            a.path.clear();
            a.speed = 0.0;
            a.chat = None;
            let p = game.pop.get_mut(pid);
            p.life = Life::Dead { year, day, cause: "assassinado".into(), by_elias: true };
            game.stat("kills", 1);
            crimes.write(CrimeEv { kind: CrimeKind::Murder, pos, victim: Some(pid), noise: 8.0, weapon });
            // family consequences
            let name = game.pop.get(pid).name();
            let spouse = game.pop.get(pid).spouse;
            let kids = game.pop.get(pid).children.clone();
            if let Some(s) = spouse {
                game.pop.get_mut(s).destiny.push("mourning".into());
                let brave = game.pop.get(s).traits.courage > 60;
                game.pop.get_mut(s).destiny.push(if brave { "revenge".into() } else { "left_city".into() });
                let fem = game.pop.get(pid).female;
                game.pop.get_mut(s).remember(year, day, format!("{} foi assassinad{}.", name, if fem { "a" } else { "o" }), Some(pid), -10);
            }
            for k in kids {
                let kid = game.pop.get_mut(k);
                kid.destiny.push("mourning".into());
                if kid.age(year) < 18 {
                    kid.destiny.push(if kid.traits.morality > 55 { "police".into() } else { "criminal".into() });
                }
            }
            let kills = game.get_stat("kills");
            if kills == 1 {
                game.write(format!("Matei {}. Minhas mãos não param de tremer.", name), true);
            } else if kills == 10 {
                game.write("Já perdi a conta. Não sei mais se estou resolvendo mistérios ou criando.".to_string(), true);
            }
        }
    } else if was_alive {
        // reaction to being hit
        let p = game.pop.get(pid);
        let fight = p.traits.courage > 60 || p.job.armed();
        if fight {
            a.state = AState::Hostile;
            a.say(*["Seu desgraçado!", "Você vai pagar!", "Vem!"].get((pid % 3) as usize).unwrap(), 2.0);
        } else {
            a.state = AState::Flee { from: pos - dir, until: game.abs_minute() + 40.0 };
            a.path.clear();
            a.say("Socorro! Socorro!", 2.5);
        }
        crimes.write(CrimeEv { kind: CrimeKind::Assault, pos, victim: Some(pid), noise: 10.0, weapon });
    }
}

/// NPCs fighting back / police shooting at Elias.
#[allow(clippy::too_many_arguments)]
pub fn hostile_ai(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut sim: ResMut<Sim>,
    map: Option<Res<CityMap>>,
    mut rt: ResMut<PlayerRt>,
    mut sfx: EventWriter<Sfx>,
    mut cam: ResMut<CamState>,
    ui: Res<UiState>,
    mut crt: ResMut<CrimeRt>,
    env: Res<EnvState>,
    mut decals: EventWriter<SpawnDecal>,
) {
    let Some(map) = map else { return };
    if ui.pauses_world() {
        return;
    }
    let m = &map.0;
    let dt = time.delta_secs().min(0.1);
    let pp = game.player.pos;
    let now = game.abs_minute();
    let mut damage = 0.0;
    let wanted = game.police.wanted;
    for i in 0..sim.agents.len() {
        let st = sim.agents[i].state;
        match st {
            AState::Hostile | AState::Chase => {
                let a = &mut sim.agents[i];
                let d = pp - a.pos;
                let dist = d.length();
                a.facing = d.y.atan2(d.x);
                let armed = a.ammo > 0 && game.pop.get(a.pid).job.armed();
                // lose track when far away or can't see in the dark
                if dist > 35.0 || (dist > 18.0 && env.light_level_player < 0.3 && rt.sneaking) {
                    a.state = AState::Normal;
                    a.replan_at = now;
                    continue;
                }
                let shoot = armed && dist < 11.0 && dist > 1.4 && (st == AState::Hostile || wanted >= 2 || rt.weapon_out) && m.line_clear(a.pos, pp, false);
                if shoot {
                    a.speed = 0.0;
                    a.pose = crate::render::character::Pose::Aim;
                    if a.shoot_cd <= 0.0 {
                        a.shoot_cd = 0.9 + (a.pid % 5) as f32 * 0.15;
                        a.ammo -= 1;
                        sfx.write(Sfx::Gunshot);
                        let hit_p = (0.55 - dist * 0.03 - if rt.moving > 3.0 { 0.2 } else { 0.0 }).max(0.08);
                        if crate::util::hashf(a.pid as i32, (now * 10.0) as i32, 7) < hit_p {
                            damage += 30.0 + crate::util::hashf(a.pid as i32, (now * 10.0) as i32, 8) * 25.0;
                        }
                    }
                } else if dist > 1.1 {
                    let sp = if dist > 4.0 { 3.6 } else { 2.2 };
                    let np = a.pos + d / dist * sp * dt;
                    a.pos = m.collide(np, 0.25);
                    a.speed = sp;
                    a.pose = crate::render::character::Pose::Run;
                } else {
                    a.speed = 0.0;
                    if st == AState::Chase {
                        // police grabs Elias
                        crt.arrest_pending = true;
                        a.pose = crate::render::character::Pose::Talk;
                    } else {
                        a.pose = crate::render::character::Pose::Punch;
                        if a.shoot_cd <= 0.0 {
                            a.shoot_cd = 0.9;
                            damage += 9.0;
                            sfx.write(Sfx::Punch);
                        }
                    }
                }
            }
            AState::Hostage => {
                let a = &mut sim.agents[i];
                let d = pp - a.pos;
                let dist = d.length();
                if dist > 1.6 {
                    let sp = if dist > 4.0 { 3.0 } else { 1.8 };
                    let np = a.pos + d / dist * sp * dt;
                    a.pos = m.collide(np, 0.25);
                    a.speed = sp;
                    a.facing = d.y.atan2(d.x);
                    a.pose = crate::render::character::Pose::Walk;
                } else {
                    a.speed = 0.0;
                    a.pose = crate::render::character::Pose::Cower;
                }
                // hostage escapes if Elias puts the weapon away or goes too far
                if !rt.weapon_out && crate::util::hashf(a.pid as i32, now as i32, 3) < dt * 0.3 || dist > 12.0 {
                    a.state = AState::Flee { from: pp, until: now + 60.0 };
                    a.path.clear();
                    a.say("Socorro! Ele me sequestrou!", 3.0);
                }
            }
            AState::Carried => {
                let a = &mut sim.agents[i];
                let back = Vec2::new(rt.facing.cos(), rt.facing.sin());
                a.pos = pp - back * if rt.dragging { 0.9 } else { 0.1 };
                a.facing = rt.facing;
                if rt.dragging && crate::util::hashf((pp.x * 3.0) as i32, (pp.y * 3.0) as i32, 1) < dt * 2.0 {
                    decals.write(SpawnDecal { kind: DecalKind::Blood, pos: a.pos, crime: None });
                }
            }
            _ => {}
        }
        sim.agents[i].shoot_cd = (sim.agents[i].shoot_cd - 0.0).max(0.0);
    }
    if damage > 0.0 {
        let armor = 1.0 - game.player.skill(Skill::Combat) as f32 * 0.03;
        game.player.health -= damage * armor;
        rt.hurt_flash = 1.0;
        cam.shake = 0.5;
        sfx.write(Sfx::Hurt);
        decals.write(SpawnDecal { kind: DecalKind::Blood, pos: pp, crime: None });
    }
}

// ------------------------------------------------------------------ perception & police

#[allow(clippy::too_many_arguments)]
pub fn process_crimes(
    mut ev: EventReader<CrimeEv>,
    mut game: ResMut<Game>,
    mut sim: ResMut<Sim>,
    map: Option<Res<CityMap>>,
    env: Res<EnvState>,
    mut crt: ResMut<CrimeRt>,
    mut toasts: ResMut<Toasts>,
    mut sfx: EventWriter<Sfx>,
) {
    let Some(map) = map else { return };
    let m = &map.0;
    let now = game.abs_minute();
    for e in ev.read() {
        let outfit = game.player.outfit;
        let place = m.building_at(e.pos).map(|b| {
            let b = &m.buildings[b];
            if b.name.is_empty() { b.address.clone() } else { b.name.clone() }
        }).unwrap_or_else(|| m.street_name_near(e.pos));
        // merge with a very recent crime at the same place
        let id = if let Some(c) = game.police.crimes.iter_mut().rev().find(|c| c.by_elias && c.pos.distance(e.pos) < 6.0 && (c.day as f32 * 1440.0 + c.minute) > now - 3.0) {
            if e.kind.severity() > c.kind.severity() {
                c.kind = e.kind;
                c.victim = e.victim.or(c.victim);
            }
            c.id
        } else {
            let id = game.police.next_id;
            game.police.next_id += 1;
            let (day, minute) = (game.day, game.minute);
            game.police.crimes.push(Crime {
                id,
                kind: e.kind,
                by_elias: true,
                victim: e.victim,
                pos: e.pos,
                day,
                minute,
                weapon: e.weapon,
                outfit,
                discovered: false,
                reported: false,
                witnesses: Vec::new(),
                body_hidden: false,
                blood: e.kind == CrimeKind::Murder || e.kind == CrimeKind::Assault,
                place,
            });
            id
        };
        crt.last_crime = Some(id);
        // who saw / heard it?
        let light = env.light_level_player.max(0.15);
        let sight = 5.0 + 11.0 * light;
        for i in 0..sim.agents.len() {
            let a = &sim.agents[i];
            if !a.active() || Some(a.pid) == e.victim && matches!(a.state, AState::Dead) {
                continue;
            }
            if matches!(a.state, AState::Talking) {
                // the person Elias is talking to certainly saw it
            }
            let d = a.pos.distance(e.pos);
            let asleep = a.act == NAct::Sleep && a.arrived;
            let hear = d < e.noise * if asleep { 0.5 } else { 1.0 };
            let see = !asleep && d < sight && m.line_clear(a.pos, e.pos, true);
            if !hear && !see {
                continue;
            }
            let pid = a.pid;
            let p = game.pop.get(pid).clone();
            let is_victim = Some(pid) == e.victim;
            if see || is_victim {
                let conf = ((1.0 - d / sight) * 0.7 + light * 0.3 + if is_victim { 0.3 } else { 0.0 }).clamp(0.1, 1.0);
                let loyal = p.elias.trust > 50 || matches!(p.elias.romance, Romance::Dating | Romance::Lovers | Romance::Married | Romance::Engaged);
                let will = p.traits.morality > 30 && !loyal && !matches!(p.job, Job::Gangster | Job::Smuggler);
                if let Some(c) = game.police.crimes.iter_mut().find(|c| c.id == id) {
                    c.discovered = true;
                    if !c.witnesses.iter().any(|w| w.pid == pid) {
                        c.witnesses.push(Witness { pid, saw_face: conf, saw_weapon: e.weapon.is_some(), saw_body: e.kind == CrimeKind::Murder, will_report: will, reported: false, silenced: None });
                    }
                }
                let (y, dd) = (game.year, game.day);
                let place = game.police.crimes.iter().find(|c| c.id == id).map(|c| c.place.clone()).unwrap_or_default();
                game.pop.get_mut(pid).remember(y, dd, format!("Vi um homem cometer {} em {}.", e.kind.label().to_lowercase(), place), None, 8);
                let a = &mut sim.agents[i];
                a.seen_elias_crime = true;
                if matches!(a.state, AState::Hostile | AState::Chase | AState::Tied { .. } | AState::Hostage | AState::HandsUp { .. }) {
                    continue;
                }
                a.chat = None;
                // police officers intervene immediately
                if matches!(p.job, Job::Police | Job::Detective) {
                    a.state = AState::Chase;
                    a.say(if game.year < 1960 { "Parado! Polícia!" } else { "Polícia! Mãos pra cima!" }, 2.5);
                    game.police.wanted = game.police.wanted.max(if e.kind.severity() >= 25 { 2 } else { 1 });
                    game.police.heat += e.kind.severity() as f32;
                    if !game.police.profile_outfits.contains(&outfit) {
                        game.police.profile_outfits.push(outfit);
                    }
                    if game.year < 1960 {
                        sfx.write(Sfx::Whistle);
                    } else {
                        sfx.write(Sfx::Siren);
                    }
                    continue;
                }
                let brave = p.traits.courage as i32 - p.traits.fear as i32 + if p.job.armed() { 40 } else { 0 };
                if brave > 45 && e.kind.severity() >= 15 && !is_victim {
                    a.state = AState::Hostile;
                    a.say("Ei! Pare aí!", 2.0);
                } else if will {
                    a.state = AState::Flee { from: e.pos, until: now + 20.0 };
                    a.path.clear();
                    a.say(if e.kind == CrimeKind::Murder { "Assassino! Assassino!" } else { "Socorro! Polícia!" }, 2.5);
                    // after fleeing they will go to the police
                    a.replan_at = now + 20.0;
                    a.knows_bodies.push(u32::MAX - id);
                } else {
                    a.state = AState::Cower { until: now + 15.0 };
                    a.say("Eu não vi nada... eu não vi nada...", 2.5);
                }
                if e.kind == CrimeKind::Murder {
                    sfx.write(Sfx::Scream);
                }
            } else if hear {
                let a = &mut sim.agents[i];
                if a.state == AState::Normal && !asleep {
                    a.state = AState::Investigate { at: e.pos, until: now + 15.0 };
                    a.path.clear();
                    a.say(if e.kind == CrimeKind::Shooting { "Isso foi um tiro?" } else { "Que barulho foi esse?" }, 2.0);
                } else if asleep && e.noise > 10.0 {
                    a.state = AState::Investigate { at: e.pos, until: now + 10.0 };
                    a.path.clear();
                    a.arrived = false;
                    a.say("Hã? Quem está aí?", 2.0);
                }
            }
        }
        let _ = &mut toasts;
    }
    // fleeing witnesses turn into reporters once they calmed down
    for i in 0..sim.agents.len() {
        let a = &sim.agents[i];
        if a.state == AState::Normal {
            if let Some(k) = a.knows_bodies.iter().position(|v| *v > u32::MAX - 100000) {
                let id = u32::MAX - a.knows_bodies[k];
                let pid = a.pid;
                let will = game.police.crimes.iter().find(|c| c.id == id).map(|c| c.witnesses.iter().any(|w| w.pid == pid && w.will_report && !w.reported && w.silenced.is_none())).unwrap_or(false);
                let a = &mut sim.agents[i];
                a.knows_bodies.remove(k);
                if will {
                    a.state = AState::Report { crime: id };
                    a.path.clear();
                }
            }
        }
    }
}

/// Reports reaching the police, wanted level, recognition by officers,
/// arrests, missing persons, bodies being found.
#[allow(clippy::too_many_arguments)]
pub fn police_system(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut sim: ResMut<Sim>,
    mut sev: ResMut<SimEvents>,
    map: Option<Res<CityMap>>,
    mut crt: ResMut<CrimeRt>,
    mut toasts: ResMut<Toasts>,
    rt: Res<PlayerRt>,
    env: Res<EnvState>,
    ui: Res<UiState>,
    mut sfx: EventWriter<Sfx>,
    mut popups: ResMut<crate::ui::screens::Popups>,
) {
    let Some(map) = map else { return };
    if ui.pauses_world() {
        return;
    }
    let m = &map.0;
    let dt = time.delta_secs();
    let now = game.abs_minute();
    // reports
    for (pid, crime) in sev.reports.drain(..) {
        let name = game.pop.get(pid).name();
        let alias_known = game.pop.get(pid).elias.alias.clone();
        if let Some(c) = game.police.crimes.iter_mut().find(|c| c.id == crime) {
            c.reported = true;
            let outfit = c.outfit;
            let sev = c.kind.severity();
            let weapon = c.weapon;
            let kind = c.kind;
            for w in c.witnesses.iter_mut() {
                if w.pid == pid {
                    w.reported = true;
                }
            }
            game.police.heat += sev as f32;
            game.police.wanted = game.police.wanted.max(if sev >= 40 { 3 } else if sev >= 15 { 2 } else { 1 });
            if !game.police.profile_outfits.contains(&outfit) {
                game.police.profile_outfits.push(outfit);
            }
            if weapon.is_some() {
                game.police.profile_weapon = weapon;
            }
            game.police.knows_face = (game.police.knows_face + 0.35).min(1.0);
            if let Some(a) = alias_known {
                if a != "forasteiro" {
                    game.police.knows_name = Some(a.clone());
                    game.police.profile_notes.push(format!("Suspeito conhecido como '{}'.", a));
                }
            }
            game.police.profile_notes.push(format!("{} denunciou {}.", name, kind.label().to_lowercase()));
            toasts.push(format!("{} denunciou você à polícia.", name));
            game.add_rep(Group::Police, -sev / 3);
            game.add_rep(Group::Citizens, -sev / 5);
            game.add_rep(Group::Crime, sev / 6);
            if kind == CrimeKind::Murder {
                let place = game.police.crimes.iter().find(|c| c.id == crime).map(|c| c.place.clone()).unwrap_or_default();
                game.news_ticker.push(format!("...a polícia procura um homem de {} ligado a um assassinato em {}...", outfit_desc(outfit), place));
                let (y, d, city, tl) = (game.year, game.day, game.city, game.timeline);
                game.papers.push(Headline { day: d, year: y, city, timeline: tl, title: "ESTRANHO SUSPEITO VISTO EM CENA DE CRIME".into(), body: format!("Testemunhas descrevem um homem de {}. A polícia pede informações.", outfit_desc(outfit)) });
            }
        }
    }
    // bodies found lying around
    crt.police_tick -= dt;
    if crt.police_tick <= 0.0 {
        crt.police_tick = 0.5;
        let bodies: Vec<(usize, Pid, Vec2)> = sim.agents.iter().enumerate().filter(|(_, a)| a.state == AState::Dead).map(|(i, a)| (i, a.pid, a.pos)).collect();
        for (_, bpid, bpos) in bodies {
            let already = game.police.crimes.iter().any(|c| c.victim == Some(bpid) && c.discovered);
            if already {
                continue;
            }
            for a in sim.agents.iter_mut() {
                if !a.active() || a.state != AState::Normal || a.pid == bpid {
                    continue;
                }
                if a.pos.distance_squared(bpos) < 36.0 && m.line_clear(a.pos, bpos, true) && !(a.act == NAct::Sleep && a.arrived) {
                    a.say("Meu Deus! Tem um corpo aqui!", 3.0);
                    sfx.write(Sfx::Scream);
                    if let Some(c) = game.police.crimes.iter_mut().find(|c| c.victim == Some(bpid)) {
                        c.discovered = true;
                        let id = c.id;
                        a.state = AState::Report { crime: id };
                        a.path.clear();
                    } else {
                        // body of someone killed earlier without a record
                        let id = game.police.next_id;
                        game.police.next_id += 1;
                        let (day, minute, outfit) = (game.day, game.minute, game.player.outfit);
                        game.police.crimes.push(Crime { id, kind: CrimeKind::Murder, by_elias: true, victim: Some(bpid), pos: bpos, day, minute, weapon: None, outfit, discovered: true, reported: false, witnesses: vec![], body_hidden: false, blood: true, place: m.street_name_near(bpos) });
                        a.state = AState::Report { crime: id };
                        a.path.clear();
                    }
                    let nm = game.pop.get(bpid).name();
                    sim_rumor(&mut game, format!("Encontraram {} morto. Dizem que foi coisa feia.", nm), Some(bpid), a.pid);
                    break;
                }
            }
        }
        // missing persons: tied, hostages, hidden bodies — families notice
        crt.missing_check -= 0.5;
        if crt.missing_check <= 0.0 {
            crt.missing_check = 20.0;
            let missing: Vec<Pid> = sim.agents.iter().filter(|a| matches!(a.state, AState::Tied { .. } | AState::Hostage | AState::Carried) || (a.state == AState::Dead && a.in_car)).map(|a| a.pid).chain(m.props.iter().flat_map(|p| p.bodies.iter().copied())).collect();
            for pid in missing {
                let key = format!("missing:{}", pid);
                if game.flag(&key) {
                    continue;
                }
                let fam = game.pop.get(pid).spouse.or(game.pop.get(pid).parents[0]);
                // after a while someone notices
                if crate::util::hashf(pid as i32, game.day, (game.minute / 60.0) as u32) < 0.25 {
                    game.set(&key);
                    let nm = game.pop.get(pid).name();
                    game.news_ticker.push(format!("...a família de {} pede notícias: desaparecido desde ontem...", nm));
                    sim_rumor(&mut game, format!("{} sumiu. Ninguém sabe de nada.", nm), Some(pid), fam.unwrap_or(pid));
                    let id = game.police.next_id;
                    game.police.next_id += 1;
                    let (day, minute, outfit) = (game.day, game.minute, game.player.outfit);
                    game.police.crimes.push(Crime { id, kind: CrimeKind::Kidnap, by_elias: true, victim: Some(pid), pos: Vec2::ZERO, day, minute, weapon: None, outfit, discovered: true, reported: true, witnesses: vec![], body_hidden: true, blood: false, place: "desconhecido".into() });
                    game.police.heat += 10.0;
                    toasts.push(format!("A família de {} comunicou o desaparecimento.", nm));
                }
            }
        }
    }
    // wanted decays slowly when nobody sees Elias
    game.police.heat = (game.police.heat - dt * 0.08).max(0.0);
    if game.police.heat < 1.0 && game.police.wanted > 0 && !sim.agents.iter().any(|a| a.state == AState::Chase) {
        game.police.wanted -= 1;
        game.police.heat = if game.police.wanted > 0 { 25.0 } else { 0.0 };
        if game.police.wanted == 0 {
            toasts.push("A polícia parou de procurar você. Por enquanto.");
        }
    }
    // officers recognise Elias
    if game.police.wanted > 0 {
        let pp = game.player.pos;
        let disguised = game.player.outfit == Outfit::Police || !game.police.profile_outfits.contains(&game.player.outfit);
        let face = game.police.knows_face;
        let visible = env.light_level_player;
        for a in sim.agents.iter_mut() {
            if a.state != AState::Normal || !matches!(game.pop.get(a.pid).job, Job::Police | Job::Detective) {
                continue;
            }
            let d = a.pos.distance(pp);
            let range = 4.0 + 10.0 * visible * if rt.sneaking { 0.5 } else { 1.0 };
            if d < range && m.line_clear(a.pos, pp, true) {
                let recog = if disguised { face * 0.6 } else { 1.0 };
                if recog > 0.45 || (rt.weapon_out && d < 8.0) {
                    a.state = AState::Chase;
                    a.say("É ele! Parado!", 2.5);
                    game.police.heat += 5.0;
                }
            }
        }
    }
    // arrest
    if crt.arrest_pending {
        crt.arrest_pending = false;
        if rt.weapon_out {
            return;
        }
        let fine = crate::economy::price(&game, 20 * game.police.wanted as i32 + game.police.heat as i32);
        let serious = game.police.crimes.iter().any(|c| c.by_elias && c.reported && c.kind == CrimeKind::Murder);
        for a in sim.agents.iter_mut() {
            if a.state == AState::Chase {
                a.state = AState::Normal;
                a.replan_at = now;
            }
        }
        if serious {
            popups.push(crate::ui::screens::Popup::plain(
                "PRESO",
                "A cela cheira a urina e ferrugem. No terceiro dia, quando o juiz lê seu nome, você ouve a esfera zumbindo atrás da parede. A cela se dissolve.",
            ));
            game.set("arrest_reset");
        } else {
            game.player.money = (game.player.money - fine).max(0);
            game.player.inv.retain(|i| !matches!(i, Item::Weapon(_) | Item::Ammo(_) | Item::Drug(_)));
            game.player.weapon = Weapon::Fists;
            game.player.mag = 0;
            game.minute += 8.0 * 60.0;
            popups.push(crate::ui::screens::Popup::plain(
                "PRESO",
                &format!("Uma noite na cela, uma multa e as armas confiscadas. O delegado anotou seu rosto. ({} de multa)", crate::economy::money_str(&game, fine)),
            ));
            if let Some(ps) = m.find_building(BKind::Police) {
                game.player.pos = m.buildings[ps].outside_px();
            }
        }
        game.police.wanted = 0;
        game.police.heat = 0.0;
        game.police.arrests += 1;
        game.police.knows_face = 1.0;
        game.stat("arrests", 1);
    }
}

pub fn outfit_desc(o: Outfit) -> &'static str {
    match o {
        Outfit::Modern => "roupas estranhas, de tecido que ninguém reconhece",
        Outfit::Suit => "terno escuro",
        Outfit::Worker => "roupa de operário",
        Outfit::Police => "uniforme policial",
        Outfit::Doctor => "jaleco branco",
        Outfit::Journalist => "paletó de repórter",
        Outfit::Gangster => "sobretudo preto",
        Outfit::Aristocrat => "traje de gala",
        Outfit::Priest => "batina",
    }
}

pub fn sim_rumor(game: &mut Game, text: String, about: Option<Pid>, first: Pid) {
    let _ = game;
    RUMOR_QUEUE.with(|q| q.borrow_mut().push((text, about, first)));
}

thread_local! {
    pub static RUMOR_QUEUE: std::cell::RefCell<Vec<(String, Option<Pid>, Pid)>> = const { std::cell::RefCell::new(Vec::new()) };
}

pub fn flush_rumors(mut sim: ResMut<Sim>, game: Res<Game>) {
    let items: Vec<(String, Option<Pid>, Pid)> = RUMOR_QUEUE.with(|q| q.borrow_mut().drain(..).collect());
    for (t, about, first) in items {
        sim.add_rumor(t, about, true, 5, vec![first], game.day);
    }
}

// ------------------------------------------------------------------ decals

pub fn spawn_decals(mut c: Commands, mut ev: EventReader<SpawnDecal>, mut meshes: ResMut<Assets<Mesh>>, mats: Option<Res<Mats>>, map: Option<Res<CityMap>>) {
    let Some(mats) = mats else { return };
    let Some(map) = map else { return };
    for e in ev.read() {
        let y = crate::world::ground_y(&map.0, e.pos) + 0.012;
        let mut mb = MB::new();
        let h = crate::util::hashf((e.pos.x * 10.0) as i32, (e.pos.y * 10.0) as i32, 3);
        match e.kind {
            DecalKind::Blood => {
                let r = 0.2 + h * 0.3;
                mb.floor(-r, -r * 0.8, r, r * 0.8, 0.0, c3([0.28, 0.01, 0.02]));
                mb.floor(r * 0.5, -r * 0.2, r * 1.2, r * 0.3, 0.001, c3([0.25, 0.01, 0.02]));
            }
            DecalKind::Casing => mb.cylinder(Vec3::ZERO, 0.02, 0.05, 6, c3([0.75, 0.6, 0.25])),
            DecalKind::Glass => {
                for k in 0..6 {
                    let a = k as f32 * 1.1 + h;
                    mb.floor(a.cos() * 0.3, a.sin() * 0.3, a.cos() * 0.3 + 0.08, a.sin() * 0.3 + 0.05, 0.0, c3([0.6, 0.7, 0.75]));
                }
            }
            DecalKind::Prints => {
                mb.floor(-0.05, -0.1, 0.05, 0.1, 0.0, c3([0.15, 0.1, 0.08]));
            }
        }
        c.spawn((
            Mesh3d(meshes.add(mb.build())),
            MeshMaterial3d(if e.kind == DecalKind::Casing { mats.glow.clone() } else { mats.plain.clone() }),
            Transform::from_xyz(e.pos.x, y, e.pos.y).with_rotation(Quat::from_rotation_y(h * 6.0)),
            Decal { kind: e.kind, pos: e.pos, crime: e.crime },
            crate::world::AgentRoot,
        ));
    }
}

pub fn fade_fx(mut c: Commands, time: Res<Time>, mut t: Query<(Entity, &mut Tracer)>, mut l: Query<(Entity, &mut MuzzleLight)>) {
    let dt = time.delta_secs();
    for (e, mut tr) in t.iter_mut() {
        tr.life -= dt;
        if tr.life <= 0.0 {
            c.entity(e).despawn();
        }
    }
    for (e, mut ml) in l.iter_mut() {
        ml.life -= dt;
        if ml.life <= 0.0 {
            c.entity(e).despawn();
        }
    }
}

/// Elias died: temporal reset handled by the narrative module.
pub fn player_death(game: Res<Game>, mut crt: ResMut<CrimeRt>, mut popups: ResMut<crate::ui::screens::Popups>, mut done: Local<bool>) {
    if game.player.health <= 0.0 && !*done {
        *done = true;
        crt.dead_time = Some(0.0);
        popups.push(crate::ui::screens::Popup::death());
    }
    if game.player.health > 0.0 {
        *done = false;
    }
}

// ------------------------------------------------------------------ trespassing

/// Residents who find Elias inside their home (or a closed shop) react: they
/// demand he leaves, then scream, flee to the police or attack.
#[allow(clippy::too_many_arguments)]
pub fn trespass_system(
    time: Res<Time>,
    game: Res<Game>,
    map: Option<Res<CityMap>>,
    mut sim: ResMut<Sim>,
    rt: Res<PlayerRt>,
    mut crt: ResMut<CrimeRt>,
    mut crimes: EventWriter<CrimeEv>,
    db: Res<crate::cases::run::CaseDb>,
) {
    let Some(map) = map else { return };
    let m = &map.0;
    if game.phase != Phase::City || rt.in_car {
        crt.trespass = None;
        return;
    }
    let pp = game.player.pos;
    let Some(b) = m.building_at(pp) else {
        crt.trespass = None;
        crt.greeted = None;
        return;
    };
    let bl = &m.buildings[b];
    let crime_b = crate::cases::run::current(&game, &db).and_then(|(_, pi)| game.cases[pi].clue_pos.first().copied().flatten()).and_then(|(x, y)| m.building_at(Vec2::new(x, y)));
    let allowed = game.player.owned.contains(&b) || game.player.safehouse == Some(b) || Some(b) == crime_b || (bl.kind.public() && bl.kind.is_open(game.hour()));
    if allowed {
        crt.trespass = None;
        return;
    }
    if crt.trespass.map(|t| t.0) != Some(b) {
        crt.trespass = Some((b, 0.0, false));
    }
    let now = game.abs_minute();
    let dt = time.delta_secs();
    let hidden = rt.hidden_in.is_some();
    let mut spotted = false;
    let mut friendly_home = false;
    for a in sim.agents.iter_mut() {
        if !a.active() || m.building_at(a.pos) != Some(b) {
            continue;
        }
        let asleep = a.act == NAct::Sleep && a.arrived;
        let d = a.pos.distance(pp);
        let sees = !hidden && !asleep && d < 7.0 && m.line_clear(a.pos, pp, true);
        let hears = asleep && d < rt.noise * 0.6;
        if !sees && !hears {
            continue;
        }
        let p = game.pop.get(a.pid);
        let friend = p.elias.trust > 55 || matches!(p.elias.romance, Romance::Dating | Romance::Lovers | Romance::Engaged | Romance::Married);
        if friend {
            if crt.greeted != Some(b) {
                crt.greeted = Some(b);
                friend_home_say(a, p.elias.met);
            }
            friendly_home = true;
            continue;
        }
        if !matches!(a.state, AState::Normal | AState::Investigate { .. }) {
            continue;
        }
        spotted = true;
        if hears {
            a.state = AState::Investigate { at: pp, until: now + 8.0 };
            a.arrived = false;
            a.path.clear();
            if a.bubble.is_none() {
                a.say(pick(a.pid, &["Hã? Quem está aí?", "Tem alguém aí embaixo?", "Querido, você ouviu isso?", "Quem anda pela casa a essa hora?"]), 2.5);
            }
            continue;
        }
        if a.bubble.is_none() {
            let t = crt.trespass.map(|t| t.1).unwrap_or(0.0);
            let lines: &[&str] = if t < 2.5 {
                if p.elias.met {
                    &["Você de novo? Quem deixou você entrar?", "O que está fazendo aqui dentro? Saia!", "Eu não convidei você. Fora!"]
                } else {
                    &["Quem é você?! O que faz na minha casa?", "Ei! Saia daqui agora!", "Como você entrou aqui?!", "Fora da minha casa, seu ladrão!", "Não se aproxime! Eu vou gritar!"]
                }
            } else {
                &["Eu vou chamar a polícia!", "SOCORRO! Tem um homem aqui dentro!", "Última vez: SAIA!"]
            };
            a.say(pick(a.pid.wrapping_add(t as u32), lines), 2.5);
        }
        // face him
        a.state = AState::Investigate { at: pp, until: now + 4.0 };
    }
    if friendly_home && !spotted {
        return;
    }
    if let Some(t) = crt.trespass.as_mut() {
        if spotted {
            t.1 += dt;
        }
        if t.1 > 5.0 && !t.2 {
            t.2 = true;
            crimes.write(CrimeEv { kind: CrimeKind::BreakIn, pos: pp, victim: None, noise: 6.0, weapon: None });
        }
    }
}

fn friend_home_say(a: &mut crate::sim::agents::Agent, met: bool) {
    if a.bubble.is_none() && met {
        a.say(pick(a.pid, &["Elias? Entra, fica à vontade.", "Não esperava você hoje. Quer um café?", "Você podia ter batido, sabia?"]), 2.5);
    }
}

fn pick(seed: u32, lines: &[&'static str]) -> &'static str {
    lines[(seed as usize).wrapping_mul(2654435761) % lines.len()]
}
