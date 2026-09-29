//! Conversations and interrogations.

use super::*;
use crate::audio::Sfx;
use crate::cases::defs::Req;
use crate::cases::run::{current, CaseDb};
use crate::items::{Gift, Item};
use crate::sim::agents::{AState, Sim};
use crate::sim::people::*;
use crate::state::{Game, Skill};
use crate::util::Rng;

#[derive(Clone, PartialEq, Debug, Default)]
pub enum Node {
    #[default]
    Root,
    Intro,
    Case,
    Pressure(u8, String),
    Evidence,
    People,
    Romance,
    Gifts,
    Witness(u32),
    Confront,
    Network,
}

#[derive(Resource, Default)]
pub struct Dlg {
    pub with: Option<Pid>,
    pub agent: Option<usize>,
    pub log: Vec<(String, String)>,
    pub node: Node,
    pub rng: Option<Rng>,
    pub end: bool,
    pub start_pursuit: Option<Pid>,
    pub open_shop: bool,
    pub resolve_case: Option<(u8, bool)>,
    pub start_date: Option<Pid>,
    /// crimes committed through conversation (hold-ups, kidnapping...)
    pub crimes: Vec<(crate::state::CrimeKind, Pid)>,
    pub hands_up: Vec<Pid>,
}

impl Dlg {
    fn say(&mut self, who: impl Into<String>, s: impl Into<String>) {
        self.log.push((who.into(), s.into()));
        if self.log.len() > 6 {
            self.log.remove(0);
        }
    }
    fn rng(&mut self) -> &mut Rng {
        if self.rng.is_none() {
            self.rng = Some(Rng::new(0xD1A1));
        }
        self.rng.as_mut().unwrap()
    }
}

pub fn start_dialogue(dlg: &mut Dlg, ui: &mut UiState, sim: &mut Sim, game: &mut Game, pid: Pid) {
    dlg.with = Some(pid);
    dlg.agent = sim.by_pid.get(&pid).copied();
    dlg.log.clear();
    dlg.end = false;
    dlg.rng = Some(Rng::new(game.abs_minute() as u64 ^ pid as u64 * 977));
    if let Some(a) = sim.agent_mut(pid) {
        a.state = AState::Talking;
        a.chat = None;
    }
    let p = game.pop.get(pid).clone();
    let greet = greeting(&p, game);
    dlg.say(p.name_known(game), greet);
    dlg.node = if p.elias.met { Node::Root } else { Node::Intro };
    let day = game.day;
    let pp = game.pop.get_mut(pid);
    pp.elias.talks += 1;
    pp.elias.last_talk_day = day;
    ui.open(Mode::Dialogue);
}

trait Known {
    fn name_known(&self, game: &Game) -> String;
}
impl Known for Person {
    fn name_known(&self, _game: &Game) -> String {
        if self.elias.met || self.case_role.is_some() {
            self.name()
        } else {
            format!("{} desconhecid{}", self.job.label(self.female), self.o())
        }
    }
}

fn greeting(p: &Person, game: &Game) -> String {
    let e = &p.elias;
    let out_of_time = game.player.outfit == crate::items::Outfit::Modern;
    if e.deja_vu > 0 && !e.met {
        return "...Desculpe. Eu te conheço? Tenho a impressão de que já perdi você antes.".into();
    }
    if !e.met {
        if out_of_time {
            return "Que roupa é essa? Você não é daqui, é?".into();
        }
        return match p.traits.sociability {
            0..=30 => "O que você quer?".into(),
            31..=70 => "Pois não?".into(),
            _ => "Boa noite! Não me lembro de você por aqui.".into(),
        };
    }
    let alias = e.alias.clone().unwrap_or_else(|| "forasteiro".into());
    match e.romance {
        Romance::Married => return format!("Você chegou, {}. Senti sua falta.", alias),
        Romance::Dating | Romance::Lovers | Romance::Engaged => return format!("{}! Estava pensando em você.", alias),
        Romance::Broken => return "Você tem coragem de falar comigo?".into(),
        _ => {}
    }
    if e.fear > 40 {
        return "P-por favor, eu não quero problemas.".into();
    }
    if e.trust > 40 {
        format!("{}, meu amigo. O que manda?", alias)
    } else if e.trust < -20 {
        "Você de novo.".into()
    } else {
        format!("Olá, {}.", alias)
    }
}

fn trust_label(p: &Person) -> (&'static str, Color) {
    let e = &p.elias;
    if e.romance != Romance::None && e.romance != Romance::Broken {
        return (e.romance.label(), Color::srgb(1.0, 0.45, 0.55));
    }
    if e.fear > 50 {
        ("Tem medo de você", Color::srgb(0.9, 0.5, 0.2))
    } else if e.trust > 50 {
        ("Confia em você", GREEN)
    } else if e.trust > 15 {
        ("Simpatiza com você", Color::srgb(0.7, 0.8, 0.6))
    } else if e.trust < -30 {
        ("Odeia você", RED)
    } else if e.trust < -5 {
        ("Desconfia de você", Color::srgb(0.9, 0.6, 0.4))
    } else {
        ("Neutro", DIM)
    }
}

pub fn build(dlg: &Dlg, game: &Game, db: &CaseDb) -> El {
    let Some(pid) = dlg.with else { return col(vec![]) };
    let p = game.pop.get(pid);
    let (tl, tc) = trust_label(p);
    let age = p.age(game.year);
    let mut hist = Vec::new();
    for (who, s) in dlg.log.iter() {
        let is_elias = who == "Elias";
        hist.push(
            col(vec![
                txt(who.to_uppercase(), 13.0, if is_elias { AMBER } else { RED }, Fnt::MonoB),
                wrap(if is_elias { s.clone() } else { format!("\u{201C}{}\u{201D}", s) }, 19.0, if is_elias { DIM } else { INK }, if is_elias { Fnt::SerifI } else { Fnt::Serif }, 820.0),
            ])
            .gap(2.0),
        );
    }
    let opts = options(dlg, game, db);
    let mut opt_els = Vec::new();
    for (i, (id, label, color)) in opts.iter().enumerate() {
        opt_els.push(opt(format!("o:{}", id), format!("{}. {}", i + 1, label), *color));
    }
    let traits = if game.player.skill(Skill::Observation) >= 3 { p.traits.describe().join(", ") } else { String::new() };
    let content = row(vec![
        col(vec![
            txt(p.name_known(game), 24.0, INK, Fnt::SerifB),
            mono(format!("{} · {} anos", p.job.label(p.female), age), 14.0, DIM),
            mono(tl, 14.0, tc),
            if traits.is_empty() { space(1.0) } else { mono(traits, 13.0, FAINT) },
            if p.elias.deja_vu > 0 { mono("◆ déjà vu temporal", 13.0, RED) } else { space(1.0) },
        ])
        .w(260.0)
        .gap(4.0),
        col(vec![col(hist).gap(10.0), hr(), col(opt_els).gap(2.0)]).gap(6.0).grow(),
    ])
    .gap(24.0)
    .align(AlignItems::FlexStart);
    El::Col(
        Sty { w: Val::Percent(100.0), h: Val::Percent(100.0), justify: JustifyContent::FlexEnd, ..default() },
        vec![content.bg(PANEL).border(Color::srgb(0.3, 0.25, 0.3)).pad(20.0).sty(|s| s.w = Val::Percent(100.0))],
    )
}

type Opt = (String, String, Color);

fn options(dlg: &Dlg, game: &Game, db: &CaseDb) -> Vec<Opt> {
    let Some(pid) = dlg.with else { return vec![] };
    let p = game.pop.get(pid);
    let mut v: Vec<Opt> = Vec::new();
    let case = current(game, db);
    let cast_idx = case.and_then(|(_, pi)| game.cases[pi].cast.iter().position(|c| *c == pid)).map(|i| i as u8);
    match &dlg.node {
        Node::Intro => {
            v.push(("intro_real".into(), "\"Meu nome é Elias Vale.\"".into(), INK));
            if let Some(id) = &game.player.identity {
                v.push(("intro_id".into(), format!("\"{}, {}.\" (identidade falsa)", id.name, id.job.to_lowercase()), INK));
            }
            v.push(("intro_fake".into(), "Inventar um nome qualquer.".into(), DIM));
            v.push(("bye".into(), "Ir embora.".into(), FAINT));
        }
        Node::Root => {
            if case.is_some() {
                v.push(("case".into(), "Perguntar sobre o caso...".into(), INK));
            }
            v.push(("people".into(), "Perguntar sobre alguém...".into(), INK));
            if !game.cases.iter().flat_map(|c| c.found.iter()).collect::<Vec<_>>().is_empty() && case.is_some() {
                v.push(("evidence".into(), "Mostrar uma evidência...".into(), INK));
            }
            v.push(("talk".into(), "Jogar conversa fora.".into(), DIM));
            // witness to Elias' crimes
            for c in game.police.crimes.iter() {
                if c.by_elias && c.witnesses.iter().any(|w| w.pid == pid && !w.reported && w.silenced.is_none()) {
                    v.push((format!("witness:{}", c.id), format!("Sobre o que você viu ({})...", c.kind.label().to_lowercase()), RED));
                }
            }
            if let (Some(_), Some(ci)) = (case, cast_idx) {
                let def = case.unwrap().0;
                if def.cast[ci as usize].role == crate::cases::defs::Role::Suspect || ci == def.culprit {
                    v.push(("confront".into(), "\"Eu sei o que você fez.\" (acusar)".into(), RED));
                }
            }
            if matches!(p.job, Job::Detective) && case.is_some() && game.city_map_building_is_police(pid) {
                v.push(("report".into(), "Entregar o culpado à polícia (quadro do caso).".into(), RED));
            }
            if p.age(game.year) >= 18 && p.spouse.is_none() || p.elias.romance != Romance::None {
                if p.age(game.year) >= 18 && p.elias.met && p.elias.romance != Romance::Married || p.elias.romance == Romance::Married {
                    v.push(("romance".into(), "Algo mais pessoal...".into(), Color::srgb(1.0, 0.5, 0.6)));
                }
            }
            if game.player.inv.iter().any(|i| matches!(i, Item::Gift(_))) {
                v.push(("gifts".into(), "Dar um presente...".into(), INK));
            }
            if is_shopkeeper(p) {
                v.push(("shop".into(), "\"O que você tem para vender?\"".into(), AMBER));
                if matches!(p.job, Job::Smuggler | Job::Forger | Job::Gangster) || (p.job == Job::Pawnbroker && game.rep(crate::state::Group::Crime) > 5) {
                    v.push(("black".into(), "\"Ouvi dizer que você arruma coisas... diferentes.\"".into(), RED));
                }
                if matches!(p.job, Job::Smuggler | Job::Gangster) {
                    v.push(("sellcar".into(), "\"Tenho um carro pra vender. Sem perguntas.\"".into(), RED));
                }
            }
            let dangerous = game.player.weapons().iter().any(|w| *w != crate::items::Weapon::Fists && *w != crate::items::Weapon::Bottle && *w != crate::items::Weapon::Club);
            let firearm = game.player.weapons().iter().any(|w| !w.stats().melee);
            if dangerous && p.age(game.year) >= 12 {
                v.push(("rob".into(), "\"Isto é um assalto. Passa tudo.\"".into(), RED));
                v.push(("tie".into(), "\"Mãos pra trás. Não grita.\" (amarrar)".into(), RED));
                v.push(("kidnap".into(), "\"Você vem comigo. Agora.\" (sequestrar)".into(), RED));
            }
            if firearm && is_shopkeeper(p) {
                v.push(("robshop".into(), "\"Esvazia o caixa! Todo mundo no chão!\"".into(), RED));
            }
            if network_role(p.job).is_some() && !p.elias.in_network && p.elias.met {
                v.push(("recruit".into(), format!("Convidar para a Rede ({}).", network_role(p.job).unwrap()), AMBER));
            }
            if p.elias.in_network {
                v.push(("netchat".into(), "Assuntos da Rede...".into(), AMBER));
            }
            if matches!(p.elias.romance, Romance::Dating | Romance::Lovers | Romance::Engaged | Romance::Married) || p.elias.trust > 30 {
                v.push(("follow".into(), "\"Venha comigo um pouco.\"".into(), DIM));
            }
            v.push(("bye".into(), "\"Até mais.\"".into(), FAINT));
        }
        Node::Case => {
            if let (Some((def, pi)), Some(ci)) = (case, cast_idx) {
                let prog = &game.cases[pi];
                for t in &def.cast[ci as usize].topics {
                    let ok = match t.req {
                        Req::None => true,
                        Req::Clue(c) => prog.has(c),
                        Req::Topic(k) => prog.topics.iter().any(|(ii, kk)| *ii == ci && kk == k),
                        Req::Trust(n) => p.elias.trust >= n,
                        Req::Both(a, b) => prog.has(a) && prog.has(b),
                    };
                    if !ok {
                        continue;
                    }
                    let done = prog.topics.iter().any(|(ii, kk)| *ii == ci && kk == t.key);
                    let lied = prog.lies_heard.iter().any(|(ii, kk)| *ii == ci && kk == t.key) && !done;
                    let color = if done { FAINT } else if lied { RED } else { INK };
                    let label = if lied { format!("{} (está mentindo)", t.q) } else { t.q.to_string() };
                    v.push((format!("topic:{}", t.key), label, color));
                    if lied {
                        v.push((format!("press:{}", t.key), "    → Pressionar...".into(), RED));
                    }
                }
            } else if let Some((def, _)) = case {
                v.push(("case_rumor".into(), format!("\"O que você sabe sobre o caso {}?\"", def.title.to_lowercase()), INK));
                v.push(("case_people".into(), "\"Alguém anda estranho por aqui?\"".into(), INK));
            }
            v.push(("back".into(), "Voltar.".into(), FAINT));
        }
        Node::Pressure(_, _) => {
            let money = game.player.money;
            let bribe = bribe_cost(p);
            v.push(("pr_press".into(), "Pressionar: \"Você está mentindo. Eu vejo nos seus olhos.\"".into(), INK));
            v.push(("pr_bluff".into(), "Blefar: \"Eu já sei de tudo. Só quero ouvir de você.\"".into(), INK));
            v.push(("pr_evidence".into(), "Mostrar a evidência que desmente.".into(), INK));
            if money >= bribe {
                v.push(("pr_bribe".into(), format!("Oferecer dinheiro (${}).", bribe), AMBER));
            } else {
                v.push(("pr_nomoney".into(), format!("Oferecer dinheiro (${} — você não tem).", bribe), FAINT));
            }
            v.push(("pr_protect".into(), "\"Eu posso te proteger. Ninguém vai saber que foi você.\"".into(), INK));
            v.push(("pr_threat".into(), "Ameaçar.".into(), RED));
            v.push(("back".into(), "Deixar para lá.".into(), FAINT));
        }
        Node::Evidence => {
            if let Some((def, pi)) = case {
                for c in game.cases[pi].found.iter() {
                    let cl = &def.clues[*c as usize];
                    v.push((format!("show:{}", c), cl.name.to_string(), INK));
                }
            }
            v.push(("back".into(), "Voltar.".into(), FAINT));
        }
        Node::People => {
            let mut names: Vec<(Pid, String)> = Vec::new();
            if let Some((_, pi)) = case {
                for (i, c) in game.cases[pi].cast.iter().enumerate() {
                    if *c != pid && i < 12 {
                        names.push((*c, game.pop.get(*c).name()));
                    }
                }
            }
            for r in p.rels.iter().take(4) {
                if !names.iter().any(|(q, _)| *q == r.to) && r.to != pid {
                    names.push((r.to, game.pop.get(r.to).name()));
                }
            }
            for (q, n) in names {
                v.push((format!("who:{}", q), format!("\"Você conhece {}?\"", n), INK));
            }
            v.push(("back".into(), "Voltar.".into(), FAINT));
        }
        Node::Romance => {
            let e = &p.elias;
            match e.romance {
                Romance::None | Romance::Broken => {
                    v.push(("rom_compliment".into(), "Fazer um elogio.".into(), INK));
                    v.push(("rom_flirt".into(), "Flertar.".into(), Color::srgb(1.0, 0.5, 0.6)));
                    if e.affection >= 20 {
                        v.push(("rom_date".into(), "Convidar para jantar.".into(), Color::srgb(1.0, 0.5, 0.6)));
                    }
                }
                Romance::Flirting => {
                    v.push(("rom_compliment".into(), "Fazer um elogio.".into(), INK));
                    v.push(("rom_date".into(), "Convidar para sair esta noite.".into(), Color::srgb(1.0, 0.5, 0.6)));
                    if e.affection >= 45 {
                        v.push(("rom_ask".into(), "Pedir em namoro.".into(), Color::srgb(1.0, 0.5, 0.6)));
                    }
                }
                Romance::Dating | Romance::Lovers => {
                    v.push(("rom_date".into(), "Sair juntos.".into(), Color::srgb(1.0, 0.5, 0.6)));
                    v.push(("rom_kiss".into(), "Beijar.".into(), Color::srgb(1.0, 0.5, 0.6)));
                    if e.affection >= 70 && game.player.has(&Item::Gift(Gift::Jewel)) {
                        v.push(("rom_propose".into(), "Pedir em casamento (dar a joia).".into(), RED));
                    } else if e.affection >= 70 {
                        v.push(("rom_noring".into(), "Pedir em casamento (precisa de uma joia).".into(), FAINT));
                    }
                    v.push(("rom_truth".into(), "Contar a verdade sobre a esfera.".into(), DIM));
                    v.push(("rom_break".into(), "Terminar.".into(), FAINT));
                }
                Romance::Engaged => {
                    v.push(("rom_wed".into(), "Casar na igreja (custa $30).".into(), RED));
                    v.push(("rom_kiss".into(), "Beijar.".into(), Color::srgb(1.0, 0.5, 0.6)));
                    v.push(("rom_break".into(), "Desfazer o noivado.".into(), FAINT));
                }
                Romance::Married => {
                    v.push(("rom_kiss".into(), "Beijar.".into(), Color::srgb(1.0, 0.5, 0.6)));
                    v.push(("rom_home".into(), "\"Como estão as coisas em casa?\"".into(), INK));
                    v.push(("rom_truth".into(), "Contar a verdade sobre a esfera.".into(), DIM));
                    v.push(("rom_break".into(), "Pedir o divórcio.".into(), FAINT));
                }
                Romance::Widowed => {}
            }
            v.push(("back".into(), "Voltar.".into(), FAINT));
        }
        Node::Gifts => {
            let mut seen = Vec::new();
            for it in game.player.inv.iter() {
                if let Item::Gift(g) = it {
                    if !seen.contains(g) {
                        seen.push(*g);
                        v.push((format!("gift:{:?}", g), g.name().to_string(), INK));
                    }
                }
            }
            v.push(("back".into(), "Voltar.".into(), FAINT));
        }
        Node::Witness(id) => {
            v.push((format!("w_persuade:{}", id), "\"Você não viu o que acha que viu.\" (persuadir)".into(), INK));
            v.push((format!("w_bribe:{}", id), format!("\"Tome ${} e esqueça tudo.\"", bribe_cost(p) * 2), AMBER));
            v.push((format!("w_threat:{}", id), "\"Se abrir a boca, você é o próximo.\"".into(), RED));
            v.push(("back".into(), "Voltar.".into(), FAINT));
        }
        Node::Confront => {
            v.push(("conf_go".into(), "Acusar usando o que está no quadro do caso.".into(), RED));
            v.push(("back".into(), "Ainda não.".into(), FAINT));
        }
        Node::Network => {
            v.push(("net_pay".into(), format!("Pagar o salário da semana (${}).", wage(p.job)), AMBER));
            v.push(("net_leave".into(), "Dispensar da Rede.".into(), FAINT));
            v.push(("back".into(), "Voltar.".into(), FAINT));
        }
    }
    v
}

impl Game {
    /// helper: is this person currently at the police station? (approximation: detective at work hours)
    pub fn city_map_building_is_police(&self, pid: Pid) -> bool {
        let p = self.pop.get(pid);
        let (s0, s1) = p.job.shift();
        let h = self.hour();
        h >= s0 && h < s1
    }
}

thread_local! {
    pub static BLACK: std::cell::RefCell<bool> = const { std::cell::RefCell::new(false) };
    pub static SELLCAR: std::cell::RefCell<bool> = const { std::cell::RefCell::new(false) };
}

pub fn is_shopkeeper(p: &Person) -> bool {
    matches!(p.job, Job::Tailor | Job::Merchant | Job::Gunsmith | Job::Pharmacist | Job::Pawnbroker | Job::Forger | Job::Bartender | Job::Vendor | Job::Baker | Job::Butcher | Job::Smuggler)
}

pub fn network_role(j: Job) -> Option<&'static str> {
    Some(match j {
        Job::Journalist => "informação",
        Job::Police | Job::Detective => "investigação criminal",
        Job::Gangster | Job::Smuggler | Job::Drifter => "infiltração",
        Job::Doctor | Job::Nurse => "medicina e cadáveres",
        Job::Photographer => "documentação",
        Job::Teacher | Job::Clerk | Job::Lawyer => "arquivos e registros",
        Job::Hacker => "computadores",
        Job::Forger => "falsificação",
        _ => return None,
    })
}

pub fn wage(j: Job) -> i32 {
    match j {
        Job::Doctor | Job::Detective | Job::Lawyer => 25,
        Job::Hacker | Job::Forger => 20,
        _ => 12,
    }
}

fn bribe_cost(p: &Person) -> i32 {
    (5.0 + p.traits.morality as f32 * 0.4 + (100 - p.traits.greed as i32) as f32 * 0.15) as i32
}

// ------------------------------------------------------------------ handling choices

#[allow(clippy::too_many_arguments)]
pub fn dialogue_input(
    mut ui: ResMut<UiState>,
    mut dlg: ResMut<Dlg>,
    mut game: ResMut<Game>,
    mut sim: ResMut<Sim>,
    db: Res<CaseDb>,
    keys: Res<ButtonInput<KeyCode>>,
    mut sfx: EventWriter<Sfx>,
    mut toasts: ResMut<hud::Toasts>,
) {
    if ui.mode != Mode::Dialogue {
        return;
    }
    let Some(pid) = dlg.with else {
        ui.close();
        return;
    };
    let mut choice: Option<String> = None;
    if let Some(c) = ui.take_click() {
        choice = c.strip_prefix("o:").map(|s| s.to_string());
    }
    let opts = options(&dlg, &game, &db);
    let digits = [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4, KeyCode::Digit5, KeyCode::Digit6, KeyCode::Digit7, KeyCode::Digit8, KeyCode::Digit9];
    for (i, k) in digits.iter().enumerate() {
        if keys.just_pressed(*k) {
            if let Some(o) = opts.get(i) {
                choice = Some(o.0.clone());
            }
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        choice = Some("bye".into());
    }
    let Some(ch) = choice else { return };
    // the option label becomes Elias' line
    if let Some(o) = opts.iter().find(|o| o.0 == ch) {
        let l = o.1.trim_start_matches("    → ").to_string();
        if !matches!(ch.as_str(), "back" | "case" | "people" | "evidence" | "romance" | "gifts" | "netchat") && !ch.starts_with("press:") {
            dlg.say("Elias", l);
        }
    }
    sfx.write(Sfx::Tick);
    handle(&ch, pid, &mut dlg, &mut game, &mut sim, &db, &mut sfx, &mut toasts);
    ui.dirty = true;
    if dlg.end {
        end_dialogue(&mut dlg, &mut ui, &mut sim);
    }
}

pub fn end_dialogue(dlg: &mut Dlg, ui: &mut UiState, sim: &mut Sim) {
    if let Some(pid) = dlg.with {
        if let Some(a) = sim.agent_mut(pid) {
            if a.state == AState::Talking {
                a.state = AState::Normal;
                a.replan_at = 0.0;
            }
        }
    }
    dlg.with = None;
    dlg.end = false;
    if ui.mode == Mode::Dialogue {
        ui.close();
    }
}

#[allow(clippy::too_many_arguments)]
fn handle(ch: &str, pid: Pid, dlg: &mut Dlg, game: &mut Game, sim: &mut Sim, db: &CaseDb, sfx: &mut EventWriter<Sfx>, toasts: &mut hud::Toasts) {
    let year = game.year;
    let day = game.day;
    let name = game.pop.get(pid).name();
    let case = current(game, db).map(|(d, pi)| (d.clone(), pi));
    let cast_idx = case.as_ref().and_then(|(_, pi)| game.cases[*pi].cast.iter().position(|c| *c == pid)).map(|i| i as u8);
    let persuasion = game.player.skill(Skill::Persuasion) as f32;
    let deception = game.player.skill(Skill::Deception) as f32;
    let r = dlg.rng().f();
    match ch {
        "bye" => {
            let p = game.pop.get(pid);
            let l = if p.elias.trust > 30 { "Até logo. Tome cuidado lá fora." } else { "Hm." };
            dlg.say(name, l);
            dlg.end = true;
        }
        "back" => dlg.node = Node::Root,
        "intro_real" | "intro_id" | "intro_fake" => {
            let alias = match ch {
                "intro_real" => "Elias".to_string(),
                "intro_id" => game.player.identity.as_ref().map(|i| i.name.clone()).unwrap_or("Elias".into()),
                _ => {
                    let fakes = ["Thomas Reed", "John Smith", "Henry Cole", "Paul Marchand", "Jack Morrow", "Frank Weiss"];
                    fakes[(pid as usize) % fakes.len()].to_string()
                }
            };
            let p = game.pop.get_mut(pid);
            p.elias.met = true;
            p.elias.alias = Some(alias.clone());
            if ch == "intro_fake" {
                p.elias.lies += 1;
            }
            let reply = if p.elias.deja_vu > 0 {
                format!("{}... Engraçado. Eu podia jurar que sabia seu nome antes de você dizer.", alias)
            } else {
                format!("Prazer, {}. Eu sou {}.", alias, p.first)
            };
            dlg.say(name, reply);
            dlg.node = Node::Root;
            if ch == "intro_real" && game.police.wanted > 0 {
                game.police.knows_name = Some("Elias Vale".into());
            }
        }
        "case" => dlg.node = Node::Case,
        "people" => dlg.node = Node::People,
        "evidence" => dlg.node = Node::Evidence,
        "romance" => dlg.node = Node::Romance,
        "gifts" => dlg.node = Node::Gifts,
        "netchat" => dlg.node = Node::Network,
        "confront" => dlg.node = Node::Confront,
        "talk" => {
            let p = game.pop.get(pid).clone();
            let known: Vec<String> = sim.rumors.iter().filter(|r| r.known.contains(&pid)).map(|r| r.text.clone()).collect();
            let line = if !known.is_empty() && r < 0.6 {
                format!("Dizem que {}", crate::sim::lines::lower_first(&known[(r * known.len() as f32) as usize % known.len()]))
            } else if r < 0.3 {
                // personal: family / worries
                if let Some(s) = p.spouse {
                    format!("{} anda cansad{} de mim, acho. Casamento é isso.", game.pop.get(s).first, game.pop.get(s).o())
                } else if p.debt > 50 {
                    format!("Devo ${} pra gente que não perdoa. Mas isso não é problema seu.", p.debt)
                } else {
                    "A vida aqui é assim: trabalho, igreja, bar. E reza pra não ser o próximo.".into()
                }
            } else {
                let mut rr = Rng::new(pid as u64 + day as u64);
                crate::sim::lines::era_small_talk(year)[rr.idx(crate::sim::lines::era_small_talk(year).len())].to_string()
            };
            dlg.say(name, line);
            if game.pop.get(pid).elias.last_talk_day == day && game.pop.get(pid).elias.talks > 3 {
                // repeated chatting annoys
                game.pop.get_mut(pid).elias.trust -= 1;
            } else {
                let soc = p.traits.sociability as i32;
                game.pop.get_mut(pid).elias.trust += 2 + soc / 40;
                game.player.train(Skill::Persuasion, 2);
            }
        }
        "case_rumor" => {
            let (def, pi) = case.clone().unwrap();
            let prog = &game.cases[pi];
            let victim = def.cast.iter().position(|c| c.role == crate::cases::defs::Role::Victim).map(|i| game.pop.get(prog.cast[i]).name());
            let line = match (r * 4.0) as i32 {
                0 => format!("Coisa horrível o que aconteceu com {}. Ninguém dorme direito desde então.", victim.unwrap_or("aquela gente".into())),
                1 => "A polícia? A polícia só aparece pra cobrar. Se quer saber de algo, pergunte na rua, não na delegacia.".into(),
                2 => "O jornal fala mais do que sabe. Mas aquela repórter... ela sabe mais do que fala.".into(),
                _ => "Eu não sei de nada. E quem sabe, não fala.".into(),
            };
            dlg.say(name, line);
        }
        "case_people" => {
            let (def, pi) = case.clone().unwrap();
            let prog = &game.cases[pi];
            // tell where a random suspect spends the evenings
            let sus: Vec<usize> = def.cast.iter().enumerate().filter(|(_, c)| c.role == crate::cases::defs::Role::Suspect).map(|(i, _)| i).collect();
            if sus.is_empty() {
                dlg.say(name, "Estranho? Aqui todo mundo é estranho.");
            } else {
                let i = sus[(r * sus.len() as f32) as usize % sus.len()];
                let spid = prog.cast[i];
                let desc = whereabouts(game, spid);
                dlg.say(name, format!("{} anda nervoso. {}", game.pop.get(spid).name(), desc));
            }
        }
        _ if ch.starts_with("who:") => {
            let q: Pid = ch[4..].parse().unwrap_or(0);
            let other = game.pop.get(q).clone();
            let p = game.pop.get(pid).clone();
            let opinion = match p.rel_to(q).map(|r| r.kind) {
                Some(RelKind::Friend) => "Gente boa. Somos amigos.",
                Some(RelKind::Rival) | Some(RelKind::Enemy) => "Não suporto. Nem me fale.",
                Some(RelKind::Lover) => "...Por que você quer saber?",
                Some(RelKind::Debtor) => "Devo dinheiro. Não me lembre.",
                Some(RelKind::Creditor) => "Me deve dinheiro, o safado.",
                Some(RelKind::Coworker) => "Trabalhamos juntos.",
                _ => "Conheço de vista.",
            };
            let line = if !other.alive() {
                match &other.life {
                    Life::Dead { .. } => format!("{} morreu. Deus o tenha.", other.first),
                    Life::Jailed => format!("{} está preso.", other.first),
                    _ => format!("{} foi embora da cidade.", other.first),
                }
            } else {
                format!("{} {} {}", opinion, whereabouts(game, q), if other.job != Job::None { format!("Trabalha como {}.", other.job.label(other.female).to_lowercase()) } else { String::new() })
            };
            dlg.say(name, line);
            // put home on the map
            if let Some(h) = other.home {
                let label = format!("Casa de {}", other.name());
                if !game.marks.iter().any(|m| m.label == label) {
                    let pos = sim.agent(q).map(|_| Vec2::ZERO).unwrap_or(Vec2::ZERO);
                    let _ = pos;
                    game.marks.push(crate::state::MapMark { pos: Vec2::new(h as f32, -1.0), label, city: game.city });
                    toasts.push(format!("Mapa: casa de {} marcada.", other.name()));
                }
            }
            dlg.node = Node::People;
        }
        _ if ch.starts_with("topic:") => {
            let key = ch[6..].to_string();
            let (def, pi) = case.clone().unwrap();
            let ci = cast_idx.unwrap();
            let t = def.cast[ci as usize].topics.iter().find(|t| t.key == key).cloned().unwrap();
            let serum = game.player.truth_serum_on == Some(pid);
            let lied_before = game.cases[pi].lies_heard.iter().any(|(i, k)| *i == ci && *k == key);
            let trust = game.pop.get(pid).elias.trust;
            if t.lie.is_some() && !serum && !(trust > 60) {
                // hiding something
                dlg.say(name, t.lie.unwrap());
                if !lied_before {
                    game.cases[pi].lies_heard.push((ci, key.clone()));
                }
                // Observation lets Elias notice the lie
                if game.player.skill(Skill::Observation) >= 3 {
                    toasts.push(format!("{} desvia o olhar. Está mentindo.", game.pop.get(pid).first));
                }
            } else {
                tell_truth(&t, ci, pi, &def, pid, dlg, game, sfx, toasts);
                if serum {
                    game.player.truth_serum_on = None;
                }
            }
        }
        _ if ch.starts_with("press:") => {
            let key = ch[6..].to_string();
            dlg.node = Node::Pressure(cast_idx.unwrap_or(0), key);
        }
        "pr_press" | "pr_bluff" | "pr_bribe" | "pr_protect" | "pr_threat" | "pr_evidence" | "pr_nomoney" => {
            let Node::Pressure(ci, key) = dlg.node.clone() else { return };
            let (def, pi) = case.clone().unwrap();
            let t = def.cast[ci as usize].topics.iter().find(|t| t.key == key).cloned().unwrap();
            let p = game.pop.get(pid).clone();
            let tr = p.traits;
            let success = match ch {
                "pr_press" => r < 0.25 + persuasion * 0.06 + (tr.fear as f32 - tr.courage as f32) / 250.0,
                "pr_bluff" => r < 0.3 + deception * 0.07 - tr.curiosity as f32 / 400.0,
                "pr_bribe" => {
                    let cost = bribe_cost(&p);
                    game.player.money -= cost;
                    game.pop.get_mut(pid).money += cost;
                    sfx.write(Sfx::Cash);
                    tr.greed > 35 || tr.morality < 55
                }
                "pr_protect" => tr.fear > 55 && p.elias.trust > -10,
                "pr_threat" => tr.fear as i32 > tr.courage as i32 - 10,
                "pr_evidence" => t.breaker.map(|b| game.cases[pi].has(b)).unwrap_or(false),
                _ => false,
            };
            if ch == "pr_nomoney" {
                dlg.say(name, "Com que dinheiro?");
                return;
            }
            if ch == "pr_threat" {
                game.pop.get_mut(pid).elias.fear += 25;
                game.pop.get_mut(pid).elias.trust -= 20;
                game.add_rep(crate::state::Group::Citizens, -3);
            }
            if success {
                let line = match ch {
                    "pr_evidence" => "...Tá. Tá bom. Você venceu. Eu conto.",
                    "pr_bribe" => "Esse dinheiro nunca existiu. Nem essa conversa.",
                    "pr_protect" => "Promete? Promete que ninguém vai saber?",
                    "pr_threat" => "Tá bom! Tá bom! Eu falo, abaixa isso!",
                    "pr_bluff" => "Se você já sabe... então não adianta mais esconder.",
                    _ => "Chega... Eu não aguento mais guardar isso.",
                };
                dlg.say(name.clone(), line);
                if !game.cases[pi].lies_broken.iter().any(|(i, k)| *i == ci && *k == key) {
                    game.cases[pi].lies_broken.push((ci, key.clone()));
                }
                let train = match ch {
                    "pr_bluff" => Skill::Deception,
                    "pr_evidence" => Skill::Forensics,
                    _ => Skill::Persuasion,
                };
                game.player.train(train, 8);
                tell_truth(&t, ci, pi, &def, pid, dlg, game, sfx, toasts);
                dlg.node = Node::Case;
            } else {
                let p = game.pop.get(pid).clone();
                let flees = def.cast[ci as usize].flees;
                if flees && (ch == "pr_threat" || ch == "pr_press" || ch == "pr_evidence") && r < 0.7 {
                    dlg.say(name, "Você não vai me pegar!");
                    dlg.start_pursuit = Some(pid);
                    dlg.end = true;
                    return;
                }
                let line = match ch {
                    "pr_bluff" => {
                        game.pop.get_mut(pid).elias.caught_lies += 1;
                        "Você não sabe de nada. Está jogando verde."
                    }
                    "pr_bribe" => "Guarde seu dinheiro. Eu não estou à venda.",
                    "pr_protect" => "Proteger? Você nem consegue se proteger.",
                    "pr_threat" => {
                        if p.traits.courage > 70 {
                            "Tente. Vamos ver quem sai andando daqui."
                        } else {
                            "Eu vou chamar a polícia!"
                        }
                    }
                    "pr_evidence" => "Isso não prova nada.",
                    _ => {
                        if p.traits.morality < 25 {
                            "(sorri) Você é bonzinho demais pra esse trabalho."
                        } else if p.traits.fear > 70 {
                            "Por favor... me deixa em paz..."
                        } else {
                            "Já disse o que tinha pra dizer."
                        }
                    }
                };
                dlg.say(name, line);
                game.pop.get_mut(pid).elias.trust -= 8;
            }
        }
        _ if ch.starts_with("show:") => {
            let c: u8 = ch[5..].parse().unwrap_or(0);
            let (def, pi) = case.clone().unwrap();
            let clue = &def.clues[c as usize];
            // is it a breaker for one of this person's lies?
            let mut broke = false;
            if let Some(ci) = cast_idx {
                for t in def.cast[ci as usize].topics.clone() {
                    if t.breaker == Some(c) && game.cases[pi].lies_heard.iter().any(|(i, k)| *i == ci && *k == t.key) && !game.cases[pi].lies_broken.iter().any(|(i, k)| *i == ci && *k == t.key) {
                        dlg.say(name.clone(), "...Onde você conseguiu isso? Tá bom. Eu falo.");
                        game.cases[pi].lies_broken.push((ci, t.key.to_string()));
                        tell_truth(&t, ci, pi, &def, pid, dlg, game, sfx, toasts);
                        game.player.train(Skill::Forensics, 6);
                        broke = true;
                        break;
                    }
                }
            }
            if !broke {
                let implicated = cast_idx.map(|ci| clue.points.contains(&ci)).unwrap_or(false);
                let p = game.pop.get(pid);
                let line = if implicated {
                    if p.traits.morality < 30 {
                        "(os olhos endurecem) Cuidado com o que você anda recolhendo por aí."
                    } else {
                        "Isso... isso não é o que parece."
                    }
                } else {
                    match (r * 3.0) as i32 {
                        0 => "Nunca vi isso na vida.",
                        1 => "E o que isso tem a ver comigo?",
                        _ => "Estranho. Mas não sei de nada.",
                    }
                };
                dlg.say(name, line);
            }
            dlg.node = Node::Evidence;
        }
        _ if ch.starts_with("witness:") => {
            let id: u32 = ch[8..].parse().unwrap_or(0);
            dlg.node = Node::Witness(id);
        }
        _ if ch.starts_with("w_persuade:") || ch.starts_with("w_bribe:") || ch.starts_with("w_threat:") => {
            let (kind, id) = ch.split_once(':').unwrap();
            let id: u32 = id.parse().unwrap_or(0);
            let p = game.pop.get(pid).clone();
            let ok = match kind {
                "w_persuade" => r < 0.2 + persuasion * 0.08 + p.elias.trust as f32 / 200.0,
                "w_bribe" => {
                    let cost = bribe_cost(&p) * 2;
                    if game.player.money >= cost {
                        game.player.money -= cost;
                        sfx.write(Sfx::Cash);
                        p.traits.greed > 30 || p.traits.morality < 60
                    } else {
                        false
                    }
                }
                _ => p.traits.fear as i32 + game.pop.get(pid).elias.fear > p.traits.courage as i32,
            };
            if kind == "w_threat" {
                game.pop.get_mut(pid).elias.fear += 30;
            }
            if ok {
                for c in game.police.crimes.iter_mut() {
                    if c.id == id {
                        for w in c.witnesses.iter_mut() {
                            if w.pid == pid {
                                w.will_report = false;
                                w.silenced = Some(kind.to_string());
                            }
                        }
                    }
                }
                if let Some(a) = sim.agent_mut(pid) {
                    if matches!(a.state, AState::Report { .. }) {
                        a.state = AState::Normal;
                    }
                }
                dlg.say(name, match kind {
                    "w_bribe" => "Que crime? Eu não vi crime nenhum.",
                    "w_threat" => "Eu... eu não vou falar nada. Juro.",
                    _ => "Talvez... talvez eu tenha me confundido. Estava escuro.",
                });
            } else {
                dlg.say(name, "Eu sei o que vi. E a polícia também vai saber.");
                let crime = id;
                dlg.end = true;
                if let Some(a) = sim.agent_mut(pid) {
                    a.state = AState::Report { crime };
                }
                return;
            }
            dlg.node = Node::Root;
        }
        "conf_go" => {
            let (def, pi) = case.clone().unwrap();
            let ci = cast_idx.unwrap();
            game.cases[pi].accused = Some(ci);
            let is_culprit = ci == def.culprit;
            let flees = def.cast[ci as usize].flees;
            if is_culprit && flees && !game.flag(&format!("caught:{}", def.id)) {
                dlg.say(name, "(empurra você e corre)");
                dlg.start_pursuit = Some(pid);
                dlg.end = true;
                return;
            }
            let evidence = game.cases[pi].found.iter().filter(|c| def.clues[**c as usize].points.contains(&ci)).count();
            let line = if is_culprit {
                if evidence >= 3 {
                    "(longo silêncio) ...Ele não quis pagar. Ninguém recusa. Ninguém."
                } else {
                    "Você não tem nada contra mim. Mas vou te dizer uma coisa: cuidado com a porta dos fundos."
                }
            } else {
                "Eu? Você está louco! Eu não fiz nada!"
            };
            dlg.say(name, line);
            dlg.resolve_case = Some((def.id, true));
            dlg.end = true;
        }
        "report" => {
            dlg.say(name, "Um culpado? Mostre o que você tem.");
            dlg.resolve_case = case.as_ref().map(|(d, _)| (d.id, false));
            dlg.end = true;
        }
        "shop" => {
            dlg.open_shop = true;
            dlg.end = true;
        }
        "black" => {
            let p = game.pop.get(pid).clone();
            if p.elias.trust > 10 || game.rep(crate::state::Group::Crime) > 10 || r < 0.35 {
                dlg.say(name, "Fala baixo. Vem cá atrás.");
                BLACK.with(|b| *b.borrow_mut() = true);
                dlg.open_shop = true;
                dlg.end = true;
            } else {
                dlg.say(name, "Não sei do que você está falando. Vai embora.");
            }
        }
        "sellcar" => {
            SELLCAR.with(|b| *b.borrow_mut() = true);
            dlg.say(name, "Deixa eu ver a lataria... Tá. Fechado.");
            dlg.end = true;
        }
        "rob" | "tie" | "kidnap" | "robshop" => {
            let p = game.pop.get(pid).clone();
            let t = p.traits;
            let armed_back = p.job.armed() && t.courage > 55;
            let mut rr = Rng::new(game.abs_minute() as u64 ^ pid as u64);
            if armed_back && r < 0.6 {
                dlg.say(name, "Você escolheu a pessoa errada.");
                if let Some(a) = sim.agent_mut(pid) {
                    a.state = AState::Hostile;
                }
                dlg.crimes.push((crate::state::CrimeKind::Assault, pid));
                dlg.end = true;
                return;
            }
            if t.courage > 80 && t.fear < 30 && r < 0.4 && ch != "robshop" {
                dlg.say(name, "Socorro! SOCORRO!");
                if let Some(a) = sim.agent_mut(pid) {
                    a.state = AState::Flee { from: game.player.pos, until: game.abs_minute() + 40.0 };
                    a.path.clear();
                }
                dlg.crimes.push((crate::state::CrimeKind::Assault, pid));
                dlg.end = true;
                return;
            }
            match ch {
                "rob" => {
                    let line = crate::sim::speech::plead(&p, &game.pop, crate::sim::speech::Situation::Robbed, &mut rr, year);
                    dlg.say(name.clone(), line);
                    let money = p.money;
                    game.player.money += money;
                    game.pop.get_mut(pid).money = 0;
                    let val = crate::items::Item::Valuable(if p.job.income() >= 7 { "Relógio de ouro".into() } else { "Aliança".into() }, 10 + (p.seed % 40) as i32);
                    toasts.push(format!("Levou {} e {}.", crate::economy::money_str(game, money), val.name(year)));
                    game.player.inv.push(val);
                    game.pop.get_mut(pid).elias.fear += 40;
                    dlg.crimes.push((crate::state::CrimeKind::Theft, pid));
                    dlg.hands_up.push(pid);
                    sfx.write(Sfx::Cash);
                }
                "tie" => {
                    let line = crate::sim::speech::plead(&p, &game.pop, crate::sim::speech::Situation::Tied, &mut rr, year);
                    dlg.say(name, line);
                    if let Some(a) = sim.agent_mut(pid) {
                        a.state = AState::Tied { since: game.abs_minute(), gagged: false };
                    }
                    dlg.crimes.push((crate::state::CrimeKind::Kidnap, pid));
                    dlg.with = None;
                }
                "kidnap" => {
                    let line = crate::sim::speech::plead(&p, &game.pop, crate::sim::speech::Situation::Hostage, &mut rr, year);
                    dlg.say(name, line);
                    if let Some(a) = sim.agent_mut(pid) {
                        a.state = AState::Hostage;
                    }
                    dlg.crimes.push((crate::state::CrimeKind::Kidnap, pid));
                    dlg.with = None;
                }
                _ => {
                    let line = crate::sim::speech::plead(&p, &game.pop, crate::sim::speech::Situation::Aimed, &mut rr, year);
                    dlg.say(name, line);
                    let cash = crate::economy::price(game, 20 + (p.seed % 100) as i32);
                    game.player.money += cash;
                    toasts.push(format!("O caixa: {}.", crate::economy::money_str(game, cash)));
                    dlg.crimes.push((crate::state::CrimeKind::Shooting, pid));
                    dlg.hands_up.push(pid);
                    sfx.write(Sfx::Cash);
                    game.stat("robberies", 1);
                }
            }
            dlg.end = true;
        }
        "recruit" => {
            let p = game.pop.get(pid).clone();
            let w = wage(p.job);
            if p.elias.trust >= 25 || r < 0.15 + persuasion * 0.05 {
                let pm = game.pop.get_mut(pid);
                pm.elias.in_network = true;
                pm.elias.loyalty = 50 + pm.traits.loyalty as i32 / 4;
                pm.elias.paid_until_day = day + 7;
                dlg.say(name.clone(), format!("Uma rede de gente que não aceita a versão oficial? ...Conte comigo. Mas eu cobro ${} por semana.", w));
                toasts.push(format!("{} entrou para a Rede.", name));
                game.write(format!("{} agora trabalha comigo. Espero não me arrepender.", name), true);
            } else {
                dlg.say(name, "Eu mal te conheço. Volte quando eu puder confiar em você.");
            }
        }
        "net_pay" => {
            let w = wage(game.pop.get(pid).job);
            if game.player.money >= w {
                game.player.money -= w;
                let pm = game.pop.get_mut(pid);
                pm.elias.paid_until_day = pm.elias.paid_until_day.max(day) + 7;
                pm.elias.loyalty += 5;
                sfx.write(Sfx::Cash);
                dlg.say(name, "Obrigado. Isso vai manter as luzes acesas.");
            } else {
                dlg.say(name, "Sem dinheiro de novo?");
            }
        }
        "net_leave" => {
            let pm = game.pop.get_mut(pid);
            pm.elias.in_network = false;
            pm.elias.trust -= 10;
            dlg.say(name, "Se é assim... boa sorte sozinho.");
            dlg.node = Node::Root;
        }
        "follow" => {
            let until = game.abs_minute() + 180.0;
            if let Some(a) = sim.agent_mut(pid) {
                a.state = AState::Follow { until };
            }
            dlg.say(name, "Vamos.");
            dlg.with = None;
            dlg.end = true;
            dlg.start_date = Some(pid);
        }
        _ if ch.starts_with("gift:") => {
            let g = match &ch[5..] {
                "Flowers" => Gift::Flowers,
                "Perfume" => Gift::Perfume,
                "Jewel" => Gift::Jewel,
                "Book" => Gift::Book,
                "Chocolates" => Gift::Chocolates,
                "Record" => Gift::Record,
                _ => Gift::Whiskey,
            };
            game.player.take(&Item::Gift(g));
            let p = game.pop.get_mut(pid);
            p.elias.gifts += 1;
            let v = g.value();
            let liked = (p.seed + g as u32) % 3 != 0;
            p.elias.affection += if liked { v } else { v / 3 };
            p.elias.trust += v / 3;
            dlg.say(name, if liked { "Pra mim? ...Ninguém nunca me deu algo assim." } else { "Obrigad... é, obrigado." });
            dlg.node = Node::Root;
        }
        _ if ch.starts_with("rom_") => romance(ch, pid, dlg, game, sfx, toasts, r),
        _ => {}
    }
    let _ = name;
}

fn whereabouts(game: &Game, pid: Pid) -> String {
    let p = game.pop.get(pid);
    let mut s = String::new();
    if let Some(w) = p.work {
        let (a, b) = p.job.shift();
        s.push_str(&format!("Trabalha das {}h às {}h. ", a as i32, (b as i32) % 24));
        let _ = w;
    }
    if p.traits.sociability > 55 {
        s.push_str("De noite vive nos bares. ");
    } else if p.traits.faith > 70 {
        s.push_str("Não sai da igreja. ");
    } else {
        s.push_str("De noite fica em casa. ");
    }
    s
}

#[allow(clippy::too_many_arguments)]
fn tell_truth(t: &crate::cases::defs::Topic, ci: u8, pi: usize, def: &crate::cases::defs::CaseDef, pid: Pid, dlg: &mut Dlg, game: &mut Game, sfx: &mut EventWriter<Sfx>, toasts: &mut hud::Toasts) {
    let name = game.pop.get(pid).name();
    dlg.say(name, t.a);
    let key = t.key.to_string();
    if !game.cases[pi].topics.iter().any(|(i, k)| *i == ci && *k == key) {
        game.cases[pi].topics.push((ci, key));
        game.pop.get_mut(pid).elias.trust += 2;
    }
    for c in t.reveals {
        if !game.cases[pi].found.contains(c) {
            game.cases[pi].found.push(*c);
            sfx.write(Sfx::Evidence);
            toasts.push(format!("DEPOIMENTO — {}", def.clues[*c as usize].name));
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn romance(ch: &str, pid: Pid, dlg: &mut Dlg, game: &mut Game, sfx: &mut EventWriter<Sfx>, toasts: &mut hud::Toasts, r: f32) {
    let name = game.pop.get(pid).name();
    let persuasion = game.player.skill(Skill::Persuasion) as i32;
    let day = game.day;
    let year = game.year;
    let married_other = game.pop.people.iter().any(|q| q.id != pid && q.elias.romance == Romance::Married && q.alive() && q.city == game.city);
    let p = game.pop.get(pid).clone();
    let has_spouse = p.spouse.is_some();
    match ch {
        "rom_compliment" => {
            let gain = 3 + persuasion + (p.traits.sociability as i32 / 30);
            game.pop.get_mut(pid).elias.affection += gain;
            dlg.say(name, if r < 0.5 { "Você fala bonito. Cuidado, que eu acredito." } else { "Ha! Obrigada. Faz tempo que ninguém repara." });
        }
        "rom_flirt" => {
            let chance = 0.3 + p.elias.trust as f32 / 150.0 + p.elias.affection as f32 / 120.0 + persuasion as f32 * 0.04 - if has_spouse { 0.25 } else { 0.0 };
            if r < chance {
                let pm = game.pop.get_mut(pid);
                pm.elias.affection += 8;
                pm.elias.romance = Romance::Flirting;
                dlg.say(name, if has_spouse { "Eu sou casada... mas você não precisa parar de sorrir assim." } else { "Você está flertando comigo? ...Continue." });
            } else {
                game.pop.get_mut(pid).elias.affection -= 3;
                dlg.say(name, "Guarde isso pra outra pessoa.");
            }
        }
        "rom_date" => {
            game.pop.get_mut(pid).elias.dates += 1;
            game.pop.get_mut(pid).elias.affection += 10;
            dlg.say(name, "Tá. Me leve a algum lugar bonito. E me pague um drinque.");
            dlg.start_date = Some(pid);
            dlg.end = true;
        }
        "rom_ask" => {
            if p.elias.affection >= 45 && r < 0.6 + p.elias.affection as f32 / 200.0 {
                game.pop.get_mut(pid).elias.romance = Romance::Dating;
                dlg.say(name.clone(), "Sim. Sim! Mas se você sumir, eu te acho.");
                toasts.push(format!("Você está namorando {}.", name));
                game.write(format!("Estou com {}. Não sei se isso é coragem ou egoísmo.", name), true);
                if married_other {
                    game.set("affair");
                }
            } else {
                dlg.say(name, "Ainda não. Eu mal sei de onde você vem.");
            }
        }
        "rom_kiss" => {
            game.pop.get_mut(pid).elias.affection += 4;
            if game.pop.get(pid).elias.romance == Romance::Dating {
                game.pop.get_mut(pid).elias.romance = Romance::Lovers;
            }
            dlg.say(name, "(um beijo longo, com gosto de chuva)");
            // someone might see — the jealous spouse or partner hears about it
            if married_other || has_spouse {
                game.set("affair");
            }
        }
        "rom_noring" => dlg.say(name, "Com que aliança, meu bem?"),
        "rom_propose" => {
            game.player.take(&Item::Gift(Gift::Jewel));
            game.pop.get_mut(pid).elias.romance = Romance::Engaged;
            dlg.say(name.clone(), "(chora) Sim. Mil vezes sim.");
            toasts.push(format!("Noivado com {}.", name));
            sfx.write(Sfx::Bell);
        }
        "rom_wed" => {
            if game.player.money >= 30 {
                game.player.money -= 30;
                let pm = game.pop.get_mut(pid);
                pm.elias.romance = Romance::Married;
                pm.spouse = None;
                dlg.say(name.clone(), "Na frente de Deus e de toda essa gente fofoqueira. Eu aceito.");
                toasts.push(format!("Você se casou com {}.", name));
                sfx.write(Sfx::Bell);
                game.write(format!("Casei com {} em {}. Se a esfera me chamar agora, eu não sei se vou.", name, year), true);
                game.set("married");
                game.stat("marriages", 1);
            } else {
                dlg.say(name, "O padre cobra trinta dólares. E você não tem.");
            }
        }
        "rom_home" => {
            let kids: Vec<String> = game.pop.people.iter().filter(|k| k.elias_child && k.parents.contains(&Some(pid)) && k.alive()).map(|k| k.first.clone()).collect();
            let line = if kids.is_empty() {
                "A casa é grande demais sem criança correndo. Só estou dizendo.".to_string()
            } else {
                format!("{} perguntou por você hoje. Disse que você cheira a chuva de outro lugar.", kids.join(" e "))
            };
            dlg.say(name, line);
            let _ = day;
        }
        "rom_truth" => {
            let dv = p.elias.deja_vu > 0;
            let line = if dv || p.traits.curiosity > 70 {
                "Uma esfera... e outras vidas. Eu deveria rir. Mas eu sonho com isso desde criança. Uma casa que não existe. Você nela."
            } else {
                "Você precisa de um médico, não de mim. ...Mas eu fico. Mesmo louco."
            };
            game.pop.get_mut(pid).elias.knows_secret.push("sphere".into());
            dlg.say(name, line);
        }
        "rom_break" => {
            let pm = game.pop.get_mut(pid);
            pm.elias.romance = Romance::Broken;
            pm.elias.affection -= 40;
            pm.elias.trust -= 30;
            dlg.say(name.clone(), "Então vá. Vá pra onde quer que você sempre vai.");
            game.write(format!("Terminei com {}. Disse que era para protegê-l{}. Talvez fosse para me proteger.", name, p.o()), true);
        }
        _ => {}
    }
}
