//! The persistent game state (everything that goes into a save file).

use crate::city::gen::CityId;
use crate::items::*;
use crate::sim::people::{Pid, Population};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Hash)]
pub enum Skill {
    Observation,
    Echo,
    Persuasion,
    Deception,
    Forensics,
    Stealth,
    Combat,
    Temporal,
}

impl Skill {
    pub const ALL: [Skill; 8] = [
        Skill::Observation,
        Skill::Echo,
        Skill::Persuasion,
        Skill::Deception,
        Skill::Forensics,
        Skill::Stealth,
        Skill::Combat,
        Skill::Temporal,
    ];
    pub fn name(self) -> &'static str {
        match self {
            Skill::Observation => "Observação",
            Skill::Echo => "Eco",
            Skill::Persuasion => "Persuasão",
            Skill::Deception => "Dissimulação",
            Skill::Forensics => "Perícia",
            Skill::Stealth => "Furtividade",
            Skill::Combat => "Combate",
            Skill::Temporal => "Temporal",
        }
    }
    pub fn perk(self, lvl: u32) -> &'static str {
        match (self, lvl) {
            (Skill::Observation, 0..=2) => "Percebe pistas óbvias.",
            (Skill::Observation, 3..=5) => "Percebe inconsistências pequenas: objetos fora do lugar brilham de leve.",
            (Skill::Observation, _) => "Detecta pistas escondidas e percebe quando alguém mente pelo olhar.",
            (Skill::Echo, 0..=2) => "Ecos de ~3 segundos, fragmentados.",
            (Skill::Echo, 3..=5) => "Ecos de ~10 segundos com mais fragmentos.",
            (Skill::Echo, _) => "Sequências inteiras. Desbloqueia Caminhada do Eco (seguir pegadas do passado).",
            (Skill::Persuasion, 0..=2) => "Conversas básicas.",
            (Skill::Persuasion, 3..=5) => "Convence testemunhas a não denunciar; preços melhores.",
            (Skill::Persuasion, _) => "Pode fazer suspeitos confessarem sem prova completa.",
            (Skill::Deception, 0..=2) => "Mentiras simples. Mentir é arriscado.",
            (Skill::Deception, 3..=5) => "Blefes melhores; 'fingir saber' funciona mais.",
            (Skill::Deception, _) => "Mentiras quase indetectáveis; falsificar documentos sozinho.",
            (Skill::Forensics, 0..=2) => "Descreve evidências por cima.",
            (Skill::Forensics, 3..=5) => "Identifica armas, horários de morte e pegadas.",
            (Skill::Forensics, _) => "Reconstrói a cena: mostra ligações extras no quadro.",
            (Skill::Stealth, 0..=2) => "Anda devagar para fazer menos barulho.",
            (Skill::Stealth, 3..=5) => "Passos silenciosos; arrombar faz menos barulho.",
            (Skill::Stealth, _) => "Quase invisível nas sombras.",
            (Skill::Combat, 0..=2) => "Elias não sabe brigar. Dois tiros matam.",
            (Skill::Combat, 3..=5) => "Esquiva e contra-ataque; mira mais firme.",
            (Skill::Combat, _) => "Desarme e derrubada; mais resistência.",
            (Skill::Temporal, 0..=1) => "Nada ainda.",
            (Skill::Temporal, 2..=3) => "Rastro de Memória: tocar objetos revela emoções.",
            (Skill::Temporal, 4..=5) => "Trava Temporal: congela uma pequena área por segundos.",
            (Skill::Temporal, _) => "Caminhada do Eco completa.",
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Identity {
    pub name: String,
    pub job: String,
    pub origin: String,
    pub quality: u8,
    pub burned: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub pos: Vec2,
    pub health: f32,
    pub money: i32,
    pub inv: Vec<Item>,
    pub weapon: Weapon,
    pub mag: i32,
    pub outfit: Outfit,
    pub owned_outfits: Vec<Outfit>,
    pub alias: String,
    pub identity: Option<Identity>,
    pub skills: HashMap<Skill, u32>,
    pub xp: HashMap<Skill, u32>,
    pub stamina: f32,
    pub addiction: f32,
    pub high: f32,
    pub stim: f32,
    pub visionary: f32,
    pub truth_serum_on: Option<Pid>,
    pub body_age: f32,
    pub mind_years: f32,
    pub safehouse: Option<usize>,
    pub owned: Vec<usize>,
    /// "city:year" of the map that `safehouse` / `owned` / opened doors refer to
    #[serde(default)]
    pub map_key: String,
    pub car: Option<u32>,
    pub hunger: f32,
    pub sleep: f32,
}

impl Default for PlayerData {
    fn default() -> Self {
        PlayerData {
            pos: Vec2::ZERO,
            health: 100.0,
            money: 23,
            inv: vec![Item::Tool(Tool::Bandage), Item::Food],
            weapon: Weapon::Fists,
            mag: 0,
            outfit: Outfit::Modern,
            owned_outfits: vec![Outfit::Modern],
            alias: "Elias Vale".into(),
            identity: None,
            skills: Skill::ALL.iter().map(|s| (*s, if *s == Skill::Observation { 1 } else { 0 })).collect(),
            xp: HashMap::new(),
            stamina: 1.0,
            addiction: 0.0,
            high: 0.0,
            stim: 0.0,
            visionary: 0.0,
            truth_serum_on: None,
            body_age: 34.0,
            mind_years: 0.0,
            safehouse: None,
            map_key: String::new(),
            owned: Vec::new(),
            car: None,
            hunger: 0.0,
            sleep: 0.0,
        }
    }
}

impl PlayerData {
    pub fn skill(&self, s: Skill) -> u32 {
        *self.skills.get(&s).unwrap_or(&0)
    }
    /// Gain experience; returns Some(new level) on level up.
    pub fn train(&mut self, s: Skill, amount: u32) -> Option<u32> {
        let x = self.xp.entry(s).or_insert(0);
        *x += amount;
        let lvl = *self.skills.get(&s).unwrap_or(&0);
        let need = 20 + lvl * 25;
        if *x >= need && lvl < 10 {
            *x -= need;
            self.skills.insert(s, lvl + 1);
            return Some(lvl + 1);
        }
        None
    }
    pub fn count(&self, it: &Item) -> usize {
        self.inv.iter().filter(|i| *i == it).count()
    }
    pub fn has(&self, it: &Item) -> bool {
        self.inv.contains(it)
    }
    pub fn take(&mut self, it: &Item) -> bool {
        if let Some(i) = self.inv.iter().position(|x| x == it) {
            self.inv.remove(i);
            true
        } else {
            false
        }
    }
    pub fn ammo(&self) -> i32 {
        self.inv.iter().map(|i| if let Item::Ammo(n) = i { *n } else { 0 }).sum()
    }
    pub fn use_ammo(&mut self, n: i32) -> i32 {
        let mut need = n;
        for it in self.inv.iter_mut() {
            if let Item::Ammo(a) = it {
                let k = (*a).min(need);
                *a -= k;
                need -= k;
            }
        }
        self.inv.retain(|i| !matches!(i, Item::Ammo(0)));
        n - need
    }
    pub fn add_ammo(&mut self, n: i32) {
        for it in self.inv.iter_mut() {
            if let Item::Ammo(a) = it {
                *a += n;
                return;
            }
        }
        self.inv.push(Item::Ammo(n));
    }
    pub fn weapons(&self) -> Vec<Weapon> {
        let mut v = vec![Weapon::Fists];
        for i in &self.inv {
            if let Item::Weapon(w) = i {
                if !v.contains(w) {
                    v.push(*w);
                }
            }
        }
        v
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum CrimeKind {
    Murder,
    Assault,
    Theft,
    BreakIn,
    CarTheft,
    Drugging,
    Kidnap,
    Arson,
    Shooting,
}

impl CrimeKind {
    pub fn label(self) -> &'static str {
        match self {
            CrimeKind::Murder => "Assassinato",
            CrimeKind::Assault => "Agressão",
            CrimeKind::Theft => "Furto",
            CrimeKind::BreakIn => "Invasão",
            CrimeKind::CarTheft => "Roubo de carro",
            CrimeKind::Drugging => "Envenenamento",
            CrimeKind::Kidnap => "Sequestro",
            CrimeKind::Arson => "Incêndio",
            CrimeKind::Shooting => "Disparos",
        }
    }
    pub fn severity(self) -> i32 {
        match self {
            CrimeKind::Murder => 60,
            CrimeKind::Kidnap => 40,
            CrimeKind::Shooting => 25,
            CrimeKind::Drugging => 25,
            CrimeKind::Assault => 15,
            CrimeKind::Arson => 30,
            CrimeKind::CarTheft => 10,
            CrimeKind::BreakIn => 8,
            CrimeKind::Theft => 5,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Crime {
    pub id: u32,
    pub kind: CrimeKind,
    pub by_elias: bool,
    pub victim: Option<Pid>,
    pub pos: Vec2,
    pub day: i32,
    pub minute: f32,
    pub weapon: Option<Weapon>,
    pub outfit: Outfit,
    pub discovered: bool,
    pub reported: bool,
    pub witnesses: Vec<Witness>,
    pub body_hidden: bool,
    pub blood: bool,
    pub place: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Witness {
    pub pid: Pid,
    pub saw_face: f32,
    pub saw_weapon: bool,
    pub saw_body: bool,
    pub will_report: bool,
    pub reported: bool,
    pub silenced: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Police {
    pub crimes: Vec<Crime>,
    pub heat: f32,
    pub wanted: u8,
    pub profile_outfits: Vec<Outfit>,
    pub profile_weapon: Option<Weapon>,
    pub knows_name: Option<String>,
    pub knows_face: f32,
    pub profile_notes: Vec<String>,
    pub next_id: u32,
    pub arrests: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Headline {
    pub day: i32,
    pub year: i32,
    pub city: CityId,
    pub timeline: u32,
    pub title: String,
    pub body: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub year: i32,
    pub day: i32,
    pub text: String,
    pub thought: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MemoryRecord {
    pub timeline: u32,
    pub year: i32,
    pub city: CityId,
    pub subject: String,
    pub status: String,
    pub killed_by: Option<String>,
    pub witnesses: u32,
    pub consequences: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MapMark {
    pub pos: Vec2,
    pub label: String,
    pub city: CityId,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Photo {
    pub year: i32,
    pub day: i32,
    pub place: String,
    pub people: Vec<String>,
    pub anomaly: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Hash)]
pub enum Group {
    Citizens,
    Police,
    Crime,
    Press,
    Elite,
}

impl Group {
    pub const ALL: [Group; 5] = [Group::Citizens, Group::Police, Group::Crime, Group::Press, Group::Elite];
    pub fn name(self) -> &'static str {
        match self {
            Group::Citizens => "Cidadãos",
            Group::Police => "Polícia",
            Group::Crime => "Submundo",
            Group::Press => "Imprensa",
            Group::Elite => "Elite",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Phase {
    /// walking around a city investigating
    City,
    /// the prologue apartment / laboratory
    Prologue,
}

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct Game {
    pub city: CityId,
    pub year: i32,
    pub day: i32,
    pub minute: f32,
    pub rain: f32,
    pub rain_target: f32,
    pub fog: f32,
    pub timeline: u32,
    pub pop: Population,
    pub player: PlayerData,
    pub police: Police,
    pub rep: Vec<(CityId, Group, i32)>,
    pub fatigue: f32,
    pub journal: Vec<JournalEntry>,
    pub papers: Vec<Headline>,
    pub photos: Vec<Photo>,
    pub marks: Vec<MapMark>,
    pub memory: Vec<MemoryRecord>,
    pub flags: HashSet<String>,
    pub closed: Vec<(CityId, usize, i32)>,
    pub seed: u64,
    pub case_idx: usize,
    pub cases: Vec<crate::cases::CaseProgress>,
    pub limbo_stage: u8,
    pub stats: HashMap<String, i32>,
    pub phase: Phase,
    pub populated: Vec<(CityId, i32)>,
    pub alterations: u32,
    pub save_slot: u32,
    pub played_secs: f32,
    pub news_ticker: Vec<String>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            city: CityId::NewOrleans,
            year: 1920,
            day: 0,
            minute: 21.0 * 60.0,
            rain: 0.7,
            rain_target: 0.7,
            fog: 0.4,
            timeline: 1,
            pop: Population::default(),
            player: PlayerData::default(),
            police: Police::default(),
            rep: Vec::new(),
            fatigue: 0.0,
            journal: Vec::new(),
            papers: Vec::new(),
            photos: Vec::new(),
            marks: Vec::new(),
            memory: Vec::new(),
            flags: HashSet::new(),
            closed: Vec::new(),
            seed: 1920,
            case_idx: 0,
            cases: Vec::new(),
            limbo_stage: 0,
            stats: HashMap::new(),
            phase: Phase::Prologue,
            populated: Vec::new(),
            alterations: 0,
            save_slot: 0,
            played_secs: 0.0,
            news_ticker: Vec::new(),
        }
    }
    pub fn hour(&self) -> f32 {
        self.minute / 60.0
    }
    pub fn abs_minute(&self) -> f32 {
        self.day as f32 * 1440.0 + self.minute
    }
    pub fn flag(&self, f: &str) -> bool {
        self.flags.contains(f)
    }
    pub fn set(&mut self, f: &str) {
        self.flags.insert(f.to_string());
    }
    pub fn stat(&mut self, k: &str, d: i32) {
        *self.stats.entry(k.to_string()).or_insert(0) += d;
    }
    pub fn get_stat(&self, k: &str) -> i32 {
        *self.stats.get(k).unwrap_or(&0)
    }
    pub fn rep(&self, g: Group) -> i32 {
        self.rep.iter().find(|(c, gg, _)| *c == self.city && *gg == g).map(|r| r.2).unwrap_or(0)
    }
    pub fn add_rep(&mut self, g: Group, d: i32) {
        let city = self.city;
        if let Some(r) = self.rep.iter_mut().find(|(c, gg, _)| *c == city && *gg == g) {
            r.2 = (r.2 + d).clamp(-100, 100);
        } else {
            self.rep.push((city, g, d.clamp(-100, 100)));
        }
    }
    pub fn write(&mut self, text: impl Into<String>, thought: bool) {
        let e = JournalEntry { year: self.year, day: self.day, text: text.into(), thought };
        self.journal.push(e);
    }
    pub fn date_str(&self) -> String {
        let months = ["jan", "fev", "mar", "abr", "mai", "jun", "jul", "ago", "set", "out", "nov", "dez"];
        let doy = self.day.rem_euclid(365);
        let m = (doy / 31).min(11) as usize;
        let d = doy % 31 + 1;
        let wd = ["Seg", "Ter", "Qua", "Qui", "Sex", "Sáb", "Dom"][self.day.rem_euclid(7) as usize];
        format!("{} {:02} {} {}", wd, d, months[m], self.year)
    }
    pub fn timeline_code(&self) -> String {
        format!("R-{:02}", self.timeline)
    }
}
