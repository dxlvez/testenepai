//! Items: weapons, drugs, tools, gifts, clothes, documents.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Weapon {
    Fists,
    Knife,
    Club,
    Axe,
    Razor,
    Bottle,
    Revolver,
    Pistol,
    Shotgun,
    Tommy,
    Magnum,
    Automatic,
}

pub struct WStats {
    pub name: &'static str,
    pub melee: bool,
    pub damage: f32,
    pub range: f32,
    pub cooldown: f32,
    pub mag: i32,
    pub noise: f32,
    pub spread: f32,
    pub pellets: u32,
    pub year: i32,
    pub price: i32,
}

impl Weapon {
    pub fn stats(self) -> WStats {
        use Weapon::*;
        let w = |name, melee, damage, range, cooldown, mag, noise, spread, pellets, year, price| WStats {
            name,
            melee,
            damage,
            range,
            cooldown,
            mag,
            noise,
            spread,
            pellets,
            year,
            price,
        };
        match self {
            Fists => w("Punhos", true, 12.0, 1.1, 0.45, 0, 4.0, 0.0, 1, 0, 0),
            Knife => w("Faca", true, 45.0, 1.2, 0.5, 0, 2.0, 0.0, 1, 0, 6),
            Club => w("Porrete", true, 26.0, 1.4, 0.7, 0, 5.0, 0.0, 1, 0, 3),
            Axe => w("Machado", true, 70.0, 1.5, 1.1, 0, 6.0, 0.0, 1, 0, 8),
            Razor => w("Navalha", true, 38.0, 1.1, 0.4, 0, 1.5, 0.0, 1, 0, 5),
            Bottle => w("Garrafa", true, 22.0, 1.2, 0.6, 0, 7.0, 0.0, 1, 0, 1),
            Revolver => w("Revólver .38", false, 55.0, 14.0, 0.55, 6, 32.0, 0.04, 1, 1900, 25),
            Pistol => w("Pistola M1911", false, 50.0, 15.0, 0.35, 7, 32.0, 0.05, 1, 1915, 35),
            Shotgun => w("Espingarda", false, 22.0, 8.0, 1.1, 2, 40.0, 0.18, 7, 1900, 30),
            Tommy => w("Submetralhadora Thompson", false, 30.0, 13.0, 0.09, 30, 40.0, 0.1, 1, 1921, 180),
            Magnum => w("Magnum .357", false, 80.0, 18.0, 0.75, 6, 38.0, 0.03, 1, 1935, 90),
            Automatic => w("Pistola 9mm", false, 45.0, 16.0, 0.25, 15, 32.0, 0.05, 1, 1980, 60),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Drug {
    /// knocks a victim out when pressed to the face from behind
    Chloroform,
    /// put in a drink: victim falls asleep minutes later (risk of overdose)
    Chloral,
    /// Elias: removes pain, heals a bit, addictive
    Laudanum,
    /// interrogation: the next conversation reveals the truth more easily
    TruthSerum,
    /// Elias: run longer, faster; crash later
    Stimulant,
    /// Elias: longer, sharper echoes, more temporal fatigue
    Visionary,
}

impl Drug {
    pub fn name(self, year: i32) -> &'static str {
        match self {
            Drug::Chloroform => "Clorofórmio",
            Drug::Chloral => "Hidrato de cloral",
            Drug::Laudanum => {
                if year < 1950 {
                    "Láudano"
                } else {
                    "Morfina"
                }
            }
            Drug::TruthSerum => {
                if year < 1936 {
                    "Escopolamina"
                } else {
                    "Pentotal sódico"
                }
            }
            Drug::Stimulant => {
                if year < 1935 {
                    "Tônico de coca"
                } else {
                    "Benzedrina"
                }
            }
            Drug::Visionary => {
                if year < 1960 {
                    "Absinto"
                } else {
                    "Ácido lisérgico"
                }
            }
        }
    }
    pub fn desc(self) -> &'static str {
        match self {
            Drug::Chloroform => "Aplicado por trás, faz a vítima desmaiar. Barulhento se ela perceber.",
            Drug::Chloral => "Coloque na bebida de alguém sentado num bar ou mesa. Dose alta pode matar.",
            Drug::Laudanum => "Alivia dor e recupera saúde. Vicia.",
            Drug::TruthSerum => "Aplicado antes de um interrogatório: mentiras ficam mais difíceis. Suspeito para testemunhas.",
            Drug::Stimulant => "Correr mais e por mais tempo. A queda vem depois.",
            Drug::Visionary => "Ecos mais longos e nítidos. Aumenta a fadiga temporal.",
        }
    }
    pub fn price(self) -> i32 {
        match self {
            Drug::Chloroform => 12,
            Drug::Chloral => 10,
            Drug::Laudanum => 6,
            Drug::TruthSerum => 30,
            Drug::Stimulant => 8,
            Drug::Visionary => 15,
        }
    }
    pub fn illegal(self, year: i32) -> bool {
        match self {
            Drug::Laudanum => year >= 1920,
            Drug::Stimulant => year >= 1920 && year < 1935 || year >= 1965,
            Drug::Visionary => year >= 1915 && year < 1933 || year >= 1968,
            Drug::Chloroform | Drug::Chloral => true,
            Drug::TruthSerum => true,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Tool {
    Lockpick,
    Camera,
    Crowbar,
    Bandage,
    Flashlight,
    ForgedPapers,
    Rope,
}

impl Tool {
    pub fn name(self) -> &'static str {
        match self {
            Tool::Lockpick => "Gazuas",
            Tool::Camera => "Câmera fotográfica",
            Tool::Crowbar => "Pé de cabra",
            Tool::Bandage => "Atadura",
            Tool::Flashlight => "Lanterna",
            Tool::ForgedPapers => "Documentos falsos",
            Tool::Rope => "Corda",
        }
    }
    pub fn price(self) -> i32 {
        match self {
            Tool::Lockpick => 8,
            Tool::Camera => 40,
            Tool::Crowbar => 4,
            Tool::Bandage => 2,
            Tool::Flashlight => 5,
            Tool::ForgedPapers => 60,
            Tool::Rope => 2,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Gift {
    Flowers,
    Perfume,
    Jewel,
    Book,
    Chocolates,
    Record,
    Whiskey,
}

impl Gift {
    pub fn name(self) -> &'static str {
        match self {
            Gift::Flowers => "Flores",
            Gift::Perfume => "Perfume",
            Gift::Jewel => "Joia",
            Gift::Book => "Livro raro",
            Gift::Chocolates => "Caixa de bombons",
            Gift::Record => "Disco de vinil",
            Gift::Whiskey => "Garrafa de uísque",
        }
    }
    pub fn price(self) -> i32 {
        match self {
            Gift::Flowers => 2,
            Gift::Perfume => 12,
            Gift::Jewel => 45,
            Gift::Book => 6,
            Gift::Chocolates => 3,
            Gift::Record => 4,
            Gift::Whiskey => 5,
        }
    }
    pub fn value(self) -> i32 {
        match self {
            Gift::Flowers => 6,
            Gift::Perfume => 12,
            Gift::Jewel => 25,
            Gift::Book => 9,
            Gift::Chocolates => 6,
            Gift::Record => 8,
            Gift::Whiskey => 7,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Outfit {
    Modern,
    Suit,
    Worker,
    Police,
    Doctor,
    Journalist,
    Gangster,
    Aristocrat,
    Priest,
}

impl Outfit {
    pub const ALL: [Outfit; 9] = [
        Outfit::Modern,
        Outfit::Suit,
        Outfit::Worker,
        Outfit::Police,
        Outfit::Doctor,
        Outfit::Journalist,
        Outfit::Gangster,
        Outfit::Aristocrat,
        Outfit::Priest,
    ];
    pub fn name(self) -> &'static str {
        match self {
            Outfit::Modern => "Roupas do seu tempo (moletom e jeans)",
            Outfit::Suit => "Terno comum",
            Outfit::Worker => "Roupa de operário",
            Outfit::Police => "Uniforme policial",
            Outfit::Doctor => "Jaleco médico",
            Outfit::Journalist => "Traje de repórter",
            Outfit::Gangster => "Sobretudo de gângster",
            Outfit::Aristocrat => "Traje de gala",
            Outfit::Priest => "Batina",
        }
    }
    pub fn price(self) -> i32 {
        match self {
            Outfit::Modern => 0,
            Outfit::Suit => 18,
            Outfit::Worker => 6,
            Outfit::Police => 0,
            Outfit::Doctor => 0,
            Outfit::Journalist => 14,
            Outfit::Gangster => 30,
            Outfit::Aristocrat => 70,
            Outfit::Priest => 0,
        }
    }
    /// Sold openly at the tailor (others must be stolen or bought black market)
    pub fn tailor(self) -> bool {
        matches!(self, Outfit::Suit | Outfit::Worker | Outfit::Journalist | Outfit::Gangster | Outfit::Aristocrat)
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Item {
    Weapon(Weapon),
    Ammo(i32),
    Drug(Drug),
    Tool(Tool),
    Gift(Gift),
    Food,
    Clothes(Outfit),
    /// evidence item of a case: (case, clue index)
    Evidence(u8, u8),
    Valuable(String, i32),
    Newspaper(u32),
}

impl Item {
    pub fn name(&self, year: i32) -> String {
        match self {
            Item::Weapon(w) => w.stats().name.to_string(),
            Item::Ammo(n) => format!("Munição ({})", n),
            Item::Drug(d) => d.name(year).to_string(),
            Item::Tool(t) => t.name().to_string(),
            Item::Gift(g) => g.name().to_string(),
            Item::Food => "Comida".to_string(),
            Item::Clothes(o) => o.name().to_string(),
            Item::Evidence(_, _) => "Evidência".to_string(),
            Item::Valuable(n, _) => n.clone(),
            Item::Newspaper(_) => "Jornal".to_string(),
        }
    }
    pub fn sell_value(&self) -> i32 {
        match self {
            Item::Weapon(w) => w.stats().price / 2,
            Item::Drug(d) => d.price() / 2,
            Item::Tool(t) => t.price() / 2,
            Item::Gift(g) => g.price() / 2,
            Item::Valuable(_, v) => *v,
            _ => 0,
        }
    }
}
