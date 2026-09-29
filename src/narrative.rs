//! The story frame: prologue (Elias' archive, the laboratory, the sphere),
//! the limbo between cases, cutscenes, case transitions, staying in a
//! timeline, dreams, radio, the endings.

use crate::audio::{MusicState, Sfx, Track};
use crate::camera::CamState;
use crate::cases::run::{apply_outcome, judge, CaseDb, CaseRt};
use crate::city::gen::CityId;
use crate::city::map::*;
use crate::sim::people::*;
use crate::state::*;
use crate::ui::screens::{Choice, Choices, Popup, PopStyle, Popups};
use crate::ui::{hud::Toasts, Mode, UiState};
use bevy::prelude::*;

// ------------------------------------------------------------------ places outside the cities

pub fn apartment_map() -> Map {
    let rows = [
        "##################",
        "#................#",
        "#..............W.#",
        "#................#",
        "#........#########",
        "#........#.......#",
        "#........D.......#",
        "#........#.......#",
        "######D###########",
        ",,,,,,,,,,,,,,,,,,",
    ];
    let mut m = Map::from_ascii(&rows, BKind::Safehouse, "Apartamento de Elias Vale");
    let b = Some(0);
    m.add_prop(PKind::Board, 1, 1, 3, 1, b);
    m.add_prop(PKind::Board, 5, 1, 3, 1, b);
    m.add_prop(PKind::Desk, 11, 2, 2, 1, b);
    m.add_prop(PKind::Typewriter, 12, 2, 1, 1, b);
    m.add_prop(PKind::Shelf, 1, 5, 1, 2, b);
    m.add_prop(PKind::Shelf, 15, 1, 2, 1, b);
    m.add_prop(PKind::Crate, 3, 6, 1, 1, b);
    m.add_prop(PKind::Crate, 6, 3, 1, 1, b);
    m.add_prop(PKind::Bed, 14, 5, 1, 2, b);
    m.add_prop(PKind::RadioSet, 11, 5, 1, 1, b);
    m.add_prop(PKind::Sofa, 3, 3, 2, 1, b);
    m.add_prop(PKind::Rug, 2, 4, 3, 2, b);
    m.spawn = tile_center(8, 3);
    m.buildings[0].lights = vec![Vec2::new(4.0, 3.0), Vec2::new(12.0, 6.0)];
    m.rebuild_prop_index();
    m
}

pub fn lab_map() -> Map {
    let rows = [
        "                          #########       ",
        "                          #.......#       ",
        "  ######################  #.......#       ",
        "  #....................####...o...#       ",
        "  #.......................D.......#       ",
        "  #....................####.......#       ",
        "  ###D##################  #.......#       ",
        "     .                    #########       ",
        "     .                                    ",
        "  ggggggggg                               ",
    ];
    let mut m = Map::from_ascii(&rows, BKind::Lab, "Laboratório Delta (abandonado)");
    let b = Some(0);
    m.add_prop(PKind::Desk, 4, 3, 2, 1, b);
    m.add_prop(PKind::Shelf, 8, 3, 2, 1, b);
    m.add_prop(PKind::Crate, 12, 5, 1, 1, b);
    m.add_prop(PKind::Crate, 16, 3, 1, 1, b);
    m.add_prop(PKind::Slab, 19, 3, 1, 2, b);
    m.add_prop(PKind::Shelf, 22, 5, 1, 1, b);
    m.add_prop(PKind::Board, 10, 3, 2, 1, b);
    m.add_prop(PKind::Sphere, 30, 3, 1, 1, b);
    m.spawn = tile_center(5, 8);
    m.buildings[0].lights = vec![Vec2::new(8.0, 4.0), Vec2::new(30.5, 3.5)];
    m.rebuild_prop_index();
    m
}

pub fn limbo_map(stage: u8) -> Map {
    let rows = [
        "..............................",
        "..............................",
        "..............................",
        "..............................",
        "..............................",
        "..............................",
        "..............................",
        "..............................",
        "..............................",
        "..............................",
        "..............................",
        "..............................",
    ];
    let mut m = Map::from_ascii(&rows, BKind::Lab, "Limbo");
    let b = Some(0);
    // the limbo slowly fills with things from the cases
    if stage >= 1 {
        m.add_prop(PKind::Chair, 8, 6, 1, 1, b);
    }
    if stage >= 2 {
        m.add_prop(PKind::Table, 9, 6, 1, 1, b);
        m.add_prop(PKind::RadioSet, 9, 5, 1, 1, b);
    }
    if stage >= 4 {
        m.add_prop(PKind::Board, 12, 2, 3, 1, b);
        m.add_prop(PKind::Shelf, 4, 2, 2, 1, b);
    }
    if stage >= 6 {
        m.add_prop(PKind::Desk, 18, 4, 2, 1, b);
        m.add_prop(PKind::Typewriter, 19, 4, 1, 1, b);
        m.add_prop(PKind::Bed, 22, 8, 1, 2, b);
    }
    if stage >= 10 {
        m.add_prop(PKind::Piano, 4, 9, 2, 1, b);
        m.add_prop(PKind::Mirror, 26, 3, 1, 1, b);
    }
    if stage >= 15 {
        m.add_prop(PKind::Grave, 14, 9, 1, 2, b);
        m.add_prop(PKind::Grave, 16, 9, 1, 2, b);
    }
    m.add_prop(PKind::Sphere, 15, 5, 1, 1, b);
    m.spawn = tile_center(15, 9);
    m.buildings[0].lights = vec![Vec2::new(15.5, 5.5)];
    m.rebuild_prop_index();
    m
}

/// Where the next door out of the limbo stands.
pub fn limbo_door() -> Vec2 {
    tile_center(15, 1)
}

// ------------------------------------------------------------------ cutscenes

#[derive(Clone)]
pub struct Shot {
    pub dur: f32,
    pub focus: Option<Vec3>,
    pub dist: Option<f32>,
    pub yaw: Option<f32>,
    pub caption: String,
    pub sub: String,
    pub fade_to: f32,
    pub sfx: Option<Sfx>,
    pub flash: bool,
}

impl Shot {
    pub fn new(dur: f32) -> Shot {
        Shot { dur, focus: None, dist: None, yaw: None, caption: String::new(), sub: String::new(), fade_to: 0.0, sfx: None, flash: false }
    }
    pub fn cap(mut self, c: &str) -> Shot {
        self.caption = c.into();
        self
    }
    pub fn sub(mut self, c: &str) -> Shot {
        self.sub = c.into();
        self
    }
    pub fn at(mut self, p: Vec3, dist: f32) -> Shot {
        self.focus = Some(p);
        self.dist = Some(dist);
        self
    }
    pub fn yaw(mut self, y: f32) -> Shot {
        self.yaw = Some(y);
        self
    }
    pub fn fade(mut self, f: f32) -> Shot {
        self.fade_to = f;
        self
    }
    pub fn sfx(mut self, s: Sfx) -> Shot {
        self.sfx = Some(s);
        self
    }
    pub fn flash(mut self) -> Shot {
        self.flash = true;
        self
    }
}

#[derive(Resource, Default)]
pub struct Cutscene {
    pub shots: Vec<Shot>,
    pub idx: usize,
    pub t: f32,
    pub fade: f32,
    pub then: Vec<String>,
    pub active: bool,
}

impl Cutscene {
    pub fn play(&mut self, shots: Vec<Shot>, then: &[&str]) {
        self.shots = shots;
        self.idx = 0;
        self.t = 0.0;
        self.active = true;
        self.then = then.iter().map(|s| s.to_string()).collect();
    }
}

#[derive(Component)]
pub struct FadeLayer;
#[derive(Component)]
pub struct CineCaption;
#[derive(Component)]
pub struct CineSub;
#[derive(Component)]
pub struct CineBars;

pub fn setup_cine(mut c: Commands, fonts: Res<crate::ui::UiFonts>) {
    c.spawn((
        Node { position_type: PositionType::Absolute, width: Val::Percent(100.0), height: Val::Percent(100.0), flex_direction: FlexDirection::Column, justify_content: JustifyContent::SpaceBetween, ..default() },
        GlobalZIndex(50),
        Pickable::IGNORE,
        CineBars,
        Visibility::Hidden,
    ))
    .with_children(|p| {
        p.spawn((Node { width: Val::Percent(100.0), height: Val::Px(70.0), ..default() }, BackgroundColor(Color::BLACK)));
        p.spawn((Node { width: Val::Percent(100.0), height: Val::Px(70.0), ..default() }, BackgroundColor(Color::BLACK)));
    });
    c.spawn((
        Node { position_type: PositionType::Absolute, width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        GlobalZIndex(60),
        Pickable::IGNORE,
        FadeLayer,
    ));
    c.spawn((
        Node { position_type: PositionType::Absolute, width: Val::Percent(100.0), top: Val::Percent(40.0), flex_direction: FlexDirection::Column, align_items: AlignItems::Center, row_gap: Val::Px(10.0), ..default() },
        GlobalZIndex(70),
        Pickable::IGNORE,
    ))
    .with_children(|p| {
        p.spawn((Text::new(""), TextFont { font: fonts.mono_b.clone(), font_size: 34.0, ..default() }, TextColor(crate::ui::INK), CineCaption, TextLayout::new_with_justify(JustifyText::Center)));
        p.spawn((Text::new(""), TextFont { font: fonts.serif_i.clone(), font_size: 21.0, ..default() }, TextColor(crate::ui::DIM), CineSub, TextLayout::new_with_justify(JustifyText::Center), Node { max_width: Val::Px(900.0), ..default() }));
    });
}

#[allow(clippy::too_many_arguments)]
pub fn run_cutscene(
    time: Res<Time>,
    mut cs: ResMut<Cutscene>,
    mut cam: ResMut<CamState>,
    mut ui: ResMut<UiState>,
    mut fade: Query<&mut BackgroundColor, With<FadeLayer>>,
    mut cap: Query<(&mut Text, &mut TextColor), (With<CineCaption>, Without<CineSub>)>,
    mut sub: Query<&mut Text, (With<CineSub>, Without<CineCaption>)>,
    mut bars: Query<&mut Visibility, With<CineBars>>,
    mut sfx: EventWriter<Sfx>,
    mut flow: ResMut<Flow>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let dt = time.delta_secs();
    if let Ok(mut b) = bars.single_mut() {
        *b = if cs.active { Visibility::Inherited } else { Visibility::Hidden };
    }
    if !cs.active {
        cs.fade = (cs.fade - dt * 1.5).max(0.0);
        if let Ok(mut f) = fade.single_mut() {
            f.0 = Color::srgba(0.0, 0.0, 0.0, cs.fade);
        }
        if let Ok((mut t, _)) = cap.single_mut() {
            if !t.is_empty() {
                **t = String::new();
            }
        }
        if let Ok(mut t) = sub.single_mut() {
            if !t.is_empty() {
                **t = String::new();
            }
        }
        return;
    }
    if ui.mode != Mode::Movie {
        ui.mode = Mode::Movie;
        ui.dirty = true;
    }
    let skip = keys.just_pressed(KeyCode::Escape);
    let Some(shot) = cs.shots.get(cs.idx).cloned() else {
        cs.active = false;
        ui.close();
        for a in cs.then.drain(..) {
            flow.actions.push(a);
        }
        return;
    };
    if cs.t == 0.0 {
        if let Some(s) = shot.sfx {
            sfx.write(s);
        }
        if let Some(f) = shot.focus {
            cam.focus = f;
        }
        if let Some(d) = shot.dist {
            cam.dist_target = d;
        }
        if let Some(y) = shot.yaw {
            cam.yaw_target = y;
        }
    }
    cs.t += dt;
    let k = (cs.t / shot.dur).min(1.0);
    let prev_fade = cs.fade;
    cs.fade = prev_fade + (shot.fade_to - prev_fade) * (dt * 2.0).min(1.0);
    let flash = if shot.flash && cs.t < 0.25 { 1.0 - cs.t * 4.0 } else { 0.0 };
    if let Ok(mut f) = fade.single_mut() {
        f.0 = if flash > 0.0 { Color::srgba(0.9, 0.05, 0.1, flash) } else { Color::srgba(0.0, 0.0, 0.0, cs.fade) };
    }
    // typewriter captions
    if let Ok((mut t, mut col)) = cap.single_mut() {
        let n = ((cs.t * 30.0) as usize).min(shot.caption.chars().count());
        **t = shot.caption.chars().take(n).collect();
        col.0 = if shot.caption.chars().all(|c| c.is_ascii_digit() || c.is_whitespace()) && !shot.caption.is_empty() { crate::ui::RED } else { crate::ui::INK };
    }
    if let Ok(mut t) = sub.single_mut() {
        let n = (((cs.t - 0.4).max(0.0) * 40.0) as usize).min(shot.sub.chars().count());
        **t = shot.sub.chars().take(n).collect();
    }
    let _ = k;
    if cs.t >= shot.dur || skip {
        cs.idx += 1;
        cs.t = 0.0;
        if skip {
            cs.idx = cs.shots.len();
        }
    }
}

// ------------------------------------------------------------------ flow actions

/// Queue of story actions executed by `flow_system` (so any module can
/// request "load the limbo", "start case 2", etc.).
#[derive(Resource, Default)]
pub struct Flow {
    pub actions: Vec<String>,
}

#[allow(clippy::too_many_arguments)]
pub fn flow_system(
    mut flow: ResMut<Flow>,
    mut game: ResMut<Game>,
    db: Res<CaseDb>,
    mut load: ResMut<crate::world::LoadCity>,
    mut popups: ResMut<Popups>,
    mut cs: ResMut<Cutscene>,
    mut music: ResMut<MusicState>,
    mut crt: ResMut<CaseRt>,
    mut choices: ResMut<Choices>,
    mut ui: ResMut<UiState>,
    mut save: ResMut<crate::save::SaveReq>,
    mut toasts: ResMut<Toasts>,
    mut sim: ResMut<crate::sim::agents::Sim>,
    mut rt: ResMut<crate::player::PlayerRt>,
    mut it: ResMut<crate::interact::Interact>,
) {
    let actions: Vec<String> = flow.actions.drain(..).collect();
    for a in actions {
        let (cmd, arg) = a.split_once(':').unwrap_or((a.as_str(), ""));
        match cmd {
            "new_game" => {
                *game = Game::new();
                game.phase = Phase::Prologue;
                game.set("prologue_apartment");
                game.year = 2025;
                game.minute = 23.0 * 60.0 + 10.0;
                game.rain = 0.9;
                game.rain_target = 0.9;
                game.player.pos = Vec2::ZERO;
                it.opened.clear();
                it.searched.clear();
                load.0 = true;
                music.want = Track::Limbo;
                rt.look_dirty = true;
                cs.play(
                    vec![
                        Shot::new(2.5).fade(1.0),
                        Shot::new(4.0).fade(1.0).cap("NOVA ORLEANS").sub("Dias de hoje. 23h10. Chove há três dias."),
                        Shot::new(5.0).fade(0.0).at(Vec3::new(8.5, 0.8, 3.5), 10.0).sub("Elias Vale tem um arquivo. Recortes, fotos, mapas, fitas. Uma pergunta que ninguém leva a sério:"),
                        Shot::new(4.5).at(Vec3::new(3.0, 1.2, 1.5), 7.0).cap("E se todos esses mistérios").sub("estiverem relacionados?"),
                        Shot::new(1.0).at(Vec3::new(8.5, 0.8, 3.5), 14.0),
                    ],
                    &["objective:Leia o quadro de cortiça e o que está sobre a mesa."],
                );
            }
            "objective" => toasts.push(arg.to_string()),
            "to_lab" => {
                game.flags.remove("prologue_apartment");
                game.set("prologue_lab");
                game.player.pos = Vec2::ZERO;
                load.0 = true;
                cs.play(
                    vec![
                        Shot::new(3.0).fade(1.0).cap("DUAS HORAS DE ESTRADA DEPOIS").sub("Laboratório Delta. Fechado desde 1971. Oficialmente, nunca existiu."),
                        Shot::new(1.5).fade(0.0),
                    ],
                    &["objective:Encontre a sala subterrânea."],
                );
            }
            "touch_sphere" => {
                music.want = Track::Limbo;
                cs.play(
                    vec![
                        Shot::new(3.5).sfx(Sfx::Hum).sub("Ela não emite som. Mas quando você chega perto..."),
                        Shot::new(2.0).cap("1918...").sfx(Sfx::Static),
                        Shot::new(1.6).cap("1947...").sfx(Sfx::Whisper),
                        Shot::new(1.3).cap("1969...").sfx(Sfx::Static),
                        Shot::new(1.1).cap("1971...").sfx(Sfx::Whisper),
                        Shot::new(1.6).cap("1920 1932 1948 1971 1986 2001").sub("Jornais. Rádios. Telefones. Gente gritando. Música. Tiros. Sirene. Tudo ao mesmo tempo.").sfx(Sfx::Static),
                        Shot::new(0.6).flash().sfx(Sfx::Alter).fade(1.0),
                        Shot::new(3.0).fade(1.0).sub("Você toca a esfera."),
                        Shot::new(4.0).fade(1.0).cap("").sub("Não existe personagem. Não existe música. Só voz."),
                        Shot::new(3.0).fade(1.0).cap("1920").sfx(Sfx::Static),
                        Shot::new(2.0).fade(1.0).cap("1921   1932").sfx(Sfx::Static),
                        Shot::new(2.0).fade(1.0).cap("1947   1968   1971   1970").sfx(Sfx::Static),
                        Shot::new(5.5).fade(1.0).cap("").sub("\"Um crime sem solução é apenas uma porta que ninguém abriu.\"").sfx(Sfx::Whisper),
                        Shot::new(3.0).fade(1.0).sub(""),
                        Shot::new(3.5).fade(1.0).sub("\"Abra a primeira.\"").sfx(Sfx::Whisper),
                    ],
                    &["start_case:1"],
                );
            }
            "start_case" => {
                let n: u8 = arg.parse().unwrap_or(1);
                start_case(&mut game, &db, n, &mut popups, &mut cs, &mut music, &mut crt, &mut sim, &mut rt);
                load.0 = true;
                save.save = Some(0);
            }
            "solve" => {
                // arg = "correct?" handled in resolve
            }
            "limbo" => {
                game.phase = Phase::Prologue;
                game.flags.retain(|f| !f.starts_with("prologue"));
                game.set("limbo");
                game.limbo_stage = game.cases.iter().filter(|c| c.solved).count() as u8;
                game.player.pos = Vec2::ZERO;
                load.0 = true;
                music.want = Track::Limbo;
                let stage = game.limbo_stage;
                let (cap, sub) = limbo_words(&game, stage);
                cs.play(
                    vec![Shot::new(2.0).fade(1.0), Shot::new(4.5).fade(0.3).cap(&cap).sub(&sub).sfx(Sfx::Static), Shot::new(1.5).fade(0.0)],
                    &["limbo_arrived"],
                );
            }
            "limbo_arrived" => {
                let next = next_case(&game, &db);
                match next {
                    Some(n) => {
                        let def = db.get(n).unwrap();
                        toasts.push(format!("Uma porta: {} — {}", def.city.upper(), def.year));
                        game.set(&format!("limbo_next:{}", n));
                        if game.limbo_stage >= 27 && !game.flag("met_other") {
                            game.set("met_other");
                            popups.push(Popup::new(
                                PopStyle::Limbo,
                                "O OUTRO ELIAS",
                                "LIMBO",
                                "Há alguém sentado na cadeira. O mesmo casaco. As mesmas mãos. Mais velho.\n\n\"Você ainda acha que está resolvendo os casos?\"\n\nSilêncio.\n\n\"Nós nunca estivemos resolvendo casos.\" Ele olha para a esfera. \"Estamos alimentando ela.\"",
                            ));
                        }
                    }
                    None => flow.actions.push("finale".into()),
                }
            }
            "stay" => {
                let years: i32 = arg.parse().unwrap_or(1);
                let log = stay_in_timeline(&mut game, years);
                popups.push(Popup::new(PopStyle::Mystery, format!("{} ANO{} DEPOIS", years, if years > 1 { "S" } else { "" }), game.city.upper(), log.join("\n\n")));
                load.0 = true;
            }
            "finale" => {
                finale_choice(&game, &mut choices, &mut ui);
            }
            "ending" => {
                let text = ending_text(arg, &game);
                music.want = Track::Finale;
                popups.push(Popup::new(PopStyle::Limbo, text.0, "FIM", text.1));
                crate::save::set_global_flag("finished");
                if game.cases.iter().filter(|c| c.layers >= 3).count() >= 20 {
                    crate::save::set_global_flag("break_thread");
                }
                popups.then("again");
            }
            "again" => {
                choices.cur = Some(Choice {
                    title: "Você faria tudo novamente?".into(),
                    body: String::new(),
                    opts: vec![("again_yes".into(), "SIM".into()), ("again_no".into(), "NÃO".into())],
                    ctx: String::new(),
                });
                ui.open(Mode::Choice);
            }
            "title" => {
                ui.open(Mode::Title);
                music.want = Track::Menu;
            }
            _ => {}
        }
    }
}

fn limbo_words(game: &Game, stage: u8) -> (String, String) {
    let last = game.cases.iter().filter(|c| c.solved).last();
    let status = last.map(|c| if c.correct { "LINHA DO TEMPO ESTABILIZADA" } else { "RESOLUÇÃO FALSA" }).unwrap_or("");
    let sub = match stage {
        0..=1 => "Escuridão. Uma cadeira que não estava ali antes.",
        2..=4 => "Uma mesa. Um rádio que toca notícias de amanhã.",
        5..=9 => "Fotografias no chão. Você reconhece rostos que ainda não conheceu.",
        10..=15 => "Portas. Dezenas de portas. Algumas estão arranhadas por dentro.",
        16..=24 => "Túmulos com nomes de pessoas que você salvou.",
        _ => "Alguém respira na escuridão. Alguém com a sua voz.",
    };
    (status.to_string(), sub.to_string())
}

pub fn next_case(game: &Game, db: &CaseDb) -> Option<u8> {
    db.0.iter().map(|c| c.id).find(|id| !game.cases.iter().any(|p| p.id == *id && p.solved))
}

#[allow(clippy::too_many_arguments)]
pub fn start_case(game: &mut Game, db: &CaseDb, n: u8, popups: &mut Popups, cs: &mut Cutscene, music: &mut MusicState, crt: &mut CaseRt, sim: &mut crate::sim::agents::Sim, rt: &mut crate::player::PlayerRt) {
    let Some(def) = db.get(n) else { return };
    let old_city = game.city;
    let old_year = game.year;
    let first_time = game.cases.iter().all(|c| c.id != n);
    game.flags.retain(|f| !f.starts_with("prologue") && f != "limbo" && !f.starts_with("limbo_next"));
    game.phase = Phase::City;
    game.case_idx = n as usize;
    let same_city = old_city == def.city && game.populated.iter().any(|(c, _)| *c == def.city);
    game.city = def.city;
    game.year = if same_city { def.year.max(old_year) } else { def.year };
    if old_city != def.city || (old_year - game.year).abs() > 15 || old_year > 2020 {
        crate::economy::convert_money_on_jump(game, old_year);
        game.player.safehouse = None;
        game.player.owned.clear();
        game.police = Police::default();
        // clothes of the old era look out of place; Elias keeps them though
    }
    game.minute = 21.0 * 60.0 + (n as f32 * 13.0) % 120.0;
    game.player.pos = Vec2::ZERO;
    game.player.health = 100.0;
    game.player.body_age = 34.0;
    crt.spawned = None;
    crt.echo = None;
    crt.red_sight = false;
    sim.agents.clear();
    rt.carrying = None;
    rt.in_car = false;
    music.want = crate::audio::track_for_year(game.year);
    if first_time {
        game.write(format!("{} — {}. {}", def.city.upper(), game.year, def.title), false);
    }
    cs.play(
        vec![
            Shot::new(1.5).fade(1.0),
            Shot::new(3.2).fade(1.0).cap(&format!("{}", def.city.upper())).sfx(Sfx::Type),
            Shot::new(3.0).fade(1.0).cap(&format!("{}", game.year)).sfx(Sfx::Type),
            Shot::new(2.0).fade(0.0),
        ],
        &[],
    );
    popups.push(Popup::new(PopStyle::Intro, format!("CASO {:02} — {}", def.id, def.title.to_uppercase()), format!("{} · {} · LINHA {}", def.city.upper(), game.year, game.timeline_code()), def.intro));
}

/// Resolve the current case after a formal accusation.
#[allow(clippy::too_many_arguments)]
pub fn resolve_case(game: &mut Game, db: &CaseDb, popups: &mut Popups, choices: &mut Choices, ui: &mut UiState, sfx: &mut EventWriter<Sfx>, toasts: &mut Toasts) {
    let Some((def, pi)) = crate::cases::run::current(game, db) else { return };
    let def = def.clone();
    let prog = game.cases[pi].clone();
    let (correct, layers) = judge(&def, &prog);
    let accused = prog.accused.map(|a| game.pop.get(prog.cast[a as usize]).name()).unwrap_or_default();
    let changes = apply_outcome(game, &def, pi, correct, layers);
    sfx.write(Sfx::Alter);
    // the game never says "wrong" — both endings look like success at first
    popups.push(Popup::new(PopStyle::Headline, if correct { def.on_true.headline } else { def.on_false.headline }, format!("{} · {}", def.city.upper(), game.year), format!("{} foi entregue à justiça.\n\n{}", accused, if correct { def.on_true.text } else { "A cidade respira aliviada. Por enquanto." })));
    let mut body = String::new();
    if layers >= 2 {
        body.push_str(&format!("A VERDADEIRA HISTÓRIA\n{}\n\n", def.story));
    }
    if layers >= 3 {
        body.push_str(&format!("A ESFERA\n{}\n\nNo verso do arquivo, alguém escreveu à mão um único número: {}", def.sphere, def.digit));
    }
    let n_people = changes.len();
    popups.push(
        Popup::new(PopStyle::Timeline, "LINHA DO TEMPO ALTERADA", format!("{} → R-{:02}", prog.timeline_or(game.timeline - 1), game.timeline), body)
            .with_lines({
                let mut v: Vec<String> = changes.into_iter().take(8).collect();
                v.push(format!("PESSOAS AFETADAS: {}", n_people));
                v
            }),
    );
    toasts.big("LINHA DO TEMPO ALTERADA", format!("CASO {:02} ENCERRADO", def.id));
    game.write(
        if correct {
            format!("Encerrei o caso de {}. Não sei se fiz justiça ou só escolhi uma versão do mundo.", def.title.to_lowercase())
        } else {
            format!("Entreguei {}. A cidade comemorou. Eu não consegui dormir.", accused)
        },
        true,
    );
    // stay or follow the thread
    choices.cur = Some(Choice {
        title: "A esfera chama".into(),
        body: "Você sente o zumbido atrás dos olhos. A próxima porta está aberta. Mas esta vida também está.".into(),
        opts: vec![
            ("follow_thread".into(), "Seguir o fio (próximo caso)".into()),
            ("stay:1".into(), "Ficar — viver um ano nesta linha do tempo".into()),
            ("stay:5".into(), "Ficar — cinco anos".into()),
            ("stay:10".into(), "Ficar — dez anos".into()),
            ("linger".into(), "Continuar por aqui mais um pouco (volte ao esconderijo para seguir)".into()),
        ],
        ctx: String::new(),
    });
    popups.then("choice_pending");
    let _ = ui;
}

trait TimelineOr {
    fn timeline_or(&self, d: u32) -> String;
}
impl TimelineOr for crate::cases::CaseProgress {
    fn timeline_or(&self, d: u32) -> String {
        format!("R-{:02}", if self.timeline > 1 { self.timeline - 1 } else { d })
    }
}

/// Living years inside a timeline: family, children, aging.
pub fn stay_in_timeline(game: &mut Game, years: i32) -> Vec<String> {
    let mut log = Vec::new();
    let mut r = crate::util::Rng::new(game.day as u64 * 7 + years as u64);
    let city = game.city;
    let from = game.year;
    let partner = game.pop.people.iter().find(|p| p.city == city && p.alive() && matches!(p.elias.romance, Romance::Married | Romance::Engaged | Romance::Lovers | Romance::Dating)).map(|p| p.id);
    for y in 0..years {
        let year = from + y + 1;
        if let Some(pt) = partner {
            let p = game.pop.get(pt).clone();
            if matches!(p.elias.romance, Romance::Engaged | Romance::Lovers | Romance::Dating) && r.chance(0.6) {
                game.pop.get_mut(pt).elias.romance = Romance::Married;
                log.push(format!("{}: você se casou com {}.", year, p.name()));
            }
            let married = game.pop.get(pt).elias.romance == Romance::Married;
            let age = game.pop.get(pt).age(year);
            let kids = game.pop.people.iter().filter(|k| k.elias_child && k.parents.contains(&Some(pt))).count();
            if married && age < 44 && kids < 4 && r.chance(0.35) {
                let female = r.chance(0.5);
                let names_m = ["Daniel", "Thomas", "Lucas", "Samuel", "Gabriel", "Henry", "Jonas"];
                let names_f = ["Alice", "Marie", "Helena", "Rose", "Julia", "Violet", "Clara"];
                let first = if female { *r.pick(&names_f) } else { *r.pick(&names_m) };
                let k = game.pop.new_person(&mut r, city, female, year, Some("Vale".into()), year);
                game.pop.get_mut(k).first = first.to_string();
                game.pop.get_mut(k).elias_child = true;
                game.pop.get_mut(k).notable = true;
                game.pop.get_mut(k).home = game.pop.get(pt).home;
                let (mother, other) = if p.female { (pt, None) } else { (pt, None) };
                game.pop.add_child(mother, other, k);
                log.push(format!("{}: nasceu {} {}, {} de {}.", year, if female { "sua filha" } else { "seu filho" }, first, if female { "filha" } else { "filho" }, p.first));
                game.write(format!("{} nasceu às 3:17 da madrugada. Eu fingi que não reparei na hora.", first), true);
            }
        }
    }
    let mut elog = Vec::new();
    game.pop.advance_years(&mut r, city, from, from + years, &mut elog);
    log.extend(elog.into_iter().take(6));
    game.year = from + years;
    game.player.body_age += years as f32;
    game.player.mind_years += years as f32;
    game.populated.push((city, game.year));
    if log.is_empty() {
        log.push("Os anos passaram devagar. A cidade mudou. Você também.".into());
    }
    log.push(format!("Você tem {:.0} anos. A esfera continua chamando, baixinho, toda noite às 3:17.", game.player.body_age));
    log
}

fn finale_choice(game: &Game, choices: &mut Choices, ui: &mut UiState) {
    let layers3 = game.cases.iter().filter(|c| c.layers >= 3).count();
    let all_true = game.cases.iter().filter(|c| c.solved && c.correct).count();
    let mut opts = vec![
        ("ending:A".into(), "Ficar no limbo e tornar-se a voz dos jornais".into()),
        ("ending:B".into(), "Destruir a esfera".into()),
    ];
    if layers3 >= 10 {
        opts.push(("ending:C".into(), "Construir a esfera. Enviá-la ao passado. Fechar o círculo.".into()));
    }
    if all_true >= 20 {
        opts.push(("ending:D".into(), "Manter todas as linhas do tempo — ou estabilizar uma só".into()));
    }
    choices.cur = Some(Choice {
        title: "CASO 30 — O ÚLTIMO CASO".into(),
        body: "A esfera flutua a um palmo do seu rosto. Atrás dela, o Outro Elias segura um machado que você já viu antes. Todas as versões da música tocam ao mesmo tempo.".into(),
        opts,
        ctx: String::new(),
    });
    ui.open(Mode::Choice);
}

fn ending_text(which: &str, game: &Game) -> (String, String) {
    let family = game.pop.people.iter().filter(|p| p.elias_child && p.alive()).count();
    match which {
        "A" => ("FINAL A — O DETETIVE".into(), "Você resolveu tudo. E por isso ficou.\n\nO limbo se enche de arquivos, portas e cadeiras. Um dia, um homem curioso toca a esfera num laboratório abandonado. Ele ouve datas. Jornais. Rádios.\n\nE uma voz — a sua — dizendo: \"Um crime sem solução é apenas uma porta que ninguém abriu.\"".into()),
        "B" => ("FINAL B — A FUGA".into(), format!("Você enfia os dedos na esfera até a luz vermelha queimar sua pele. Ela racha como gelo.\n\nAs linhas do tempo desabam umas sobre as outras. Você acorda no seu apartamento, em 2025, com chuva.\n\nNinguém lembra do Homem do Machado, do homem de Somerton, das cartas do Zodíaco. Só você.{}", if family > 0 { format!("\n\nNa gaveta, uma foto amarelada de {} criança{} que nunca existiram. Você guarda. Todo dia.", family, if family > 1 { "s" } else { "" }) } else { String::new() })),
        "C" => ("FINAL C — O CÍRCULO".into(), "Os números dos casos formam coordenadas. Elas apontam para um terreno baldio em Nova Orleans, 2001. O Laboratório Delta ainda não existe.\n\nPorque você ainda não o construiu.\n\nVocê passa trinta anos juntando os fragmentos. Em 1971, com as próprias mãos, você envia a esfera ao passado — para que um homem obcecado por mistérios a encontre.\n\nVocê é a causa do próprio mistério.".into()),
        _ => ("FINAL D — A ESCOLHA".into(), "Você entende, enfim: a esfera é um arquivo vivo de possibilidades. Você pode deixar todas as linhas existirem — e aceitar que em algumas delas as pessoas continuarão morrendo às 3:17.\n\nOu pode estabilizar uma única realidade. Uma só. E milhões de vidas alternativas deixam de existir, sem nunca saber.\n\nO jogo não diz qual escolha é certa. Ninguém nunca disse.".into()),
    }
}

// ------------------------------------------------------------------ small text generators

pub fn filler_headline(game: &Game) -> String {
    let y = game.year;
    let pool: &[&str] = if y < 1930 {
        &["PREFEITURA PROMETE LUZ ELÉTRICA EM TODAS AS RUAS", "LEI SECA: APREENDIDAS 300 GARRAFAS NO PORTO", "BAILE BENEFICENTE REÚNE A ELITE DO GARDEN DISTRICT", "CHUVAS ALAGAM O TREMÉ PELA TERCEIRA VEZ"]
    } else if y < 1946 {
        &["FILAS DA SOPA DOBRAM NO INVERNO", "SINDICATO E POLÍCIA EM CONFRONTO NO LOOP", "RÁDIO ANUNCIA: A GUERRA NA EUROPA SE AGRAVA"]
    } else if y < 1960 {
        &["CORPO SEM IDENTIDADE CONTINUA UM ENIGMA", "GOVERNO NEGA TESTES SECRETOS NO DESERTO", "PRAIA DE GLENELG LOTADA NO FERIADO"]
    } else if y < 1980 {
        &["NOVA CARTA CIFRADA CHEGA À REDAÇÃO", "MANIFESTAÇÃO CONTRA A GUERRA TOMA AS RUAS", "AVIÃO DESAPARECE SEM DEIXAR RASTRO"]
    } else {
        &["METRÔ REGISTRA RECORDE DE ASSALTOS", "MAIS UMA CRIANÇA DESAPARECIDA NO BAIRRO", "BOLSA DESPENCA E ARRASTA PEQUENOS INVESTIDORES"]
    };
    pool[(game.day as usize) % pool.len()].to_string()
}

pub fn radio_line(game: &Game) -> String {
    let lines = [
        "...previsão de chuva para toda a madrugada... e agora, para os apaixonados...",
        "...interrompemos a programação para um boletim da polícia...",
        "...(estática)... Elias... (estática)... não resolva o último caso...",
        "...a música que vocês pediram, direto do salão, ao vivo...",
        "...às três e dezessete da manhã, segundo testemunhas, o céu ficou vermelho...",
    ];
    let i = ((game.minute as i32 / 7) + game.day) as usize % lines.len();
    if game.fatigue > 40.0 && game.day % 2 == 0 {
        return lines[2].to_string();
    }
    lines[i].to_string()
}

pub fn dream(game: &Game) -> Option<String> {
    let fam: Vec<&Person> = game.pop.people.iter().filter(|p| p.elias_child).collect();
    let lost: Vec<&&Person> = fam.iter().filter(|p| p.life == Life::Unborn || p.city != game.city).collect();
    let r = crate::util::hashf(game.day, game.year, 99);
    if !lost.is_empty() && r < 0.6 {
        let k = lost[0];
        return Some(format!("Você anda pela casa. {} está brincando no tapete. Alguém chama da cozinha. Você entra na sala: não há ninguém. A casa muda. Agora é outra década. Você abre uma porta — e é o limbo.", k.first));
    }
    if game.fatigue > 30.0 && r < 0.5 {
        return Some("Um rádio chama seu nome em seis décadas diferentes. Uma mulher fecha a cortina de uma janela que você nunca viu. Você acorda com gosto de ferrugem na boca.".into());
    }
    if r < 0.25 {
        return Some("Você sonha com uma linha vermelha atravessando a cidade, costurando casas, pessoas, anos. Ela termina na sua mão.".into());
    }
    None
}

/// Temporal fatigue: glitches in the world once Elias has altered too much.
pub fn fatigue_system(time: Res<Time>, mut game: ResMut<Game>, crt: Res<CaseRt>, mut sim: ResMut<crate::sim::agents::Sim>, mut t: Local<f32>, ui: Res<UiState>) {
    if ui.pauses_world() {
        return;
    }
    let dt = time.delta_secs();
    if crt.red_sight {
        game.fatigue = (game.fatigue + dt * 0.25).min(100.0);
    }
    let alt = game.alterations as f32;
    let base = (alt * 0.4).min(60.0);
    if game.fatigue < base {
        game.fatigue += dt * 0.01;
    }
    *t -= dt;
    if *t > 0.0 {
        return;
    }
    *t = 25.0;
    if game.fatigue < 25.0 || sim.agents.is_empty() {
        return;
    }
    // uncanny moments: someone calls Elias by name, says a line from another decade
    let n = sim.agents.len();
    let i = (game.minute as usize * 31 + game.day as usize) % n;
    let pp = game.player.pos;
    if sim.agents[i].pos.distance(pp) < 14.0 && sim.agents[i].active() {
        let lines = [
            "Elias? ...Desculpe, achei que fosse outra pessoa.",
            "Você de novo. Você sempre volta às 3:17.",
            "Eu já vi você morrer. Numa outra noite.",
            "Minha filha desenhou você. Antes de você chegar.",
            "Que música é essa? Parece de outro tempo.",
        ];
        let l = lines[(game.day as usize + i) % lines.len()];
        sim.agents[i].say(l, 4.0);
    }
}

pub fn city_for_case(db: &CaseDb, n: u8) -> Option<CityId> {
    db.get(n).map(|d| d.city)
}
