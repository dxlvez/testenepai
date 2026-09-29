//! Money per world: every era/city has its own prices and currency.
//! Base prices are authored in 1920 dollars and scaled.

use crate::city::gen::CityId;
use crate::items::*;
use crate::state::Game;
use crate::util::Rng;

/// Price multiplier for the current world.
pub fn factor(city: CityId, year: i32) -> f32 {
    let usd = if year < 1930 {
        1.0
    } else if year < 1945 {
        0.85
    } else if year < 1960 {
        1.4
    } else if year < 1976 {
        2.0
    } else if year < 2000 {
        4.5
    } else {
        6.0
    };
    usd * city.currency_factor()
}

pub fn symbol(city: CityId, year: i32) -> &'static str {
    city.currency(year)
}

pub fn price(game: &Game, base: i32) -> i32 {
    ((base as f32 * factor(game.city, game.year)).round() as i32).max(1)
}

pub fn money_str(game: &Game, v: i32) -> String {
    format!("{}{}", symbol(game.city, game.year), v)
}

/// Loot found in a container, by building wealth and kind.
pub fn loot(r: &mut Rng, kind: crate::city::map::BKind, pkind: crate::city::map::PKind, wealth: f32, year: i32) -> Vec<Item> {
    use crate::city::map::{BKind, PKind};
    let mut v = Vec::new();
    let rich = wealth > 0.7 || kind == BKind::Mansion || kind == BKind::Bank;
    let n = if rich { r.range(1, 4) } else { r.range(0, 3) };
    for _ in 0..n {
        let roll = r.f();
        let it = match pkind {
            PKind::Safe => {
                if roll < 0.5 {
                    Item::Valuable("Maço de cédulas".into(), r.range(40, 160))
                } else if roll < 0.8 {
                    Item::Valuable("Colar de pérolas".into(), r.range(60, 200))
                } else {
                    Item::Valuable("Barra de ouro".into(), r.range(150, 400))
                }
            }
            PKind::Wardrobe => {
                if roll < 0.35 {
                    Item::Valuable("Casaco de pele".into(), r.range(15, 50))
                } else if roll < 0.55 {
                    Item::Valuable("Relógio de bolso".into(), r.range(10, 45))
                } else if roll < 0.7 && rich {
                    Item::Gift(Gift::Jewel)
                } else if roll < 0.8 {
                    Item::Valuable("Anel de ouro".into(), r.range(20, 70))
                } else {
                    Item::Valuable("Cartas de amor antigas".into(), 1)
                }
            }
            PKind::Desk | PKind::Shelf => {
                if kind == BKind::Pharmacy {
                    r.pick(&[Item::Drug(Drug::Laudanum), Item::Drug(Drug::Chloral), Item::Drug(Drug::Stimulant), Item::Tool(Tool::Bandage)]).clone()
                } else if kind == BKind::GunShop {
                    if roll < 0.5 {
                        Item::Ammo(r.range(6, 18))
                    } else {
                        Item::Weapon(if year >= 1980 { Weapon::Automatic } else { Weapon::Revolver })
                    }
                } else if kind == BKind::Police {
                    if roll < 0.5 {
                        Item::Ammo(r.range(6, 12))
                    } else {
                        Item::Clothes(Outfit::Police)
                    }
                } else if kind == BKind::Hospital {
                    if roll < 0.4 {
                        Item::Clothes(Outfit::Doctor)
                    } else {
                        r.pick(&[Item::Drug(Drug::Chloroform), Item::Drug(Drug::Laudanum), Item::Drug(Drug::TruthSerum), Item::Tool(Tool::Bandage)]).clone()
                    }
                } else if roll < 0.4 {
                    Item::Valuable("Dinheiro trocado".into(), r.range(2, 20))
                } else if roll < 0.6 {
                    Item::Valuable("Caneta-tinteiro".into(), r.range(4, 15))
                } else if roll < 0.75 && year >= 1950 {
                    Item::Valuable("Rádio portátil".into(), r.range(10, 35))
                } else {
                    Item::Valuable("Talheres de prata".into(), r.range(12, 40))
                }
            }
            PKind::Bed => {
                if roll < 0.5 {
                    Item::Valuable("Economias debaixo do colchão".into(), r.range(5, 60))
                } else if roll < 0.7 {
                    Item::Weapon(Weapon::Revolver)
                } else {
                    Item::Valuable("Fotografia de família".into(), 0)
                }
            }
            PKind::Crate | PKind::Barrel => {
                if roll < 0.4 && year < 1934 {
                    Item::Valuable("Garrafas de uísque contrabandeado".into(), r.range(15, 60))
                } else if roll < 0.6 {
                    Item::Ammo(r.range(4, 20))
                } else if roll < 0.75 {
                    Item::Weapon(*r.pick(&[Weapon::Shotgun, Weapon::Club, Weapon::Knife, Weapon::Tommy]))
                } else {
                    Item::Valuable("Mercadoria sem nota".into(), r.range(8, 40))
                }
            }
            PKind::Counter => Item::Valuable("Dinheiro do caixa".into(), r.range(10, 80)),
            _ => Item::Valuable("Bugiganga".into(), r.range(1, 8)),
        };
        v.push(it);
    }
    v
}

/// When Elias jumps to another era his cash becomes worthless antique notes.
pub fn convert_money_on_jump(game: &mut Game, old_city: crate::city::gen::CityId, old_year: i32) {
    let m = game.player.money;
    if m > 0 {
        let sym = match old_city.currency(old_year).trim() {
            "£" => "libras",
            "RM" => "reichsmarks",
            "DM" => "marcos",
            "kr" => "coroas",
            _ => "dólares",
        };
        game.player.inv.push(Item::Valuable(format!("Cédulas antigas de {} ({} {})", old_year, m, sym), (m / 4).max(1)));
    }
    game.player.money = price(game, 15);
}
