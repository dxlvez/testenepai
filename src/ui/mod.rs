//! User interface framework: every screen is described as a tree of `El`
//! elements by a pure function of the game state and rebuilt when dirty.

pub mod board;
pub mod dialogue;
pub mod hud;
pub mod menu;
pub mod overlay;
pub mod pause;
pub mod screens;

use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Mode {
    #[default]
    None,
    Pause,
    Dialogue,
    Board,
    Journal,
    Map,
    Inventory,
    Network,
    Shop,
    Cutscene,
    Choice,
    Title,
    Minigame,
    /// in-engine cutscene: input blocked, world keeps living
    Movie,
}

#[derive(Resource, Default)]
pub struct UiState {
    pub mode: Mode,
    pub dirty: bool,
    pub clicked: Option<String>,
    pub hovered: Option<String>,
    pub tab: usize,
    pub sub: usize,
    pub scroll: i32,
    pub sel: usize,
    pub rebinding: Option<crate::keys::Action>,
    pub text_input: Option<String>,
    pub confirm: Option<String>,
    pub message: Option<String>,
}

impl UiState {
    pub fn blocks_input(&self) -> bool {
        self.mode != Mode::None
    }
    pub fn pauses_world(&self) -> bool {
        matches!(
            self.mode,
            Mode::Pause | Mode::Board | Mode::Journal | Mode::Map | Mode::Inventory | Mode::Network | Mode::Cutscene | Mode::Choice | Mode::Title | Mode::Shop | Mode::Minigame
        )
    }
    pub fn open(&mut self, m: Mode) {
        self.mode = m;
        self.dirty = true;
        self.tab = 0;
        self.sub = 0;
        self.scroll = 0;
        self.sel = 0;
        self.clicked = None;
    }
    pub fn close(&mut self) {
        self.mode = Mode::None;
        self.dirty = true;
        self.rebinding = None;
        self.text_input = None;
        self.confirm = None;
    }
    pub fn take_click(&mut self) -> Option<String> {
        self.clicked.take()
    }
}

#[derive(Resource, Clone)]
pub struct UiFonts {
    pub mono: Handle<Font>,
    pub mono_b: Handle<Font>,
    pub serif: Handle<Font>,
    pub serif_b: Handle<Font>,
    pub serif_i: Handle<Font>,
}

pub fn load_fonts(mut c: Commands, mut fonts: ResMut<Assets<Font>>) {
    let f = |fonts: &mut Assets<Font>, b: &'static [u8]| fonts.add(Font::try_from_bytes(b.to_vec()).expect("font"));
    c.insert_resource(UiFonts {
        mono: f(&mut fonts, include_bytes!("../../assets/fonts/FreeMono.ttf")),
        mono_b: f(&mut fonts, include_bytes!("../../assets/fonts/FreeMonoBold.ttf")),
        serif: f(&mut fonts, include_bytes!("../../assets/fonts/LiberationSerif-Regular.ttf")),
        serif_b: f(&mut fonts, include_bytes!("../../assets/fonts/LiberationSerif-Bold.ttf")),
        serif_i: f(&mut fonts, include_bytes!("../../assets/fonts/LiberationSerif-Italic.ttf")),
    });
}

// ------------------------------------------------------------------ palette

pub const INK: Color = Color::srgb(0.86, 0.84, 0.80);
pub const DIM: Color = Color::srgb(0.55, 0.53, 0.52);
pub const FAINT: Color = Color::srgb(0.34, 0.32, 0.36);
pub const RED: Color = Color::srgb(0.93, 0.08, 0.14);
pub const DARKRED: Color = Color::srgb(0.45, 0.03, 0.07);
pub const PAPER: Color = Color::srgb(0.84, 0.79, 0.68);
pub const PAPER_INK: Color = Color::srgb(0.12, 0.1, 0.09);
pub const AMBER: Color = Color::srgb(1.0, 0.72, 0.38);
pub const PANEL: Color = Color::srgba(0.03, 0.025, 0.04, 0.93);
pub const GREEN: Color = Color::srgb(0.45, 0.75, 0.5);

#[derive(Clone, Copy, PartialEq)]
pub enum Fnt {
    Mono,
    MonoB,
    Serif,
    SerifB,
    SerifI,
}

impl UiFonts {
    pub fn get(&self, f: Fnt) -> Handle<Font> {
        match f {
            Fnt::Mono => self.mono.clone(),
            Fnt::MonoB => self.mono_b.clone(),
            Fnt::Serif => self.serif.clone(),
            Fnt::SerifB => self.serif_b.clone(),
            Fnt::SerifI => self.serif_i.clone(),
        }
    }
}

// ------------------------------------------------------------------ elements

#[derive(Clone)]
pub struct Sty {
    pub w: Val,
    pub h: Val,
    pub min_w: Val,
    pub max_w: Val,
    pub pad: UiRect,
    pub margin: UiRect,
    pub gap: f32,
    pub bg: Option<Color>,
    pub border: Option<Color>,
    pub border_w: f32,
    pub align: AlignItems,
    pub justify: JustifyContent,
    pub grow: f32,
    pub abs: Option<(Val, Val)>,
    pub wrap: bool,
    pub radius: f32,
    pub scroll: bool,
}

impl Default for Sty {
    fn default() -> Self {
        Sty {
            w: Val::Auto,
            h: Val::Auto,
            min_w: Val::Auto,
            max_w: Val::Auto,
            pad: UiRect::ZERO,
            margin: UiRect::ZERO,
            gap: 0.0,
            bg: None,
            border: None,
            border_w: 1.0,
            align: AlignItems::Stretch,
            justify: JustifyContent::FlexStart,
            grow: 0.0,
            abs: None,
            wrap: false,
            radius: 0.0,
            scroll: false,
        }
    }
}

#[derive(Clone)]
pub enum El {
    Col(Sty, Vec<El>),
    Row(Sty, Vec<El>),
    Text { s: String, size: f32, color: Color, font: Fnt, max_w: Option<f32> },
    Btn { id: String, sty: Sty, kids: Vec<El>, active: bool },
    Img { h: Handle<Image>, w: f32, hgt: f32, kids: Vec<El> },
}

pub fn col(kids: Vec<El>) -> El {
    El::Col(Sty::default(), kids)
}
pub fn row(kids: Vec<El>) -> El {
    El::Row(Sty { align: AlignItems::Center, ..default() }, kids)
}
pub fn txt(s: impl Into<String>, size: f32, color: Color, font: Fnt) -> El {
    El::Text { s: s.into(), size, color, font, max_w: None }
}
pub fn t(s: impl Into<String>) -> El {
    txt(s, 17.0, INK, Fnt::Serif)
}
pub fn mono(s: impl Into<String>, size: f32, color: Color) -> El {
    txt(s, size, color, Fnt::Mono)
}
pub fn wrap(s: impl Into<String>, size: f32, color: Color, font: Fnt, max_w: f32) -> El {
    El::Text { s: s.into(), size, color, font, max_w: Some(max_w) }
}
pub fn space(h: f32) -> El {
    El::Col(Sty { h: Val::Px(h), w: Val::Px(h), ..default() }, vec![])
}
pub fn grow() -> El {
    El::Col(Sty { grow: 1.0, ..default() }, vec![])
}
pub fn btn(id: impl Into<String>, label: impl Into<String>) -> El {
    El::Btn {
        id: id.into(),
        sty: Sty {
            pad: UiRect::axes(Val::Px(14.0), Val::Px(7.0)),
            bg: Some(Color::srgba(0.08, 0.06, 0.09, 0.95)),
            border: Some(FAINT),
            align: AlignItems::Center,
            justify: JustifyContent::Center,
            ..default()
        },
        kids: vec![mono(label, 16.0, INK)],
        active: false,
    }
}
pub fn btn_w(id: impl Into<String>, label: impl Into<String>, w: f32) -> El {
    let mut b = btn(id, label);
    if let El::Btn { sty, .. } = &mut b {
        sty.w = Val::Px(w);
    }
    b
}
/// A list-style option (dialogue answers, menu entries).
pub fn opt(id: impl Into<String>, label: impl Into<String>, color: Color) -> El {
    El::Btn {
        id: id.into(),
        sty: Sty { pad: UiRect::axes(Val::Px(10.0), Val::Px(5.0)), ..default() },
        kids: vec![txt(label, 18.0, color, Fnt::Serif)],
        active: false,
    }
}

impl El {
    pub fn sty(mut self, f: impl FnOnce(&mut Sty)) -> El {
        match &mut self {
            El::Col(s, _) | El::Row(s, _) => f(s),
            El::Btn { sty, .. } => f(sty),
            _ => {}
        }
        self
    }
    pub fn bg(self, c: Color) -> El {
        self.sty(|s| s.bg = Some(c))
    }
    pub fn border(self, c: Color) -> El {
        self.sty(|s| s.border = Some(c))
    }
    pub fn pad(self, p: f32) -> El {
        self.sty(|s| s.pad = UiRect::all(Val::Px(p)))
    }
    pub fn gap(self, g: f32) -> El {
        self.sty(|s| s.gap = g)
    }
    pub fn w(self, w: f32) -> El {
        self.sty(|s| s.w = Val::Px(w))
    }
    pub fn h(self, h: f32) -> El {
        self.sty(|s| s.h = Val::Px(h))
    }
    pub fn wp(self, w: f32) -> El {
        self.sty(|s| s.w = Val::Percent(w))
    }
    pub fn hp(self, h: f32) -> El {
        self.sty(|s| s.h = Val::Percent(h))
    }
    pub fn center(self) -> El {
        self.sty(|s| {
            s.align = AlignItems::Center;
            s.justify = JustifyContent::Center;
        })
    }
    pub fn align(self, a: AlignItems) -> El {
        self.sty(|s| s.align = a)
    }
    pub fn justify(self, j: JustifyContent) -> El {
        self.sty(|s| s.justify = j)
    }
    pub fn grow(self) -> El {
        self.sty(|s| s.grow = 1.0)
    }
    pub fn abs(self, l: f32, t: f32) -> El {
        self.sty(|s| s.abs = Some((Val::Px(l), Val::Px(t))))
    }
    pub fn margin(self, m: f32) -> El {
        self.sty(|s| s.margin = UiRect::all(Val::Px(m)))
    }
    pub fn wrap_row(self) -> El {
        self.sty(|s| s.wrap = true)
    }
    pub fn active(mut self, a: bool) -> El {
        if let El::Btn { active, .. } = &mut self {
            *active = a;
        }
        self
    }
    pub fn scroll(self) -> El {
        self.sty(|s| s.scroll = true)
    }
}

#[derive(Component)]
pub struct UiBtn {
    pub id: String,
    pub active: bool,
    pub bg: Color,
    pub border: Option<Color>,
}

#[derive(Component)]
pub struct OverlayRoot;

fn node_from(sty: &Sty, dir: FlexDirection) -> Node {
    let mut n = Node {
        flex_direction: dir,
        width: sty.w,
        height: sty.h,
        min_width: sty.min_w,
        max_width: sty.max_w,
        padding: sty.pad,
        margin: sty.margin,
        row_gap: Val::Px(sty.gap),
        column_gap: Val::Px(sty.gap),
        align_items: sty.align,
        justify_content: sty.justify,
        flex_grow: sty.grow,
        flex_wrap: if sty.wrap { FlexWrap::Wrap } else { FlexWrap::NoWrap },
        border: if sty.border.is_some() { UiRect::all(Val::Px(sty.border_w)) } else { UiRect::ZERO },
        overflow: if sty.scroll { Overflow::clip() } else { Overflow::visible() },
        ..default()
    };
    if let Some((l, t)) = sty.abs {
        n.position_type = PositionType::Absolute;
        n.left = l;
        n.top = t;
    }
    n
}

pub fn spawn_el(p: &mut ChildSpawnerCommands, el: &El, fonts: &UiFonts) {
    match el {
        El::Col(s, kids) | El::Row(s, kids) => {
            let dir = if matches!(el, El::Col(..)) { FlexDirection::Column } else { FlexDirection::Row };
            let mut e = p.spawn(node_from(s, dir));
            if let Some(bg) = s.bg {
                e.insert(BackgroundColor(bg));
            }
            if let Some(b) = s.border {
                e.insert(BorderColor(b));
            }
            e.with_children(|c| {
                for k in kids {
                    spawn_el(c, k, fonts);
                }
            });
        }
        El::Text { s, size, color, font, max_w } => {
            let mut e = p.spawn((
                Text::new(s.clone()),
                TextFont { font: fonts.get(*font), font_size: *size, ..default() },
                TextColor(*color),
            ));
            if let Some(w) = max_w {
                e.insert(Node { max_width: Val::Px(*w), ..default() });
            }
        }
        El::Img { h, w, hgt, kids } => {
            p.spawn((ImageNode::new(h.clone()), Node { width: Val::Px(*w), height: Val::Px(*hgt), ..default() })).with_children(|c| {
                for k in kids {
                    spawn_el(c, k, fonts);
                }
            });
        }
        El::Btn { id, sty, kids, active } => {
            let bg = if *active { Some(Color::srgba(0.3, 0.04, 0.07, 0.95)) } else { sty.bg };
            let mut e = p.spawn((node_from(sty, FlexDirection::Row), Button, UiBtn { id: id.clone(), active: *active, bg: bg.unwrap_or(Color::NONE), border: sty.border }));
            e.insert(BackgroundColor(bg.unwrap_or(Color::NONE)));
            if let Some(b) = sty.border {
                e.insert(BorderColor(if *active { RED } else { b }));
            }
            e.with_children(|c| {
                for k in kids {
                    spawn_el(c, k, fonts);
                }
            });
        }
    }
}

/// Button hover / click feedback + reporting clicks to UiState.
pub fn button_system(
    mut q: Query<(&Interaction, &UiBtn, &mut BackgroundColor, Option<&mut BorderColor>), Changed<Interaction>>,
    mut ui: ResMut<UiState>,
    mut sfx: EventWriter<crate::audio::Sfx>,
) {
    for (i, b, mut bg, border) in q.iter_mut() {
        match i {
            Interaction::Pressed => {
                ui.clicked = Some(b.id.clone());
                sfx.write(crate::audio::Sfx::Click);
            }
            Interaction::Hovered => {
                bg.0 = if b.bg.alpha() > 0.9 && (b.id.starts_with("clue:") || b.id.starts_with("person:")) { Color::srgb(0.95, 0.9, 0.8) } else { Color::srgba(0.28, 0.05, 0.08, 0.95) };
                if let Some(mut bc) = border {
                    bc.0 = RED;
                }
                if ui.hovered.as_deref() != Some(b.id.as_str()) {
                    sfx.write(crate::audio::Sfx::Tick);
                }
                ui.hovered = Some(b.id.clone());
            }
            Interaction::None => {
                bg.0 = b.bg;
                if let Some(mut bc) = border {
                    bc.0 = if b.active { RED } else { b.border.unwrap_or(FAINT) };
                }
            }
        }
    }
}

/// Scale the 1280x720 design to the window.
pub fn ui_scale(windows: Query<&Window>, mut s: ResMut<UiScale>) {
    if let Ok(w) = windows.single() {
        let k = (w.height() / 720.0).min(w.width() / 1280.0).max(0.5);
        if (s.0 - k).abs() > 0.01 {
            s.0 = k;
        }
    }
}

/// Full-screen dim backdrop with centered content.
pub fn modal(content: El) -> El {
    El::Col(
        Sty {
            w: Val::Percent(100.0),
            h: Val::Percent(100.0),
            bg: Some(Color::srgba(0.0, 0.0, 0.0, 0.72)),
            align: AlignItems::Center,
            justify: JustifyContent::Center,
            ..default()
        },
        vec![content],
    )
}

pub fn panel(kids: Vec<El>) -> El {
    col(kids).bg(PANEL).border(Color::srgb(0.35, 0.3, 0.38)).pad(22.0).gap(8.0)
}

pub fn title(s: impl Into<String>) -> El {
    txt(s, 30.0, INK, Fnt::SerifB)
}

pub fn hr() -> El {
    El::Col(Sty { h: Val::Px(1.0), w: Val::Percent(100.0), bg: Some(FAINT), margin: UiRect::vertical(Val::Px(6.0)), ..default() }, vec![])
}
