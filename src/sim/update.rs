//! Per-frame simulation of all citizens.

use super::agents::*;
use super::lines;
use super::people::*;
use crate::city::map::*;
use crate::render::character::Pose;
use crate::state::Game;
use crate::world::CityMap;
use bevy::prelude::*;

pub const WALK: f32 = 1.25;
pub const RUN: f32 = 3.4;

/// Events other systems read: someone arrived at the police to report,
/// a person noticed a body, etc.
#[derive(Resource, Default)]
pub struct SimEvents {
    pub reports: Vec<(Pid, u32)>,
    pub bodies_found: Vec<(Pid, Pid)>,
    pub chats: Vec<(Pid, Pid)>,
}

pub fn sim_update(time: Res<Time>, mut game: ResMut<Game>, map: Option<Res<CityMap>>, mut sim: ResMut<Sim>, mut ev: ResMut<SimEvents>, ui: Res<crate::ui::UiState>) {
    let Some(map) = map else { return };
    if ui.pauses_world() {
        return;
    }
    let m = &map.0;
    let dt = time.delta_secs().min(0.1);
    let now = game.abs_minute();
    let hour = game.hour();
    let day = game.day;
    let year = game.year;
    let rain = game.rain;
    let player = game.player.pos;
    sim.tick += 1;
    let tick = sim.tick;
    let mut pather = sim.pather.take().unwrap_or_else(|| super::path::Pather::new(m));
    let mut rng = sim.rng.take().unwrap_or_else(|| crate::util::Rng::new(7));
    let police_station = m.find_building(BKind::Police);
    let n = sim.agents.len();
    let mut new_chats: Vec<(usize, usize)> = Vec::new();

    for i in 0..n {
        // --- timers
        {
            let a = &mut sim.agents[i];
            if let Some((_, t)) = &mut a.bubble {
                *t -= dt;
                if *t <= 0.0 {
                    a.bubble = None;
                }
            }
            a.shoot_cd = (a.shoot_cd - dt).max(0.0);
            a.greet_cd = (a.greet_cd - dt).max(0.0);
            a.alert = (a.alert - dt * 0.02).max(0.0);
            a.drunk = (a.drunk - dt * 0.002).max(0.0);
        }
        let state = sim.agents[i].state;
        let pid = sim.agents[i].pid;
        match state {
            AState::Dead | AState::Carried | AState::Arrested => {
                sim.agents[i].speed = 0.0;
                continue;
            }
            AState::Unconscious { until } => {
                let a = &mut sim.agents[i];
                a.speed = 0.0;
                a.pose = Pose::Lie;
                if now >= until {
                    a.state = AState::Normal;
                    a.alert = 1.0;
                    a.replan_at = now;
                    a.say("Minha cabeça... o que aconteceu?", 4.0);
                    let (y, d) = (year, day);
                    game.pop.get_mut(pid).remember(y, d, "Acordei no chão sem lembrar de nada.", None, 5);
                }
                continue;
            }
            AState::Talking => {
                let a = &mut sim.agents[i];
                a.speed = 0.0;
                let d = player - a.pos;
                if d.length() > 0.01 {
                    a.facing = d.y.atan2(d.x);
                }
                if a.pose != Pose::Sit && a.pose != Pose::Lie {
                    a.pose = Pose::Talk;
                }
                continue;
            }
            AState::Cower { until } => {
                let a = &mut sim.agents[i];
                a.speed = 0.0;
                a.pose = Pose::Cower;
                if now >= until {
                    a.state = AState::Normal;
                    a.replan_at = now;
                }
                continue;
            }
            AState::Flee { from, until } => {
                if now >= until {
                    sim.agents[i].state = AState::Normal;
                    sim.agents[i].replan_at = now;
                } else {
                    let a = &mut sim.agents[i];
                    if a.path.is_empty() || a.pi >= a.path.len() {
                        let away = (a.pos - from).normalize_or_zero();
                        let dir = if away == Vec2::ZERO { Vec2::X } else { away };
                        let tgt = m.nearest_open(a.pos + dir * 14.0 + vec2(rng.rangef(-4.0, 4.0), rng.rangef(-4.0, 4.0)));
                        a.path = pather.find(m, a.pos, tgt, 6000).unwrap_or_default();
                        a.pi = 0;
                    }
                    step_path(&mut sim.agents[i], m, dt, RUN * 0.9);
                    sim.agents[i].pose = Pose::Run;
                }
                continue;
            }
            AState::Report { crime } => {
                let Some(ps) = police_station else {
                    sim.agents[i].state = AState::Normal;
                    continue;
                };
                let door = m.buildings[ps].outside_px();
                let a = &mut sim.agents[i];
                if a.pos.distance(door) < 1.2 {
                    ev.reports.push((pid, crime));
                    a.state = AState::Normal;
                    a.replan_at = now;
                    a.say("Eu vi tudo! Eu vi quem foi!", 3.0);
                    continue;
                }
                if a.path.is_empty() || a.pi >= a.path.len() {
                    a.path = pather.find(m, a.pos, door, 20000).unwrap_or_default();
                    a.pi = 0;
                    if a.path.is_empty() {
                        // cannot reach: gives up
                        a.state = AState::Normal;
                        continue;
                    }
                }
                step_path(&mut sim.agents[i], m, dt, WALK * 1.6);
                sim.agents[i].pose = Pose::Walk;
                continue;
            }
            AState::Investigate { at, until } => {
                let a = &mut sim.agents[i];
                if now >= until {
                    a.state = AState::Normal;
                    a.replan_at = now;
                    continue;
                }
                if a.pos.distance(at) > 1.3 {
                    if a.path.is_empty() || a.pi >= a.path.len() {
                        a.path = pather.find(m, a.pos, m.nearest_open(at), 6000).unwrap_or_default();
                        a.pi = 0;
                    }
                    step_path(&mut sim.agents[i], m, dt, WALK * 1.2);
                    sim.agents[i].pose = Pose::Walk;
                } else {
                    a.speed = 0.0;
                    a.pose = Pose::Idle;
                }
                continue;
            }
            AState::Follow { until } => {
                let a = &mut sim.agents[i];
                if now >= until {
                    a.state = AState::Normal;
                    a.replan_at = now;
                    continue;
                }
                let d = a.pos.distance(player);
                if d > 2.0 {
                    let dir = (player - a.pos).normalize_or_zero();
                    let sp = if d > 5.0 { RUN * 0.8 } else { WALK * 1.2 };
                    let np = a.pos + dir * sp * dt;
                    a.pos = m.collide(np, 0.25);
                    a.facing = dir.y.atan2(dir.x);
                    a.speed = sp;
                    a.pose = if sp > 2.0 { Pose::Run } else { Pose::Walk };
                    if d > 30.0 {
                        a.pos = m.nearest_open(player + vec2(1.0, 1.0));
                    }
                } else {
                    a.speed = 0.0;
                    a.pose = Pose::Idle;
                }
                continue;
            }
            AState::Hostile | AState::Chase | AState::Escape { .. } | AState::Hostage => {
                // driven by crime/pursuit systems
                continue;
            }
            AState::Tied { gagged, .. } => {
                let a = &mut sim.agents[i];
                a.speed = 0.0;
                a.pose = Pose::Sit;
                a.y = -0.35;
                if !gagged && a.bubble.is_none() && rng.chance(dt * 0.05) {
                    let p = game.pop.get(pid);
                    let l = super::speech::plead(p, &game.pop, super::speech::Situation::Tied, &mut rng, year);
                    sim.agents[i].say(l, 4.0);
                }
                continue;
            }
            AState::HandsUp { until } => {
                let a = &mut sim.agents[i];
                a.speed = 0.0;
                a.pose = Pose::Cower;
                if now >= until {
                    a.state = AState::Flee { from: player, until: now + 30.0 };
                    a.path.clear();
                }
                continue;
            }
            AState::Normal => {}
        }

        // pinned by a case script (e.g. a body, a witness waiting)
        if let Some(p) = sim.agents[i].pinned {
            let a = &mut sim.agents[i];
            a.pos = p;
            a.speed = 0.0;
            continue;
        }

        // --- chatting
        if let Some((other, until)) = sim.agents[i].chat {
            if now >= until || sim.agent(other).map(|o| !o.active()).unwrap_or(true) {
                sim.agents[i].chat = None;
            } else {
                let op = sim.agent(other).map(|o| o.pos).unwrap_or(sim.agents[i].pos);
                let a = &mut sim.agents[i];
                let d = op - a.pos;
                a.facing = d.y.atan2(d.x);
                a.speed = 0.0;
                if a.pose != Pose::Sit {
                    a.pose = Pose::Talk;
                }
                // alternate lines
                let slot = ((now * 0.35) as u32 + a.pid) % 2 == 0;
                if a.bubble.is_none() && slot && rng.chance(dt * 0.6) {
                    let rumor = sim.rumors.iter().filter(|r| r.known.contains(&pid)).max_by_key(|r| r.heat).map(|r| r.text.clone());
                    let sp = game.pop.get(pid);
                    let ot = game.pop.get(other);
                    // the one who spoke last answers; others start a new topic
                    let heard = sim.agent(other).and_then(|o| o.bubble.clone()).is_some();
                    let line = if heard && rng.chance(0.6) { super::speech::reply(sp, &mut rng) } else { super::speech::chatter(sp, ot, &game.pop, rumor.as_deref(), year, &mut rng) };
                    let _ = lines::friend_lines;
                    sim.agents[i].say(line, 4.5);
                    // gossip spreads
                    let ids: Vec<usize> = sim.rumors.iter().enumerate().filter(|(_, r)| r.known.contains(&pid) && !r.known.contains(&other)).map(|(k, _)| k).collect();
                    for k in ids {
                        sim.rumors[k].known.push(other);
                    }
                }
                continue;
            }
        }

        // --- schedule
        if now >= sim.agents[i].replan_at {
            let p = game.pop.get(pid);
            let (act, b) = plan(p, &game.pop, m, hour, day, rain, year);
            let a_act = sim.agents[i].act;
            let a_b = sim.agents[i].target_b;
            let changed = act != a_act || b != a_b || !sim.agents[i].arrived && sim.agents[i].path.is_empty();
            if changed || matches!(act, Act::Stroll | Act::Patrol | Act::Play) {
                let (target, pose) = match b {
                    Some(bi) if bi < m.buildings.len() => spot_for(p, m, bi, act, &mut rng),
                    _ => {
                        let base = match act {
                            Act::Play => p.home.map(|h| m.buildings[h].outside_px()).unwrap_or(sim.agents[i].pos),
                            Act::Patrol => police_station.map(|s| m.buildings[s].outside_px()).unwrap_or(sim.agents[i].pos),
                            _ => sim.agents[i].pos,
                        };
                        (stroll_target(m, base, &mut rng), Pose::Idle)
                    }
                };
                let a = &mut sim.agents[i];
                a.act = act;
                a.target_b = b;
                a.target = target;
                a.arrived = false;
                a.pose = Pose::Walk;
                let path = pather.find(m, a.pos, target, 25000);
                match path {
                    Some(p) => {
                        a.path = p;
                        a.pi = 0;
                    }
                    None => {
                        // unreachable: teleport when nobody is looking (far from player)
                        if a.pos.distance(player) > 25.0 && target.distance(player) > 25.0 {
                            a.pos = target;
                        }
                        a.path.clear();
                    }
                }
                sim.agents[i].replan_at = now + 20.0 + rng.rangef(0.0, 25.0);
                sim.agents[i].chat_line = pose as u8;
            } else {
                sim.agents[i].replan_at = now + 15.0 + rng.rangef(0.0, 15.0);
            }
        }

        // --- moving
        let a = &mut sim.agents[i];
        if !a.arrived {
            let speed = match a.act {
                Act::Stroll | Act::Play => WALK * 0.8,
                Act::Patrol => WALK * 0.9,
                _ => WALK,
            } * if rain > 0.5 { 1.25 } else { 1.0 };
            if a.pi >= a.path.len() {
                a.arrived = true;
                a.speed = 0.0;
                a.pos = if a.target.distance(a.pos) < 1.5 { a.target } else { a.pos };
                // arrival pose
                let pose = pose_for(a.act, a.target_b, m, a.target);
                a.pose = pose;
                if pose == Pose::Lie {
                    a.y = 0.45;
                }
            } else {
                a.y = 0.0;
                step_path(a, m, dt, speed);
                a.pose = if a.speed > 2.0 { Pose::Run } else { Pose::Walk };
                // stuck detection
                if a.pos.distance(a.last_pos) < 0.01 {
                    a.stuck += dt;
                    if a.stuck > 2.5 {
                        a.stuck = 0.0;
                        a.pi += 1;
                        if a.pi >= a.path.len() {
                            a.pos = m.nearest_open(a.target);
                        }
                    }
                } else {
                    a.stuck = 0.0;
                }
                a.last_pos = a.pos;
            }
        } else if matches!(a.act, Act::Stroll | Act::Play | Act::Patrol) && rng.chance(dt * 0.05) {
            a.replan_at = now;
        }

        // --- look for someone to chat with
        if (tick + i as u64) % 30 == 0 {
            let a = &sim.agents[i];
            let can_chat = a.chat.is_none()
                && a.bubble.is_none()
                && matches!(a.act, Act::Stroll | Act::Drink | Act::Party | Act::Visit(_) | Act::Shop | Act::Eat | Act::Idle | Act::Home | Act::Church)
                && game.pop.get(pid).age(year) >= 8;
            if can_chat {
                let soc = game.pop.get(pid).traits.sociability as f32 / 100.0;
                if rng.chance(0.15 + soc * 0.3) {
                    let pos = a.pos;
                    let mut best: Option<usize> = None;
                    for j in 0..n {
                        if j == i {
                            continue;
                        }
                        let b = &sim.agents[j];
                        if b.chat.is_some() || !b.active() || b.state != AState::Normal || b.act == Act::Sleep {
                            continue;
                        }
                        if b.pos.distance_squared(pos) < 3.2 && m.line_clear(pos, b.pos, true) {
                            best = Some(j);
                            break;
                        }
                    }
                    if let Some(j) = best {
                        new_chats.push((i, j));
                    }
                }
            }
        }
    }

    for (i, j) in new_chats {
        if sim.agents[i].chat.is_some() || sim.agents[j].chat.is_some() {
            continue;
        }
        let dur = now + rng.rangef(3.0, 9.0);
        let (pi, pj) = (sim.agents[i].pid, sim.agents[j].pid);
        sim.agents[i].chat = Some((pj, dur));
        sim.agents[j].chat = Some((pi, dur));
        ev.chats.push((pi, pj));
        // acquaintances grow
        if game.pop.get(pi).rel_to(pj).is_none() && rng.chance(0.2) {
            game.pop.relate(pi, pj, RelKind::Friend, 15);
            game.pop.relate(pj, pi, RelKind::Friend, 15);
        }
    }
    sim.pather = Some(pather);
    sim.rng = Some(rng);
}

fn pose_for(act: Act, b: Option<usize>, m: &Map, target: Vec2) -> Pose {
    match act {
        Act::Sleep => Pose::Lie,
        Act::Church | Act::Mourn => Pose::Pray,
        Act::Party => {
            if (target.x as i32 + target.y as i32) % 3 == 0 {
                Pose::Idle
            } else {
                Pose::Dance
            }
        }
        _ => {
            if let Some(bi) = b {
                if let Some(s) = m.buildings[bi].spots.iter().find(|s| s.pos.distance_squared(target) < 0.05) {
                    return match s.kind {
                        SpotKind::Seat => Pose::Sit,
                        SpotKind::Pray => Pose::Pray,
                        SpotKind::Stage => Pose::Play,
                        SpotKind::Bed => Pose::Lie,
                        SpotKind::Work => {
                            if act == Act::Work {
                                Pose::Work
                            } else {
                                Pose::Idle
                            }
                        }
                        _ => Pose::Idle,
                    };
                }
            }
            Pose::Idle
        }
    }
}

pub fn step_path(a: &mut Agent, m: &Map, dt: f32, speed: f32) {
    if a.pi >= a.path.len() {
        a.speed = 0.0;
        return;
    }
    let tgt = a.path[a.pi];
    let d = tgt - a.pos;
    let dist = d.length();
    let step = speed * dt;
    if dist <= step.max(0.05) {
        a.pos = tgt;
        a.pi += 1;
    } else {
        let dir = d / dist;
        a.pos += dir * step;
        a.facing = dir.y.atan2(dir.x);
    }
    let _ = m;
    a.speed = speed;
}
