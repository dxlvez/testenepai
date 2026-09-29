//! Heads-up display: clock, place, money, health, weapon, prompts, toasts,
//! speech bubbles above people, echo captions.

use super::*;
use crate::camera::MainCam;
use crate::cases::run::{current, CaseDb, CaseRt};
use crate::player::PlayerRt;
use crate::sim::agents::Sim;
use crate::state::Game;
use crate::world::CityMap;
use crate::util::{clock_str, fmt_money};

#[derive(Resource, Default)]
pub struct Toasts {
    pub items: Vec<(String, f32)>,
    pub big: Option<(String, String, f32)>,
}

impl Toasts {
    pub fn push(&mut self, s: impl Into<String>) {
        self.items.push((s.into(), 5.0));
        if self.items.len() > 5 {
            self.items.remove(0);
        }
    }
    /// Big centred title card (e.g. "LINHA DO TEMPO ALTERADA")
    pub fn big(&mut self, title: impl Into<String>, sub: impl Into<String>) {
        self.big = Some((title.into(), sub.into(), 4.5));
    }
}

#[derive(Resource, Default)]
pub struct Prompt(pub Option<String>);

/// Transient HUD state for the minimalist layout.
#[derive(Resource, Default)]
pub struct HudMem {
    pub place: String,
    pub place_t: f32,
    pub tick: String,
    pub tick_t: f32,
    pub obj: String,
    pub obj_t: f32,
}

#[derive(Component)]
pub struct HudText(pub u8);

#[derive(Component)]
pub struct HudRoot;
#[derive(Component)]
pub struct HudTopLeft;
#[derive(Component)]
pub struct HudTopRight;
#[derive(Component)]
pub struct HudPrompt;
#[derive(Component)]
pub struct HudToasts;
#[derive(Component)]
pub struct HudBig;
#[derive(Component)]
pub struct HudBigSub;
#[derive(Component)]
pub struct HudCaption;
#[derive(Component)]
pub struct HudObjective;
#[derive(Component)]
pub struct HudHealth;
#[derive(Component)]
pub struct HudStamina;
#[derive(Component)]
pub struct HudWeapon;
#[derive(Component)]
pub struct Bubble(pub usize);
#[derive(Component)]
pub struct HudVignette;
#[derive(Component)]
pub struct HudTicker;
#[derive(Component)]
pub struct HudCrosshair;

pub fn setup_hud(mut c: Commands, fonts: Res<UiFonts>) {
    let tf = |f: &Handle<Font>, s: f32| TextFont { font: f.clone(), font_size: s, ..default() };
    // red vignette for damage / red sight
    c.spawn((
        Node { position_type: PositionType::Absolute, width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() },
        BackgroundColor(Color::NONE),
        HudVignette,
        GlobalZIndex(-1),
    ));
    c.spawn((
        Node { position_type: PositionType::Absolute, width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() },
        HudRoot,
        Pickable::IGNORE,
    ))
    .with_children(|p| {
        p.spawn((
            Node { position_type: PositionType::Absolute, left: Val::Px(18.0), top: Val::Px(14.0), flex_direction: FlexDirection::Column, row_gap: Val::Px(3.0), ..default() },
        ))
        .with_children(|p| {
            p.spawn((Text::new(""), tf(&fonts.mono_b, 15.0), TextColor(INK), HudTopLeft, HudText(0)));
            p.spawn((Text::new(""), tf(&fonts.serif_i, 16.0), TextColor(DIM), HudObjective, HudText(1), Node { max_width: Val::Px(420.0), ..default() }));
        });
        p.spawn((
            Node { position_type: PositionType::Absolute, right: Val::Px(18.0), top: Val::Px(14.0), flex_direction: FlexDirection::Column, align_items: AlignItems::FlexEnd, row_gap: Val::Px(4.0), ..default() },
        ))
        .with_children(|p| {
            p.spawn((Text::new(""), tf(&fonts.mono, 14.0), TextColor(INK), HudTopRight, HudText(2), TextLayout::new_with_justify(JustifyText::Right)));
            p.spawn((Node { width: Val::Px(160.0), height: Val::Px(5.0), ..default() }, BackgroundColor(Color::NONE)))
                .with_children(|p| {
                    p.spawn((Node { width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() }, BackgroundColor(RED), HudHealth));
                });
            p.spawn((Node { width: Val::Px(160.0), height: Val::Px(3.0), ..default() }, BackgroundColor(Color::NONE)))
                .with_children(|p| {
                    p.spawn((Node { width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() }, BackgroundColor(Color::srgb(0.6, 0.6, 0.7)), HudStamina));
                });
            p.spawn((Text::new(""), tf(&fonts.mono, 14.0), TextColor(AMBER), HudWeapon, HudText(3), TextLayout::new_with_justify(JustifyText::Right)));
        });
        // prompt at bottom centre
        p.spawn((
            Node { position_type: PositionType::Absolute, bottom: Val::Px(70.0), width: Val::Percent(100.0), justify_content: JustifyContent::Center, ..default() },
        ))
        .with_children(|p| {
            p.spawn((Text::new(""), tf(&fonts.mono_b, 17.0), TextColor(INK), HudPrompt, HudText(4), TextLayout::new_with_justify(JustifyText::Center)));
        });
        // echo caption
        p.spawn((
            Node { position_type: PositionType::Absolute, top: Val::Percent(22.0), width: Val::Percent(100.0), justify_content: JustifyContent::Center, ..default() },
        ))
        .with_children(|p| {
            p.spawn((Text::new(""), tf(&fonts.serif_i, 26.0), TextColor(Color::srgb(0.85, 0.9, 1.0)), HudCaption, HudText(5), TextLayout::new_with_justify(JustifyText::Center)));
        });
        // toasts bottom left
        p.spawn((
            Node { position_type: PositionType::Absolute, left: Val::Px(18.0), bottom: Val::Px(18.0), flex_direction: FlexDirection::Column, ..default() },
        ))
        .with_children(|p| {
            p.spawn((Text::new(""), tf(&fonts.mono, 14.0), TextColor(INK), HudToasts, HudText(6)));
        });
        // news ticker bottom right
        p.spawn((
            Node { position_type: PositionType::Absolute, right: Val::Px(18.0), bottom: Val::Px(18.0), max_width: Val::Px(520.0), ..default() },
        ))
        .with_children(|p| {
            p.spawn((Text::new(""), tf(&fonts.serif_i, 14.0), TextColor(FAINT), HudTicker, HudText(7), TextLayout::new_with_justify(JustifyText::Right)));
        });
        // big title
        p.spawn((
            Node { position_type: PositionType::Absolute, top: Val::Percent(38.0), width: Val::Percent(100.0), flex_direction: FlexDirection::Column, align_items: AlignItems::Center, ..default() },
        ))
        .with_children(|p| {
            p.spawn((Text::new(""), tf(&fonts.mono_b, 40.0), TextColor(RED), HudBig, HudText(8)));
            p.spawn((Text::new(""), tf(&fonts.serif_i, 20.0), TextColor(INK), HudBigSub, HudText(9)));
        });
        // crosshair (weapon aiming)
        p.spawn((
            Node { position_type: PositionType::Absolute, width: Val::Px(14.0), height: Val::Px(14.0), border: UiRect::all(Val::Px(2.0)), ..default() },
            BorderColor(RED),
            BorderRadius::all(Val::Px(7.0)),
            Visibility::Hidden,
            HudCrosshair,
        ));
        // bubble pool
        for i in 0..24 {
            p.spawn((
                Node { position_type: PositionType::Absolute, max_width: Val::Px(260.0), padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)), ..default() },
                BackgroundColor(Color::srgba(0.02, 0.02, 0.03, 0.72)),
                Visibility::Hidden,
                Bubble(i),
            ))
            .with_children(|p| {
                p.spawn((Text::new(""), tf(&fonts.serif, 15.0), TextColor(INK)));
            });
        }
    });
}

#[allow(clippy::too_many_arguments)]
pub fn update_hud(
    time: Res<Time>,
    game: Res<Game>,
    map: Option<Res<CityMap>>,
    prompt: Res<Prompt>,
    mut toasts: ResMut<Toasts>,
    rt: Res<PlayerRt>,
    crt: Res<CaseRt>,
    db: Res<CaseDb>,
    ui: Res<UiState>,
    mut texts: Query<(&HudText, &mut Text, &mut TextColor)>,
    mut bars: ParamSet<(Query<&mut Node, With<HudHealth>>, Query<&mut Node, With<HudStamina>>)>,
    mut vis: Query<&mut Visibility, With<HudRoot>>,
    mut vignette: Query<&mut BackgroundColor, With<HudVignette>>,
    mut mem: ResMut<HudMem>,
    act: Res<crate::keys::Act>,
    crime_rt: Res<crate::crime::CrimeRt>,
) {
    let dt = time.delta_secs();
    let in_world = matches!(ui.mode, Mode::None | Mode::Dialogue);
    if let Ok(mut v) = vis.single_mut() {
        *v = if matches!(ui.mode, Mode::Title) || game.phase == crate::state::Phase::Prologue && ui.mode == Mode::Cutscene { Visibility::Hidden } else { Visibility::Inherited };
    }
    let place = map
        .as_ref()
        .map(|m| {
            let m = &m.0;
            let p = game.player.pos;
            if let Some(b) = m.building_at(p) {
                let b = &m.buildings[b];
                if b.name.is_empty() {
                    format!("{} · {}", b.kind.label(), b.address)
                } else {
                    b.name.clone()
                }
            } else {
                let d = m.district_at(p).map(|d| m.districts[d].name.clone()).unwrap_or_default();
                format!("{} · {}", m.street_name_near(p), d)
            }
        })
        .unwrap_or_default();
    if place != mem.place {
        mem.place = place.clone();
        mem.place_t = 5.0;
    }
    mem.place_t -= dt;
    let mut out: [Option<String>; 10] = Default::default();
    let mut obj_col = DIM;
    let show_place = mem.place_t > 0.0 || act.held(crate::keys::Action::Map);
    out[0] = Some(if show_place { format!("{}  {}\n{}", clock_str(game.minute), game.date_str(), place) } else { clock_str(game.minute) });
    let wanted = match game.police.wanted {
        0 => String::new(),
        1 => "PROCURADO ●\n".into(),
        2 => "PROCURADO ●●\n".into(),
        _ => "PROCURADO ●●●\n".into(),
    };
    let _ = fmt_money;
    out[2] = Some(format!("{}{}", wanted, crate::economy::money_str(&game, game.player.money)));
    out[4] = Some(if in_world && ui.mode == Mode::None { prompt.0.clone().unwrap_or_default() } else { String::new() });
    for t in toasts.items.iter_mut() {
        t.1 -= dt;
    }
    toasts.items.retain(|t| t.1 > 0.0);
    out[6] = Some(toasts.items.iter().map(|t| format!("▸ {}", t.0)).collect::<Vec<_>>().join("\n"));
    let mut big = (String::new(), String::new());
    if let Some((a, b, tm)) = &mut toasts.big {
        *tm -= dt;
        big = (a.clone(), b.clone());
        if *tm <= 0.0 {
            toasts.big = None;
        }
    }
    out[8] = Some(big.0);
    out[9] = Some(big.1);
    out[5] = Some(crt.caption.as_ref().map(|c| c.0.clone()).unwrap_or_default());
    let obj = objective(&game, &db);
    if obj != mem.obj {
        mem.obj = obj.clone();
        mem.obj_t = 8.0;
    }
    mem.obj_t -= dt;
    let show = mem.obj_t > 0.0 || crt.red_sight || act.held(crate::keys::Action::CaseBoard);
    out[1] = Some(if show { obj } else { String::new() });
    if crt.red_sight {
        obj_col = RED;
    }
    let tick = game.news_ticker.last().cloned().unwrap_or_default();
    if tick != mem.tick {
        mem.tick = tick.clone();
        mem.tick_t = 9.0;
    }
    mem.tick_t -= dt;
    out[7] = Some(if tick.is_empty() || mem.tick_t <= 0.0 { String::new() } else { format!("rádio » {}", tick) });
    let w = game.player.weapon.stats();
    let weapon = if !rt.weapon_out {
        String::new()
    } else if w.melee {
        w.name.to_string()
    } else {
        format!("{}  {}/{}", w.name, game.player.mag, game.player.ammo())
    };
    // stealth eye: only while crouched or when someone is getting suspicious
    let sus = crime_rt.stealth_alert;
    let stealth = if rt.sneaking || sus > 0.05 {
        let state = if sus >= 1.0 {
            "VISTO"
        } else if sus > 0.35 {
            "alguém desconfia"
        } else if sus > 0.05 {
            "um ruído..."
        } else {
            "oculto"
        };
        let bar: String = (0..10).map(|i| if (i as f32) < sus.min(1.0) * 10.0 { '█' } else { '░' }).collect();
        format!("{} furtivo · {}\n{}\n", if sus >= 1.0 { "◉" } else { "◌" }, state, bar)
    } else {
        String::new()
    };
    out[3] = Some(format!("{}{}", stealth, weapon));
    for (tag, mut t, mut col) in texts.iter_mut() {
        if let Some(Some(s)) = out.get(tag.0 as usize) {
            if **t != *s {
                **t = s.clone();
            }
        }
        if tag.0 == 1 {
            col.0 = obj_col;
        }
    }
    if let Ok(mut n) = bars.p0().single_mut() {
        n.width = Val::Percent(game.player.health.clamp(0.0, 100.0));
        n.display = if game.player.health < 99.0 { Display::Flex } else { Display::None };
    }
    if let Ok(mut n) = bars.p1().single_mut() {
        n.width = Val::Percent(game.player.stamina.clamp(0.0, 1.0) * 100.0);
        n.display = if game.player.stamina < 0.99 { Display::Flex } else { Display::None };
    }
    if let Ok(mut bg) = vignette.single_mut() {
        let hurt = rt.hurt_flash;
        let a = (hurt * 0.4).max(if crt.red_sight { 0.18 } else { 0.0 }).max(if game.player.health < 30.0 { 0.12 } else { 0.0 });
        bg.0 = Color::srgba(0.5, 0.0, 0.02, a);
    }
}

fn objective(game: &Game, db: &CaseDb) -> String {
    if game.phase == crate::state::Phase::Prologue {
        return "Explore o laboratório abandonado.".into();
    }
    let Some((def, pi)) = current(game, db) else {
        return String::new();
    };
    let prog = &game.cases[pi];
    let n = prog.found.len();
    let total = def.clues.len();
    let step = if n == 0 {
        "Comece pela cena do crime (marcada no mapa [M])."
    } else if n < 4 {
        "Examine a cena, converse com testemunhas e suspeitos."
    } else if n < total / 2 {
        "Siga as pistas. Algumas mentem. Algumas pessoas também."
    } else {
        "Monte o quadro [Tab] e acuse — na delegacia ou cara a cara."
    };
    format!("CASO {:02} — {}\n{}  ({} evidências)", def.id, def.title.to_uppercase(), step, n)
}

/// Position speech bubbles over people's heads.
pub fn update_bubbles(
    sim: Res<Sim>,
    game: Res<Game>,
    map: Option<Res<CityMap>>,
    cams: Query<(&Camera, &GlobalTransform), With<MainCam>>,
    mut q: Query<(&Bubble, &mut Node, &mut Visibility, &Children)>,
    mut texts: Query<&mut Text>,
    ui: Res<UiState>,
    scale: Res<UiScale>,
    vis: Res<crate::render::city3d::CityVis>,
) {
    let Ok((cam, gt)) = cams.single() else { return };
    let Some(map) = map else { return };
    let pp = game.player.pos;
    // collect nearest speakers
    let mut speakers: Vec<(f32, usize)> = sim
        .agents
        .iter()
        .enumerate()
        .filter(|(_, a)| a.bubble.is_some() && a.pos.distance_squared(pp) < 18.0 * 18.0)
        .filter(|(_, a)| match map.0.building_at(a.pos) {
            Some(b) => vis.buildings.get(b).map(|v| v.cut).unwrap_or(true),
            None => true,
        })
        .map(|(i, a)| (a.pos.distance_squared(pp), i))
        .collect();
    speakers.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let show = matches!(ui.mode, Mode::None);
    // screen positions first, then push overlapping bubbles apart (upwards)
    let k = scale.0.max(0.01);
    let mut placed: Vec<(usize, Vec2)> = Vec::new();
    for (slot, (_, ai)) in speakers.iter().enumerate().take(8) {
        let a = &sim.agents[*ai];
        if let Ok(sp) = cam.world_to_viewport(gt, Vec3::new(a.pos.x, 2.25, a.pos.y)) {
            let mut p = Vec2::new(sp.x / k - 60.0, sp.y / k - 30.0);
            for _ in 0..6 {
                let hit = placed.iter().any(|(_, q)| (q.x - p.x).abs() < 190.0 && (q.y - p.y).abs() < 36.0);
                if !hit {
                    break;
                }
                p.y -= 38.0;
            }
            placed.push((slot, p));
        }
    }
    for (b, mut node, mut v, children) in q.iter_mut() {
        let Some((_, ai)) = speakers.get(b.0) else {
            *v = Visibility::Hidden;
            continue;
        };
        if !show {
            *v = Visibility::Hidden;
            continue;
        }
        let a = &sim.agents[*ai];
        let Some((_, pos)) = placed.iter().find(|(sl, _)| *sl == b.0) else {
            *v = Visibility::Hidden;
            continue;
        };
        *v = Visibility::Inherited;
        node.left = Val::Px(pos.x);
        node.top = Val::Px(pos.y);
        if let Some(&ch) = children.first() {
            if let Ok(mut t) = texts.get_mut(ch) {
                let s = &a.bubble.as_ref().unwrap().0;
                if **t != *s {
                    **t = s.clone();
                }
            }
        }
    }
}
