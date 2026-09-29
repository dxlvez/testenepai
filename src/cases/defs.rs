//! Case definitions: cast, clues, echoes, topics, solution, consequences.

use crate::city::gen::CityId;
use crate::city::map::BKind;
use crate::sim::people::Job;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    Victim,
    Suspect,
    Witness,
    Contact,
}

/// A place in the city, resolved at case start.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Place {
    /// nth building of a kind
    B(BKind, u8),
    /// home of a cast member
    HomeOf(u8),
    /// workplace of a cast member
    WorkOf(u8),
    /// outside, near the door of nth building of a kind
    Near(BKind, u8),
    Park,
    Cemetery,
    Docks,
    Woods,
    Alley(u8),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Req {
    None,
    Clue(u8),
    Topic(&'static str),
    Trust(i32),
    Both(u8, u8),
}

#[derive(Clone, Debug)]
pub struct Topic {
    pub key: &'static str,
    /// what Elias asks
    pub q: &'static str,
    /// the truthful answer
    pub a: &'static str,
    /// what they say first if they are hiding something
    pub lie: Option<&'static str>,
    /// clue that breaks the lie when shown
    pub breaker: Option<u8>,
    /// clues (social testimony) revealed by the true answer
    pub reveals: &'static [u8],
    pub req: Req,
}

#[derive(Clone, Debug)]
pub struct Cast {
    pub first: &'static str,
    pub last: &'static str,
    pub female: bool,
    pub age: i32,
    pub job: Job,
    pub role: Role,
    pub dead: bool,
    pub home: Place,
    pub desc: &'static str,
    pub topics: Vec<Topic>,
    /// personality presets: (fear, courage, morality, greed)
    pub temper: (u8, u8, u8, u8),
    /// runs away when cornered
    pub flees: bool,
    /// recurring soul (Clara = 1, the other investigator = 2, ...)
    pub soul: Option<u8>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ClueKind {
    Physical,
    Document,
    Testimony,
    Temporal,
}

impl ClueKind {
    pub fn label(self) -> &'static str {
        match self {
            ClueKind::Physical => "FÍSICA",
            ClueKind::Document => "DOCUMENTO",
            ClueKind::Testimony => "DEPOIMENTO",
            ClueKind::Temporal => "TEMPORAL",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Look3d {
    Blood,
    Paper,
    Weapon,
    Object,
    Photo,
    Prints,
    Glow,
    None,
}

#[derive(Clone, Debug)]
pub struct Clue {
    pub name: &'static str,
    pub desc: &'static str,
    /// extra detail with Forensics >= 3
    pub forensic: Option<&'static str>,
    pub kind: ClueKind,
    /// where it lies (Physical / Document / Temporal). Testimony clues have None.
    pub place: Option<Place>,
    pub look: Look3d,
    /// Observation level needed to notice it
    pub hidden: u8,
    /// cast members it points at
    pub points: &'static [u8],
    pub red_herring: bool,
    /// Network specialist that can obtain it remotely
    pub network: Option<Job>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ghost {
    Struggle,
    Flee,
    Drag,
    Write,
    Hide,
    Argue,
    Vanish,
    Walk,
    Wait,
}

#[derive(Clone, Debug)]
pub struct Echo {
    pub place: Place,
    pub ghost: Ghost,
    pub frames: &'static [&'static str],
    /// clue unlocked by witnessing the echo
    pub unlocks: u8,
    pub min_level: u8,
}

#[derive(Clone, Debug, Default)]
pub struct Outcome {
    pub text: &'static str,
    pub headline: &'static str,
    pub radio: &'static str,
    /// (cast idx, destiny tag) — e.g. "jailed", "dead", "left_city", "police", "criminal"
    pub fates: Vec<(u8, &'static str)>,
    pub close: Vec<BKind>,
    pub flags: Vec<&'static str>,
}

#[derive(Clone, Debug)]
pub struct CaseDef {
    pub id: u8,
    pub title: &'static str,
    pub city: CityId,
    pub year: i32,
    pub intro: &'static str,
    pub brief: &'static str,
    pub start: Place,
    pub cast: Vec<Cast>,
    pub clues: Vec<Clue>,
    pub echoes: Vec<Echo>,
    pub culprit: u8,
    pub methods: &'static [&'static str],
    pub method: u8,
    pub motives: &'static [&'static str],
    pub motive: u8,
    pub anomalies: &'static [&'static str],
    pub anomaly: u8,
    pub threads: &'static [(u8, u8)],
    pub story: &'static str,
    pub sphere: &'static str,
    pub on_true: Outcome,
    pub on_false: Outcome,
    /// digit hidden in the case file (the secret coordinates)
    pub digit: &'static str,
    pub reward: i32,
}

// ------------------------------------------------------------------ helpers for concise content

pub fn topic(key: &'static str, q: &'static str, a: &'static str) -> Topic {
    Topic { key, q, a, lie: None, breaker: None, reveals: &[], req: Req::None }
}

impl Topic {
    pub fn lie(mut self, lie: &'static str, breaker: u8) -> Topic {
        self.lie = Some(lie);
        self.breaker = Some(breaker);
        self
    }
    pub fn reveals(mut self, r: &'static [u8]) -> Topic {
        self.reveals = r;
        self
    }
    pub fn req(mut self, r: Req) -> Topic {
        self.req = r;
        self
    }
}

pub fn person(first: &'static str, last: &'static str, female: bool, age: i32, job: Job, role: Role, home: Place, desc: &'static str) -> Cast {
    Cast { first, last, female, age, job, role, dead: false, home, desc, topics: Vec::new(), temper: (50, 50, 50, 50), flees: false, soul: None }
}

impl Cast {
    pub fn dead(mut self) -> Cast {
        self.dead = true;
        self
    }
    pub fn topics(mut self, t: Vec<Topic>) -> Cast {
        self.topics = t;
        self
    }
    pub fn temper(mut self, fear: u8, courage: u8, morality: u8, greed: u8) -> Cast {
        self.temper = (fear, courage, morality, greed);
        self
    }
    pub fn flees(mut self) -> Cast {
        self.flees = true;
        self
    }
    pub fn soul(mut self, s: u8) -> Cast {
        self.soul = Some(s);
        self
    }
}

pub fn clue(name: &'static str, kind: ClueKind, place: Option<Place>, look: Look3d, desc: &'static str, points: &'static [u8]) -> Clue {
    Clue { name, desc, forensic: None, kind, place, look, hidden: 0, points, red_herring: false, network: None }
}

pub fn testimony(name: &'static str, desc: &'static str, points: &'static [u8]) -> Clue {
    clue(name, ClueKind::Testimony, None, Look3d::None, desc, points)
}

impl Clue {
    pub fn forensic(mut self, f: &'static str) -> Clue {
        self.forensic = Some(f);
        self
    }
    pub fn hidden(mut self, h: u8) -> Clue {
        self.hidden = h;
        self
    }
    pub fn herring(mut self) -> Clue {
        self.red_herring = true;
        self
    }
    pub fn network(mut self, j: Job) -> Clue {
        self.network = Some(j);
        self
    }
}

pub fn echo(place: Place, ghost: Ghost, frames: &'static [&'static str], unlocks: u8) -> Echo {
    Echo { place, ghost, frames, unlocks, min_level: 0 }
}

pub fn outcome(text: &'static str, headline: &'static str, radio: &'static str, fates: Vec<(u8, &'static str)>) -> Outcome {
    Outcome { text, headline, radio, fates, close: Vec::new(), flags: Vec::new() }
}

impl Outcome {
    pub fn close(mut self, k: Vec<BKind>) -> Outcome {
        self.close = k;
        self
    }
    pub fn flags(mut self, f: Vec<&'static str>) -> Outcome {
        self.flags = f;
        self
    }
}
