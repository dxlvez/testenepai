//! Builds the current modal screen and routes keyboard/mouse input for
//! every screen except dialogue (handled in dialogue.rs).

use super::board::{build_board, BoardSel};
use super::dialogue::{build as build_dialogue, Dlg};
use super::hud::Toasts;
use super::screens::*;
use super::*;
use crate::audio::Sfx;
use crate::cases::run::{current, CaseDb};
use crate::economy::price;
use crate::items::*;
use crate::keys::{Act, Action, Bindings, Settings};
use crate::narrative::Flow;
use crate::state::Game;

#[derive(Resource, Default)]
pub struct OverlayMem {
    pub last_mode: Option<Mode>,
    pub blink_t: f32,
    pub saves: Vec<crate::save::SaveMeta>,
    pub secret: bool,
}

#[allow(clippy::too_many_arguments)]
pub fn overlay_system(
    mut c: Commands,
    mut ui: ResMut<UiState>,
    fonts: Res<UiFonts>,
    roots: Query<Entity, With<OverlayRoot>>,
    game: Res<Game>,
    db: Res<CaseDb>,
    dlg: Res<Dlg>,
    popups: Res<Popups>,
    choices: Res<Choices>,
    lock: Res<Lockpick>,
    shop: Res<Shop>,
    (settings, binds, sel, mapimg, map): (Res<Settings>, Res<Bindings>, Res<BoardSel>, Res<MapImage>, Option<Res<crate::world::CityMap>>),
    mut mem: ResMut<OverlayMem>,
    time: Res<Time>,
) {
    mem.blink_t += time.delta_secs();
    if ui.mode == Mode::Title && mem.blink_t > 0.6 {
        mem.blink_t = 0.0;
        ui.dirty = true;
    }
    if mem.last_mode != Some(ui.mode) {
        mem.last_mode = Some(ui.mode);
        ui.dirty = true;
        if matches!(ui.mode, Mode::Title | Mode::Pause) {
            mem.saves = crate::save::list();
            mem.secret = crate::save::global_flag("break_thread");
        }
    }
    if !ui.dirty {
        return;
    }
    ui.dirty = false;
    for e in roots.iter() {
        c.entity(e).despawn();
    }
    let el = match ui.mode {
        Mode::None | Mode::Movie => return,
        Mode::Title => {
            if ui.sub == 0 {
                super::menu::build_title(&mem.saves, (time.elapsed_secs() * 1.6) as i32 % 2 == 0, mem.secret)
            } else {
                super::pause::build_pause(&ui, &settings, &binds, &mem.saves, true)
            }
        }
        Mode::Pause => super::pause::build_pause(&ui, &settings, &binds, &mem.saves, false),
        Mode::Dialogue => build_dialogue(&dlg, &game, &db),
        Mode::Board => build_board(&game, &db, &ui, &sel),
        Mode::Journal => build_journal(&game, &ui, &db),
        Mode::Map => match &map {
            Some(m) => build_map(&game, &mapimg, &m.0, &db),
            None => modal(panel(vec![t("Sem mapa aqui.")])),
        },
        Mode::Inventory => build_bag(&game, &ui),
        Mode::Network => build_network(&game, &ui),
        Mode::Shop => build_shop(&shop, &game, &ui),
        Mode::Cutscene => match popups.q.front() {
            Some(p) => build_popup(p),
            None => return,
        },
        Mode::Choice => match &choices.cur {
            Some(ch) => build_choice(ch),
            None => return,
        },
        Mode::Minigame => build_lockpick(&lock),
    };
    c.spawn((Node { position_type: PositionType::Absolute, width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() }, OverlayRoot, GlobalZIndex(20)))
        .with_children(|p| spawn_el(p, &el, &fonts));
}

/// Open screens from the world.
#[allow(clippy::too_many_arguments)]
pub fn ui_toggles(
    act: Res<Act>,
    mut ui: ResMut<UiState>,
    popups: Res<Popups>,
    game: Res<Game>,
    mut sfx: EventWriter<Sfx>,
    mut sel: ResMut<BoardSel>,
    mut mapimg: ResMut<MapImage>,
    map: Option<Res<crate::world::CityMap>>,
    mut images: ResMut<Assets<Image>>,
) {
    if ui.mode == Mode::None && !popups.q.is_empty() {
        ui.open(Mode::Cutscene);
        return;
    }
    let toggle = |ui: &mut UiState, m: Mode, sfx: &mut EventWriter<Sfx>| {
        if ui.mode == m {
            ui.close();
        } else if ui.mode == Mode::None {
            ui.open(m);
            sfx.write(Sfx::Paper);
        }
    };
    if act.just(Action::Pause) {
        match ui.mode {
            Mode::None => {
                ui.open(Mode::Pause);
                ui.message = None;
            }
            Mode::Pause => {
                if ui.sub == 0 {
                    ui.close();
                } else if ui.rebinding.is_none() {
                    ui.sub = 0;
                    ui.dirty = true;
                }
            }
            Mode::Board | Mode::Journal | Mode::Map | Mode::Inventory | Mode::Network => ui.close(),
            _ => {}
        }
        return;
    }
    if game.phase == crate::state::Phase::City || ui.mode != Mode::None {
        if act.just(Action::CaseBoard) {
            sel.accusing = false;
            toggle(&mut ui, Mode::Board, &mut sfx);
        }
        if act.just(Action::Map) {
            if let Some(m) = &map {
                let key = (game.city, game.year);
                if mapimg.for_city != Some(key) || mapimg.w != m.0.w as u32 {
                    let (h, w, hh) = build_map_image(&m.0, &mut images);
                    mapimg.handle = Some(h);
                    mapimg.w = w;
                    mapimg.h = hh;
                    mapimg.for_city = Some(key);
                }
            }
            toggle(&mut ui, Mode::Map, &mut sfx);
        }
    }
    if act.just(Action::Journal) {
        toggle(&mut ui, Mode::Journal, &mut sfx);
    }
    if act.just(Action::Inventory) {
        toggle(&mut ui, Mode::Inventory, &mut sfx);
    }
    if act.just(Action::Network) {
        toggle(&mut ui, Mode::Network, &mut sfx);
    }
}

/// Popups (cards, headlines, intros, TIMELINE ALTERED).
pub fn popup_system(time: Res<Time>, mut ui: ResMut<UiState>, mut popups: ResMut<Popups>, keys: Res<ButtonInput<KeyCode>>, mut flow: ResMut<Flow>, mut sfx: EventWriter<Sfx>, mut choices: ResMut<Choices>) {
    if ui.mode != Mode::Cutscene {
        return;
    }
    let dt = time.delta_secs();
    let clicked = ui.take_click().map(|c| c == "popup_next").unwrap_or(false);
    let next = keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::KeyE) || clicked || keys.just_pressed(KeyCode::Escape);
    let Some(p) = popups.q.front_mut() else {
        ui.close();
        return;
    };
    let total = p.body.chars().count() as f32;
    if p.reveal < total {
        let before = p.reveal as i32;
        p.reveal = (p.reveal + dt * 55.0).min(total);
        if (p.reveal as i32) / 4 != before / 4 {
            sfx.write(Sfx::Type);
        }
        ui.dirty = true;
        if next {
            p.reveal = total;
        }
        return;
    }
    if next {
        popups.q.pop_front();
        if popups.q.is_empty() {
            ui.close();
            let acts: Vec<String> = popups.on_close.drain(..).collect();
            for a in acts {
                if a == "choice_pending" {
                    if choices.cur.is_some() {
                        ui.open(Mode::Choice);
                    }
                } else {
                    flow.actions.push(a);
                }
            }
        } else {
            ui.dirty = true;
        }
    }
}

/// Choice menus: keyboard numbers & clicks; story choices are handled here,
/// world choices by interact::handle_choices.
#[allow(clippy::too_many_arguments)]
pub fn choice_input(mut ui: ResMut<UiState>, mut choices: ResMut<Choices>, keys: Res<ButtonInput<KeyCode>>, mut flow: ResMut<Flow>, mut game: ResMut<Game>, mut sfx: EventWriter<Sfx>, mut app_exit: EventWriter<AppExit>) {
    if ui.mode != Mode::Choice {
        return;
    }
    let Some(ch) = choices.cur.clone() else {
        ui.close();
        return;
    };
    let mut pick: Option<String> = ui.take_click().and_then(|c| c.strip_prefix("c:").map(|s| s.to_string()));
    let digits = [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4, KeyCode::Digit5, KeyCode::Digit6, KeyCode::Digit7, KeyCode::Digit8, KeyCode::Digit9];
    for (i, k) in digits.iter().enumerate() {
        if keys.just_pressed(*k) {
            if let Some(o) = ch.opts.get(i) {
                pick = Some(o.0.clone());
            }
        }
    }
    if keys.just_pressed(KeyCode::Escape) && ch.opts.iter().any(|o| o.0 == "cancel") {
        pick = Some("cancel".into());
    }
    let Some(p) = pick else { return };
    sfx.write(Sfx::Click);
    choices.cur = None;
    let story = match p.as_str() {
        "follow_thread" => {
            flow.actions.push("limbo".into());
            true
        }
        "linger" => {
            game.set("linger");
            true
        }
        "sphere_again" => {
            choices.cur = Some(crate::narrative::sphere_calls());
            ui.open(Mode::Choice);
            return;
        }
        "again_yes" => {
            flow.actions.push("new_game".into());
            true
        }
        "again_no" => {
            flow.actions.push("title".into());
            true
        }
        "quit_game" => {
            app_exit.write(AppExit::Success);
            true
        }
        _ => {
            if let Some(y) = p.strip_prefix("stay:") {
                flow.actions.push(format!("stay:{}", y));
                true
            } else if let Some(e) = p.strip_prefix("ending:") {
                flow.actions.push(format!("ending:{}", e));
                true
            } else {
                false
            }
        }
    };
    if story {
        ui.close();
    } else {
        choices.picked = Some((p, ch.ctx.clone()));
        // world handler closes the menu
    }
}

#[allow(clippy::too_many_arguments)]
pub fn pause_input(
    mut ui: ResMut<UiState>,
    mut settings: ResMut<Settings>,
    mut binds: ResMut<Bindings>,
    keys: Res<ButtonInput<KeyCode>>,
    mut save: ResMut<crate::save::SaveReq>,
    mut flow: ResMut<Flow>,
    mut app_exit: EventWriter<AppExit>,
    mut windows: Query<&mut Window>,
    mut mem: ResMut<OverlayMem>,
) {
    if !matches!(ui.mode, Mode::Pause | Mode::Title) {
        return;
    }
    // waiting for a key to rebind
    if let Some(a) = ui.rebinding {
        if let Some(k) = keys.get_just_pressed().next() {
            if *k != KeyCode::Escape || a == Action::Pause {
                binds.map.insert(a, *k);
                settings.bindings = binds.clone();
                settings.save();
            }
            ui.rebinding = None;
            ui.dirty = true;
        }
        return;
    }
    let Some(id) = ui.take_click() else { return };
    let mut changed_settings = false;
    match id.as_str() {
        "resume" => ui.close(),
        "to_save" => {
            ui.sub = 1;
            mem.saves = crate::save::list();
        }
        "to_load" | "t_load" => {
            ui.sub = 2;
            mem.saves = crate::save::list();
        }
        "to_settings" | "t_settings" => ui.sub = 3,
        "to_controls" | "t_controls" => ui.sub = 4,
        "back" => ui.sub = 0,
        "to_title" => {
            save.save = Some(0);
            flow.actions.push("title".into());
        }
        "quit" | "t_quit" => {
            app_exit.write(AppExit::Success);
        }
        "t_new" => {
            ui.close();
            flow.actions.push("new_game".into());
        }
        "t_continue" => {
            let latest = mem.saves.iter().max_by(|a, b| a.played_secs.partial_cmp(&b.played_secs).unwrap()).map(|m| m.slot).unwrap_or(0);
            save.load = Some(latest);
            ui.close();
        }
        "t_break" => {
            ui.close();
            flow.actions.push("new_game".into());
            flow.actions.push("objective:Desta vez, você sabe o que há no laboratório. Você não precisa entrar.".into());
        }
        "binds_reset" => {
            binds.reset();
            settings.bindings = binds.clone();
            changed_settings = true;
        }
        s if s.starts_with("save:") => {
            let slot: u32 = s[5..].parse().unwrap_or(1);
            save.save = Some(slot);
            ui.message = Some(format!("Salvo no arquivo {:03}.", slot));
            ui.sub = 0;
        }
        s if s.starts_with("load:") => {
            let slot: u32 = s[5..].parse().unwrap_or(1);
            save.load = Some(slot);
            ui.close();
            return;
        }
        s if s.starts_with("bind:") => {
            let i: usize = s[5..].parse().unwrap_or(0);
            ui.rebinding = Action::ALL.get(i).copied();
        }
        "master-" | "master+" | "music-" | "music+" | "sfx-" | "sfx+" | "cam-" | "cam+" => {
            let d = if id.ends_with('+') { 0.1 } else { -0.1 };
            match &id[..id.len() - 1] {
                "master" => settings.master = (settings.master + d).clamp(0.0, 1.0),
                "music" => settings.music = (settings.music + d).clamp(0.0, 1.0),
                "sfx" => settings.sfx = (settings.sfx + d).clamp(0.0, 1.0),
                _ => settings.cam_speed = (settings.cam_speed + d * 2.0).clamp(0.4, 2.0),
            }
            changed_settings = true;
        }
        "fullscreen" => {
            settings.fullscreen = !settings.fullscreen;
            if let Ok(mut w) = windows.single_mut() {
                w.mode = if settings.fullscreen { bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current) } else { bevy::window::WindowMode::Windowed };
            }
            changed_settings = true;
        }
        "shadows" => {
            settings.shadows = !settings.shadows;
            changed_settings = true;
        }
        "fog" => {
            settings.fog = !settings.fog;
            changed_settings = true;
        }
        "grain" => {
            settings.grain = !settings.grain;
            changed_settings = true;
        }
        "quality" => {
            settings.quality = if settings.quality >= 1 { 0 } else { 1 };
            changed_settings = true;
        }
        _ => {}
    }
    if changed_settings {
        settings.save();
    }
    ui.dirty = true;
}

#[allow(clippy::too_many_arguments)]
pub fn board_input(
    mut ui: ResMut<UiState>,
    mut game: ResMut<Game>,
    db: Res<CaseDb>,
    mut sel: ResMut<BoardSel>,
    mut popups: ResMut<Popups>,
    mut choices: ResMut<Choices>,
    mut sfx: EventWriter<Sfx>,
    mut toasts: ResMut<Toasts>,
) {
    if ui.mode != Mode::Board {
        return;
    }
    let Some(id) = ui.take_click() else { return };
    let Some((_, pi)) = current(&game, &db) else { return };
    let (k, v) = id.split_once(':').unwrap_or((id.as_str(), ""));
    let n: u8 = v.parse().unwrap_or(0);
    match k {
        "tab" => ui.tab = n as usize,
        "clue" => {
            sel.clue = if sel.clue == Some(n) { None } else { Some(n) };
            sfx.write(Sfx::Paper);
        }
        "person" => {
            if let Some(c) = sel.clue.take() {
                let prog = &mut game.cases[pi];
                if let Some(i) = prog.links.iter().position(|l| *l == (c, n)) {
                    prog.links.remove(i);
                } else {
                    prog.links.push((c, n));
                    sfx.write(Sfx::Click);
                }
            }
        }
        "acc" => game.cases[pi].accused = Some(n),
        "meth" => game.cases[pi].method = Some(n),
        "mot" => game.cases[pi].motive = Some(n),
        "anom" => game.cases[pi].sphere = Some(n),
        "accuse_final" => {
            if game.cases[pi].accused.is_none() {
                toasts.push("Escolha quem acusar.");
            } else {
                ui.close();
                sel.accusing = false;
                crate::narrative::resolve_case(&mut game, &db, &mut popups, &mut choices, &mut ui, &mut sfx, &mut toasts);
                return;
            }
        }
        _ => {}
    }
    ui.dirty = true;
}

#[allow(clippy::too_many_arguments)]
pub fn misc_input(
    mut ui: ResMut<UiState>,
    mut game: ResMut<Game>,
    keys: Res<ButtonInput<KeyCode>>,
    mut shop: ResMut<Shop>,
    mut sfx: EventWriter<Sfx>,
    mut toasts: ResMut<Toasts>,
    mut rt: ResMut<crate::player::PlayerRt>,
    mut popups: ResMut<Popups>,
    mut sim: ResMut<crate::sim::agents::Sim>,
    db: Res<CaseDb>,
) {
    let click = ui.take_click();
    match ui.mode {
        Mode::Inventory => {
            let n_items = {
                let mut v: Vec<&Item> = Vec::new();
                for it in &game.player.inv {
                    if !v.contains(&it) {
                        v.push(it);
                    }
                }
                v.len()
            };
            if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
                ui.sel = (ui.sel + 1).min(n_items.saturating_sub(1));
                ui.dirty = true;
            }
            if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
                ui.sel = ui.sel.saturating_sub(1);
                ui.dirty = true;
            }
            let mut uniq: Vec<Item> = Vec::new();
            for it in &game.player.inv {
                if !uniq.contains(it) {
                    uniq.push(it.clone());
                }
            }
            let Some(c) = click else { return };
            if let Some(i) = c.strip_prefix("bag:") {
                ui.sel = i.parse().unwrap_or(0);
            } else if let Some(o) = c.strip_prefix("outfit:") {
                if let Some(of) = game.player.owned_outfits.iter().find(|x| format!("{:?}", x) == o).copied() {
                    game.player.outfit = of;
                    rt.look_dirty = true;
                    toasts.push(format!("Vestindo: {}.", of.name()));
                }
            } else if c == "bag_use" {
                if let Some(it) = uniq.get(ui.sel).cloned() {
                    use_item(&mut game, &mut rt, &it, &mut toasts, &mut popups, &mut sfx);
                }
            } else if c == "bag_drop" {
                if let Some(it) = uniq.get(ui.sel).cloned() {
                    game.player.take(&it);
                    toasts.push(format!("Largou {}.", it.name(game.year)));
                }
            }
            ui.dirty = true;
        }
        Mode::Shop => {
            if keys.just_pressed(KeyCode::Escape) {
                ui.close();
                return;
            }
            let Some(c) = click else { return };
            if c == "shop_close" {
                ui.close();
                return;
            }
            if let Some(i) = c.strip_prefix("buy:") {
                let i: usize = i.parse().unwrap_or(0);
                if let Some((it, base)) = shop.stock.get(i).cloned() {
                    let p = price(&game, base);
                    if game.player.money >= p {
                        game.player.money -= p;
                        match it {
                            Item::Clothes(o) => {
                                if !game.player.owned_outfits.contains(&o) {
                                    game.player.owned_outfits.push(o);
                                }
                                game.player.outfit = o;
                                rt.look_dirty = true;
                            }
                            Item::Ammo(n) => game.player.add_ammo(n),
                            Item::Tool(crate::items::Tool::ForgedPapers) => {
                                let names = ["Thomas Reed", "Henry Cole", "Jack Morrow", "Paul Marchand", "Frank Weiss", "Victor Lane"];
                                let n = names[(game.day as usize) % names.len()];
                                game.player.identity = Some(crate::state::Identity { name: n.into(), job: "Jornalista".into(), origin: "Chicago".into(), quality: 60, burned: false });
                                toasts.push(format!("Nova identidade: {}.", n));
                            }
                            other => game.player.inv.push(other),
                        }
                        sfx.write(Sfx::Cash);
                        if let Some(k) = shop.keeper {
                            game.pop.get_mut(k).money += p;
                            game.pop.get_mut(k).elias.trust += 1;
                        }
                        if shop.black {
                            game.add_rep(crate::state::Group::Crime, 1);
                        }
                    } else {
                        toasts.push("Dinheiro insuficiente.");
                    }
                }
            } else if let Some(i) = c.strip_prefix("sell:") {
                let i: usize = i.parse().unwrap_or(0);
                if i < game.player.inv.len() {
                    let it = game.player.inv.remove(i);
                    let fence = shop.black || matches!(shop.kind, Some(crate::city::map::BKind::Pawn));
                    let v = price(&game, (it.sell_value() as f32 * if fence { 0.6 } else { 0.4 }) as i32);
                    game.player.money += v;
                    sfx.write(Sfx::Cash);
                    if matches!(it, Item::Valuable(..)) {
                        game.stat("fenced", v);
                    }
                }
            }
            ui.dirty = true;
        }
        Mode::Journal | Mode::Network => {
            if keys.just_pressed(KeyCode::ArrowDown) {
                ui.scroll += 1;
                ui.dirty = true;
            }
            if keys.just_pressed(KeyCode::ArrowUp) {
                ui.scroll = (ui.scroll - 1).max(0);
                ui.dirty = true;
            }
            if let Some(c) = click {
                if let Some(t) = c.strip_prefix("tab:") {
                    ui.tab = t.parse().unwrap_or(0);
                    ui.scroll = 0;
                } else if let Some(rest) = c.strip_prefix("task:") {
                    network_task(rest, &mut game, &mut sim, &db, &mut toasts);
                }
                ui.dirty = true;
            }
        }
        Mode::Map => {
            if keys.just_pressed(KeyCode::Escape) {
                ui.close();
            }
        }
        _ => {}
    }
}

fn use_item(game: &mut Game, rt: &mut crate::player::PlayerRt, it: &Item, toasts: &mut Toasts, popups: &mut Popups, sfx: &mut EventWriter<Sfx>) {
    match it {
        Item::Weapon(w) => {
            game.player.weapon = *w;
            rt.weapon_out = true;
            toasts.push(format!("{} em mãos.", w.stats().name));
        }
        Item::Tool(crate::items::Tool::Bandage) => {
            game.player.take(it);
            game.player.health = (game.player.health + 30.0).min(100.0);
            toasts.push("Ferimento enfaixado.");
        }
        Item::Food => {
            game.player.take(it);
            game.player.health = (game.player.health + 8.0).min(100.0);
            game.player.hunger = 0.0;
        }
        Item::Drug(Drug::Laudanum) => {
            game.player.take(it);
            game.player.health = (game.player.health + 25.0).min(100.0);
            game.player.addiction += 12.0;
            game.player.high = 60.0;
            toasts.push("Um calor doce sobe pela nuca. A dor fica longe.");
            if game.player.addiction > 40.0 {
                game.write("Preciso de mais uma dose. Só mais uma. Não é vício se é para suportar o tempo.", true);
            }
        }
        Item::Drug(Drug::Stimulant) => {
            game.player.take(it);
            game.player.stim = 120.0;
            game.player.stamina = 1.0;
            game.player.addiction += 6.0;
            toasts.push("O coração dispara. Você poderia correr até o rio.");
        }
        Item::Drug(Drug::Visionary) => {
            game.player.take(it);
            game.player.visionary = 180.0;
            game.fatigue += 8.0;
            game.player.addiction += 8.0;
            toasts.push("As bordas do mundo tremem. Os ecos vão ser mais nítidos.");
        }
        Item::Clothes(o) => {
            if !game.player.owned_outfits.contains(o) {
                game.player.owned_outfits.push(*o);
            }
            game.player.take(it);
            game.player.outfit = *o;
            rt.look_dirty = true;
        }
        Item::Newspaper(_) => {
            popups.push(Popup::new(PopStyle::Headline, crate::narrative::filler_headline(game), game.date_str(), ""));
        }
        _ => {}
    }
    sfx.write(Sfx::Click);
}

fn network_task(rest: &str, game: &mut Game, sim: &mut crate::sim::agents::Sim, db: &CaseDb, toasts: &mut Toasts) {
    let mut it = rest.split(':');
    let pid: u32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let task = it.next().unwrap_or("");
    let key = format!("task:{}:{}:{}", pid, task, game.day);
    if game.flag(&key) {
        toasts.push("Já pediu isso hoje.");
        return;
    }
    game.set(&key);
    let p = game.pop.get(pid).clone();
    let loyal = p.elias.loyalty;
    let unpaid = game.day > p.elias.paid_until_day;
    if unpaid && loyal < 40 {
        toasts.push(format!("{}: \"Primeiro me paga.\"", p.first));
        return;
    }
    let Some((def, pi)) = current(game, db) else {
        toasts.push("Não há caso para investigar.");
        return;
    };
    let def = def.clone();
    match task {
        "research" | "special" => {
            // find a clue this specialist could obtain
            let cand = def.clues.iter().enumerate().find(|(i, c)| !game.cases[pi].has(*i as u8) && (c.network == Some(p.job) || task == "research" && c.kind == crate::cases::defs::ClueKind::Document));
            if let Some((i, c)) = cand {
                game.cases[pi].found.push(i as u8);
                toasts.push(format!("{} trouxe: {}", p.first, c.name));
                game.write(format!("{} voltou com algo: {}.", p.first, c.name), false);
            } else {
                toasts.push(format!("{} não encontrou nada novo.", p.first));
            }
        }
        "follow" => {
            let sus: Vec<usize> = def.cast.iter().enumerate().filter(|(_, c)| c.role == crate::cases::defs::Role::Suspect).map(|(i, _)| i).collect();
            if let Some(&i) = sus.get((game.day as usize) % sus.len().max(1)) {
                let spid = game.cases[pi].cast[i];
                let where_ = sim.agent(spid).map(|a| a.act.label()).unwrap_or("desaparecido");
                toasts.push(format!("{} seguiu {}: agora está {}.", p.first, game.pop.get(spid).name(), where_));
            }
        }
        _ => {}
    }
    // betrayal risk for low loyalty, ambitious members
    if loyal < 25 && p.traits.ambition > 60 {
        game.police.heat += 20.0;
        game.police.knows_name = Some("Elias Vale".into());
        toasts.push(format!("Algo me diz que {} andou conversando com a polícia.", p.first));
        game.write(format!("{} me traiu. Ou vai trair. Nesta cidade, dá no mesmo.", p.first), true);
    }
    let _ = sim;
}
