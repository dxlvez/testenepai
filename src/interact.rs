//! What Elias can do with the world around him: talk, examine evidence,
//! search containers, loot bodies, drag and hide bodies, break into houses
//! through doors or windows, tie people up, clean crime scenes, sleep/save.

use crate::audio::Sfx;
use crate::cases::run::{current, CaseDb, CaseRt, ClueMarker};
use crate::city::map::*;
use crate::crime::{CrimeEv, Decal, DecalKind, SpawnDecal};
use crate::economy;
use crate::items::*;
use crate::keys::{Act, Action};
use crate::player::PlayerRt;
use crate::sim::agents::{AState, Sim};
use crate::sim::people::*;
use crate::state::*;
use crate::ui::dialogue::{start_dialogue, Dlg};
use crate::ui::hud::{Prompt, Toasts};
use crate::ui::screens::{Choice, Choices, Lockpick, Popup, PopStyle, Popups};
use crate::ui::{Mode, UiState};
use crate::world::CityMap;
use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Target {
    Npc(usize),
    Body(usize),
    Clue(u8),
    Container(usize),
    Door(usize, (i32, i32)),
    Window((i32, i32)),
    Bed(usize),
    NewsStand,
    Blood(Entity),
    Radio,
    Drop(Entity),
    Water,
    Car(u32),
}

#[derive(Resource, Default)]
pub struct Interact {
    pub target: Option<Target>,
    /// doors/windows opened by Elias: (x, y)
    pub opened: Vec<(i32, i32)>,
    pub searched: Vec<usize>,
}

/// An item lying on the ground (weapons dropped by the dead, etc.)
#[derive(Component)]
pub struct DropItem {
    pub item: Item,
    pub pos: Vec2,
}

fn key(b: &crate::keys::Bindings, a: Action) -> String {
    crate::keys::key_name(b.key(a))
}

/// Buildings Elias may freely walk into right now.
pub fn can_enter(m: &Map, game: &Game, b: usize, it: &Interact, (x, y): (i32, i32), sim: &Sim) -> bool {
    if game.phase != crate::state::Phase::City {
        return true;
    }
    let bl = &m.buildings[b];
    if it.opened.contains(&(x, y)) || game.player.owned.contains(&b) || game.player.safehouse == Some(b) {
        return true;
    }
    if bl.kind.public() && bl.kind.is_open(game.hour()) {
        return true;
    }
    if !bl.locked && bl.kind == BKind::Market {
        return true;
    }
    // invited: a resident who likes Elias is home
    bl.residents.iter().any(|r| {
        let p = game.pop.get(*r);
        (p.elias.trust > 55 || matches!(p.elias.romance, Romance::Dating | Romance::Lovers | Romance::Engaged | Romance::Married)) && sim.agent(*r).map(|a| m.building_at(a.pos) == Some(b)).unwrap_or(false)
    })
}

#[allow(clippy::too_many_arguments)]
pub fn find_target(
    game: Res<Game>,
    map: Option<Res<CityMap>>,
    sim: Res<Sim>,
    rt: Res<PlayerRt>,
    mut it: ResMut<Interact>,
    mut prompt: ResMut<Prompt>,
    clues: Query<(&ClueMarker, &Transform, &Visibility)>,
    decals: Query<(Entity, &Decal)>,
    drops: Query<(Entity, &DropItem)>,
    db: Res<CaseDb>,
    crt: Res<CaseRt>,
    binds: Res<crate::keys::Bindings>,
    cars: Res<crate::vehicles::Cars>,
) {
    let Some(map) = map else { return };
    if game.phase != crate::state::Phase::City {
        // the prologue / limbo have their own interactions
        it.target = None;
        return;
    }
    let m = &map.0;
    let pp = game.player.pos;
    let fwd = Vec2::new(rt.facing.cos(), rt.facing.sin());
    let mut best: Option<(f32, Target, String)> = None;
    let mut consider = |d: f32, t: Target, s: String| {
        if best.as_ref().map(|b| d < b.0).unwrap_or(true) {
            best = Some((d, t, s));
        }
    };
    let e = key(&binds, Action::Interact);
    if rt.in_car {
        it.target = None;
        prompt.0 = Some(format!("[{}] Sair do carro", e));
        return;
    }
    let score = |p: Vec2| {
        let d = p - pp;
        let l = d.length();
        l - d.normalize_or_zero().dot(fwd) * 0.4
    };
    for (i, a) in sim.agents.iter().enumerate() {
        let d = a.pos.distance(pp);
        if d > 1.8 || (a.in_car && matches!(a.state, AState::Dead)) {
            continue;
        }
        let p = game.pop.get(a.pid);
        match a.state {
            AState::Dead => consider(score(a.pos), Target::Body(i), format!("[{}] Revistar corpo   [{}] Arrastar", e, key(&binds, Action::Drag))),
            AState::Unconscious { .. } => consider(score(a.pos), Target::Body(i), format!("[{}] {} (desacordad{})   [{}] Carregar", e, p.first, p.o(), key(&binds, Action::Drag))),
            AState::Tied { .. } => consider(score(a.pos), Target::Npc(i), format!("[{}] {} (amarrad{})", e, p.first, p.o())),
            AState::Carried => {}
            _ => {
                let name = if p.elias.met || p.case_role.is_some() { p.name() } else { p.job.label(p.female).to_string() };
                consider(score(a.pos) - 0.4, Target::Npc(i), format!("[{}] Conversar com {}", e, name));
            }
        }
    }
    for (cm, tr, v) in clues.iter() {
        if *v == Visibility::Hidden {
            continue;
        }
        let p = Vec2::new(tr.translation.x, tr.translation.z);
        if p.distance(pp) < 1.6 {
            let name = current(&game, &db).map(|(d, _)| d.clues[cm.idx as usize].name).unwrap_or("algo");
            consider(score(p) - 0.3, Target::Clue(cm.idx), format!("[{}] Examinar: {}", e, name));
        }
    }
    for (ent, d) in decals.iter() {
        if d.pos.distance(pp) < 1.1 && matches!(d.kind, DecalKind::Blood | DecalKind::Casing | DecalKind::Glass) {
            let what = match d.kind {
                DecalKind::Blood => "Limpar o sangue",
                DecalKind::Casing => "Recolher a cápsula",
                _ => "Varrer os cacos",
            };
            consider(score(d.pos) + 0.6, Target::Blood(ent), format!("[{}] {}", e, what));
        }
    }
    for (ent, d) in drops.iter() {
        if d.pos.distance(pp) < 1.3 {
            consider(score(d.pos) - 0.2, Target::Drop(ent), format!("[{}] Pegar: {}", e, d.item.name(game.year)));
        }
    }
    for car in cars.list.iter() {
        if car.pos.distance(pp) < 2.2 {
            let label = match (car.horse, car.driver.is_some(), car.owner_elias || !car.locked) {
                (false, true, _) => "Tirar o motorista do carro",
                (false, false, true) => "Entrar no carro",
                (false, false, false) => "Carro trancado",
                (true, true, _) => "Tirar o cocheiro da carroça",
                (true, false, true) => "Subir na carroça",
                (true, false, false) => "Carroça amarrada",
            };
            consider(score(car.pos) + 0.3, Target::Car(car.id), format!("[{}] {}", e, label));
        }
    }
    let crime_b = current(&game, &db).and_then(|(_, pi)| game.cases[pi].clue_pos.first().copied().flatten()).and_then(|(x, y)| m.building_at(Vec2::new(x, y)));
    // tiles around: doors, windows, containers, beds
    let (tx, ty) = to_tile(pp);
    for yy in ty - 1..=ty + 1 {
        for xx in tx - 1..=tx + 1 {
            let c = tile_center(xx, yy);
            let dist = score(c);
            match m.get(xx, yy) {
                Tile::Door => {
                    if let Some(b) = m.building_at_tile(xx, yy) {
                        if Some(b) != crime_b && !can_enter(m, &game, b, &it, (xx, yy), &sim) {
                            consider(dist + 0.7, Target::Door(b, (xx, yy)), format!("[{}] Porta trancada — {}", e, if m.buildings[b].name.is_empty() { m.buildings[b].kind.label().to_string() } else { m.buildings[b].name.clone() }));
                        }
                    }
                }
                Tile::Window => {
                    if !it.opened.contains(&(xx, yy)) {
                        consider(dist + 0.9, Target::Window((xx, yy)), format!("[{}] Janela", e));
                    }
                }
                Tile::Water if rt.carrying.is_some() => consider(dist, Target::Water, format!("[{}] Jogar o corpo na água", key(&binds, Action::Hide))),
                _ => {}
            }
            if let Some(pi) = m.prop_at_tile(xx, yy) {
                let p = &m.props[pi];
                if rt.carrying.is_some() && p.kind.hides_body() {
                    consider(dist - 0.1, Target::Container(pi), format!("[{}] Esconder o corpo: {}", key(&binds, Action::Hide), p.kind.label()));
                } else if p.kind == PKind::Bed && p.building.is_some() && (game.player.safehouse == p.building || game.player.owned.contains(&p.building.unwrap())) {
                    consider(dist, Target::Bed(pi), format!("[{}] Dormir / salvar", e));
                } else if p.kind.container() && p.building.is_some() && !it.searched.contains(&pi) {
                    consider(dist + 0.2, Target::Container(pi), format!("[{}] Vasculhar: {}", e, p.kind.label()));
                } else if p.kind == PKind::NewsStand {
                    consider(dist, Target::NewsStand, format!("[{}] Comprar jornal", e));
                } else if p.kind == PKind::RadioSet {
                    consider(dist, Target::Radio, format!("[{}] Ouvir o rádio", e));
                }
            }
        }
    }
    let _ = crt;
    it.target = best.as_ref().map(|b| b.1);
    let extra = if crate::cases::run::current(&game, &db).is_some() && crt.echo_pos.iter().any(|p| p.distance(pp) < 3.0) && crt.echo.is_none() {
        format!("   [{}] Eco", key(&binds, Action::Echo))
    } else {
        String::new()
    };
    prompt.0 = best.map(|b| format!("{}{}", b.2, extra)).or(if extra.is_empty() { None } else { Some(extra.trim().to_string()) });
}

#[allow(clippy::too_many_arguments)]
pub fn do_interact(
    act: Res<Act>,
    mut it: ResMut<Interact>,
    mut game: ResMut<Game>,
    mut sim: ResMut<Sim>,
    map: Option<ResMut<CityMap>>,
    mut rt: ResMut<PlayerRt>,
    mut ui: ResMut<UiState>,
    mut dlg: ResMut<Dlg>,
    db: Res<CaseDb>,
    (mut toasts, mut popups, mut choices): (ResMut<Toasts>, ResMut<Popups>, ResMut<Choices>),
    mut sfx: EventWriter<Sfx>,
    mut crimes: EventWriter<CrimeEv>,
    mut c: Commands,
    (drops, decals): (Query<&DropItem>, Query<&Decal>),
    (mut decal_ev, mut lock): (EventWriter<SpawnDecal>, ResMut<Lockpick>),
) {
    let Some(mut map) = map else { return };
    if ui.blocks_input() || game.phase != crate::state::Phase::City {
        return;
    }
    let pp = game.player.pos;
    let year = game.year;
    // drop / carry bodies
    if act.just(Action::Drag) {
        if let Some(pid) = rt.carrying.take() {
            if let Some(a) = sim.agent_mut(pid) {
                a.state = if a.health > 0.0 && game.pop.get(pid).alive() { AState::Unconscious { until: 0.0 } } else { AState::Dead };
                if let AState::Unconscious { .. } = a.state {
                    a.state = AState::Unconscious { until: game.abs_minute() + 60.0 };
                }
                a.pos = map.0.nearest_open(a.pos);
            }
            rt.dragging = false;
            return;
        }
        if let Some(Target::Body(i)) = it.target {
            let pid = sim.agents[i].pid;
            let heavy = game.pop.get(pid).look.girth > 1.1;
            sim.agents[i].state = AState::Carried;
            rt.carrying = Some(pid);
            rt.dragging = heavy || game.pop.get(pid).look.height > 1.0;
            toasts.push(if rt.dragging { "Pesado demais para carregar: arrastando (deixa rastro)." } else { "Carregando nos ombros." });
            // moving a body is evidence tampering if seen
            crimes.write(CrimeEv { kind: CrimeKind::Assault, pos: pp, victim: Some(pid), noise: 2.0, weapon: None });
            return;
        }
    }
    if act.just(Action::Hide) && rt.carrying.is_some() {
        match it.target {
            Some(Target::Container(pi)) => {
                let pid = rt.carrying.take().unwrap();
                map.0.props[pi].bodies.push(pid);
                if let Some(a) = sim.agent_mut(pid) {
                    a.state = AState::Dead;
                    a.in_car = true; // hidden from view
                    a.pos = map.0.props[pi].center();
                }
                for cr in game.police.crimes.iter_mut().filter(|c| c.victim == Some(pid)) {
                    cr.body_hidden = true;
                }
                rt.dragging = false;
                toasts.push(format!("Corpo escondido: {}.", map.0.props[pi].kind.label()));
                game.stat("hidden_bodies", 1);
                sfx.write(Sfx::Door);
            }
            Some(Target::Water) => {
                let pid = rt.carrying.take().unwrap();
                if let Some(a) = sim.agent_mut(pid) {
                    a.state = AState::Dead;
                    a.in_car = true;
                    a.pos = Vec2::new(-100.0, -100.0);
                }
                for cr in game.police.crimes.iter_mut().filter(|c| c.victim == Some(pid)) {
                    cr.body_hidden = true;
                }
                rt.dragging = false;
                toasts.push("O rio leva o corpo. Talvez devolva um dia.");
                game.stat("hidden_bodies", 1);
            }
            _ => {}
        }
        return;
    }
    if !act.just(Action::Interact) {
        return;
    }
    let Some(t) = it.target else { return };
    match t {
        Target::Npc(i) => {
            let pid = sim.agents[i].pid;
            if let AState::Tied { gagged, .. } = sim.agents[i].state {
                let name = game.pop.get(pid).first.clone();
                choices.cur = Some(Choice {
                    title: format!("{} está amarrad{}", name, game.pop.get(pid).o()),
                    body: "Os olhos acompanham cada movimento seu.".into(),
                    opts: vec![
                        ("tied_talk".into(), "Interrogar".into()),
                        ("tied_gag".into(), if gagged { "Tirar a mordaça".into() } else { "Amordaçar".into() }),
                        ("tied_take".into(), "Soltar as pernas e levar junto (refém)".into()),
                        ("tied_free".into(), "Desamarrar e deixar ir".into()),
                        ("tied_chloro".into(), "Apagar com clorofórmio".into()),
                        ("cancel".into(), "Deixar assim".into()),
                    ],
                    ctx: format!("{}", pid),
                });
                ui.open(Mode::Choice);
                return;
            }
            start_dialogue(&mut dlg, &mut ui, &mut sim, &mut game, pid);
        }
        Target::Body(i) => {
            let pid = sim.agents[i].pid;
            let dead = sim.agents[i].state == AState::Dead;
            let mut found = Vec::new();
            let money = game.pop.get(pid).money;
            if money > 0 {
                found.push(economy::money_str(&game, money));
            }
            game.pop.get_mut(pid).money = 0;
            game.player.money += money;
            // armed people carry their weapon and ammo
            let job = game.pop.get(pid).job;
            let key = format!("looted:{}", pid);
            if job.armed() && !game.flag(&key) {
                let w = match job {
                    Job::Police | Job::Detective => {
                        if year >= 1980 {
                            Weapon::Automatic
                        } else if year >= 1935 {
                            Weapon::Magnum
                        } else {
                            Weapon::Revolver
                        }
                    }
                    Job::Gangster => {
                        if (1921..1950).contains(&year) && pid % 3 == 0 {
                            Weapon::Tommy
                        } else {
                            Weapon::Pistol
                        }
                    }
                    Job::Gunsmith => Weapon::Shotgun,
                    _ => Weapon::Revolver,
                };
                if !game.player.inv.contains(&Item::Weapon(w)) {
                    game.player.inv.push(Item::Weapon(w));
                    found.push(w.stats().name.to_string());
                }
                let ammo = sim.agents[i].ammo.max(0);
                if ammo > 0 {
                    game.player.add_ammo(ammo);
                    found.push(format!("{} balas", ammo));
                    sim.agents[i].ammo = 0;
                }
            }
            if !game.flag(&key) && (pid % 4 == 0) {
                let v = Item::Valuable(if pid % 8 == 0 { "Relógio de bolso".into() } else { "Aliança de ouro".into() }, 15 + (pid % 30) as i32);
                found.push(v.name(year));
                game.player.inv.push(v);
            }
            game.set(&key);
            toasts.push(if found.is_empty() { "Nada de valor.".to_string() } else { format!("Encontrado: {}", found.join(", ")) });
            if !dead {
                choices.cur = Some(Choice {
                    title: "Pessoa desacordada".into(),
                    body: String::new(),
                    opts: vec![("ko_tie".into(), "Amarrar (corda)".into()), ("cancel".into(), "Deixar".into())],
                    ctx: format!("{}", pid),
                });
                ui.open(Mode::Choice);
            }
            sfx.write(Sfx::Paper);
        }
        Target::Clue(idx) => {
            if let Some((def, pi)) = current(&game, &db) {
                let cl = &def.clues[idx as usize];
                let new = !game.cases[pi].found.contains(&idx);
                if new {
                    game.cases[pi].found.push(idx);
                    game.player.train(Skill::Observation, 6);
                    game.player.train(Skill::Forensics, 3);
                    sfx.write(Sfx::Evidence);
                }
                let forensic = if game.player.skill(Skill::Forensics) >= 2 { cl.forensic.map(|f| format!("\n\nPERÍCIA: {}", f)).unwrap_or_default() } else if cl.forensic.is_some() { "\n\n(Com mais Perícia você perceberia algo mais.)".into() } else { String::new() };
                let n = game.cases[pi].found.iter().position(|c| *c == idx).unwrap_or(0) + 1;
                popups.push(Popup::new(PopStyle::Evidence, cl.name, format!("EVIDÊNCIA #{:03} · {}", n, cl.kind.label()), format!("{}{}", cl.desc, forensic)));
                if matches!(cl.look, crate::cases::defs::Look3d::Weapon | crate::cases::defs::Look3d::Paper) {
                    game.player.inv.push(Item::Evidence(def.id, idx));
                }
            }
        }
        Target::Container(pi) => {
            let prop = map.0.props[pi].clone();
            it.searched.push(pi);
            let Some(b) = prop.building else {
                // street bins and crates belong to nobody
                toasts.push(if prop.bodies.is_empty() { "Só lixo molhado." } else { "Há um corpo aqui dentro." });
                return;
            };
            let bl = &map.0.buildings[b];
            let wealth = map.0.districts.get(bl.district).map(|d| d.wealth).unwrap_or(0.5);
            let mut r = crate::util::Rng::new(pi as u64 * 31 + game.timeline as u64);
            let mut items = economy::loot(&mut r, bl.kind, prop.kind, wealth, year);
            if prop.kind == PKind::Safe && !game.player.has(&Item::Tool(Tool::Crowbar)) && game.player.skill(Skill::Stealth) < 3 {
                toasts.push("Cofre trancado. Precisa de um pé de cabra.");
                it.searched.retain(|x| *x != pi);
                return;
            }
            if !prop.bodies.is_empty() {
                toasts.push("Há um corpo aqui dentro.");
            }
            let own = game.player.owned.contains(&b) || game.player.safehouse == Some(b);
            if items.is_empty() {
                toasts.push("Nada que valha a pena.");
            } else {
                let names: Vec<String> = items.iter().map(|i| i.name(year)).collect();
                toasts.push(format!("Pegou: {}", names.join(", ")));
                game.player.inv.append(&mut items);
                sfx.write(Sfx::Paper);
                if !own {
                    crimes.write(CrimeEv { kind: CrimeKind::Theft, pos: pp, victim: bl.owner.or(bl.residents.first().copied()), noise: 2.5, weapon: None });
                    game.stat("thefts", 1);
                }
            }
        }
        Target::Door(b, pos) => {
            let has_pick = game.player.has(&Item::Tool(Tool::Lockpick));
            let mut opts = vec![];
            if has_pick {
                opts.push(("door_pick".into(), "Usar as gazuas (silencioso)".into()));
            }
            opts.push(("door_kick".into(), "Arrombar com o ombro (barulho)".into()));
            if game.player.has(&Item::Tool(Tool::Crowbar)) {
                opts.push(("door_bar".into(), "Forçar com pé de cabra".into()));
            }
            opts.push(("knock".into(), "Bater na porta".into()));
            opts.push(("cancel".into(), "Deixar para lá".into()));
            let bl = &map.0.buildings[b];
            let home = bl.residents.iter().filter(|r| sim.agent(**r).map(|a| map.0.building_at(a.pos) == Some(b)).unwrap_or(false)).count();
            choices.cur = Some(Choice {
                title: if bl.name.is_empty() { format!("{} — {}", bl.kind.label(), bl.address) } else { bl.name.clone() },
                body: if home > 0 { format!("Há luz lá dentro. {} pessoa(s) em casa.", home) } else { "Nenhum som lá dentro.".into() },
                opts,
                ctx: format!("{},{},{}", b, pos.0, pos.1),
            });
            ui.open(Mode::Choice);
        }
        Target::Window(pos) => {
            let mut opts = vec![("win_break".into(), "Quebrar o vidro (muito barulho)".into())];
            if game.player.has(&Item::Tool(Tool::Crowbar)) {
                opts.insert(0, ("win_force".into(), "Forçar com pé de cabra (pouco barulho)".into()));
            }
            if game.player.has(&Item::Tool(Tool::Lockpick)) {
                opts.insert(0, ("win_pick".into(), "Destravar o trinco com as gazuas".into()));
            }
            opts.push(("peek".into(), "Espiar lá dentro".into()));
            opts.push(("cancel".into(), "Deixar para lá".into()));
            choices.cur = Some(Choice { title: "Janela".into(), body: String::new(), opts, ctx: format!("{},{}", pos.0, pos.1) });
            ui.open(Mode::Choice);
        }
        Target::Bed(_) => {
            let mut opts: Vec<(String, String)> = vec![
                ("sleep8".into(), "Dormir até de manhã (salva o jogo)".into()),
                ("sleep2".into(), "Cochilar 2 horas".into()),
                ("wait_night".into(), "Esperar anoitecer".into()),
                ("save".into(), "Salvar sem dormir".into()),
            ];
            if crate::narrative::case_closed(&game) {
                opts.insert(0, ("sphere_again".into(), "Fechar os olhos e ouvir a esfera".into()));
            }
            opts.push(("cancel".into(), "Voltar".into()));
            choices.cur = Some(Choice { title: "Seu quarto".into(), body: "A chuva bate no vidro. Aqui ninguém te procura.".into(), opts, ctx: String::new() });
            ui.open(Mode::Choice);
        }
        Target::NewsStand => {
            let pr = economy::price(&game, 1);
            if game.player.money >= pr {
                game.player.money -= pr;
                let h = game.papers.iter().rev().find(|h| h.city == game.city).cloned();
                let (title, body) = match h {
                    Some(h) => (h.title, h.body),
                    None => (crate::narrative::filler_headline(&game), "O resto do jornal fala do tempo, de preços e de um baile de caridade.".into()),
                };
                popups.push(Popup::new(PopStyle::Headline, title, format!("{} · {}", game.city.upper(), game.date_str()), body));
                sfx.write(Sfx::Paper);
            }
        }
        Target::Radio => {
            let line = game.news_ticker.last().cloned().unwrap_or_else(|| crate::narrative::radio_line(&game));
            popups.push(Popup::new(PopStyle::Mystery, "...", "RÁDIO", line));
            sfx.write(Sfx::Static);
        }
        Target::Blood(e) => {
            if let Ok(d) = decals.get(e) {
                let kind = d.kind;
                c.entity(e).despawn();
                rt.action_lock = 1.2;
                rt.pose = crate::render::character::Pose::Pray;
                if kind == DecalKind::Blood {
                    // blood removed from crimes nearby
                    for cr in game.police.crimes.iter_mut() {
                        if cr.pos.distance(d.pos) < 5.0 {
                            cr.blood = false;
                        }
                    }
                }
                game.player.train(Skill::Stealth, 1);
                toasts.push(match kind {
                    DecalKind::Blood => "Sangue limpo.",
                    DecalKind::Casing => "Cápsula recolhida.",
                    _ => "Cacos varridos.",
                });
            }
        }
        Target::Drop(e) => {
            if let Ok(d) = drops.get(e) {
                match &d.item {
                    Item::Ammo(n) => game.player.add_ammo(*n),
                    other => game.player.inv.push(other.clone()),
                }
                toasts.push(format!("Pegou {}.", d.item.name(year)));
                c.entity(e).despawn();
            }
        }
        Target::Water => {}
        Target::Car(id) => {
            crate::vehicles::car_interact(id, &mut game, &mut sim, &mut rt, &mut choices, &mut ui, &mut toasts, &mut crimes, &mut sfx);
        }
    }
    let _ = (&mut decal_ev, &mut lock);
}

/// Handle choices picked in Choice menus.
#[allow(clippy::too_many_arguments)]
pub fn handle_choices(
    mut choices: ResMut<Choices>,
    mut ui: ResMut<UiState>,
    mut game: ResMut<Game>,
    mut sim: ResMut<Sim>,
    map: Option<Res<CityMap>>,
    mut it: ResMut<Interact>,
    mut toasts: ResMut<Toasts>,
    mut crimes: EventWriter<CrimeEv>,
    mut decal_ev: EventWriter<SpawnDecal>,
    mut sfx: EventWriter<Sfx>,
    mut lock: ResMut<Lockpick>,
    mut rt: ResMut<PlayerRt>,
    mut save_req: ResMut<crate::save::SaveReq>,
    mut dlg: ResMut<Dlg>,
    mut popups: ResMut<Popups>,
) {
    let Some((id, ctx)) = choices.picked.take() else { return };
    let Some(map) = map else { return };
    let m = &map.0;
    let pp = game.player.pos;
    let year = game.year;
    let stealth = game.player.skill(Skill::Stealth) as f32;
    let open_tile = |it: &mut Interact, x: i32, y: i32| {
        if !it.opened.contains(&(x, y)) {
            it.opened.push((x, y));
        }
    };
    match id.as_str() {
        "cancel" => {}
        "door_pick" | "win_pick" => {
            let parts: Vec<i32> = ctx.split(',').filter_map(|s| s.parse().ok()).collect();
            *lock = Lockpick {
                active: true,
                pins: if year < 1950 { 3 } else { 4 },
                done: 0,
                pos: 0.0,
                dir: 1.0,
                zone: (0.4, 0.55),
                fails: 0,
                ctx: if id == "door_pick" { format!("door:{},{}", parts.get(1).unwrap_or(&0), parts.get(2).unwrap_or(&0)) } else { format!("door:{},{}", parts.first().unwrap_or(&0), parts.get(1).unwrap_or(&0)) },
                result: None,
                era_label: if year < 1950 { "Fechadura de ferro".into() } else { "Fechadura tambor".into() },
            };
            ui.open(Mode::Minigame);
            return;
        }
        "door_kick" | "door_bar" | "win_break" | "win_force" => {
            let parts: Vec<i32> = ctx.split(',').filter_map(|s| s.parse().ok()).collect();
            let (x, y) = if id.starts_with("door") { (parts[1], parts[2]) } else { (parts[0], parts[1]) };
            let noise = match id.as_str() {
                "door_kick" => 14.0,
                "win_break" => 16.0,
                "door_bar" => 7.0 - stealth * 0.4,
                _ => 5.0 - stealth * 0.4,
            };
            open_tile(&mut it, x, y);
            if id == "win_break" {
                decal_ev.write(SpawnDecal { kind: DecalKind::Glass, pos: tile_center(x, y), crime: None });
                sfx.write(Sfx::Glass);
            } else {
                sfx.write(Sfx::Door);
            }
            let b = m.building_at_tile(x, y);
            let victim = b.and_then(|b| m.buildings[b].residents.first().copied());
            crimes.write(CrimeEv { kind: CrimeKind::BreakIn, pos: tile_center(x, y), victim, noise, weapon: None });
            toasts.push(if id.starts_with("win") { "A janela está aberta. Dá para passar.".to_string() } else { "A porta cedeu.".to_string() });
            game.stat("breakins", 1);
        }
        "knock" => {
            let parts: Vec<usize> = ctx.split(',').filter_map(|s| s.parse().ok()).collect();
            let b = parts[0];
            let home: Vec<Pid> = m.buildings[b].residents.iter().copied().filter(|r| sim.agent(*r).map(|a| m.building_at(a.pos) == Some(b) && a.active()).unwrap_or(false)).collect();
            sfx.write(Sfx::Door);
            if let Some(&r) = home.first() {
                let p = game.pop.get(r);
                let welcome = p.elias.trust > 25 || matches!(p.elias.romance, Romance::Dating | Romance::Lovers | Romance::Engaged | Romance::Married);
                if welcome {
                    open_tile(&mut it, parts[1] as i32, parts[2] as i32);
                    toasts.push(format!("{} abre a porta: \"Entra, está chovendo.\"", p.first));
                } else if let Some(a) = sim.agent_mut(r) {
                    let door = tile_center(parts[1] as i32, parts[2] as i32);
                    a.pos = m.nearest_open(door + (m.buildings[b].center_px() - door).normalize_or_zero() * 1.2);
                    a.state = AState::Normal;
                    a.say("Quem é? O que quer a essa hora?", 3.0);
                    start_dialogue(&mut dlg, &mut ui, &mut sim, &mut game, r);
                    return;
                }
            } else {
                toasts.push("Ninguém atende.");
            }
        }
        "peek" => {
            let parts: Vec<i32> = ctx.split(',').filter_map(|s| s.parse().ok()).collect();
            if let Some(b) = m.building_at_tile(parts[0], parts[1]) {
                let inside: Vec<String> = sim.agents.iter().filter(|a| m.building_at(a.pos) == Some(b) && a.active()).map(|a| {
                    let p = game.pop.get(a.pid);
                    format!("{} ({})", if p.elias.met { p.first.clone() } else { p.job.label(p.female).to_string() }, a.act.label())
                }).collect();
                toasts.push(if inside.is_empty() { "Vazio. Móveis no escuro.".to_string() } else { format!("Lá dentro: {}", inside.join(", ")) });
            }
        }
        "sleep8" | "sleep2" | "wait_night" | "save" => {
            let hours = match id.as_str() {
                "sleep8" => {
                    let h = game.hour();
                    if h < 7.0 { 7.0 - h } else { 31.0 - h }
                }
                "sleep2" => 2.0,
                "wait_night" => {
                    let h = game.hour();
                    if h < 20.0 { 20.0 - h } else { 0.0 }
                }
                _ => 0.0,
            };
            advance_time(&mut game, &mut sim, m, hours);
            if hours >= 6.0 {
                game.player.health = (game.player.health + 40.0).min(100.0);
                game.player.stamina = 1.0;
                game.player.sleep = 0.0;
                game.fatigue = (game.fatigue - 5.0).max(0.0);
                // dreams
                if let Some(d) = crate::narrative::dream(&game) {
                    popups.push(Popup::new(PopStyle::Mystery, "", "SONHO", d));
                }
            }
            save_req.save = Some(0);
            toasts.push("Jogo salvo.");
        }
        "tied_talk" => {
            let pid: Pid = ctx.parse().unwrap_or(0);
            game.pop.get_mut(pid).elias.fear += 10;
            start_dialogue(&mut dlg, &mut ui, &mut sim, &mut game, pid);
            return;
        }
        "tied_gag" => {
            let pid: Pid = ctx.parse().unwrap_or(0);
            if let Some(a) = sim.agent_mut(pid) {
                if let AState::Tied { since, gagged } = a.state {
                    a.state = AState::Tied { since, gagged: !gagged };
                    a.bubble = None;
                }
            }
        }
        "tied_take" => {
            let pid: Pid = ctx.parse().unwrap_or(0);
            if rt.weapon_out && !game.player.weapon.stats().melee || game.player.weapons().iter().any(|w| *w != Weapon::Fists) {
                if let Some(a) = sim.agent_mut(pid) {
                    a.state = AState::Hostage;
                    a.y = 0.0;
                }
                rt.weapon_out = true;
                if game.player.weapon == Weapon::Fists {
                    game.player.weapon = game.player.weapons()[1];
                }
                toasts.push("Refém caminhando com você. Se guardar a arma, pode fugir.");
                crimes.write(CrimeEv { kind: CrimeKind::Kidnap, pos: pp, victim: Some(pid), noise: 3.0, weapon: Some(game.player.weapon) });
            } else {
                toasts.push("Sem uma arma, ninguém vai te obedecer.");
            }
        }
        "tied_free" => {
            let pid: Pid = ctx.parse().unwrap_or(0);
            if let Some(a) = sim.agent_mut(pid) {
                a.state = AState::Flee { from: pp, until: game.abs_minute() + 30.0 };
                a.y = 0.0;
                a.path.clear();
                a.say("...Obrigad... não, espera. Você é louco!", 3.0);
                // freed people report the kidnapping
                a.knows_bodies.push(u32::MAX - game.police.crimes.iter().rev().find(|c| c.victim == Some(pid)).map(|c| c.id).unwrap_or(0));
            }
        }
        "tied_chloro" => {
            let pid: Pid = ctx.parse().unwrap_or(0);
            if game.player.take(&Item::Drug(Drug::Chloroform)) {
                if let Some(a) = sim.agent_mut(pid) {
                    a.state = AState::Unconscious { until: game.abs_minute() + 240.0 };
                }
                toasts.push("Apagou. Por umas quatro horas.");
            } else {
                toasts.push("Você não tem clorofórmio.");
            }
        }
        "ko_tie" => {
            let pid: Pid = ctx.parse().unwrap_or(0);
            if let Some(a) = sim.agent_mut(pid) {
                a.state = AState::Tied { since: game.abs_minute(), gagged: false };
            }
            toasts.push("Amarrad@. A corda aperta.".replace('@', game.pop.get(pid).o()));
            crimes.write(CrimeEv { kind: CrimeKind::Kidnap, pos: pp, victim: Some(pid), noise: 2.0, weapon: None });
        }
        other => {
            if crate::vehicles::handle_car_choice(other, &ctx, &mut game, &mut sim, &mut rt, &mut toasts, &mut crimes, &mut sfx, &mut decal_ev, &mut lock, &mut ui) {
                return;
            }
        }
    }
    if ui.mode == Mode::Choice {
        ui.close();
    }
}

/// Skip time: people teleport to where they should be.
pub fn advance_time(game: &mut Game, sim: &mut Sim, m: &Map, hours: f32) {
    if hours <= 0.0 {
        return;
    }
    game.minute += hours * 60.0;
    while game.minute >= 1440.0 {
        game.minute -= 1440.0;
        game.day += 1;
    }
    let now = game.abs_minute();
    let mut rng = crate::util::Rng::new(now as u64);
    for a in sim.agents.iter_mut() {
        if a.state == AState::Normal {
            let p = game.pop.get(a.pid);
            let (act, b) = crate::sim::agents::plan(p, &game.pop, m, game.hour(), game.day, game.rain, game.year);
            if let Some(b) = b {
                let (pos, pose) = crate::sim::agents::spot_for(p, m, b, act, &mut rng);
                a.pos = pos;
                a.pose = pose;
                a.act = act;
                a.target_b = Some(b);
                a.target = pos;
                a.arrived = true;
                a.path.clear();
                a.y = if pose == crate::render::character::Pose::Lie { 0.45 } else { 0.0 };
            }
            a.replan_at = now + 5.0;
        }
    }
}

/// Lockpicking minigame logic.
pub fn lockpick_system(time: Res<Time>, mut lock: ResMut<Lockpick>, keys: Res<ButtonInput<KeyCode>>, mut ui: ResMut<UiState>, mut it: ResMut<Interact>, mut sfx: EventWriter<Sfx>, mut game: ResMut<Game>, mut crimes: EventWriter<CrimeEv>, mut toasts: ResMut<Toasts>, binds: Res<crate::keys::Bindings>) {
    if ui.mode != Mode::Minigame || !lock.active {
        return;
    }
    let dt = time.delta_secs();
    let speed = 0.9 + lock.done as f32 * 0.25 - game.player.skill(Skill::Stealth) as f32 * 0.05;
    lock.pos += lock.dir * dt * speed;
    if lock.pos > 1.0 {
        lock.pos = 1.0;
        lock.dir = -1.0;
    }
    if lock.pos < 0.0 {
        lock.pos = 0.0;
        lock.dir = 1.0;
    }
    ui.dirty = true;
    if keys.just_pressed(KeyCode::Escape) {
        lock.active = false;
        ui.close();
        return;
    }
    if keys.just_pressed(binds.key(Action::Interact)) || keys.just_pressed(KeyCode::Space) {
        if lock.pos >= lock.zone.0 && lock.pos <= lock.zone.1 {
            lock.done += 1;
            sfx.write(Sfx::Lockpick);
            let h = crate::util::hashf(lock.done as i32, game.day, 4);
            let w = 0.15 - lock.done as f32 * 0.02 + game.player.skill(Skill::Stealth) as f32 * 0.01;
            let s = 0.1 + h * (0.8 - w);
            lock.zone = (s, s + w.max(0.06));
            if lock.done >= lock.pins {
                lock.active = false;
                if let Some(rest) = lock.ctx.strip_prefix("door:") {
                    let p: Vec<i32> = rest.split(',').filter_map(|s| s.parse().ok()).collect();
                    if p.len() == 2 {
                        it.opened.push((p[0], p[1]));
                        crimes.write(CrimeEv { kind: CrimeKind::BreakIn, pos: tile_center(p[0], p[1]), victim: None, noise: 1.0, weapon: None });
                    }
                } else if let Some(rest) = lock.ctx.strip_prefix("car:") {
                    if let Ok(id) = rest.parse::<u32>() {
                        crate::vehicles::unlock_car(id);
                    }
                }
                game.player.train(Skill::Stealth, 5);
                toasts.push("Clique. Aberto.");
                ui.close();
            }
        } else {
            lock.fails += 1;
            sfx.write(Sfx::Click);
            if lock.fails >= 3 {
                lock.active = false;
                toasts.push("A gazua escorregou com um estalo alto.");
                let pos = game.player.pos;
                crimes.write(CrimeEv { kind: CrimeKind::BreakIn, pos, victim: None, noise: 9.0, weapon: None });
                ui.close();
            }
        }
    }
}
