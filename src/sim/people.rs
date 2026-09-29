//! The persistent population: every person that ever exists in any timeline
//! lives here, with family tree, job, home, personality, relationships and
//! memories. Runtime movement lives in `agents.rs`.

use crate::city::gen::CityId;
use crate::city::map::{BKind, Map, SpotKind};
use crate::render::character::{Garment, Hair, Hat, Look};
use crate::util::Rng;
use serde::{Deserialize, Serialize};

pub type Pid = u32;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Job {
    None,
    Child,
    Student,
    Butcher,
    Baker,
    Musician,
    Bartender,
    Waiter,
    Cook,
    Dockworker,
    Police,
    Detective,
    Doctor,
    Nurse,
    Priest,
    Journalist,
    Photographer,
    Clerk,
    Banker,
    Merchant,
    Tailor,
    Pharmacist,
    Gunsmith,
    Pawnbroker,
    Farmer,
    FactoryWorker,
    Gangster,
    Smuggler,
    Forger,
    Dancer,
    Driver,
    Teacher,
    Lawyer,
    Politician,
    Aristocrat,
    Retired,
    Drifter,
    RadioHost,
    HotelClerk,
    Maid,
    Gravedigger,
    Vendor,
    Scientist,
    Hacker,
    Housewife,
    Madam,
}

impl Job {
    pub fn label(self, female: bool) -> &'static str {
        use Job::*;
        match (self, female) {
            (None, _) => "Desempregado",
            (Child, _) => "Criança",
            (Student, _) => "Estudante",
            (Butcher, false) => "Açougueiro",
            (Butcher, true) => "Açougueira",
            (Baker, false) => "Padeiro",
            (Baker, true) => "Padeira",
            (Musician, false) => "Músico",
            (Musician, true) => "Cantora",
            (Bartender, _) => "Atendente de bar",
            (Waiter, false) => "Garçom",
            (Waiter, true) => "Garçonete",
            (Cook, false) => "Cozinheiro",
            (Cook, true) => "Cozinheira",
            (Dockworker, _) => "Estivador",
            (Police, false) => "Policial",
            (Police, true) => "Policial",
            (Detective, false) => "Detetive",
            (Detective, true) => "Detetive",
            (Doctor, false) => "Médico",
            (Doctor, true) => "Médica",
            (Nurse, false) => "Enfermeiro",
            (Nurse, true) => "Enfermeira",
            (Priest, _) => "Padre",
            (Journalist, false) => "Jornalista",
            (Journalist, true) => "Jornalista",
            (Photographer, false) => "Fotógrafo",
            (Photographer, true) => "Fotógrafa",
            (Clerk, false) => "Escriturário",
            (Clerk, true) => "Datilógrafa",
            (Banker, false) => "Banqueiro",
            (Banker, true) => "Banqueira",
            (Merchant, false) => "Comerciante",
            (Merchant, true) => "Comerciante",
            (Tailor, false) => "Alfaiate",
            (Tailor, true) => "Costureira",
            (Pharmacist, false) => "Farmacêutico",
            (Pharmacist, true) => "Farmacêutica",
            (Gunsmith, _) => "Armeiro",
            (Pawnbroker, _) => "Penhorista",
            (Farmer, false) => "Fazendeiro",
            (Farmer, true) => "Fazendeira",
            (FactoryWorker, false) => "Operário",
            (FactoryWorker, true) => "Operária",
            (Gangster, false) => "Gângster",
            (Gangster, true) => "Contrabandista",
            (Smuggler, _) => "Contrabandista",
            (Forger, false) => "Falsário",
            (Forger, true) => "Falsária",
            (Dancer, false) => "Dançarino",
            (Dancer, true) => "Dançarina",
            (Driver, false) => "Motorista",
            (Driver, true) => "Motorista",
            (Teacher, false) => "Professor",
            (Teacher, true) => "Professora",
            (Lawyer, false) => "Advogado",
            (Lawyer, true) => "Advogada",
            (Politician, false) => "Vereador",
            (Politician, true) => "Vereadora",
            (Aristocrat, false) => "Herdeiro",
            (Aristocrat, true) => "Herdeira",
            (Retired, false) => "Aposentado",
            (Retired, true) => "Aposentada",
            (Drifter, false) => "Andarilho",
            (Drifter, true) => "Andarilha",
            (RadioHost, false) => "Locutor",
            (RadioHost, true) => "Locutora",
            (HotelClerk, false) => "Recepcionista",
            (HotelClerk, true) => "Recepcionista",
            (Maid, false) => "Criado",
            (Maid, true) => "Criada",
            (Gravedigger, _) => "Coveiro",
            (Vendor, false) => "Feirante",
            (Vendor, true) => "Feirante",
            (Scientist, false) => "Cientista",
            (Scientist, true) => "Cientista",
            (Hacker, false) => "Técnico de computação",
            (Hacker, true) => "Técnica de computação",
            (Housewife, _) => "Dona de casa",
            (Madam, _) => "Cafetina",
        }
    }
    /// Where this job is performed.
    pub fn workplace(self) -> Option<BKind> {
        use Job::*;
        Some(match self {
            Butcher | Vendor => BKind::Market,
            Baker | Merchant => BKind::General,
            Musician | Dancer => BKind::Club,
            Bartender => BKind::Bar,
            Waiter | Cook => BKind::Restaurant,
            Dockworker | Smuggler => BKind::Warehouse,
            Police | Detective => BKind::Police,
            Doctor | Nurse => BKind::Hospital,
            Priest => BKind::Church,
            Journalist | Photographer => BKind::Newspaper,
            Clerk | Lawyer | Politician => BKind::Office,
            Banker => BKind::Bank,
            Tailor => BKind::Clothing,
            Pharmacist => BKind::Pharmacy,
            Gunsmith => BKind::GunShop,
            Pawnbroker | Forger => BKind::Pawn,
            Farmer => BKind::Farmhouse,
            FactoryWorker => BKind::Factory,
            Gangster => BKind::Bar,
            Driver => BKind::Station,
            Teacher => BKind::Office,
            RadioHost => BKind::Radio,
            HotelClerk | Maid => BKind::Hotel,
            Gravedigger => BKind::Church,
            Scientist => BKind::Hospital,
            Hacker => BKind::Office,
            Madam => BKind::Cabaret,
            _ => return Option::None,
        })
    }
    /// (start hour, end hour) of the shift.
    pub fn shift(self) -> (f32, f32) {
        use Job::*;
        match self {
            Baker | Butcher | Vendor | Farmer => (5.0, 14.0),
            Musician | Dancer | Madam => (20.0, 27.0),
            Bartender | Gangster => (16.0, 25.0),
            Police => (7.0, 19.0),
            Dockworker | FactoryWorker => (6.0, 16.0),
            Nurse => (19.0, 31.0),
            Priest => (6.0, 12.0),
            Journalist => (9.0, 19.0),
            Cook | Waiter => (10.0, 22.0),
            HotelClerk => (8.0, 20.0),
            Smuggler => (22.0, 28.0),
            _ => (8.0, 17.0),
        }
    }
    pub fn income(self) -> i32 {
        use Job::*;
        match self {
            Banker | Aristocrat | Politician | Lawyer | Doctor => 12,
            Detective | Merchant | Pharmacist | Gunsmith | Pawnbroker | Madam | Gangster => 7,
            Child | Student | None | Drifter | Housewife | Retired => 0,
            _ => 4,
        }
    }
    pub fn armed(self) -> bool {
        matches!(self, Job::Police | Job::Detective | Job::Gangster | Job::Smuggler | Job::Gunsmith)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Traits {
    pub curiosity: u8,
    pub loyalty: u8,
    pub jealousy: u8,
    pub courage: u8,
    pub fear: u8,
    pub ambition: u8,
    pub morality: u8,
    pub greed: u8,
    pub sociability: u8,
    pub faith: u8,
}

impl Traits {
    pub fn random(r: &mut Rng) -> Traits {
        let mut t = || (r.range(5, 96)) as u8;
        Traits {
            curiosity: t(),
            loyalty: t(),
            jealousy: t(),
            courage: t(),
            fear: t(),
            ambition: t(),
            morality: t(),
            greed: t(),
            sociability: t(),
            faith: t(),
        }
    }
    pub fn describe(&self) -> Vec<&'static str> {
        let mut v = Vec::new();
        if self.courage > 75 {
            v.push("corajoso");
        }
        if self.fear > 75 {
            v.push("medroso");
        }
        if self.greed > 75 {
            v.push("ganancioso");
        }
        if self.morality > 80 {
            v.push("íntegro");
        }
        if self.morality < 20 {
            v.push("sem escrúpulos");
        }
        if self.jealousy > 80 {
            v.push("ciumento");
        }
        if self.loyalty > 80 {
            v.push("leal");
        }
        if self.sociability > 80 {
            v.push("falante");
        }
        if self.sociability < 20 {
            v.push("recluso");
        }
        if self.faith > 80 {
            v.push("devoto");
        }
        if self.curiosity > 80 {
            v.push("curioso");
        }
        if self.ambition > 80 {
            v.push("ambicioso");
        }
        v
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum RelKind {
    Friend,
    Rival,
    Lover,
    Crush,
    Enemy,
    Coworker,
    Debtor,
    Creditor,
    Secret,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rel {
    pub to: Pid,
    pub kind: RelKind,
    pub val: i8,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Romance {
    None,
    Flirting,
    Dating,
    Lovers,
    Engaged,
    Married,
    Broken,
    Widowed,
}

impl Romance {
    pub fn label(self) -> &'static str {
        match self {
            Romance::None => "—",
            Romance::Flirting => "Flerte",
            Romance::Dating => "Namorando",
            Romance::Lovers => "Amantes",
            Romance::Engaged => "Noivos",
            Romance::Married => "Casados",
            Romance::Broken => "Separados",
            Romance::Widowed => "Viúvo(a)",
        }
    }
}

/// How this person relates to Elias in the current timeline.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EliasRel {
    pub met: bool,
    pub alias: Option<String>,
    pub trust: i32,
    pub affection: i32,
    pub fear: i32,
    pub respect: i32,
    pub romance: Romance,
    pub lies: u32,
    pub caught_lies: u32,
    pub talks: u32,
    pub last_talk_day: i32,
    pub gifts: u32,
    pub dates: u32,
    pub knows_secret: Vec<String>,
    pub topics_done: Vec<String>,
    pub deja_vu: u8,
    pub in_network: bool,
    pub loyalty: i32,
    pub paid_until_day: i32,
    pub bribed: bool,
    pub threatened: bool,
    pub betrayed_by_elias: bool,
}

impl Default for EliasRel {
    fn default() -> Self {
        EliasRel {
            met: false,
            alias: None,
            trust: 0,
            affection: 0,
            fear: 0,
            respect: 0,
            romance: Romance::None,
            lies: 0,
            caught_lies: 0,
            talks: 0,
            last_talk_day: -1,
            gifts: 0,
            dates: 0,
            knows_secret: Vec::new(),
            topics_done: Vec::new(),
            deja_vu: 0,
            in_network: false,
            loyalty: 50,
            paid_until_day: 0,
            bribed: false,
            threatened: false,
            betrayed_by_elias: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Life {
    Alive,
    Dead { year: i32, day: i32, cause: String, by_elias: bool },
    Missing,
    Jailed,
    Moved(CityId),
    Unborn,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Memory {
    pub day: i32,
    pub year: i32,
    pub text: String,
    pub about: Option<Pid>,
    pub weight: i8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: Pid,
    pub first: String,
    pub last: String,
    pub female: bool,
    pub birth: i32,
    pub city: CityId,
    pub job: Job,
    pub home: Option<usize>,
    pub work: Option<usize>,
    pub bed: Option<usize>,
    pub traits: Traits,
    pub look: Look,
    pub spouse: Option<Pid>,
    pub parents: [Option<Pid>; 2],
    pub children: Vec<Pid>,
    pub rels: Vec<Rel>,
    pub elias: EliasRel,
    pub life: Life,
    pub money: i32,
    pub debt: i32,
    pub memories: Vec<Memory>,
    pub destiny: Vec<String>,
    /// recurring soul across timelines (sphere touched)
    pub soul: Option<u8>,
    /// person belongs to a case script
    pub case_role: Option<(u8, u8)>,
    pub addicted: u8,
    pub health: f32,
    pub notable: bool,
    pub seed: u32,
    /// Elias' own child
    pub elias_child: bool,
}

impl Person {
    pub fn name(&self) -> String {
        format!("{} {}", self.first, self.last)
    }
    pub fn age(&self, year: i32) -> i32 {
        year - self.birth
    }
    pub fn alive(&self) -> bool {
        self.life == Life::Alive
    }
    pub fn pron(&self) -> &'static str {
        if self.female {
            "ela"
        } else {
            "ele"
        }
    }
    pub fn o(&self) -> &'static str {
        if self.female {
            "a"
        } else {
            "o"
        }
    }
    pub fn rel_to(&self, p: Pid) -> Option<&Rel> {
        self.rels.iter().find(|r| r.to == p)
    }
    pub fn remember(&mut self, year: i32, day: i32, text: impl Into<String>, about: Option<Pid>, weight: i8) {
        self.memories.push(Memory { day, year, text: text.into(), about, weight });
        if self.memories.len() > 40 {
            // forget the least important old memory
            if let Some(i) = self.memories.iter().enumerate().min_by_key(|(_, m)| m.weight.abs()).map(|(i, _)| i) {
                self.memories.remove(i);
            }
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Population {
    pub people: Vec<Person>,
}

use crate::city::names::{first_names, last_names};

// ------------------------------------------------------------------ looks

/// Police hat and uniform colour for each city and era.
pub fn police_look(city: CityId, year: i32) -> (Hat, [f32; 3]) {
    match city {
        CityId::London => (Hat::Bobby, [0.06, 0.07, 0.12]),
        CityId::Bavaria => (Hat::Shako, [0.2, 0.26, 0.18]),
        CityId::Berlin => (Hat::PoliceCap, [0.2, 0.26, 0.22]),
        CityId::Adelaide => (Hat::PoliceCap, [0.08, 0.1, 0.18]),
        CityId::Bergen => (Hat::PoliceCap, [0.05, 0.06, 0.1]),
        CityId::SanFrancisco | CityId::Portland if year >= 1960 => (Hat::PoliceCap, [0.07, 0.08, 0.14]),
        CityId::LosAngeles => (Hat::PoliceCap, [0.05, 0.06, 0.12]),
        CityId::NewYork => (Hat::PoliceCap, [0.08, 0.1, 0.2]),
        _ => {
            if year < 1935 {
                (Hat::Bobby, [0.1, 0.12, 0.2])
            } else {
                (Hat::PoliceCap, [0.1, 0.12, 0.2])
            }
        }
    }
}

fn hair_pal_default() -> &'static [[f32; 3]] {
    &[[0.08, 0.06, 0.05], [0.2, 0.12, 0.07], [0.35, 0.22, 0.12], [0.55, 0.4, 0.22], [0.12, 0.1, 0.1], [0.45, 0.2, 0.1]]
}

pub fn era_look(r: &mut Rng, female: bool, age: i32, job: Job, year: i32, city: CityId) -> Look {
    let skin_pal: &[[f32; 3]] = &[
        [0.93, 0.78, 0.66],
        [0.86, 0.68, 0.54],
        [0.72, 0.52, 0.38],
        [0.55, 0.37, 0.25],
        [0.40, 0.26, 0.18],
        [0.30, 0.20, 0.14],
        [0.88, 0.74, 0.58],
    ];
    let european = matches!(city, CityId::Bavaria | CityId::Berlin | CityId::Bergen | CityId::London | CityId::Adelaide | CityId::Portland);
    let skin = if european && r.chance(0.92) { skin_pal[r.idx(2)] } else if city == CityId::SanFrancisco && r.chance(0.2) { [0.9, 0.76, 0.58] } else { *r.pick(skin_pal) };
    let hair_pal: &[[f32; 3]] = if matches!(city, CityId::Bergen | CityId::Bavaria) { &[[0.75, 0.6, 0.35], [0.6, 0.45, 0.25], [0.35, 0.22, 0.12], [0.85, 0.72, 0.45]] } else { hair_pal_default() };
    let hair_col = *r.pick(hair_pal);
    let dark: &[[f32; 3]] = &[
        [0.12, 0.12, 0.14],
        [0.2, 0.18, 0.16],
        [0.25, 0.22, 0.18],
        [0.16, 0.18, 0.24],
        [0.3, 0.27, 0.22],
        [0.22, 0.14, 0.12],
        [0.35, 0.33, 0.3],
    ];
    let bright: &[[f32; 3]] = &[
        [0.55, 0.15, 0.2],
        [0.2, 0.35, 0.5],
        [0.6, 0.5, 0.25],
        [0.3, 0.45, 0.3],
        [0.5, 0.3, 0.5],
        [0.7, 0.65, 0.55],
        [0.45, 0.2, 0.12],
    ];
    let modern = year >= 1960;
    let top = if modern && r.chance(0.5) { *r.pick(bright) } else { *r.pick(dark) };
    let bottom = *r.pick(dark);
    let accent = *r.pick(bright);
    let mut garment = if female {
        if year < 1960 || r.chance(0.5) {
            Garment::Dress
        } else {
            Garment::Shirt
        }
    } else if year < 1960 {
        if r.chance(0.6) {
            Garment::Suit
        } else {
            Garment::Shirt
        }
    } else if r.chance(0.4) {
        Garment::Suit
    } else {
        Garment::Shirt
    };
    let mut hat = if year < 1960 {
        if female {
            if r.chance(0.5) {
                if year < 1935 {
                    Hat::Cloche
                } else {
                    Hat::None
                }
            } else {
                Hat::None
            }
        } else {
            *r.pick(&[Hat::Fedora, Hat::FlatCap, Hat::Bowler, Hat::Fedora, Hat::None, Hat::Boater])
        }
    } else if r.chance(0.15) {
        Hat::Beanie
    } else {
        Hat::None
    };
    let mut top = top;
    let mut hat_col = *r.pick(dark);
    // regional dress
    match city {
        CityId::Bavaria if year < 1950 => {
            if female {
                garment = Garment::Dress;
                top = *r.pick(&[[0.45, 0.12, 0.12], [0.15, 0.25, 0.18], [0.2, 0.2, 0.35]]);
                if r.chance(0.6) {
                    hat = Hat::Headscarf;
                }
            } else {
                hat = *r.pick(&[Hat::Alpine, Hat::Alpine, Hat::FlatCap]);
                hat_col = [0.18, 0.22, 0.16];
                if r.chance(0.5) {
                    garment = Garment::Overalls;
                }
            }
        }
        CityId::London if (1939..1946).contains(&year) && !female && r.chance(0.15) => {
            hat = Hat::Helmet;
            hat_col = [0.25, 0.25, 0.22];
        }
        CityId::Bergen => {
            if r.chance(0.5) {
                garment = Garment::Shirt;
                top = *r.pick(&[[0.6, 0.58, 0.52], [0.2, 0.25, 0.4], [0.45, 0.1, 0.1]]);
                hat = if r.chance(0.5) { Hat::Beanie } else { hat };
            }
        }
        CityId::LosAngeles | CityId::NewYork if year >= 1980 && r.chance(0.3) => {
            hat = Hat::Cap;
            hat_col = *r.pick(bright);
        }
        _ => {}
    }
    match job {
        Job::Police => {
            garment = Garment::Uniform;
            let (h, c) = police_look(city, year);
            hat = h;
            top = c;
            hat_col = c;
        }
        Job::Doctor | Job::Scientist => garment = Garment::LabCoat,
        Job::Nurse => {
            garment = Garment::Dress;
            top = [0.9, 0.9, 0.88];
            hat = Hat::NurseCap;
        }
        Job::Priest => {
            garment = Garment::Robe;
            top = [0.05, 0.05, 0.06];
            hat = Hat::None;
        }
        Job::Butcher | Job::Cook | Job::Baker | Job::Bartender | Job::Vendor => garment = Garment::Apron,
        Job::Dockworker | Job::FactoryWorker | Job::Farmer => {
            garment = Garment::Overalls;
            if !female {
                hat = Hat::FlatCap;
            }
        }
        Job::Gangster | Job::Detective => {
            garment = Garment::LongCoat;
            if year < 1970 {
                hat = Hat::Fedora;
            }
        }
        Job::Aristocrat | Job::Banker | Job::Lawyer | Job::Politician => {
            if !female {
                garment = Garment::Suit;
            }
        }
        Job::Maid => {
            garment = Garment::Apron;
            hat = Hat::Headscarf;
        }
        Job::Drifter => {
            garment = Garment::LongCoat;
            top = [0.2, 0.18, 0.14];
        }
        _ => {}
    }
    let hair = if female {
        if year < 1935 {
            *r.pick(&[Hair::Bob, Hair::Bun, Hair::Bob])
        } else if year >= 1965 {
            *r.pick(&[Hair::Long, Hair::Afro, Hair::Bob, Hair::Long])
        } else {
            *r.pick(&[Hair::Bun, Hair::Bob, Hair::Long])
        }
    } else if age > 55 && r.chance(0.4) {
        Hair::Bald
    } else if year >= 1965 && r.chance(0.3) {
        *r.pick(&[Hair::Long, Hair::Afro])
    } else {
        Hair::Short
    };
    let child_scale = if age < 16 { 0.55 + age as f32 / 16.0 * 0.4 } else { 1.0 };
    Look {
        female,
        skin,
        hair_col,
        hair,
        hat: if age < 12 { Hat::None } else { hat },
        hat_col,
        garment: if age < 12 && garment == Garment::Suit { Garment::Shirt } else { garment },
        top,
        bottom,
        accent,
        height: (if female { 0.94 } else { 1.0 }) * r.rangef(0.93, 1.07) * child_scale,
        girth: r.rangef(0.9, 1.25),
        beard: !female && age > 20 && r.chance(if year < 1935 { 0.3 } else { 0.15 }),
        age: age as f32,
    }
}

impl Population {
    pub fn get(&self, id: Pid) -> &Person {
        &self.people[id as usize]
    }
    pub fn get_mut(&mut self, id: Pid) -> &mut Person {
        &mut self.people[id as usize]
    }

    pub fn new_person(&mut self, r: &mut Rng, city: CityId, female: bool, birth: i32, last: Option<String>, year: i32) -> Pid {
        let id = self.people.len() as Pid;
        let first = r.pick(first_names(city, female, year)).to_string();
        let last = last.unwrap_or_else(|| r.pick(last_names(city)).to_string());
        let age = year - birth;
        let job = if age < 6 {
            Job::Child
        } else if age < 17 {
            Job::Student
        } else if age > 68 {
            Job::Retired
        } else {
            Job::None
        };
        let look = era_look(r, female, age, job, year, city);
        self.people.push(Person {
            id,
            first,
            last,
            female,
            birth,
            city,
            job,
            home: None,
            work: None,
            bed: None,
            traits: Traits::random(r),
            look,
            spouse: None,
            parents: [None, None],
            children: Vec::new(),
            rels: Vec::new(),
            elias: EliasRel::default(),
            life: Life::Alive,
            money: r.range(2, 60),
            debt: if r.chance(0.3) { r.range(10, 200) } else { 0 },
            memories: Vec::new(),
            destiny: Vec::new(),
            soul: None,
            case_role: None,
            addicted: 0,
            health: 100.0,
            notable: false,
            seed: r.next() as u32,
            elias_child: false,
        });
        id
    }

    pub fn marry(&mut self, a: Pid, b: Pid) {
        self.people[a as usize].spouse = Some(b);
        self.people[b as usize].spouse = Some(a);
        let lb = self.people[b as usize].female;
        if lb {
            let l = self.people[a as usize].last.clone();
            self.people[b as usize].last = l;
        } else {
            let l = self.people[b as usize].last.clone();
            self.people[a as usize].last = l;
        }
    }

    pub fn add_child(&mut self, parent_a: Pid, parent_b: Option<Pid>, child: Pid) {
        self.people[child as usize].parents = [Some(parent_a), parent_b];
        self.people[parent_a as usize].children.push(child);
        if let Some(b) = parent_b {
            self.people[b as usize].children.push(child);
        }
    }

    pub fn relate(&mut self, a: Pid, b: Pid, kind: RelKind, val: i8) {
        if a == b {
            return;
        }
        let pa = &mut self.people[a as usize];
        if let Some(r) = pa.rels.iter_mut().find(|r| r.to == b) {
            r.kind = kind;
            r.val = val;
        } else {
            pa.rels.push(Rel { to: b, kind, val });
        }
    }

    pub fn in_city(&self, city: CityId, year: i32) -> Vec<Pid> {
        self.people
            .iter()
            .filter(|p| p.city == city && p.life == Life::Alive && p.birth <= year)
            .map(|p| p.id)
            .collect()
    }

    /// Give every adult without a job one appropriate for the city, and
    /// assign workplaces / homes / beds on the map.
    pub fn assign_to_map(&mut self, r: &mut Rng, map: &mut Map, city: CityId, year: i32) {
        let ids = self.in_city(city, year);
        // clear previous assignments on map
        for b in map.buildings.iter_mut() {
            b.residents.clear();
            for s in b.spots.iter_mut() {
                s.owner = None;
            }
        }
        // homes: keep the household together
        for &id in &ids {
            let p = &self.people[id as usize];
            if let Some(h) = p.home {
                if h < map.buildings.len() && !map.buildings[h].closed_forever {
                    map.buildings[h].residents.push(id);
                    continue;
                }
            }
            self.people[id as usize].home = None;
        }
        // staff buildings
        for &id in &ids {
            let job = self.people[id as usize].job;
            if let Some(k) = job.workplace() {
                let places = map.buildings_of(k);
                let cur = self.people[id as usize].work;
                if let Some(w) = cur {
                    if places.contains(&w) {
                        continue;
                    }
                }
                if places.is_empty() {
                    self.people[id as usize].work = None;
                } else {
                    // least staffed
                    let mut best = places[0];
                    let mut best_n = usize::MAX;
                    for &pl in &places {
                        let n = ids.iter().filter(|&&o| self.people[o as usize].work == Some(pl)).count();
                        if n < best_n {
                            best_n = n;
                            best = pl;
                        }
                    }
                    self.people[id as usize].work = Some(best);
                }
            }
        }
        // beds
        for &id in &ids {
            let home = match self.people[id as usize].home {
                Some(h) => h,
                None => continue,
            };
            let b = &mut map.buildings[home];
            if let Some((i, _)) = b.spots.iter().enumerate().find(|(_, s)| s.kind == SpotKind::Bed && s.owner.is_none()) {
                b.spots[i].owner = Some(id);
                self.people[id as usize].bed = Some(i);
            } else {
                self.people[id as usize].bed = None;
            }
        }
        let _ = r;
    }

    /// Populate a fresh city with households.
    pub fn generate_city(&mut self, r: &mut Rng, map: &mut Map, city: CityId, year: i32, target: usize) {
        let mut homes: Vec<usize> = map
            .buildings
            .iter()
            .filter(|b| matches!(b.kind, BKind::House | BKind::Apartment | BKind::Mansion | BKind::Farmhouse) && !b.closed_forever)
            .map(|b| b.id)
            .collect();
        r.shuffle(&mut homes);
        // jobs that must be filled first
        let mut needed: Vec<Job> = vec![
            Job::Police, Job::Police, Job::Police, Job::Police, Job::Police, Job::Police, Job::Detective, Job::Detective,
            Job::Doctor, Job::Doctor, Job::Nurse, Job::Nurse, Job::Priest, Job::Gravedigger, Job::Journalist, Job::Journalist,
            Job::Photographer, Job::Bartender, Job::Bartender, Job::Bartender, Job::Bartender, Job::Bartender,
            Job::Musician, Job::Musician, Job::Musician, Job::Musician, Job::Dancer, Job::Dancer, Job::Madam,
            Job::Waiter, Job::Waiter, Job::Cook, Job::Cook, Job::Butcher, Job::Vendor, Job::Vendor, Job::Vendor, Job::Baker,
            Job::Merchant, Job::Merchant, Job::Merchant, Job::Merchant, Job::Tailor, Job::Pharmacist, Job::Gunsmith,
            Job::Pawnbroker, Job::Forger, Job::Banker, Job::Clerk, Job::Clerk, Job::Lawyer, Job::Politician, Job::HotelClerk,
            Job::HotelClerk, Job::Maid, Job::Maid, Job::Dockworker, Job::Dockworker, Job::Dockworker, Job::Dockworker,
            Job::Smuggler, Job::Smuggler, Job::FactoryWorker, Job::FactoryWorker, Job::FactoryWorker, Job::Farmer,
            Job::Gangster, Job::Gangster, Job::Gangster, Job::Gangster, Job::Driver, Job::Teacher, Job::Aristocrat,
            Job::Aristocrat, Job::Drifter, Job::Drifter,
        ];
        if year >= 1925 {
            needed.push(Job::RadioHost);
        }
        if year >= 1980 {
            needed.push(Job::Hacker);
            needed.push(Job::Hacker);
        }
        if year >= 1940 {
            needed.push(Job::Scientist);
        }
        let mut count = self.in_city(city, year).len();
        let mut hi = 0;
        while count < target && hi < homes.len() {
            let home = homes[hi];
            hi += 1;
            let kind = map.buildings[home].kind;
            let beds = map.buildings[home].spots.iter().filter(|s| s.kind == SpotKind::Bed).count().max(1);
            let size = match kind {
                BKind::Apartment => beds.min(3),
                BKind::Mansion => 3,
                BKind::Farmhouse => 4,
                _ => beds.min(4),
            };
            // household: couple + kids, or single
            let adult_age = r.range(22, 60);
            let female = r.chance(0.5);
            let a = self.new_person(r, city, female, year - adult_age, None, year);
            self.people[a as usize].home = Some(home);
            let mut members = vec![a];
            if size >= 2 && r.chance(0.75) {
                let age_b = (adult_age + r.range(-6, 6)).clamp(19, 75);
                let b = self.new_person(r, city, !female, year - age_b, None, year);
                self.marry(a, b);
                self.people[b as usize].home = Some(home);
                members.push(b);
                let kids = (size as i32 - 2).max(0).min(r.range(0, 4));
                for _ in 0..kids {
                    let kage = r.range(1, (adult_age - 18).clamp(2, 24));
                    let last = self.people[a as usize].last.clone();
                    let (mother, father) = if self.people[a as usize].female { (a, b) } else { (b, a) };
                    let fk = r.chance(0.5);
                    let k = self.new_person(r, city, fk, year - kage, Some(last), year);
                    self.add_child(mother, Some(father), k);
                    self.people[k as usize].home = Some(home);
                    members.push(k);
                }
                // an old parent sometimes lives with them
                if r.chance(0.15) {
                    let fg = r.chance(0.6);
                    let gb = year - adult_age - r.range(22, 32);
                    let g = self.new_person(r, city, fg, gb, None, year);
                    let last = self.people[a as usize].last.clone();
                    self.people[g as usize].last = last;
                    self.add_child(g, None, a);
                    self.people[g as usize].home = Some(home);
                    members.push(g);
                }
            }
            if kind == BKind::Mansion {
                self.people[a as usize].job = Job::Aristocrat;
            }
            if kind == BKind::Farmhouse {
                for &m in &members {
                    if self.people[m as usize].age(year) >= 17 {
                        self.people[m as usize].job = Job::Farmer;
                    }
                }
            }
            // give adults jobs
            for &m in &members {
                let age = self.people[m as usize].age(year);
                if (17..=68).contains(&age) && self.people[m as usize].job == Job::None {
                    let female = self.people[m as usize].female;
                    let j = if let Some(pos) = needed.iter().position(|j| {
                        // era/gender plausibility
                        let male_only = year < 1940 && matches!(j, Job::Police | Job::Detective | Job::Doctor | Job::Priest | Job::Dockworker | Job::Gangster | Job::Politician | Job::Banker | Job::Lawyer | Job::Gravedigger | Job::Smuggler | Job::Farmer);
                        let female_only = matches!(j, Job::Madam | Job::Maid) || (year < 1960 && matches!(j, Job::Nurse | Job::Dancer));
                        !(male_only && female) && !(female_only && !female) && !(*j == Job::Priest && female)
                    }) {
                        needed.remove(pos)
                    } else if female && year < 1960 && r.chance(0.5) {
                        Job::Housewife
                    } else {
                        *r.pick(&[Job::Dockworker, Job::FactoryWorker, Job::Clerk, Job::Merchant, Job::Waiter, Job::Vendor, Job::Driver, Job::None])
                    };
                    self.people[m as usize].job = j;
                    let age = self.people[m as usize].age(year);
                    self.people[m as usize].look = era_look(r, female, age, j, year, city);
                }
            }
            count += members.len();
        }
        self.weave_relations(r, city, year);
        self.assign_to_map(r, map, city, year);
    }

    /// Friendships, rivalries, affairs, debts between the people of a city.
    pub fn weave_relations(&mut self, r: &mut Rng, city: CityId, year: i32) {
        let ids = self.in_city(city, year);
        if ids.len() < 4 {
            return;
        }
        for &a in &ids {
            if self.people[a as usize].age(year) < 14 {
                continue;
            }
            let n = 1 + (self.people[a as usize].traits.sociability / 30) as usize;
            for _ in 0..n {
                let b = *r.pick(&ids);
                if b == a || self.people[b as usize].age(year) < 14 || self.people[a as usize].rel_to(b).is_some() {
                    continue;
                }
                let same_work = self.people[a as usize].work.is_some() && self.people[a as usize].work == self.people[b as usize].work;
                let roll = r.f();
                let (k, v) = if same_work && roll < 0.5 {
                    (RelKind::Coworker, 20)
                } else if roll < 0.55 {
                    (RelKind::Friend, r.range(20, 80) as i8)
                } else if roll < 0.72 {
                    (RelKind::Rival, -(r.range(20, 70) as i8))
                } else if roll < 0.8 {
                    (RelKind::Enemy, -(r.range(50, 100) as i8))
                } else if roll < 0.9 {
                    (RelKind::Debtor, -20)
                } else {
                    // secret crush / affair
                    let married = self.people[a as usize].spouse.is_some();
                    if married && self.people[a as usize].traits.morality < 45 {
                        (RelKind::Lover, 70)
                    } else {
                        (RelKind::Crush, 50)
                    }
                };
                self.relate(a, b, k, v);
                let back = match k {
                    RelKind::Debtor => (RelKind::Creditor, 10),
                    RelKind::Crush => (RelKind::Friend, 30),
                    other => (other, v),
                };
                self.relate(b, a, back.0, back.1);
                if k == RelKind::Debtor {
                    let amt = r.range(20, 300);
                    self.people[a as usize].debt += amt;
                }
            }
        }
    }

    /// Simulate the years between two dates: aging, births, marriages,
    /// deaths and destinies. Used when Elias stays in a timeline or when a
    /// city is revisited decades later.
    pub fn advance_years(&mut self, r: &mut Rng, city: CityId, from: i32, to: i32, log: &mut Vec<String>) {
        for year in from..to {
            let ids = self.in_city(city, year);
            for &id in &ids {
                let (age, female, spouse) = {
                    let p = &self.people[id as usize];
                    (p.age(year), p.female, p.spouse)
                };
                // death
                let pdeath = if age > 85 { 0.25 } else if age > 70 { 0.07 } else if age > 55 { 0.02 } else { 0.003 };
                if r.chance(pdeath) {
                    self.people[id as usize].life = Life::Dead { year, day: r.range(1, 365), cause: "causas naturais".into(), by_elias: false };
                    if self.people[id as usize].notable {
                        log.push(format!("{} morreu em {} ({} anos).", self.people[id as usize].name(), year, age));
                    }
                    if let Some(s) = spouse {
                        self.people[s as usize].spouse = None;
                        if self.people[s as usize].elias.romance == Romance::Married {
                            self.people[s as usize].elias.romance = Romance::Widowed;
                        }
                    }
                    continue;
                }
                // marriage
                if spouse.is_none() && (20..45).contains(&age) && r.chance(0.12) {
                    let cands: Vec<Pid> = ids
                        .iter()
                        .copied()
                        .filter(|&o| {
                            let q = &self.people[o as usize];
                            q.alive() && q.female != female && q.spouse.is_none() && (q.age(year) - age).abs() < 8 && q.parents != self.people[id as usize].parents && q.elias.romance == Romance::None
                        })
                        .collect();
                    if !cands.is_empty() && self.people[id as usize].elias.romance == Romance::None {
                        let o = *r.pick(&cands);
                        self.marry(id, o);
                        let home = self.people[id as usize].home;
                        self.people[o as usize].home = home;
                    }
                }
                // births
                if female && (18..42).contains(&age) {
                    if let Some(s) = self.people[id as usize].spouse {
                        let kids = self.people[id as usize].children.len();
                        if r.chance(if kids < 2 { 0.2 } else { 0.07 }) {
                            let last = self.people[s as usize].last.clone();
                            let fk = r.chance(0.5);
                            let k = self.new_person(r, city, fk, year, Some(last), year);
                            self.add_child(id, Some(s), k);
                            let home = self.people[id as usize].home;
                            self.people[k as usize].home = home;
                            let elias_kid = self.people[s as usize].id == u32::MAX; // placeholder
                            let _ = elias_kid;
                        }
                    }
                }
                // coming of age: get a job shaped by destiny
                if age == 18 {
                    let destiny = self.people[id as usize].destiny.clone();
                    let j = if destiny.iter().any(|d| d == "police") {
                        Job::Police
                    } else if destiny.iter().any(|d| d == "criminal") {
                        Job::Gangster
                    } else if destiny.iter().any(|d| d == "journalist") {
                        Job::Journalist
                    } else {
                        *r.pick(&[Job::Clerk, Job::Merchant, Job::Dockworker, Job::FactoryWorker, Job::Waiter, Job::Nurse, Job::Teacher, Job::Musician])
                    };
                    self.people[id as usize].job = j;
                    if self.people[id as usize].notable || self.people[id as usize].elias_child {
                        log.push(format!("{} se tornou {}.", self.people[id as usize].name(), j.label(female).to_lowercase()));
                    }
                }
                if age == 68 && !matches!(self.people[id as usize].job, Job::Aristocrat) {
                    self.people[id as usize].job = Job::Retired;
                }
                // leaving town
                if self.people[id as usize].destiny.iter().any(|d| d == "left_city") && r.chance(0.5) {
                    self.people[id as usize].life = Life::Moved(city);
                }
            }
        }
        // refresh looks for the new age
        let ids = self.in_city(city, to);
        for id in ids {
            let (female, age, job, seed) = {
                let p = &self.people[id as usize];
                (p.female, p.age(to), p.job, p.seed)
            };
            let mut rr = Rng::new(seed as u64 + to as u64);
            let old = self.people[id as usize].look.clone();
            let mut l = era_look(&mut rr, female, age, job, to, city);
            l.skin = old.skin;
            l.hair_col = old.hair_col;
            self.people[id as usize].look = l;
        }
    }
}
