//! Chases. Police hunting Elias (last seen position, footprints, search,
//! reinforcements, surrender, escape) and suspects fleeing from Elias.

use crate::audio::Sfx;
use crate::city::map::*;
use crate::keys::{Act, Action};
use crate::player::PlayerRt;
use crate::sim::agents::{AState, Agent, Sim};
use crate::sim::people::*;
use crate::state::*;
use crate::ui::hud::Toasts;
use crate::ui::UiState;
use crate::world::CityMap;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Hunt {
    /// where the police last saw Elias
    pub last_seen: Option<Vec2>,
    pub unseen_for: f32,
    /// footprints Elias leaves while running / in rain / on mud
    pub trail: Vec<(Vec2, f32)>,
    pub trail_t: f32,
    pub reinforce_t: f32,
    pub active: bool,
    /// suspect fleeing from Elias
    pub fugitive: Option<Pid>,
    pub fugitive_lost: f32,
    pub surrendered: bool,
}

#[derive(Component)]
pub struct Reinforcement;

#[allow(clippy::too_many_arguments)]
pub fn police_hunt(
    time: Res<Time>,
    mut hunt: ResMut<Hunt>,
    mut game: ResMut<Game>,
    mut sim: ResMut<Sim>,
    map: Option<Res<CityMap>>,
    mut rt: ResMut<PlayerRt>,
    act: Res<Act>,
    mut toasts: ResMut<Toasts>,
    mut sfx: EventWriter<Sfx>,
    env: Res<crate::env::EnvState>,
    ui: Res<UiState>,
    mut crt: ResMut<crate::crime::CrimeRt>,
    mut spawn_req: ResMut<crate::world::SpawnAgents>,
) {
    let Some(map) = map else { return };
    if ui.pauses_world() {
        return;
    }
    let m = &map.0;
    let dt = time.delta_secs().min(0.1);
    let now = game.abs_minute();
    let pp = game.player.pos;
    let chasers: Vec<usize> = sim.agents.iter().enumerate().filter(|(_, a)| a.state == AState::Chase).map(|(i, _)| i).collect();
    hunt.active = !chasers.is_empty() || game.police.wanted >= 2 && hunt.last_seen.is_some();

    // footprints: running, wet streets, mud and blood leave a trail
    hunt.trail_t -= dt;
    let (tx, ty) = to_tile(pp);
    let soft = matches!(m.get(tx, ty), Tile::Dirt | Tile::Grass | Tile::Field | Tile::Sand);
    if hunt.trail_t <= 0.0 && rt.moving > 0.0 && (rt.running || game.rain > 0.4 || soft) && rt.hidden_in.is_none() && !rt.in_car {
        hunt.trail_t = 0.35;
        hunt.trail.push((pp, now));
        if hunt.trail.len() > 200 {
            hunt.trail.remove(0);
        }
    }
    hunt.trail.retain(|(_, t)| now - *t < 90.0);

    // surrender
    if act.just(Action::Surrender) && !chasers.is_empty() {
        rt.weapon_out = false;
        rt.pose = crate::render::character::Pose::Cower;
        rt.action_lock = 2.0;
        hunt.surrendered = true;
        crt.arrest_pending = true;
        toasts.push("Você levanta as mãos.");
        return;
    }

    // can any chaser see Elias?
    let hidden = rt.hidden_in.is_some();
    let mut seen = false;
    for &i in &chasers {
        let a = &sim.agents[i];
        let d = a.pos.distance(pp);
        let sight = 4.0 + 14.0 * env.light_level_player.max(0.25) * if rt.sneaking { 0.6 } else { 1.0 };
        if !hidden && d < sight && m.line_clear(a.pos, pp, true) {
            seen = true;
        }
    }
    if seen {
        hunt.last_seen = Some(pp);
        hunt.unseen_for = 0.0;
    } else if !chasers.is_empty() {
        hunt.unseen_for += dt;
    }
    // chasers who lost sight go to the last seen spot, then follow the trail, then search
    for &i in &chasers {
        let a = &mut sim.agents[i];
        let d = a.pos.distance(pp);
        let can_see = !hidden && d < 18.0 && m.line_clear(a.pos, pp, true);
        if can_see {
            continue; // crime::hostile_ai moves them straight at Elias
        }
        a.state = AState::Investigate { at: hunt.last_seen.unwrap_or(pp), until: now + 12.0 };
        a.path.clear();
    }
    // investigators near the trail follow the freshest footprint
    for a in sim.agents.iter_mut() {
        if let AState::Investigate { at, until } = a.state {
            if !matches!(game.pop.get(a.pid).job, Job::Police | Job::Detective) || game.police.wanted == 0 {
                continue;
            }
            if a.pos.distance(at) < 1.5 {
                // look for the next footprint ahead
                let next = hunt.trail.iter().filter(|(p, _)| p.distance(a.pos) < 7.0).max_by(|x, y| x.1.partial_cmp(&y.1).unwrap()).map(|(p, _)| *p);
                if let Some(n) = next {
                    if n.distance(at) > 0.5 {
                        a.state = AState::Investigate { at: n, until: until.max(now + 6.0) };
                        a.path.clear();
                        if a.bubble.is_none() {
                            a.say("Pegadas... ele foi por aqui.", 2.0);
                        }
                    }
                } else if a.bubble.is_none() {
                    let lines = ["Procurem nos becos!", "Ele não pode ter ido longe.", "Olhem dentro das caçambas!", "Cerquem o quarteirão!"];
                    a.say(lines[(a.pid as usize + game.day as usize) % lines.len()], 2.5);
                }
            }
            // spot Elias again
            let d = a.pos.distance(pp);
            if !hidden && d < 10.0 * env.light_level_player.max(0.3) && m.line_clear(a.pos, pp, true) {
                a.state = AState::Chase;
                a.say("Lá está ele!", 2.0);
                hunt.last_seen = Some(pp);
                hunt.unseen_for = 0.0;
            }
            // hiding spot check: an officer searching right next to Elias' container may find him
            if let Some(pi) = rt.hidden_in {
                let c = m.props[pi].center();
                if a.pos.distance(c) < 1.3 && crate::util::hashf(a.pid as i32, (now * 3.0) as i32, 9) < dt * 0.4 {
                    rt.hidden_in = None;
                    a.state = AState::Chase;
                    a.say("Achei você!", 2.0);
                    sfx.write(Sfx::Door);
                }
            }
        }
    }
    // reinforcements for serious crimes
    hunt.reinforce_t -= dt;
    if game.police.wanted >= 2 && !chasers.is_empty() && hunt.reinforce_t <= 0.0 {
        hunt.reinforce_t = 40.0;
        let n = game.police.wanted as usize;
        let at = hunt.last_seen.unwrap_or(pp);
        spawn_req.police.push((n, at));
        sfx.write(if game.year < 1960 { Sfx::Whistle } else { Sfx::Siren });
        toasts.push(if game.year < 1935 { "Apitos por toda parte. Mais guardas chegando." } else { "Sirenes. Reforços chegando." });
    }
    // escape
    if !chasers.is_empty() && hunt.unseen_for > 35.0 {
        for &i in &chasers {
            sim.agents[i].state = AState::Normal;
            sim.agents[i].replan_at = now;
        }
        hunt.unseen_for = 0.0;
        hunt.last_seen = None;
        game.police.heat *= 0.5;
        toasts.push("Você despistou a polícia.");
        game.stat("escapes", 1);
        game.player.train(Skill::Stealth, 10);
    }
    if chasers.is_empty() && !sim.agents.iter().any(|a| matches!(a.state, AState::Investigate { .. }) && matches!(game.pop.get(a.pid).job, Job::Police)) {
        hunt.last_seen = None;
    }
}

/// Hiding inside wardrobes, dumpsters, under beds, in hay.
pub fn hide_self(act: Res<Act>, mut rt: ResMut<PlayerRt>, game: Res<Game>, map: Option<Res<CityMap>>, mut toasts: ResMut<Toasts>, ui: Res<UiState>, mut sfx: EventWriter<Sfx>) {
    let Some(map) = map else { return };
    if ui.blocks_input() || rt.carrying.is_some() || !act.just(Action::Hide) {
        return;
    }
    if rt.hidden_in.is_some() {
        rt.hidden_in = None;
        toasts.push("Você sai do esconderijo.");
        sfx.write(Sfx::Door);
        return;
    }
    let m = &map.0;
    let (tx, ty) = to_tile(game.player.pos);
    for yy in ty - 1..=ty + 1 {
        for xx in tx - 1..=tx + 1 {
            if let Some(pi) = m.prop_at_tile(xx, yy) {
                if m.props[pi].kind.hides_body() {
                    rt.hidden_in = Some(pi);
                    rt.weapon_out = false;
                    toasts.push(format!("Escondido: {}. Prenda a respiração.", m.props[pi].kind.label()));
                    sfx.write(Sfx::Door);
                    return;
                }
            }
        }
    }
}

/// A suspect running from Elias across the city.
#[allow(clippy::too_many_arguments)]
pub fn fugitive_system(
    time: Res<Time>,
    mut hunt: ResMut<Hunt>,
    mut game: ResMut<Game>,
    mut sim: ResMut<Sim>,
    map: Option<Res<CityMap>>,
    act: Res<Act>,
    mut toasts: ResMut<Toasts>,
    mut dlg: ResMut<crate::ui::dialogue::Dlg>,
    mut ui: ResMut<UiState>,
    ui_ro: Res<crate::keys::Bindings>,
    db: Res<crate::cases::run::CaseDb>,
) {
    let Some(map) = map else { return };
    let m = &map.0;
    let dt = time.delta_secs().min(0.1);
    // a dialogue started a chase
    if let Some(pid) = dlg.start_pursuit.take() {
        let hideout = m.find_building(BKind::Abandoned).or_else(|| m.find_building(BKind::Warehouse)).unwrap_or(0);
        if let Some(a) = sim.agent_mut(pid) {
            a.state = AState::Escape { hideout };
            a.path.clear();
            a.say("Me deixa em paz!", 2.0);
        }
        hunt.fugitive = Some(pid);
        hunt.fugitive_lost = 0.0;
        toasts.big("PERSEGUIÇÃO", format!("{} está fugindo!", game.pop.get(pid).name()));
        let _ = &ui_ro;
    }
    let Some(pid) = hunt.fugitive else { return };
    if ui.pauses_world() {
        return;
    }
    let pp = game.player.pos;
    let now = game.abs_minute();
    let Some(&idx) = sim.by_pid.get(&pid) else {
        hunt.fugitive = None;
        return;
    };
    let AState::Escape { hideout } = sim.agents[idx].state else {
        hunt.fugitive = None;
        return;
    };
    let target = m.buildings[hideout].center_px();
    let mut pather = sim.pather.take().unwrap_or_else(|| crate::sim::path::Pather::new(m));
    {
        let a: &mut Agent = &mut sim.agents[idx];
        if a.path.is_empty() || a.pi >= a.path.len() {
            // run through markets and alleys: a detour point first
            let detour = m.nearest_open(a.pos + (a.pos - pp).normalize_or_zero() * 10.0);
            let goal = if a.pos.distance(target) > 12.0 && crate::util::hashf(a.pid as i32, now as i32, 2) < 0.5 { detour } else { target };
            a.path = pather.find(m, a.pos, goal, 20000).unwrap_or_default();
            a.pi = 0;
            // hide inside the hideout once reached
            if a.pos.distance(target) < 2.0 {
                a.state = AState::Normal;
                a.pinned = Some(a.pos);
                hunt.fugitive = None;
                toasts.push(format!("Você perdeu {} de vista. Alguém deve saber onde se esconde.", game.pop.get(pid).first));
                game.set(&format!("hideout:{}:{}", pid, hideout));
                sim.pather = Some(pather);
                return;
            }
        }
        let tired = (now - a.replan_at).max(0.0) * 0.0;
        let _ = tired;
        crate::sim::update::step_path(a, m, dt, 3.9);
        a.pose = crate::render::character::Pose::Run;
    }
    sim.pather = Some(pather);
    let a = &sim.agents[idx];
    let d = a.pos.distance(pp);
    if d < 1.4 && (act.just(Action::Interact) || act.just(Action::Attack)) {
        // tackled
        let apos = a.pos;
        let a = &mut sim.agents[idx];
        a.state = AState::Normal;
        a.pinned = Some(apos);
        hunt.fugitive = None;
        let ci = game.case_idx;
        game.set(&format!("caught:{}", ci));
        toasts.big("PEGO!", "");
        crate::ui::dialogue::start_dialogue(&mut dlg, &mut ui, &mut sim, &mut game, pid);
        let _ = db;
        return;
    }
    if d > 30.0 || !m.line_clear(a.pos, pp, true) && d > 14.0 {
        hunt.fugitive_lost += dt;
        if hunt.fugitive_lost > 12.0 {
            hunt.fugitive = None;
            toasts.push(format!("{} sumiu nas ruas. Talvez volte ao esconderijo.", game.pop.get(pid).first));
            game.set(&format!("hideout:{}:{}", pid, hideout));
        }
    } else {
        hunt.fugitive_lost = 0.0;
    }
}
