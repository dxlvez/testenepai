//! Modal screens: popups/cutscene cards, choice menus, lockpick minigame,
//! the bag (inventory), shops, the map, journal, network & relationships.

use super::*;
use crate::audio::Sfx;
use crate::cases::run::CaseDb;
use crate::economy::{money_str, price};
use crate::items::*;
use crate::sim::people::*;
use crate::state::*;
use std::collections::VecDeque;

// ------------------------------------------------------------------ popups

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PopStyle {
    Plain,
    Evidence,
    Headline,
    Intro,
    Timeline,
    Death,
    Limbo,
    Mystery,
}

#[derive(Clone, Debug)]
pub struct Popup {
    pub title: String,
    pub sub: String,
    pub body: String,
    pub style: PopStyle,
    pub lines: Vec<String>,
    /// typewriter reveal
    pub reveal: f32,
}

impl Popup {
    pub fn plain(title: &str, body: &str) -> Popup {
        Popup { title: title.into(), sub: String::new(), body: body.into(), style: PopStyle::Plain, lines: vec![], reveal: 0.0 }
    }
    pub fn new(style: PopStyle, title: impl Into<String>, sub: impl Into<String>, body: impl Into<String>) -> Popup {
        Popup { title: title.into(), sub: sub.into(), body: body.into(), style, lines: vec![], reveal: 0.0 }
    }
    pub fn with_lines(mut self, l: Vec<String>) -> Popup {
        self.lines = l;
        self
    }
    pub fn death() -> Popup {
        Popup::new(PopStyle::Death, "Você morreu.", "", "Silêncio. Depois, estática. Uma voz de rádio, muito longe: \"...ainda não, Elias...\"")
    }
}

#[derive(Resource, Default)]
pub struct Popups {
    pub q: VecDeque<Popup>,
    pub on_close: Vec<String>,
}

impl Popups {
    pub fn push(&mut self, p: Popup) {
        self.q.push_back(p);
    }
    pub fn then(&mut self, action: impl Into<String>) {
        self.on_close.push(action.into());
    }
}

pub fn build_popup(p: &Popup) -> El {
    let shown: String = p.body.chars().take(p.reveal as usize).collect();
    let (title_col, bg, font) = match p.style {
        PopStyle::Evidence => (PAPER_INK, PAPER, Fnt::MonoB),
        PopStyle::Headline => (PAPER_INK, Color::srgb(0.88, 0.85, 0.76), Fnt::SerifB),
        PopStyle::Timeline => (RED, Color::srgba(0.02, 0.0, 0.01, 0.97), Fnt::MonoB),
        PopStyle::Death | PopStyle::Limbo | PopStyle::Mystery => (INK, Color::srgb(0.0, 0.0, 0.0), Fnt::SerifI),
        PopStyle::Intro => (INK, Color::srgba(0.01, 0.01, 0.02, 0.97), Fnt::MonoB),
        PopStyle::Plain => (INK, PANEL, Fnt::SerifB),
    };
    let body_col = if matches!(p.style, PopStyle::Evidence | PopStyle::Headline) { PAPER_INK } else { INK };
    let mut kids = vec![];
    if !p.sub.is_empty() {
        kids.push(mono(p.sub.clone(), 14.0, if matches!(p.style, PopStyle::Evidence | PopStyle::Headline) { Color::srgb(0.4, 0.1, 0.1) } else { RED }));
    }
    let title_size = match p.style {
        PopStyle::Headline => 34.0,
        PopStyle::Timeline => 32.0,
        PopStyle::Death => 38.0,
        _ => 28.0,
    };
    kids.push(wrap(p.title.clone(), title_size, title_col, font, 760.0));
    kids.push(space(8.0));
    kids.push(wrap(shown, if p.style == PopStyle::Mystery || p.style == PopStyle::Limbo { 22.0 } else { 19.0 }, body_col, if matches!(p.style, PopStyle::Evidence) { Fnt::Mono } else { Fnt::Serif }, 760.0));
    if !p.lines.is_empty() && p.reveal as usize >= p.body.chars().count() {
        kids.push(space(10.0));
        for l in &p.lines {
            kids.push(mono(format!("· {}", l), 15.0, if p.style == PopStyle::Timeline { INK } else { body_col }));
        }
    }
    kids.push(space(14.0));
    kids.push(mono("[Enter] continuar", 13.0, if matches!(p.style, PopStyle::Evidence | PopStyle::Headline) { Color::srgb(0.35, 0.3, 0.25) } else { FAINT }));
    let card = col(kids).bg(bg).pad(34.0).gap(4.0).w(840.0);
    let card = if matches!(p.style, PopStyle::Death | PopStyle::Limbo | PopStyle::Mystery) { card } else { card.border(Color::srgb(0.3, 0.25, 0.25)) };
    El::Col(
        Sty {
            w: Val::Percent(100.0),
            h: Val::Percent(100.0),
            bg: Some(if matches!(p.style, PopStyle::Death | PopStyle::Limbo | PopStyle::Mystery) { Color::BLACK } else { Color::srgba(0.0, 0.0, 0.0, 0.6) }),
            align: AlignItems::Center,
            justify: JustifyContent::Center,
            ..default()
        },
        vec![btn_card(card)],
    )
}

fn btn_card(card: El) -> El {
    El::Btn { id: "popup_next".into(), sty: Sty { ..default() }, kids: vec![card], active: false }
}

// ------------------------------------------------------------------ choice menus

#[derive(Clone, Debug, Default)]
pub struct Choice {
    pub title: String,
    pub body: String,
    pub opts: Vec<(String, String)>,
    pub ctx: String,
}

#[derive(Resource, Default)]
pub struct Choices {
    pub cur: Option<Choice>,
    pub picked: Option<(String, String)>,
}

pub fn build_choice(c: &Choice) -> El {
    let mut kids = vec![title(c.title.clone())];
    if !c.body.is_empty() {
        kids.push(wrap(c.body.clone(), 17.0, DIM, Fnt::Serif, 520.0));
    }
    kids.push(hr());
    for (i, (id, label)) in c.opts.iter().enumerate() {
        kids.push(opt(format!("c:{}", id), format!("{}. {}", i + 1, label), INK));
    }
    modal(panel(kids).w(580.0))
}

// ------------------------------------------------------------------ lockpick minigame

#[derive(Resource, Default)]
pub struct Lockpick {
    pub active: bool,
    pub pins: u32,
    pub done: u32,
    pub pos: f32,
    pub dir: f32,
    pub zone: (f32, f32),
    pub fails: u32,
    pub ctx: String,
    pub result: Option<bool>,
    pub era_label: String,
}

pub fn build_lockpick(l: &Lockpick) -> El {
    let w = 520.0;
    let bar = El::Row(
        Sty { w: Val::Px(w), h: Val::Px(26.0), bg: Some(Color::srgb(0.1, 0.1, 0.12)), border: Some(FAINT), ..default() },
        vec![
            El::Col(Sty { abs: Some((Val::Px(l.zone.0 * w), Val::Px(0.0))), w: Val::Px((l.zone.1 - l.zone.0) * w), h: Val::Px(24.0), bg: Some(Color::srgba(0.2, 0.7, 0.3, 0.6)), ..default() }, vec![]),
            El::Col(Sty { abs: Some((Val::Px(l.pos * w - 2.0), Val::Px(0.0))), w: Val::Px(4.0), h: Val::Px(24.0), bg: Some(RED), ..default() }, vec![]),
        ],
    );
    let pins: String = (0..l.pins).map(|i| if i < l.done { "■ " } else { "□ " }).collect();
    modal(
        panel(vec![
            title(l.era_label.clone()),
            mono(format!("Pinos: {}", pins), 18.0, INK),
            bar,
            wrap("Aperte [E] quando o marcador vermelho estiver na zona verde. Três erros e você faz barulho demais.", 15.0, DIM, Fnt::Serif, 520.0),
            mono(format!("Erros: {}/3   [Esc] desistir", l.fails), 14.0, FAINT),
        ])
        .w(580.0),
    )
}

// ------------------------------------------------------------------ bag / inventory

pub fn build_bag(game: &Game, ui: &UiState) -> El {
    let p = &game.player;
    let mut items: Vec<(String, usize, &Item)> = Vec::new();
    for it in &p.inv {
        let n = it.name(game.year);
        if let Some(e) = items.iter_mut().find(|e| e.2 == it) {
            e.1 += 1;
        } else {
            items.push((n, 1, it));
        }
    }
    let mut list = vec![];
    for (i, (n, c, it)) in items.iter().enumerate() {
        let val = it.sell_value();
        let tag = match it {
            Item::Weapon(_) => "arma",
            Item::Ammo(_) => "munição",
            Item::Drug(_) => "substância",
            Item::Tool(_) => "ferramenta",
            Item::Gift(_) => "presente",
            Item::Clothes(_) => "roupa",
            Item::Valuable(..) => "valor",
            Item::Evidence(..) => "evidência",
            Item::Food => "comida",
            Item::Newspaper(_) => "jornal",
        };
        let label = format!("{}{}   [{}]{}", n, if *c > 1 { format!(" x{}", c) } else { String::new() }, tag, if val > 0 { format!("  ~{}", money_str(game, price(game, val))) } else { String::new() });
        list.push(opt(format!("bag:{}", i), label, if ui.sel == i { AMBER } else { INK }).active(ui.sel == i));
    }
    if list.is_empty() {
        list.push(t("A bolsa está vazia."));
    }
    // detail of selected
    let mut detail = vec![];
    if let Some((n, _, it)) = items.get(ui.sel) {
        detail.push(txt(n.clone(), 22.0, INK, Fnt::SerifB));
        let d = match it {
            Item::Weapon(w) => {
                let s = w.stats();
                format!("Dano {:.0} · alcance {:.0}m · {}", s.damage, s.range, if s.melee { "corpo a corpo".into() } else { format!("pente de {}", s.mag) })
            }
            Item::Drug(d) => d.desc().to_string(),
            Item::Tool(Tool::Lockpick) => "Abre portas, janelas e carros sem barulho.".into(),
            Item::Tool(Tool::Crowbar) => "Força janelas e cofres. Faz algum barulho.".into(),
            Item::Tool(Tool::Camera) => "Fotografe evidências e pessoas. Fotos antigas revelam diferenças entre linhas do tempo.".into(),
            Item::Tool(Tool::Bandage) => "Recupera um pouco de saúde.".into(),
            Item::Tool(Tool::Rope) => "Sempre na bolsa. Para amarrar quem precisa ficar quieto.".into(),
            Item::Tool(_) => String::new(),
            Item::Gift(_) => "Um presente pode mudar o dia de alguém.".into(),
            Item::Clothes(o) => format!("Vestir: {}.", o.name()),
            Item::Valuable(_, v) => format!("Vale cerca de {} no mercado negro.", money_str(game, price(game, *v))),
            Item::Food => "Mata a fome.".into(),
            _ => String::new(),
        };
        detail.push(wrap(d, 16.0, DIM, Fnt::Serif, 380.0));
        detail.push(space(10.0));
        let use_label = match it {
            Item::Weapon(_) => Some("Equipar"),
            Item::Drug(Drug::Laudanum) | Item::Drug(Drug::Stimulant) | Item::Drug(Drug::Visionary) => Some("Usar em si mesmo"),
            Item::Tool(Tool::Bandage) | Item::Food => Some("Usar"),
            Item::Clothes(_) => Some("Vestir"),
            Item::Newspaper(_) => Some("Ler"),
            _ => None,
        };
        if let Some(u) = use_label {
            detail.push(btn("bag_use", u));
        }
        if !matches!(it, Item::Tool(Tool::Rope)) {
            detail.push(btn("bag_drop", "Largar"));
        }
    }
    let outfits: Vec<El> = p.owned_outfits.iter().map(|o| opt(format!("outfit:{:?}", o), format!("{}{}", if *o == p.outfit { "● " } else { "  " }, o.name()), if *o == p.outfit { AMBER } else { DIM })).collect();
    modal(
        panel(vec![
            row(vec![title("BOLSA"), grow(), mono(format!("{}   saúde {:.0}   ", money_str(game, p.money), p.health), 16.0, AMBER)]),
            hr(),
            row(vec![
                col(list).gap(1.0).w(560.0),
                col(vec![col(detail).gap(4.0), hr(), mono("ROUPAS", 13.0, RED), col(outfits)]).w(400.0).gap(4.0),
            ])
            .gap(20.0)
            .align(AlignItems::FlexStart),
            mono("[I] ou [Esc] fechar   [↑↓] selecionar", 13.0, FAINT),
        ])
        .w(1020.0),
    )
}

// ------------------------------------------------------------------ shops

#[derive(Resource, Default)]
pub struct Shop {
    pub keeper: Option<Pid>,
    pub kind: Option<crate::city::map::BKind>,
    pub black: bool,
    pub stock: Vec<(Item, i32)>,
}

pub fn shop_stock(kind: Option<crate::city::map::BKind>, job: Job, year: i32, black: bool) -> Vec<(Item, i32)> {
    use crate::city::map::BKind;
    let mut v: Vec<(Item, i32)> = Vec::new();
    if black {
        v.push((Item::Weapon(Weapon::Revolver), 40));
        v.push((Item::Weapon(if year >= 1980 { Weapon::Automatic } else { Weapon::Pistol }), 55));
        if year >= 1921 && year < 1960 {
            v.push((Item::Weapon(Weapon::Tommy), 220));
        }
        if year >= 1935 {
            v.push((Item::Weapon(Weapon::Magnum), 110));
        }
        v.push((Item::Ammo(12), 6));
        for d in [Drug::Chloroform, Drug::Chloral, Drug::Laudanum, Drug::TruthSerum, Drug::Stimulant, Drug::Visionary] {
            v.push((Item::Drug(d), d.price() + 4));
        }
        v.push((Item::Tool(Tool::Lockpick), 10));
        v.push((Item::Tool(Tool::ForgedPapers), 70));
        v.push((Item::Clothes(Outfit::Police), 45));
        v.push((Item::Clothes(Outfit::Doctor), 30));
        v.push((Item::Clothes(Outfit::Priest), 20));
        return v;
    }
    match (kind, job) {
        (Some(BKind::Clothing), _) | (_, Job::Tailor) => {
            for o in Outfit::ALL {
                if o.tailor() {
                    v.push((Item::Clothes(o), o.price()));
                }
            }
        }
        (Some(BKind::GunShop), _) | (_, Job::Gunsmith) => {
            v.push((Item::Weapon(Weapon::Revolver), 25));
            v.push((Item::Weapon(Weapon::Shotgun), 30));
            v.push((Item::Weapon(if year >= 1980 { Weapon::Automatic } else { Weapon::Pistol }), 35));
            v.push((Item::Weapon(Weapon::Knife), 6));
            v.push((Item::Ammo(12), 4));
            v.push((Item::Tool(Tool::Flashlight), 5));
        }
        (Some(BKind::Pharmacy), _) | (_, Job::Pharmacist) => {
            v.push((Item::Tool(Tool::Bandage), 2));
            v.push((Item::Drug(Drug::Laudanum), 6));
            if year >= 1935 {
                v.push((Item::Drug(Drug::Stimulant), 8));
            }
        }
        (Some(BKind::Pawn), _) | (_, Job::Pawnbroker) | (_, Job::Forger) => {
            v.push((Item::Gift(Gift::Jewel), 45));
            v.push((Item::Tool(Tool::Camera), 40));
            v.push((Item::Tool(Tool::Lockpick), 8));
            v.push((Item::Tool(Tool::Crowbar), 4));
            v.push((Item::Weapon(Weapon::Razor), 5));
            v.push((Item::Gift(Gift::Record), 4));
        }
        (Some(BKind::Bar), _) | (Some(BKind::Club), _) | (_, Job::Bartender) => {
            v.push((Item::Gift(Gift::Whiskey), 5));
            v.push((Item::Food, 1));
        }
        _ => {
            v.push((Item::Food, 1));
            v.push((Item::Gift(Gift::Flowers), 2));
            v.push((Item::Gift(Gift::Chocolates), 3));
            v.push((Item::Gift(Gift::Book), 6));
            v.push((Item::Gift(Gift::Perfume), 12));
            v.push((Item::Tool(Tool::Crowbar), 4));
            v.push((Item::Tool(Tool::Flashlight), 5));
            if year >= 1925 {
                v.push((Item::Tool(Tool::Camera), 40));
            }
        }
    }
    v
}

pub fn build_shop(shop: &Shop, game: &Game, ui: &UiState) -> El {
    let keeper = shop.keeper.map(|k| game.pop.get(k).name()).unwrap_or_default();
    let mut buy = vec![];
    for (i, (it, base)) in shop.stock.iter().enumerate() {
        let p = price(game, *base);
        let can = game.player.money >= p;
        buy.push(opt(format!("buy:{}", i), format!("{}  —  {}", it.name(game.year), money_str(game, p)), if can { INK } else { FAINT }));
    }
    let mut sell = vec![];
    let fence = shop.black || matches!(shop.kind, Some(crate::city::map::BKind::Pawn));
    for (i, it) in game.player.inv.iter().enumerate() {
        let v = it.sell_value();
        let stolen_ok = fence || !matches!(it, Item::Valuable(..));
        if v > 0 && stolen_ok && !matches!(it, Item::Evidence(..) | Item::Tool(Tool::Rope)) {
            let pv = price(game, (v as f32 * if fence { 0.6 } else { 0.4 }) as i32);
            sell.push(opt(format!("sell:{}", i), format!("{}  +{}", it.name(game.year), money_str(game, pv)), DIM));
        }
    }
    if sell.is_empty() {
        sell.push(mono(if fence { "Nada que interesse." } else { "Objetos roubados só no penhor ou no mercado negro." }, 14.0, FAINT));
    }
    let _ = ui;
    modal(
        panel(vec![
            row(vec![title(if shop.black { "MERCADO NEGRO".to_string() } else { keeper.clone() }), grow(), mono(money_str(game, game.player.money), 18.0, AMBER)]),
            if shop.black { wrap(format!("{} olha para os lados antes de abrir o casaco.", keeper), 15.0, DIM, Fnt::SerifI, 900.0) } else { space(1.0) },
            hr(),
            row(vec![
                col(vec![mono("COMPRAR", 13.0, RED), col(buy).gap(1.0)]).w(480.0),
                col(vec![mono("VENDER", 13.0, RED), col(sell).gap(1.0)]).w(480.0),
            ])
            .gap(20.0)
            .align(AlignItems::FlexStart),
            btn("shop_close", "Sair [Esc]"),
        ])
        .w(1000.0),
    )
}

// ------------------------------------------------------------------ journal

pub fn build_journal(game: &Game, ui: &UiState, db: &CaseDb) -> El {
    let tabs = ["DIÁRIO", "JORNAIS", "MEMÓRIA DO MUNDO", "LINHAS DO TEMPO", "HABILIDADES", "CASOS", "FOTOS"];
    let mut tab_row = vec![];
    for (i, t) in tabs.iter().enumerate() {
        tab_row.push(btn(format!("tab:{}", i), *t).active(ui.tab == i));
    }
    let mut body: Vec<El> = vec![];
    let per = 11usize;
    match ui.tab {
        0 => {
            let entries: Vec<&JournalEntry> = game.journal.iter().rev().collect();
            for e in entries.iter().skip(ui.scroll.max(0) as usize).take(per) {
                body.push(col(vec![
                    mono(format!("{} · dia {}", e.year, e.day + 1), 12.0, FAINT),
                    wrap(e.text.clone(), 18.0, if e.thought { Color::srgb(0.82, 0.8, 0.9) } else { INK }, if e.thought { Fnt::SerifI } else { Fnt::Serif }, 960.0),
                ]));
            }
            if game.journal.is_empty() {
                body.push(t("Páginas em branco."));
            }
        }
        1 => {
            for h in game.papers.iter().rev().skip(ui.scroll.max(0) as usize).take(6) {
                body.push(
                    col(vec![
                        mono(format!("{} · {} · LINHA R-{:02}", h.city.upper(), h.year, h.timeline), 12.0, Color::srgb(0.4, 0.1, 0.1)),
                        wrap(h.title.clone(), 22.0, PAPER_INK, Fnt::SerifB, 940.0),
                        wrap(h.body.clone(), 15.0, Color::srgb(0.25, 0.22, 0.2), Fnt::Serif, 940.0),
                    ])
                    .bg(PAPER)
                    .pad(10.0),
                );
            }
            if game.papers.is_empty() {
                body.push(t("Nenhum jornal guardado."));
            }
        }
        2 => {
            for r in game.memory.iter().rev().skip(ui.scroll.max(0) as usize).take(4) {
                let mut k = vec![
                    mono(format!("LINHA R-{:02} · {} · {}", r.timeline, r.city.upper(), r.year), 12.0, RED),
                    txt(r.subject.clone(), 18.0, INK, Fnt::MonoB),
                    mono(format!("STATUS: {}", r.status), 14.0, if r.status.contains("FALSA") { RED } else { GREEN }),
                ];
                for c in r.consequences.iter().take(6) {
                    k.push(mono(format!("  → {}", c), 13.0, DIM));
                }
                body.push(col(k).gap(2.0));
            }
            // people Elias killed
            let dead: Vec<&Person> = game.pop.people.iter().filter(|p| matches!(p.life, Life::Dead { by_elias: true, .. })).collect();
            if !dead.is_empty() {
                body.push(hr());
                body.push(mono(format!("PESSOAS QUE VOCÊ MATOU: {}", dead.len()), 14.0, RED));
                for p in dead.iter().rev().take(6) {
                    let fam: Vec<String> = p.spouse.iter().chain(p.children.iter()).map(|q| {
                        let q = game.pop.get(*q);
                        let tag = if q.destiny.iter().any(|d| d == "police") { " → entrou para a polícia" } else if q.destiny.iter().any(|d| d == "criminal") { " → virou criminoso" } else if q.destiny.iter().any(|d| d == "left_city") { " → deixou a cidade" } else if q.destiny.iter().any(|d| d == "revenge") { " → quer vingança" } else { "" };
                        format!("{}{}", q.first, tag)
                    }).collect();
                    body.push(mono(format!("  {} ({}) — {}", p.name(), p.job.label(p.female), if fam.is_empty() { "sem família".into() } else { fam.join(", ") }), 13.0, DIM));
                }
            }
            if game.memory.is_empty() && dead.is_empty() {
                body.push(t("O mundo ainda não lembra de nada que você fez."));
            }
        }
        3 => {
            // timeline graph
            let mut line = String::from("ORIGINAL ─────────────────────────────\n");
            for c in game.cases.iter().filter(|c| c.solved) {
                let def = db.get(c.id);
                line.push_str(&format!(
                    "  {} ── R-{:02} ●  CASO {:02} {}  {}\n",
                    c.year,
                    c.timeline,
                    c.id,
                    def.map(|d| d.title).unwrap_or(""),
                    if c.correct { format!("[{} camada{}]", c.layers, if c.layers == 1 { "" } else { "s" }) } else { "[RESOLUÇÃO FALSA]".into() }
                ));
            }
            body.push(mono(line, 15.0, INK));
            body.push(hr());
            let changed = game.alterations;
            let deaths = game.pop.people.iter().filter(|p| matches!(p.life, Life::Dead { .. })).count();
            body.push(mono(format!("ALTERAÇÕES: {}   PESSOAS MORTAS NA HISTÓRIA: {}   LINHA ATUAL: {}", changed, deaths, game.timeline_code()), 15.0, RED));
            body.push(mono(format!("FADIGA TEMPORAL: {:.0}%", game.fatigue), 15.0, DIM));
        }
        4 => {
            for s in Skill::ALL {
                let l = game.player.skill(s);
                let xp = *game.player.xp.get(&s).unwrap_or(&0);
                body.push(row(vec![
                    txt(format!("{:<14}", s.name()), 18.0, INK, Fnt::MonoB).sty(|_| {}),
                    mono(format!("nível {:>2}  ({} xp)", l, xp), 15.0, AMBER),
                    space(20.0),
                    wrap(s.perk(l), 15.0, DIM, Fnt::Serif, 560.0),
                ]));
            }
            body.push(hr());
            let rep: Vec<String> = Group::ALL.iter().map(|g| format!("{} {:+}", g.name(), game.rep(*g))).collect();
            body.push(mono(format!("REPUTAÇÃO EM {}: {}", game.city.upper(), rep.join("  ·  ")), 14.0, DIM));
            body.push(mono(format!("Idade do corpo: {:.0} · anos vividos entre linhas: {:.0}", game.player.body_age, game.player.mind_years), 14.0, DIM));
        }
        5 => {
            for c in db.0.iter() {
                let prog = game.cases.iter().find(|p| p.id == c.id);
                let status = match prog {
                    Some(p) if p.solved && p.correct => format!("RESOLVIDO · {} camada(s)", p.layers),
                    Some(p) if p.solved => "RESOLUÇÃO FALSA".into(),
                    Some(p) if p.started => "EM ANDAMENTO".into(),
                    _ => "—".into(),
                };
                let known = prog.map(|p| p.started).unwrap_or(false);
                body.push(mono(format!("CASO {:02}  {:<38} {}  {}", c.id, if known { c.title } else { "???" }, if known { format!("{} {}", c.city.upper(), c.year) } else { "".into() }, status), 14.0, if known { INK } else { FAINT }));
            }
        }
        _ => {
            for ph in game.photos.iter().rev().skip(ui.scroll.max(0) as usize).take(6) {
                let mut k = vec![
                    mono(format!("{} · dia {} · {}", ph.year, ph.day + 1, ph.place), 12.0, PAPER_INK),
                    wrap(if ph.people.is_empty() { "Ninguém na foto.".to_string() } else { format!("Na foto: {}", ph.people.join(", ")) }, 16.0, PAPER_INK, Fnt::Serif, 900.0),
                ];
                if let Some(a) = &ph.anomaly {
                    k.push(wrap(a.clone(), 15.0, Color::srgb(0.55, 0.08, 0.1), Fnt::SerifI, 900.0));
                }
                body.push(col(k).bg(PAPER).pad(10.0));
            }
            if game.photos.is_empty() {
                body.push(t("Nenhuma foto. Compre uma câmera e aperte [V] para fotografar."));
            }
        }
    }
    modal(
        panel(vec![
            row(tab_row).gap(6.0),
            hr(),
            col(body).gap(10.0).h(520.0).scroll(),
            row(vec![mono("[↑↓] rolar   [J] ou [Esc] fechar", 13.0, FAINT)]),
        ])
        .w(1060.0),
    )
}

// ------------------------------------------------------------------ network & relationships

pub fn build_network(game: &Game, ui: &UiState) -> El {
    let tabs = ["REDE", "RELACIONAMENTOS", "FAMÍLIA"];
    let mut tab_row = vec![];
    for (i, t) in tabs.iter().enumerate() {
        tab_row.push(btn(format!("tab:{}", i), *t).active(ui.tab == i));
    }
    let mut body = vec![];
    match ui.tab {
        0 => {
            let members: Vec<&Person> = game.pop.people.iter().filter(|p| p.elias.in_network && p.alive() && p.city == game.city).collect();
            if members.is_empty() {
                body.push(wrap("A Rede ainda não existe nesta linha do tempo. Converse com jornalistas, ex-policiais, médicos, fotógrafos, criminosos... e convide quem confiar em você.", 17.0, DIM, Fnt::SerifI, 900.0));
            }
            for p in members {
                let role = crate::ui::dialogue::network_role(p.job).unwrap_or("?");
                let unpaid = game.day > p.elias.paid_until_day;
                body.push(
                    col(vec![
                        row(vec![txt(p.name(), 20.0, INK, Fnt::SerifB), space(10.0), mono(format!("{} · lealdade {}", role, p.elias.loyalty), 14.0, if p.elias.loyalty < 30 { RED } else { AMBER })]),
                        mono(if unpaid { "SALÁRIO ATRASADO — a lealdade cai a cada dia".to_string() } else { format!("pago até o dia {}", p.elias.paid_until_day + 1) }, 13.0, if unpaid { RED } else { FAINT }),
                        row(vec![
                            btn(format!("task:{}:research", p.id), "Pesquisar o caso"),
                            btn(format!("task:{}:follow", p.id), "Seguir um suspeito"),
                            btn(format!("task:{}:special", p.id), "Tarefa especial"),
                        ])
                        .gap(6.0),
                    ])
                    .gap(3.0),
                );
            }
        }
        1 => {
            let mut rels: Vec<&Person> = game.pop.people.iter().filter(|p| p.elias.met && p.city == game.city && (p.elias.trust.abs() > 10 || p.elias.romance != Romance::None || p.elias.fear > 20)).collect();
            rels.sort_by_key(|p| -(p.elias.affection + p.elias.trust));
            for p in rels.iter().skip(ui.scroll.max(0) as usize).take(12) {
                let status = if !p.alive() { "✝ morto(a) / ausente".to_string() } else if p.elias.romance != Romance::None { p.elias.romance.label().to_string() } else if p.elias.trust > 40 { "amigo(a)".into() } else if p.elias.trust < -20 { "inimigo(a)".into() } else if p.elias.fear > 40 { "tem medo de você".into() } else { "conhecido(a)".into() };
                let hint = if p.elias.affection > 60 { "procura você" } else if p.elias.affection > 30 { "gosta da sua companhia" } else if p.elias.trust < -30 { "fala mal de você" } else { "" };
                body.push(row(vec![
                    txt(format!("{:<26}", p.name()), 17.0, INK, Fnt::Serif),
                    mono(format!("{:<24}", p.job.label(p.female)), 13.0, DIM),
                    mono(format!("{:<18}", status), 13.0, if p.elias.romance != Romance::None { Color::srgb(1.0, 0.5, 0.6) } else { AMBER }),
                    mono(hint, 13.0, FAINT),
                    if p.elias.deja_vu > 0 { mono("  ◆ déjà vu", 13.0, RED) } else { space(1.0) },
                ]));
            }
            if rels.is_empty() {
                body.push(t("Ninguém nesta cidade se lembra de você."));
            }
        }
        _ => {
            let kids: Vec<&Person> = game.pop.people.iter().filter(|p| p.elias_child).collect();
            let partners: Vec<&Person> = game.pop.people.iter().filter(|p| matches!(p.elias.romance, Romance::Married | Romance::Widowed) || p.children.iter().any(|c| kids.iter().any(|k| k.id == *c))).collect();
            if partners.is_empty() && kids.is_empty() {
                body.push(wrap("Você não tem família em nenhuma linha do tempo. Ainda.", 17.0, DIM, Fnt::SerifI, 900.0));
            }
            for p in partners {
                body.push(txt(format!("{} ── ELIAS VALE   ({} · {})", p.name().to_uppercase(), p.city.name(), p.elias.romance.label()), 18.0, INK, Fnt::MonoB));
                for k in kids.iter().filter(|k| k.parents.contains(&Some(p.id))) {
                    let st = match &k.life {
                        Life::Alive => format!("{} anos · {}", k.age(game.year).max(0), k.job.label(k.female)),
                        Life::Unborn => "NUNCA NASCEU nesta linha do tempo".into(),
                        Life::Dead { year, .. } => format!("morreu em {}", year),
                        _ => "longe".into(),
                    };
                    body.push(mono(format!("     └── {}  ({}, nasc. {})  {}", k.name(), if k.female { "filha" } else { "filho" }, k.birth, st), 15.0, if k.life == Life::Unborn { RED } else { DIM }));
                }
            }
        }
    }
    modal(panel(vec![row(tab_row).gap(6.0), hr(), col(body).gap(8.0).h(520.0).scroll(), mono("[N] ou [Esc] fechar", 13.0, FAINT)]).w(1060.0))
}

// ------------------------------------------------------------------ map

#[derive(Resource, Default)]
pub struct MapImage {
    pub handle: Option<Handle<Image>>,
    pub for_city: Option<(crate::city::gen::CityId, i32)>,
    pub w: u32,
    pub h: u32,
}

pub fn build_map_image(m: &crate::city::map::Map, images: &mut Assets<Image>) -> (Handle<Image>, u32, u32) {
    use crate::city::map::Tile;
    use bevy::asset::RenderAssetUsages;
    use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
    let (w, h) = (m.w as u32, m.h as u32);
    let mut data = Vec::with_capacity((w * h * 4) as usize);
    for y in 0..m.h {
        for x in 0..m.w {
            let c: [u8; 3] = match m.get(x, y) {
                Tile::Water => [18, 22, 32],
                Tile::Road => [58, 55, 52],
                Tile::Sidewalk | Tile::Plaza => [88, 84, 78],
                Tile::Alley => [48, 44, 42],
                Tile::Grass => [34, 44, 30],
                Tile::Field => [60, 56, 34],
                Tile::Dirt => [66, 54, 40],
                Tile::Dock => [70, 54, 38],
                Tile::Rail => [40, 36, 36],
                Tile::Floor | Tile::Door => [120, 108, 92],
                Tile::Wall | Tile::Window => [150, 138, 120],
                Tile::Fence => [80, 80, 80],
                Tile::Gravel => [72, 70, 68],
                Tile::Sand => [150, 140, 110],
                Tile::Void => [0, 0, 0],
            };
            data.extend_from_slice(&[c[0], c[1], c[2], 255]);
        }
    }
    let mut im = Image::new(Extent3d { width: w, height: h, depth_or_array_layers: 1 }, TextureDimension::D2, data, TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::RENDER_WORLD);
    im.sampler = bevy::image::ImageSampler::nearest();
    (images.add(im), w, h)
}

pub fn build_map(game: &Game, img: &MapImage, m: &crate::city::map::Map, db: &CaseDb) -> El {
    let Some(h) = &img.handle else { return modal(panel(vec![t("Mapa indisponível.")])) };
    let scale = (1100.0 / img.w as f32).min(560.0 / img.h as f32);
    let (w, hh) = (img.w as f32 * scale, img.h as f32 * scale);
    let mut marks: Vec<El> = Vec::new();
    let pin = |x: f32, y: f32, color: Color, label: &str, size: f32| -> El {
        El::Col(
            Sty { abs: Some((Val::Px(x * scale - size / 2.0), Val::Px(y * scale - size / 2.0))), ..default() },
            vec![row(vec![
                El::Col(Sty { w: Val::Px(size), h: Val::Px(size), bg: Some(color), border: Some(Color::BLACK), ..default() }, vec![]),
                space(3.0),
                mono(label, 11.0, color),
            ])],
        )
    };
    // important buildings
    for b in &m.buildings {
        use crate::city::map::BKind::*;
        if matches!(b.kind, Police | Hospital | Church | Hotel | Newspaper | Station | Pawn | Clothing | GunShop | Club | Market) && !b.closed_forever {
            let c = b.center_px();
            marks.push(pin(c.x, c.y, Color::srgb(0.6, 0.6, 0.55), if b.name.is_empty() { b.kind.label() } else { &b.name }, 5.0));
        }
    }
    // player marks (houses learnt in conversations)
    for mk in game.marks.iter().filter(|mk| mk.city == game.city) {
        let pos = if mk.pos.y < 0.0 { m.buildings.get(mk.pos.x as usize).map(|b| b.center_px()).unwrap_or(Vec2::ZERO) } else { mk.pos };
        marks.push(pin(pos.x, pos.y, AMBER, &mk.label, 7.0));
    }
    // case: crime scene
    if let Some((def, pi)) = crate::cases::run::current(game, db) {
        let prog = &game.cases[pi];
        if let Some(Some((x, y))) = prog.clue_pos.first() {
            marks.push(pin(*x, *y, RED, &format!("Cena do crime — {}", def.title), 9.0));
        }
    }
    if let Some(sh) = game.player.safehouse {
        if let Some(b) = m.buildings.get(sh) {
            let c = b.center_px();
            marks.push(pin(c.x, c.y, GREEN, "Seu esconderijo", 8.0));
        }
    }
    let pp = game.player.pos;
    marks.push(pin(pp.x, pp.y, Color::WHITE, "VOCÊ", 10.0));
    let map_el = El::Img { h: h.clone(), w, hgt: hh, kids: marks };
    modal(panel(vec![row(vec![title(format!("{} — {}", game.city.upper(), game.year)), grow(), mono("[M] ou [Esc] fechar", 13.0, FAINT)]), map_el]))
}

pub fn sfx_ok(sfx: &mut EventWriter<Sfx>) {
    sfx.write(Sfx::Paper);
}
