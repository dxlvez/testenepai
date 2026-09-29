//! Procedural city generation. Every city has its own style (layout,
//! architecture, water, outskirts, vegetation) and every city of every era
//! is generated from style + seed, so the same city in two different years
//! keeps its streets while the timeline decides which buildings survived.

use super::map::*;
use super::style::*;
use crate::util::Rng;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Hash)]
pub enum CityId {
    NewOrleans,
    Chicago,
    Bavaria,
    London,
    Adelaide,
    Berlin,
    Bergen,
    SanFrancisco,
    Portland,
    NewYork,
    LosAngeles,
}

impl CityId {
    pub fn name(self) -> &'static str {
        match self {
            CityId::NewOrleans => "Nova Orleans",
            CityId::Chicago => "Chicago",
            CityId::Bavaria => "Hinterfeld, Baviera",
            CityId::London => "Londres",
            CityId::Adelaide => "Adelaide",
            CityId::Berlin => "Berlim",
            CityId::Bergen => "Bergen",
            CityId::SanFrancisco => "São Francisco",
            CityId::Portland => "Portland",
            CityId::NewYork => "Nova York",
            CityId::LosAngeles => "Los Angeles",
        }
    }
    pub fn upper(self) -> &'static str {
        match self {
            CityId::NewOrleans => "NOVA ORLEANS",
            CityId::Chicago => "CHICAGO",
            CityId::Bavaria => "HINTERFELD, BAVIERA",
            CityId::London => "LONDRES",
            CityId::Adelaide => "ADELAIDE",
            CityId::Berlin => "BERLIM",
            CityId::Bergen => "BERGEN",
            CityId::SanFrancisco => "SÃO FRANCISCO",
            CityId::Portland => "PORTLAND",
            CityId::NewYork => "NOVA YORK",
            CityId::LosAngeles => "LOS ANGELES",
        }
    }
    fn seed(self) -> u64 {
        1000 + self as u64 * 7919
    }
    fn districts(self) -> [(&'static str, f32, f32); 6] {
        match self {
            CityId::NewOrleans => [("Tremé", 0.3, 0.5), ("French Quarter", 0.5, 0.6), ("Garden District", 0.9, 0.1), ("Storyville", 0.3, 0.9), ("Porto", 0.2, 0.7), ("Marigny", 0.5, 0.4)],
            CityId::Chicago => [("Near North", 0.8, 0.2), ("The Loop", 0.7, 0.4), ("Gold Coast", 1.0, 0.1), ("Levee", 0.2, 0.9), ("Stockyards", 0.2, 0.7), ("Little Italy", 0.4, 0.6)],
            CityId::Bavaria => [("Oberdorf", 0.6, 0.2), ("Kirchplatz", 0.6, 0.2), ("Mühlweg", 0.5, 0.2), ("Unterdorf", 0.3, 0.3), ("Bachgasse", 0.3, 0.3), ("Hofacker", 0.4, 0.3)],
            CityId::London => [("Whitechapel", 0.2, 0.8), ("Soho", 0.5, 0.6), ("Mayfair", 1.0, 0.1), ("Stepney", 0.2, 0.7), ("Docklands", 0.2, 0.7), ("Holborn", 0.6, 0.4)],
            CityId::Adelaide => [("North Terrace", 0.7, 0.2), ("Centro", 0.6, 0.3), ("Glenelg", 0.6, 0.3), ("Hindley", 0.3, 0.7), ("Port Adelaide", 0.2, 0.6), ("Somerton", 0.5, 0.2)],
            CityId::Berlin => [("Wedding", 0.3, 0.6), ("Mitte", 0.5, 0.5), ("Charlottenburg", 0.9, 0.2), ("Kreuzberg", 0.3, 0.6), ("Friedrichshain", 0.3, 0.6), ("Tiergarten", 0.6, 0.3)],
            CityId::Bergen => [("Sandviken", 0.4, 0.3), ("Sentrum", 0.6, 0.3), ("Nordnes", 0.7, 0.2), ("Nøstet", 0.4, 0.4), ("Bryggen", 0.4, 0.5), ("Møhlenpris", 0.5, 0.3)],
            CityId::SanFrancisco => [("Haight-Ashbury", 0.3, 0.5), ("Downtown", 0.7, 0.4), ("Nob Hill", 1.0, 0.1), ("Tenderloin", 0.2, 0.9), ("Embarcadero", 0.3, 0.6), ("Chinatown", 0.4, 0.5)],
            CityId::Portland => [("Old Town", 0.3, 0.6), ("Downtown", 0.6, 0.4), ("West Hills", 0.9, 0.1), ("Skid Road", 0.2, 0.8), ("Waterfront", 0.3, 0.6), ("Pearl", 0.5, 0.3)],
            CityId::NewYork => [("Harlem", 0.3, 0.6), ("Midtown", 0.8, 0.4), ("Upper East Side", 1.0, 0.1), ("Hell's Kitchen", 0.2, 0.9), ("Lower East Side", 0.3, 0.7), ("Chelsea", 0.5, 0.4)],
            CityId::LosAngeles => [("Hollywood", 0.6, 0.5), ("Downtown", 0.6, 0.5), ("Beverly Hills", 1.0, 0.1), ("Skid Row", 0.1, 0.9), ("San Pedro", 0.3, 0.6), ("Echo Park", 0.4, 0.5)],
        }
    }
    fn streets(self) -> (&'static [&'static str], &'static [&'static str]) {
        match self {
            CityId::NewOrleans => (&["Rua Iberville", "Rua Bienville", "Rua Conti", "Rua St. Louis", "Rua Toulouse", "Rua Dumaine"], &["Rua Burgundy", "Rua Dauphine", "Rua Bourbon", "Rua Royal", "Rua Chartres", "Rua Decatur", "Rua Rampart", "Rua Esplanade"]),
            CityId::Chicago => (&["Rua Division", "Rua Chicago", "Rua Grand", "Rua Madison", "Rua Harrison", "Rua Roosevelt"], &["Av. Clark", "Rua LaSalle", "Rua Wells", "Rua State", "Av. Wabash", "Av. Michigan", "Rua Halsted", "Rua Canal", "Rua Dearborn"]),
            CityId::Bavaria => (&["Dorfstraße", "Kirchgasse", "Mühlweg", "Feldweg"], &["Hauptstraße", "Schulgasse", "Brunnenweg", "Waldweg", "Bachstraße"]),
            CityId::London => (&["Commercial Road", "Whitechapel Road", "Brick Lane", "Fleet Street", "Cable Street", "The Strand"], &["Mile End", "Dean Street", "Old Compton", "Bow Lane", "Cheapside", "Drury Lane", "Shoe Lane", "Bell Lane"]),
            CityId::Adelaide => (&["North Terrace", "Rua Rundle", "Rua Grenfell", "Rua Waymouth", "Rua Franklin", "South Terrace"], &["West Terrace", "Rua Morphett", "Rua King William", "Rua Pulteney", "Rua Frome", "East Terrace", "Rua Jetty", "Esplanada"]),
            CityId::Berlin => (&["Invalidenstraße", "Torstraße", "Unter den Linden", "Leipziger Straße", "Kochstraße", "Oranienstraße"], &["Friedrichstraße", "Chausseestraße", "Brunnenstraße", "Wilhelmstraße", "Lindenstraße", "Prinzenstraße", "Heinrich-Heine-Str."]),
            CityId::Bergen => (&["Strandgaten", "Kong Oscars gate", "Øvregaten", "Vetrlidsallmenningen", "Marken", "Nygårdsgaten"], &["Bryggen", "Torget", "Lille Øvregaten", "Nordnesgaten", "Kaigaten", "Allehelgens gate"]),
            CityId::SanFrancisco => (&["Rua Haight", "Rua Geary", "Rua Post", "Rua Market", "Rua Mission", "Rua Folsom"], &["Rua Stanyan", "Rua Masonic", "Rua Divisadero", "Rua Van Ness", "Rua Polk", "Rua Powell", "Rua Kearny", "Embarcadero"]),
            CityId::Portland => (&["Burnside", "Couch St", "Oak St", "Alder St", "Morrison", "Salmon St"], &["1st Avenue", "2nd Avenue", "Broadway", "Park Avenue", "10th Avenue", "Naito Parkway"]),
            CityId::NewYork => (&["Rua 125", "Rua 96", "Rua 72", "Rua 42", "Rua 14", "Rua Canal"], &["Av. Amsterdam", "Broadway", "Av. Columbus", "Quinta Avenida", "Av. Madison", "Av. Lexington", "Bowery", "Av. Park", "Av. Lenox"]),
            CityId::LosAngeles => (&["Sunset Blvd", "Hollywood Blvd", "Wilshire Blvd", "Olympic Blvd", "Pico Blvd", "Venice Blvd"], &["Vine St", "Highland Ave", "La Brea", "Western Ave", "Vermont Ave", "Alvarado St", "Figueroa St"]),
        }
    }
}

pub struct Grid {
    pub x0: i32,
    pub y0: i32,
    pub bx: i32,
    pub by: i32,
    pub px: i32,
    pub py: i32,
    pub st: i32,
}

pub const OUT: i32 = 30;
pub const RAIL: i32 = 5;

pub fn grid_of(city: CityId, year: i32) -> Grid {
    let s = style(city, year);
    Grid { x0: OUT, y0: RAIL, bx: s.bx, by: s.by, px: s.px, py: s.py, st: s.st }
}

fn water_h(s: &Style) -> i32 {
    match s.water {
        Water::Bottom | Water::Fjord => 14,
        _ => 3,
    }
}

#[derive(Clone, Copy, PartialEq)]
enum BlockRole {
    Normal,
    Park,
    Cemetery,
    Plaza,
    Ruin,
}

struct Ctx<'a> {
    m: Map,
    rng: Rng,
    city: CityId,
    year: i32,
    s: Style,
    g: Grid,
    names: &'a mut NameGen,
}

pub struct NameGen {
    used: std::collections::HashSet<String>,
}

pub fn generate(city: CityId, year: i32) -> Map {
    let mut names = NameGen { used: Default::default() };
    let s = style(city, year);
    let g = grid_of(city, year);
    let extra_right = if s.water == Water::Right { 16 } else { 0 };
    let w = OUT + g.bx * g.px + g.st + extra_right;
    let h = RAIL + g.by * g.py + g.st + water_h(&s);
    let (gpx, gpy) = (g.px, g.py);
    let mut ctx = Ctx { m: Map::new(w, h, Tile::Grass), rng: Rng::new(city.seed()), city, year, s, g, names: &mut names };
    ctx.m.name = city.name().to_string();
    ctx.m.city = Some(city);
    ctx.m.year = year;
    ctx.base_layout();
    ctx.districts();
    ctx.blocks();
    ctx.outskirts();
    ctx.water_features();
    ctx.special_features();
    ctx.street_furniture();
    ctx.m.rebuild_prop_index();
    let spawn = ctx
        .m
        .find_building(BKind::Hotel)
        .or_else(|| ctx.m.find_building(BKind::Bar))
        .map(|b| ctx.m.buildings[b].outside_px())
        .unwrap_or(vec2((OUT + gpx * 2) as f32 + 1.5, (RAIL + gpy) as f32 + 1.5));
    ctx.m.spawn = spawn;
    ctx.m
}

impl<'a> Ctx<'a> {
    fn cx0(&self) -> i32 {
        OUT
    }
    fn cy0(&self) -> i32 {
        RAIL
    }
    fn city_w(&self) -> i32 {
        self.g.bx * self.g.px + self.g.st
    }

    fn base_layout(&mut self) {
        let (w, h) = (self.m.w, self.m.h);
        let wh = water_h(&self.s);
        let (px, py, st) = (self.g.px, self.g.py, self.g.st);
        let cw = self.city_w();
        for y in 0..h {
            for x in 0..w {
                let cx = x - self.cx0();
                let cy = y - self.cy0();
                let in_city = cx >= 0 && cx < cw && cy >= 0 && cy < self.g.by * py + st;
                let t = if matches!(self.s.water, Water::Bottom | Water::Fjord) && y >= h - wh {
                    Tile::Water
                } else if self.s.water == Water::Right && cx >= cw + 3 {
                    Tile::Water
                } else if matches!(self.s.water, Water::Bottom | Water::Fjord) && y >= h - wh - 1 {
                    if self.s.outskirts == Outskirts::Beach && cx < 0 { Tile::Sand } else { Tile::Gravel }
                } else if cx < 0 {
                    match self.s.outskirts {
                        Outskirts::Fields | Outskirts::Snow => {
                            if y < RAIL {
                                Tile::Grass
                            } else if (y - RAIL) % 12 < 9 && cx < -3 {
                                Tile::Field
                            } else {
                                Tile::Grass
                            }
                        }
                        Outskirts::Beach => {
                            if cx < -18 {
                                Tile::Sand
                            } else {
                                Tile::Grass
                            }
                        }
                        _ => Tile::Grass,
                    }
                } else if !in_city {
                    Tile::Gravel
                } else if cx % px < st || cy % py < st {
                    if self.s.street == Surface::Dirt { Tile::Dirt } else { Tile::Road }
                } else {
                    let bx = cx % px - st;
                    let by = cy % py - st;
                    if bx == 0 || by == 0 || bx == px - st - 1 || by == py - st - 1 {
                        if self.s.arch == Arch::Village { Tile::Grass } else { Tile::Sidewalk }
                    } else {
                        Tile::Alley
                    }
                };
                self.m.set(x, y, t);
            }
        }
        // a road leading into the outskirts
        let ry = self.cy0() + py * (self.g.by / 2) + 1;
        for x in 2..self.cx0() {
            for d in 0..2 {
                self.m.set(x, ry + d, Tile::Dirt);
            }
        }
        let (hn, vn) = self.city.streets();
        for i in 0..=self.g.by {
            let y = self.cy0() + i * py + 1;
            self.m.streets.push((hn[i as usize % hn.len()].to_string(), true, y));
        }
        for i in 0..=self.g.bx {
            let x = self.cx0() + i * px + 1;
            self.m.streets.push((vn[i as usize % vn.len()].to_string(), false, x));
        }
    }

    fn districts(&mut self) {
        let ds = self.city.districts();
        let bx = self.g.bx;
        let by = self.g.by;
        let cols = [0, (bx / 3).max(1), (bx * 2 / 3).max(2), bx];
        let rows = [0, (by / 2).max(1), by];
        let mut i = 0;
        for r in 0..2 {
            for c in 0..3 {
                let (name, wealth, danger) = ds[i];
                self.m.districts.push(District {
                    name: name.to_string(),
                    x: self.cx0() + cols[c] * self.g.px,
                    y: self.cy0() + rows[r] * self.g.py,
                    w: (cols[c + 1] - cols[c]) * self.g.px + self.g.st,
                    h: (rows[r + 1] - rows[r]) * self.g.py + self.g.st,
                    wealth,
                    danger,
                });
                i += 1;
            }
        }
        self.m.districts.push(District { name: "Arredores".to_string(), x: 0, y: 0, w: self.cx0(), h: self.m.h, wealth: 0.3, danger: 0.4 });
    }

    fn block_role(&mut self, bx: i32, by: i32) -> BlockRole {
        let (cbx, cby) = (self.g.bx / 2, self.g.by / 2);
        if bx == cbx && by == cby {
            return if self.s.arch == Arch::Village { BlockRole::Plaza } else { BlockRole::Park };
        }
        if bx == 0 && by == self.g.by - 1 {
            return BlockRole::Cemetery;
        }
        if bx == (self.g.bx - 2).max(1) && by == 1.min(self.g.by - 1) && self.s.arch != Arch::Village {
            return BlockRole::Plaza;
        }
        if self.s.ruins > 0.0 && self.rng.chance(self.s.ruins * 0.5) && !(bx == cbx && by == cby) {
            return BlockRole::Ruin;
        }
        BlockRole::Normal
    }

    fn blocks(&mut self) {
        let (bx_n, by_n) = (self.g.bx, self.g.by);
        let mid = (bx_n / 2, by_n / 2);
        let mut required: Vec<(BKind, i32, (i32, i32))> = vec![
            (BKind::Police, 1, (mid.0, (mid.1 - 1).max(0))),
            (BKind::Church, 1, ((mid.0 - 1).max(0), mid.1)),
            (BKind::Hotel, 1, ((mid.0 - 1).max(0), (mid.1 - 1).max(0))),
            (BKind::Bar, 0, ((mid.0 - 2).max(0), mid.1)),
            (BKind::Bar, 0, ((mid.0 + 1).min(bx_n - 1), (mid.1 + 1).min(by_n - 1))),
            (BKind::Restaurant, 0, ((mid.0 - 1).max(0), (mid.1 - 1).max(0))),
            (BKind::General, 0, ((mid.0 - 2).max(0), (mid.1 - 1).max(0))),
            (BKind::Clothing, 0, (mid.0, (mid.1 - 1).max(0))),
            (BKind::Pharmacy, 0, ((mid.0 - 1).max(0), mid.1)),
            (BKind::Pawn, 0, (0, (mid.1).max(0))),
            (BKind::GunShop, 0, (1.min(bx_n - 1), (by_n - 2).max(0))),
            (BKind::Office, 0, ((mid.0 + 1).min(bx_n - 1), (mid.1 - 1).max(0))),
            (BKind::Warehouse, 1, (1.min(bx_n - 1), by_n - 1)),
            (BKind::Abandoned, 1, (0, (by_n - 2).max(0))),
        ];
        if self.s.arch != Arch::Village {
            required.extend([
                (BKind::Hospital, 1, ((mid.0 + 1).min(bx_n - 1), mid.1)),
                (BKind::Station, 1, (mid.0, 0)),
                (BKind::Hotel, 0, ((mid.0 + 1).min(bx_n - 1), (mid.1 + 1).min(by_n - 1))),
                (BKind::Mansion, 1, (bx_n - 1, 0)),
                (BKind::Mansion, 1, (bx_n - 1, 1.min(by_n - 1))),
                (BKind::Warehouse, 1, (2.min(bx_n - 1), by_n - 1)),
                (BKind::Factory, 1, (bx_n - 1, by_n - 1)),
                (BKind::Club, 0, ((mid.0 - 1).max(0), (mid.1 + 1).min(by_n - 1))),
                (BKind::Club, 0, (mid.0, (mid.1 + 1).min(by_n - 1))),
                (BKind::Cabaret, 0, (mid.0, (mid.1 + 1).min(by_n - 1))),
                (BKind::Bar, 0, (0, 1.min(by_n - 1))),
                (BKind::Bar, 0, ((mid.0 + 2).min(bx_n - 1), (by_n - 1))),
                (BKind::Restaurant, 0, ((mid.0 + 1).min(bx_n - 1), 1.min(by_n - 1))),
                (BKind::Newspaper, 0, ((mid.0 + 1).min(bx_n - 1), 1.min(by_n - 1))),
                (BKind::Bank, 0, ((mid.0 + 1).min(bx_n - 1), 0)),
                (BKind::Office, 0, ((mid.0 + 2).min(bx_n - 1), mid.1)),
            ]);
            if self.year >= 1925 {
                required.push((BKind::Radio, 0, ((mid.0 + 2).min(bx_n - 1), mid.1)));
            }
        } else {
            // a village: the doctor lives in a house, the inn is a bar
            required.push((BKind::Hospital, 0, (mid.0, (mid.1 + 1).min(by_n - 1))));
            required.push((BKind::Mansion, 1, (bx_n - 1, 0)));
            required.push((BKind::Barn, 1, (0, 0)));
            required.push((BKind::Barn, 1, (bx_n - 1, by_n - 1)));
            required.push((BKind::Farmhouse, 1, (bx_n - 1, 1.min(by_n - 1))));
        }
        let mut placed = vec![false; required.len()];
        for by in 0..by_n {
            for bx in 0..bx_n {
                let st = self.g.st;
                let x0 = self.cx0() + bx * self.g.px + st + 1;
                let y0 = self.cy0() + by * self.g.py + st + 1;
                let iw = self.g.px - st - 2;
                let ih = self.g.py - st - 2;
                match self.block_role(bx, by) {
                    BlockRole::Park => {
                        self.park(x0, y0, iw, ih);
                        continue;
                    }
                    BlockRole::Cemetery => {
                        self.cemetery(x0, y0, iw, ih);
                        continue;
                    }
                    BlockRole::Plaza => {
                        self.plaza(x0, y0, iw, ih);
                        continue;
                    }
                    BlockRole::Ruin => {
                        self.ruins(x0, y0, iw, ih);
                        // still place required buildings of this block somewhere else later
                        continue;
                    }
                    BlockRole::Normal => {}
                }
                let half_h = ih / 2;
                for half in 0..2 {
                    let hy = y0 + half * half_h;
                    let facing_up = half == 0;
                    let mut used_half = false;
                    for i in 0..required.len() {
                        let (k, size, pref) = required[i];
                        if placed[i] || size != 1 || pref != (bx, by) {
                            continue;
                        }
                        self.building(k, (x0, hy, iw, half_h), facing_up);
                        placed[i] = true;
                        used_half = true;
                        break;
                    }
                    if used_half {
                        continue;
                    }
                    let mut cx = 0;
                    while cx < iw {
                        let remaining = iw - cx;
                        let (lo, hi) = match self.s.arch {
                            Arch::HighRise | Arch::Stone => (6, 11),
                            Arch::Village => (7, 11),
                            Arch::Colonial | Arch::Stucco => (6, 10),
                            _ => (5, 9),
                        };
                        let mut w = self.rng.range(lo, hi).min(remaining);
                        if remaining - w < lo {
                            w = remaining;
                        }
                        // villages and colonial towns leave gaps (gardens) between houses
                        let gap = if matches!(self.s.arch, Arch::Village | Arch::Colonial | Arch::Stucco) && w > 7 && remaining > w { 1 } else { 0 };
                        let rect = (x0 + cx, hy, w - gap, half_h);
                        let mut kind = None;
                        for i in 0..required.len() {
                            let (k, size, pref) = required[i];
                            if !placed[i] && size == 0 && pref == (bx, by) {
                                kind = Some(k);
                                placed[i] = true;
                                break;
                            }
                        }
                        let kind = kind.unwrap_or_else(|| self.filler_kind(bx, by));
                        self.building(kind, rect, facing_up);
                        if gap > 0 {
                            for yy in hy..hy + half_h {
                                self.m.set(x0 + cx + w - 1, yy, Tile::Grass);
                            }
                        }
                        cx += w;
                    }
                }
            }
        }
        // anything required that landed on a ruin block: put it in any house slot
        for i in 0..required.len() {
            if placed[i] {
                continue;
            }
            let k = required[i].0;
            if let Some(b) = self.m.buildings.iter().position(|b| b.kind == BKind::House && !b.name.is_empty() == false && b.w >= 6) {
                self.m.buildings[b].kind = k;
                let n = self.building_name(k);
                self.m.buildings[b].name = n;
                self.m.buildings[b].locked = false;
            }
        }
    }

    fn filler_kind(&mut self, bx: i32, by: i32) -> BKind {
        let rich = bx >= self.g.bx - 2 && by <= 1;
        let poor = by >= self.g.by - 2;
        let r = self.rng.f();
        if self.s.arch == Arch::Village {
            return if r < 0.7 { BKind::House } else if r < 0.85 { BKind::Barn } else { BKind::Farmhouse };
        }
        if rich {
            if r < 0.7 {
                BKind::House
            } else {
                BKind::Apartment
            }
        } else if r < if poor { 0.3 } else { 0.45 } {
            BKind::House
        } else if r < 0.82 {
            BKind::Apartment
        } else if r < 0.95 {
            BKind::General
        } else {
            BKind::Abandoned
        }
    }

    fn building_name(&mut self, k: BKind) -> String {
        let pools = super::names::building_pool(self.city, k, self.year);
        for _ in 0..8 {
            let n = self.rng.pick(pools).to_string();
            if n.is_empty() {
                return String::new();
            }
            if !self.names.used.contains(&n) || k == BKind::Abandoned {
                self.names.used.insert(n.clone());
                return n;
            }
        }
        format!("{} {}", k.label(), self.m.buildings.len())
    }

    /// Create a building occupying rect (x, y, w, h) with the door facing the
    /// street above (facing_up) or below.
    fn building(&mut self, kind: BKind, (x, y, w, h): (i32, i32, i32, i32), facing_up: bool) -> usize {
        let id = self.m.buildings.len();
        for yy in y..y + h {
            for xx in x..x + w {
                let edge = xx == x || yy == y || xx == x + w - 1 || yy == y + h - 1;
                self.m.set(xx, yy, if edge { Tile::Wall } else { Tile::Floor });
                self.m.owner[(yy * self.m.w + xx) as usize] = id as u32;
            }
        }
        let dy = if facing_up { y } else { y + h - 1 };
        let dx = x + 1 + self.rng.range(0, (w - 2).max(1));
        let dx = dx.min(x + w - 2);
        self.m.set(dx, dy, Tile::Door);
        let mut doors = vec![(dx, dy)];
        let win_step = if self.rng.chance(0.5) { 2 } else { 3 };
        for xx in x + 1..x + w - 1 {
            if xx != dx && (xx - x) % win_step == 0 {
                self.m.set(xx, dy, Tile::Window);
            }
        }
        // side / back windows sometimes (entry points for burglars)
        let back = if facing_up { y + h - 1 } else { y };
        if self.rng.chance(0.6) {
            let wx = x + 1 + self.rng.range(0, (w - 2).max(1));
            if wx != dx && self.m.get(wx, back) == Tile::Wall {
                self.m.set(wx, back, Tile::Window);
            }
        }
        if w >= 12 {
            let bxx = x + w - 3;
            self.m.set(bxx, back, Tile::Door);
            doors.push((bxx, back));
        }
        let district = self.m.district_at(vec2(x as f32 + 0.5, y as f32 + 0.5)).unwrap_or(0);
        let name = self.building_name(kind);
        let street = self.m.street_name_near(vec2(dx as f32, dy as f32));
        let number = 10 + (id * 7) % 190;
        self.m.buildings.push(Building {
            id,
            kind,
            name,
            x,
            y,
            w,
            h,
            doors,
            spots: Vec::new(),
            lights: Vec::new(),
            district,
            address: format!("{}, {}", street, number),
            residents: Vec::new(),
            owner: None,
            closed_forever: false,
            locked: matches!(kind, BKind::House | BKind::Apartment | BKind::Mansion | BKind::Abandoned | BKind::Warehouse | BKind::Farmhouse | BKind::Barn),
        });
        self.furnish(id, facing_up);
        id
    }

    fn free(&self, x: i32, y: i32) -> bool {
        if self.m.get(x, y) != Tile::Floor {
            return false;
        }
        if self.m.props.iter().any(|p| x >= p.x && y >= p.y && x < p.x + p.w && y < p.y + p.h) {
            return false;
        }
        // keep the tile in front of any door clear
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if self.m.get(x + dx, y + dy) == Tile::Door {
                return false;
            }
        }
        true
    }

    fn area_free(&self, x: i32, y: i32, w: i32, h: i32) -> bool {
        for yy in y..y + h {
            for xx in x..x + w {
                if !self.free(xx, yy) {
                    return false;
                }
            }
        }
        true
    }

    fn put(&mut self, k: PKind, x: i32, y: i32, w: i32, h: i32, b: usize) -> Option<usize> {
        if !self.area_free(x, y, w, h) {
            return None;
        }
        let i = self.m.add_prop(k, x, y, w, h, Some(b));
        self.m.props[i].tint = 0.8 + self.rng.f() * 0.4;
        Some(i)
    }

    /// Place a piece with its back flush against a wall (and its front open), anywhere in the rect.
    /// `lw`/`lh` are the piece's own width/depth; it may be turned to fit the wall.
    #[allow(clippy::too_many_arguments)]
    fn place_wall(&mut self, k: PKind, rx: i32, ry: i32, rw: i32, rh: i32, lw: i32, lh: i32, b: usize) -> Option<usize> {
        let low = matches!(k, PKind::Sofa | PKind::Bed | PKind::Desk | PKind::Armchair | PKind::Table | PKind::Bathtub | PKind::RadioSet | PKind::Tv | PKind::Nightstand | PKind::Plant);
        let is_back = |t: Tile| t == Tile::Wall || (low && t == Tile::Window);
        let mut cands: Vec<(i32, i32, i32, i32, u8)> = Vec::new();
        for y in ry..ry + rh {
            for x in rx..rx + rw {
                for rot in 0..4u8 {
                    let (w, h) = if rot % 2 == 1 { (lh, lw) } else { (lw, lh) };
                    if x + w > rx + rw || y + h > ry + rh {
                        continue;
                    }
                    // rot: 0 = back towards +z, 1 = +x, 2 = -z, 3 = -x
                    let (back, front): (Vec<(i32, i32)>, Vec<(i32, i32)>) = match rot {
                        0 => ((x..x + w).map(|a| (a, y + h)).collect(), (x..x + w).map(|a| (a, y - 1)).collect()),
                        1 => ((y..y + h).map(|a| (x + w, a)).collect(), (y..y + h).map(|a| (x - 1, a)).collect()),
                        2 => ((x..x + w).map(|a| (a, y - 1)).collect(), (x..x + w).map(|a| (a, y + h)).collect()),
                        _ => ((y..y + h).map(|a| (x - 1, a)).collect(), (y..y + h).map(|a| (x + w, a)).collect()),
                    };
                    if back.iter().all(|&(a, c)| is_back(self.m.get(a, c))) && front.iter().all(|&(a, c)| self.free(a, c)) && self.area_free(x, y, w, h) {
                        cands.push((x, y, w, h, rot));
                    }
                }
            }
        }
        if cands.is_empty() {
            return None;
        }
        let (x, y, w, h, rot) = cands[self.rng.idx(cands.len())];
        let i = self.put(k, x, y, w, h, b)?;
        self.m.props[i].rot = rot;
        Some(i)
    }

    /// A chair next to a table at offset (dx, dy), turned to face the table.
    fn chair_at(&mut self, tx: i32, ty: i32, dx: i32, dy: i32, b: usize) -> Option<usize> {
        let i = self.put(PKind::Chair, tx + dx, ty + dy, 1, 1, b)?;
        // the backrest points away from the table
        self.m.props[i].rot = if dx > 0 {
            1
        } else if dx < 0 {
            3
        } else if dy > 0 {
            0
        } else {
            2
        };
        Some(i)
    }

    /// Corner pieces (plants, floor lamps, coat stands): two walls touching.
    fn place_corner(&mut self, k: PKind, rx: i32, ry: i32, rw: i32, rh: i32, b: usize) -> Option<usize> {
        let mut cands = Vec::new();
        for y in ry..ry + rh {
            for x in rx..rx + rw {
                let wx = self.m.get(x - 1, y) == Tile::Wall || self.m.get(x + 1, y) == Tile::Wall;
                let wy = self.m.get(x, y - 1) == Tile::Wall || self.m.get(x, y + 1) == Tile::Wall;
                if wx && wy && self.free(x, y) {
                    cands.push((x, y));
                }
            }
        }
        if cands.is_empty() {
            return None;
        }
        let (x, y) = cands[self.rng.idx(cands.len())];
        self.put(k, x, y, 1, 1, b)
    }

    /// Try to place a prop somewhere inside the rect. Furniture that belongs
    /// against a wall goes against a wall; corner pieces go in corners.
    fn scatter(&mut self, k: PKind, rx: i32, ry: i32, rw: i32, rh: i32, w: i32, h: i32, b: usize) -> Option<usize> {
        use PKind::*;
        if matches!(k, Plant | FloorLamp) {
            if let Some(i) = self.place_corner(k, rx, ry, rw, rh, b) {
                return Some(i);
            }
        }
        if matches!(k, Bed | Wardrobe | Shelf | Stove | Fridge | Sink | Toilet | Sofa | Tv | Piano | Desk | Bathtub | RadioSet | Mirror | Safe | Board | Altar | Nightstand | Armchair | Counter | Typewriter) {
            // beds and tubs are placed by their own length/width, others as given
            if let Some(i) = self.place_wall(k, rx, ry, rw, rh, w.min(h).max(if matches!(k, Sofa | Piano | Desk | Shelf) { w.max(h) } else { 0 }), if matches!(k, Sofa | Piano | Desk | Shelf) { w.min(h) } else { w.max(h) }, b) {
                return Some(i);
            }
            return None;
        }
        // free-standing pieces (tables, rugs): keep a walkway around them
        for _ in 0..40 {
            let x = rx + self.rng.range(0, (rw - w + 1).max(1));
            let y = ry + self.rng.range(0, (rh - h + 1).max(1));
            let clear = (x - 1..x + w + 1).all(|a| (y - 1..y + h + 1).all(|c| (a >= x && a < x + w && c >= y && c < y + h) || self.m.get(a, c) != Tile::Wall || matches!(k, Rug)));
            if !clear && self.rng.chance(0.8) {
                continue;
            }
            if let Some(i) = self.put(k, x, y, w, h, b) {
                return Some(i);
            }
        }
        for _ in 0..30 {
            let x = rx + self.rng.range(0, (rw - w + 1).max(1));
            let y = ry + self.rng.range(0, (rh - h + 1).max(1));
            if let Some(i) = self.put(k, x, y, w, h, b) {
                return Some(i);
            }
        }
        None
    }

    fn spot(&mut self, b: usize, kind: SpotKind, x: i32, y: i32) {
        self.m.buildings[b].spots.push(Spot { kind, pos: tile_center(x, y), owner: None });
    }

    fn partition_v(&mut self, b: usize, at: i32) {
        let bl = self.m.buildings[b].clone();
        let gap = bl.y + 1 + self.rng.range(0, (bl.h - 2).max(1));
        for yy in bl.y + 1..bl.y + bl.h - 1 {
            if yy != gap {
                self.m.set(at, yy, Tile::Wall);
            }
        }
    }

    fn partition_h(&mut self, b: usize, at: i32, x0: i32, x1: i32) {
        let gap = x0 + self.rng.range(0, (x1 - x0).max(1));
        for xx in x0..x1 {
            if xx != gap {
                self.m.set(xx, at, Tile::Wall);
            }
        }
        let _ = b;
    }

    /// Stove (+ fridge/icebox) along the back wall with a "cook" spot in front.
    fn cook_at(&mut self, b: usize, rx: i32, ry: i32, rw: i32, rh: i32, facing_up: bool) {
        let back = if facing_up { ry + rh - 1 } else { ry };
        let inward = if facing_up { -1 } else { 1 };
        if let Some(p) = self.scatter(PKind::Stove, rx, back, rw, 1, 1, 1, b) {
            let (px, py) = (self.m.props[p].x, self.m.props[p].y);
            self.spot(b, SpotKind::Cook, px, py + inward);
            if self.year >= 1920 {
                self.put(PKind::Fridge, px + 1, py, 1, 1, b).or_else(|| self.put(PKind::Fridge, px - 1, py, 1, 1, b));
            }
            if self.rng.chance(0.6) {
                self.put(PKind::Counter, px - 1, py, 1, 1, b).or_else(|| self.put(PKind::Counter, px + 2, py, 1, 1, b));
            }
        }
    }

    /// A small bathroom walled off in a corner (toilet, tub or basin).
    fn bath_corner(&mut self, b: usize, rx: i32, ry: i32, rw: i32, rh: i32, facing_up: bool) {
        if rw < 2 || rh < 4 {
            // too small for a room: just a toilet in a corner
            if let Some(p) = self.scatter(PKind::Toilet, rx, ry, rw, rh, 1, 1, b) {
                let (px, py) = (self.m.props[p].x, self.m.props[p].y);
                self.spot(b, SpotKind::Toilet, px, py);
            }
            return;
        }
        let bw = rw.min(3);
        let (by, wall_y) = if facing_up { (ry + rh - 2, ry + rh - 3) } else { (ry, ry + 2) };
        let bx = rx + rw - bw;
        // wall with a door gap
        let gap = bx + self.rng.range(0, bw);
        for xx in bx..bx + bw {
            if xx != gap && self.m.get(xx, wall_y) == Tile::Floor {
                self.m.set(xx, wall_y, Tile::Wall);
            }
        }
        if bw < rw && self.m.get(bx - 1, by) == Tile::Floor {
            for yy in by..by + 2 {
                self.m.set(bx - 1, yy, Tile::Wall);
            }
        }
        if let Some(p) = self.scatter(PKind::Toilet, bx, by, bw, 2, 1, 1, b) {
            let (px, py) = (self.m.props[p].x, self.m.props[p].y);
            self.spot(b, SpotKind::Toilet, px, py);
        }
        if bw >= 2 && self.rng.chance(0.7) {
            if let Some(p) = self.scatter(PKind::Bathtub, bx, by, bw, 2, 2, 1, b) {
                let (px, py) = (self.m.props[p].x, self.m.props[p].y);
                self.spot(b, SpotKind::Wash, px, py);
            }
        }
        if let Some(p) = self.scatter(PKind::Sink, bx, by, bw, 2, 1, 1, b) {
            let (px, py) = (self.m.props[p].x, self.m.props[p].y);
            self.spot(b, SpotKind::Wash, px, py);
        }
    }

    /// A dwelling: kitchen, living corner, bathroom and bedroom, furnished by era and wealth.
    #[allow(clippy::too_many_arguments)]
    fn home(&mut self, b: usize, ix: i32, iy: i32, iw: i32, ih: i32, facing_up: bool, rooms: bool) {
        let rich = self.m.districts.get(self.m.buildings[b].district).map(|d| d.wealth).unwrap_or(0.5) > 0.6;
        let tv_year = if rich { 1952 } else { 1962 };
        if rooms {
            let at = ix + iw / 2;
            self.partition_v(b, at);
            let (bx, bw) = (at + 1, ix + iw - at - 1);
            // bathroom first so the bedroom furniture doesn't block it
            self.bath_corner(b, bx, iy, bw, ih, facing_up);
            let beds = if self.rng.chance(0.5) { 2 } else { 1 };
            for _ in 0..beds {
                if let Some(p) = self.scatter(PKind::Bed, bx, iy, bw, ih, 1, 2, b) {
                    let (px, py) = (self.m.props[p].x, self.m.props[p].y);
                    self.spot(b, SpotKind::Bed, px, py);
                    let (bw, bh) = (self.m.props[p].w, self.m.props[p].h);
                    self.place_wall(PKind::Nightstand, px - 1, py - 1, bw + 2, bh + 2, 1, 1, b);
                }
            }
            self.scatter(PKind::Wardrobe, bx, iy, bw, ih, 1, 1, b);
            if self.rng.chance(0.4) {
                self.scatter(PKind::Mirror, bx, iy, bw, ih, 1, 1, b);
            }
            // kitchen + living room on the left
            let lw = at - ix;
            self.cook_at(b, ix, iy, lw, ih, facing_up);
            if let Some(t) = self.scatter(PKind::Table, ix, iy, lw, ih, 1, 1, b) {
                let (tx, ty) = (self.m.props[t].x, self.m.props[t].y);
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1)] {
                    if self.chair_at(tx, ty, dx, dy, b).is_some() {
                        self.spot(b, SpotKind::Seat, tx + dx, ty + dy);
                    }
                }
            }
            if let Some(p) = self.scatter(PKind::Sofa, ix, iy, lw, ih, 2, 1, b) {
                let (px, py) = (self.m.props[p].x, self.m.props[p].y);
                self.spot(b, SpotKind::Lounge, px, py);
                self.spot(b, SpotKind::Lounge, px + 1, py);
            } else if let Some(p) = self.scatter(PKind::Armchair, ix, iy, lw, ih, 1, 1, b) {
                let (px, py) = (self.m.props[p].x, self.m.props[p].y);
                self.spot(b, SpotKind::Lounge, px, py);
            }
            if self.year >= tv_year {
                self.scatter(PKind::Tv, ix, iy, lw, ih, 1, 1, b);
            } else if self.year >= 1925 && self.rng.chance(0.7) {
                self.scatter(PKind::RadioSet, ix, iy, lw, ih, 1, 1, b);
            }
            if self.rng.chance(0.5) {
                self.scatter(PKind::Rug, ix, iy, lw, ih, 2, 1, b);
            }
            if self.rng.chance(0.5) {
                self.scatter(PKind::Shelf, ix, iy, lw, ih, 1, 1, b);
            }
            if self.rng.chance(0.4) {
                self.scatter(PKind::Plant, ix, iy, iw, ih, 1, 1, b);
            }
            if self.year >= 1920 && self.rng.chance(0.5) {
                self.scatter(PKind::FloorLamp, ix, iy, lw, ih, 1, 1, b);
            }
        } else {
            // studio: everything in one room
            self.cook_at(b, ix, iy, iw, ih, facing_up);
            if let Some(p) = self.scatter(PKind::Bed, ix, iy, iw, ih, 1, 2, b) {
                let (px, py) = (self.m.props[p].x, self.m.props[p].y);
                self.spot(b, SpotKind::Bed, px, py);
                let (bw, bh) = (self.m.props[p].w, self.m.props[p].h);
                    self.place_wall(PKind::Nightstand, px - 1, py - 1, bw + 2, bh + 2, 1, 1, b);
            }
            if let Some(p) = self.scatter(PKind::Toilet, ix, iy, iw, ih, 1, 1, b) {
                let (px, py) = (self.m.props[p].x, self.m.props[p].y);
                self.spot(b, SpotKind::Toilet, px, py);
            }
            if let Some(t) = self.scatter(PKind::Table, ix, iy, iw, ih, 1, 1, b) {
                let (tx, ty) = (self.m.props[t].x, self.m.props[t].y);
                if self.chair_at(tx, ty, 1, 0, b).or_else(|| self.chair_at(tx, ty, -1, 0, b)).is_some() {
                    let cp = self.m.props.last().map(|p| (p.x, p.y)).unwrap_or((tx, ty));
                    self.spot(b, SpotKind::Seat, cp.0, cp.1);
                }
            }
            if let Some(p) = self.scatter(PKind::Armchair, ix, iy, iw, ih, 1, 1, b) {
                let (px, py) = (self.m.props[p].x, self.m.props[p].y);
                self.spot(b, SpotKind::Lounge, px, py);
            }
            self.scatter(PKind::Wardrobe, ix, iy, iw, ih, 1, 1, b);
            if self.year >= tv_year + 5 && self.rng.chance(0.6) {
                self.scatter(PKind::Tv, ix, iy, iw, ih, 1, 1, b);
            } else if self.year >= 1925 && self.rng.chance(0.4) {
                self.scatter(PKind::RadioSet, ix, iy, iw, ih, 1, 1, b);
            }
        }
    }

    fn furnish(&mut self, b: usize, facing_up: bool) {
        let bl = self.m.buildings[b].clone();
        let (ix, iy, iw, ih) = (bl.x + 1, bl.y + 1, bl.w - 2, bl.h - 2);
        // interior light positions: one per ~5 tiles
        let mut lights = Vec::new();
        let nx = (iw / 5).max(1);
        for i in 0..nx {
            lights.push(vec2(ix as f32 + (i as f32 + 0.5) * iw as f32 / nx as f32, iy as f32 + ih as f32 / 2.0));
        }
        self.m.buildings[b].lights = lights;
        let back = if facing_up { iy + ih - 1 } else { iy };
        match bl.kind {
            BKind::House | BKind::Safehouse => {
                self.home(b, ix, iy, iw, ih, facing_up, iw >= 6);
            }
            BKind::Apartment => {
                // several small flats, each a studio with its own bed, stove and toilet
                let units = (iw / 3).max(1);
                let uw = iw / units;
                for u in 0..units {
                    let ux = ix + u * uw;
                    if u > 0 {
                        let gap_y = if facing_up { iy } else { iy + ih - 1 };
                        for yy in iy..iy + ih {
                            if yy != gap_y {
                                self.m.set(ux, yy, Tile::Wall);
                            }
                        }
                    }
                    let sx = if u > 0 { ux + 1 } else { ux };
                    let sw = if u > 0 { uw - 1 } else { uw };
                    self.home(b, sx, iy, sw, ih, facing_up, false);
                }
            }
            BKind::Bar | BKind::Restaurant | BKind::Club | BKind::Cabaret => {
                // counter along the back wall
                let cw = (iw - 2).max(2).min(6);
                let cy = if facing_up { back - 1 } else { back + 1 };
                for xx in ix + 1..ix + 1 + cw {
                    if self.put(PKind::Counter, xx, cy, 1, 1, b).is_some() && xx == ix + 2 {
                        self.spot(b, SpotKind::Work, xx, back);
                    }
                }
                // back bar with bottles along the wall behind the counter
                for xx in (ix..ix + iw - 1).step_by(2) {
                    if let Some(p) = self.put(PKind::BottleShelf, xx, back, 2, 1, b) {
                        self.m.props[p].rot = if facing_up { 0 } else { 2 };
                    }
                }
                let sy = if facing_up { cy - 1 } else { cy + 1 };
                for xx in ix + 1..ix + 1 + cw {
                    if xx % 2 == 0 && self.put(PKind::Stool, xx, sy, 1, 1, b).is_some() {
                        self.spot(b, SpotKind::Seat, xx, sy);
                    }
                }
                if matches!(bl.kind, BKind::Club | BKind::Cabaret) {
                    if let Some(p) = self.scatter(PKind::Piano, ix, iy, iw, ih, 2, 1, b) {
                        let (px, py) = (self.m.props[p].x, self.m.props[p].y);
                        let sy2 = if self.free(px, py + 1) { py + 1 } else { py - 1 };
                        self.spot(b, SpotKind::Stage, px, sy2);
                    }
                }
                for _ in 0..(iw * ih / 10).max(1) {
                    if let Some(t) = self.scatter(PKind::Table, ix, iy, iw, ih, 1, 1, b) {
                        let (tx, ty) = (self.m.props[t].x, self.m.props[t].y);
                        for (dx, dy) in [(1, 0), (-1, 0)] {
                            if self.chair_at(tx, ty, dx, dy, b).is_some() {
                                self.spot(b, SpotKind::Seat, tx + dx, ty + dy);
                            }
                        }
                    }
                }
                // period details
                let yr = self.year;
                if bl.kind == BKind::Bar {
                    if yr >= 1950 {
                        if let Some(t) = self.scatter(PKind::PoolTable, ix, iy, iw, ih, 2, 1, b) {
                            let (tx, ty) = (self.m.props[t].x, self.m.props[t].y);
                            self.spot(b, SpotKind::Stand, tx, ty + 1);
                        }
                    } else if self.rng.chance(0.6) {
                        if let Some(p) = self.scatter(PKind::Piano, ix, iy, iw, ih, 2, 1, b) {
                            let (px, py) = (self.m.props[p].x, self.m.props[p].y);
                            let sy2 = if self.free(px, py + 1) { py + 1 } else { py - 1 };
                            self.spot(b, SpotKind::Stage, px, sy2);
                        }
                    }
                }
                if matches!(bl.kind, BKind::Bar | BKind::Restaurant) && yr >= 1940 && self.rng.chance(0.7) {
                    self.scatter(PKind::Jukebox, ix, iy, iw, ih, 1, 1, b);
                }
                if matches!(bl.kind, BKind::Club | BKind::Cabaret) {
                    if let Some(st) = self.place_wall(PKind::Stage, ix, iy, iw, ih, 3, 2, b) {
                        let (sx, sy3) = (self.m.props[st].x, self.m.props[st].y);
                        self.spot(b, SpotKind::Stage, sx + 1, sy3 + 1);
                        if yr >= 1925 {
                            self.put(PKind::DrumKit, sx + 2, sy3, 1, 1, b);
                        }
                    }
                }
                self.scatter(PKind::CoatRack, ix, iy, iw, ih, 1, 1, b);
                self.scatter(PKind::Plant, ix, iy, iw, ih, 1, 1, b);
                self.spot(b, SpotKind::Work, ix + iw - 1, iy + ih / 2);
            }
            BKind::Church => {
                let ay = if facing_up { back } else { iy };
                self.put(PKind::Altar, ix + iw / 2 - 1, ay, 2, 1, b);
                self.spot(b, SpotKind::Work, ix + iw / 2 + 1, ay);
                let rows: Vec<i32> = if facing_up { (iy + 1..back - 1).step_by(2).collect() } else { (iy + 2..iy + ih).step_by(2).collect() };
                for ry in rows {
                    for sx in [ix + 1, ix + iw / 2 + 1] {
                        let pw = (iw / 2 - 2).max(1);
                        if self.put(PKind::Pew, sx, ry, pw, 1, b).is_some() {
                            for k in 0..pw {
                                let sy = ry + if facing_up { -1 } else { -1 };
                                if self.m.get(sx + k, sy) == Tile::Floor {
                                    self.spot(b, SpotKind::Pray, sx + k, sy);
                                }
                            }
                        }
                    }
                }
            }
            BKind::Police => {
                // desks in front, cells in the back
                for i in 0..3 {
                    let dx = ix + 1 + i * 3;
                    if dx + 1 < ix + iw && self.put(PKind::Desk, dx, iy + ih / 2 - 1, 2, 1, b).is_some() {
                        self.spot(b, SpotKind::Work, dx, iy + ih / 2);
                    }
                }
                let cx0 = ix + iw - 5;
                self.put(PKind::CellBars, cx0, back, 4, 1, b);
                self.put(PKind::Board, ix, back, 2, 1, b);
                self.put(PKind::Shelf, ix + 3, back, 2, 1, b);
                for _ in 0..3 {
                    self.scatter(PKind::FilingCabinet, ix, iy, iw, ih, 1, 1, b);
                }
                self.scatter(PKind::CoatRack, ix, iy, iw, ih, 1, 1, b);
                self.spot(b, SpotKind::Guard, ix + iw - 2, iy + ih / 2);
            }
            BKind::Hospital => {
                let at = ix + iw - 5;
                self.partition_v(b, at);
                for i in 0..((at - ix) / 2) {
                    let bx = ix + i * 2;
                    if let Some(p) = self.put(PKind::Bed, bx, back.min(iy + ih - 2), 1, 2, b) {
                        let _ = p;
                    }
                }
                // morgue
                self.put(PKind::Slab, at + 1, iy + 1, 1, 2, b);
                self.put(PKind::Slab, at + 3, iy + 1, 1, 2, b);
                self.put(PKind::Shelf, at + 1, iy + ih - 1, 2, 1, b);
                self.spot(b, SpotKind::Work, ix + 1, iy + ih / 2);
                self.spot(b, SpotKind::Work, at + 2, iy + ih - 2);
            }
            BKind::Hotel => {
                // lobby at front, rooms at back
                let front = if facing_up { iy } else { iy + ih - 1 };
                self.put(PKind::Counter, ix + 1, if facing_up { front + 2 } else { front - 2 }, 3, 1, b);
                self.spot(b, SpotKind::Work, ix + 2, if facing_up { front + 3 } else { front - 3 });
                let ry = if facing_up { iy + 4 } else { iy };
                let rh = ih - 4;
                if rh >= 1 {
                    self.partition_h(b, if facing_up { ry - 1 } else { ry + rh }, ix, ix + iw);
                    let mut x = ix;
                    while x + 3 <= ix + iw {
                        if let Some(p) = self.put(PKind::Bed, x + 1, ry, 1, 2.min(rh), b) {
                            let (px, py) = (self.m.props[p].x, self.m.props[p].y);
                            self.spot(b, SpotKind::Bed, px, py);
                        }
                        x += 3;
                    }
                }
                // reception: key board behind the desk; lobby with rug, armchairs, lamps; stairs up
                let kr_y = if facing_up { front + 3 } else { front - 3 };
                if let Some(k) = self.put(PKind::KeyRack, ix + 1, kr_y, 3, 1, b) {
                    self.m.props[k].rot = if facing_up { 0 } else { 2 };
                }
                self.scatter(PKind::Sofa, ix, iy, iw, ih, 2, 1, b);
                self.scatter(PKind::Armchair, ix, iy, iw, ih, 1, 1, b);
                self.scatter(PKind::Armchair, ix, iy, iw, ih, 1, 1, b);
                self.scatter(PKind::Rug, ix + iw / 2, iy, iw / 2, ih, 2, 2, b);
                self.place_wall(PKind::Stairs, ix, iy, iw, ih, 1, 3, b);
                self.scatter(PKind::Plant, ix, iy, iw, ih, 1, 1, b);
                self.scatter(PKind::Plant, ix, iy, iw, ih, 1, 1, b);
                self.scatter(PKind::FloorLamp, ix, iy, iw, ih, 1, 1, b);
                self.scatter(PKind::CoatRack, ix, iy, iw, ih, 1, 1, b);
            }
            BKind::Market => {}
            BKind::Clothing | BKind::General | BKind::GunShop | BKind::Pharmacy | BKind::Pawn => {
                let cy = if facing_up { back - 1 } else { back + 1 };
                self.put(PKind::Counter, ix + 1, cy, (iw - 2).max(1).min(4), 1, b);
                self.spot(b, SpotKind::Work, ix + 2, back);
                for xx in (ix..ix + iw).step_by(2) {
                    let sy = if facing_up { back } else { back };
                    if xx != ix + 2 {
                        self.put(PKind::Shelf, xx, sy, 1, 1, b);
                    }
                }
                if bl.kind == BKind::Clothing {
                    self.scatter(PKind::Mirror, ix, iy, iw, ih, 1, 1, b);
                    for _ in 0..3 {
                        self.scatter(PKind::ClothesRack, ix, iy, iw, ih, 2, 1, b);
                    }
                    self.scatter(PKind::CoatRack, ix, iy, iw, ih, 1, 1, b);
                }
                self.scatter(PKind::Plant, ix, iy, iw, ih, 1, 1, b);
                if bl.kind == BKind::Pawn {
                    self.scatter(PKind::Safe, ix, iy, iw, ih, 1, 1, b);
                }
            }
            BKind::Newspaper | BKind::Office | BKind::Bank | BKind::Radio => {
                for i in 0..(iw / 3) {
                    let dx = ix + i * 3;
                    if self.put(PKind::Desk, dx, iy + ih / 2 - 1, 2, 1, b).is_some() {
                        self.spot(b, SpotKind::Work, dx, iy + ih / 2);
                        if bl.kind == BKind::Newspaper {
                            let _ = self.put(PKind::Typewriter, dx + 1, iy + ih / 2 - 1, 1, 1, b);
                        }
                        if self.rng.chance(0.6) {
                            self.scatter(PKind::FilingCabinet, ix, iy, iw, ih, 1, 1, b);
                        }
                    }
                }
                if bl.kind == BKind::Bank {
                    self.scatter(PKind::Safe, ix, back, iw, 1, 1, 1, b);
                }
                if bl.kind == BKind::Radio {
                    self.scatter(PKind::RadioSet, ix, back, iw, 1, 2, 1, b);
                }
                self.scatter(PKind::Shelf, ix, back, iw, 1, 2, 1, b);
            }
            BKind::Warehouse | BKind::Factory | BKind::Barn | BKind::Abandoned => {
                let n = iw * ih / 7;
                for _ in 0..n {
                    let k = match bl.kind {
                        BKind::Barn => PKind::Hay,
                        _ => {
                            if self.rng.chance(0.6) {
                                PKind::Crate
                            } else {
                                PKind::Barrel
                            }
                        }
                    };
                    self.scatter(k, ix, iy, iw, ih, 1, 1, b);
                }
                self.spot(b, SpotKind::Work, ix + 1, iy + 1);
                self.spot(b, SpotKind::Work, ix + iw - 2, iy + ih - 2);
            }
            BKind::Mansion => {
                let at = ix + iw / 2;
                self.partition_v(b, at);
                self.partition_v(b, ix + iw / 4);
                for _ in 0..2 {
                    if let Some(p) = self.scatter(PKind::Bed, at + 1, iy, ix + iw - at - 1, ih, 2, 2, b) {
                        let (px, py) = (self.m.props[p].x, self.m.props[p].y);
                        self.spot(b, SpotKind::Bed, px, py);
                        self.spot(b, SpotKind::Bed, px + 1, py);
                    }
                }
                self.scatter(PKind::Wardrobe, at + 1, iy, ix + iw - at - 1, ih, 1, 1, b);
                self.scatter(PKind::Piano, ix + iw / 4 + 1, iy, at - ix - iw / 4 - 1, ih, 2, 1, b);
                self.scatter(PKind::Rug, ix + iw / 4 + 1, iy, at - ix - iw / 4 - 1, ih, 2, 2, b);
                self.scatter(PKind::Sofa, ix + iw / 4 + 1, iy, at - ix - iw / 4 - 1, ih, 2, 1, b);
                if let Some(t) = self.scatter(PKind::Table, ix, iy, iw / 4, ih, 1, 2, b) {
                    let (tx, ty) = (self.m.props[t].x, self.m.props[t].y);
                    for (dx, dy) in [(1, 0), (-1, 0), (1, 1), (-1, 1)] {
                        if self.chair_at(tx, ty, dx, dy, b).is_some() {
                            self.spot(b, SpotKind::Seat, tx + dx, ty + dy);
                        }
                    }
                }
                self.scatter(PKind::Safe, ix, iy, iw, ih, 1, 1, b);
                self.scatter(PKind::Desk, ix, iy, iw, ih, 2, 1, b);
                // bathroom corner and the kitchen range
                self.bath_corner(b, at + 1, iy, ix + iw - at - 1, ih, facing_up);
                self.cook_at(b, ix, iy, iw / 4, ih, facing_up);
                self.scatter(PKind::Armchair, ix + iw / 4 + 1, iy, at - ix - iw / 4 - 1, ih, 1, 1, b).map(|p| {
                    let (px, py) = (self.m.props[p].x, self.m.props[p].y);
                    self.spot(b, SpotKind::Seat, px, py);
                });
                self.scatter(PKind::FloorLamp, ix, iy, iw, ih, 1, 1, b);
                self.spot(b, SpotKind::Work, ix + 1, iy + 1);
            }
            BKind::Station => {
                for i in 0..(iw / 4) {
                    let px = ix + 1 + i * 4;
                    if self.put(PKind::Bench, px, iy + ih / 2, 2, 1, b).is_some() {
                        self.spot(b, SpotKind::Seat, px, iy + ih / 2 + 1);
                    }
                }
                self.put(PKind::Counter, ix + iw - 4, iy + 1, 3, 1, b);
                self.spot(b, SpotKind::Work, ix + iw - 3, iy + 1 + if facing_up { 1 } else { 1 });
                self.scatter(PKind::Phone, ix, iy, iw, ih, 1, 1, b);
            }
            BKind::Farmhouse => {
                let at = ix + iw / 2;
                self.partition_v(b, at);
                for _ in 0..3 {
                    if let Some(p) = self.scatter(PKind::Bed, at + 1, iy, ix + iw - at - 1, ih, 1, 2, b) {
                        let (px, py) = (self.m.props[p].x, self.m.props[p].y);
                        self.spot(b, SpotKind::Bed, px, py);
                    }
                }
                self.cook_at(b, ix, iy, at - ix, ih, facing_up);
                self.bath_corner(b, at + 1, iy, ix + iw - at - 1, ih, facing_up);
                if let Some(t) = self.scatter(PKind::Table, ix, iy, at - ix, ih, 1, 2, b) {
                    let (tx, ty) = (self.m.props[t].x, self.m.props[t].y);
                    for (dx, dy) in [(1, 0), (-1, 0), (1, 1), (-1, 1)] {
                        if self.chair_at(tx, ty, dx, dy, b).is_some() {
                            self.spot(b, SpotKind::Seat, tx + dx, ty + dy);
                        }
                    }
                }
                self.scatter(PKind::Wardrobe, ix, iy, iw, ih, 1, 1, b);
            }
            BKind::Lab => {}
        }
        // guarantee at least one standing spot
        if self.m.buildings[b].spots.is_empty() {
            self.spot(b, SpotKind::Stand, ix + iw / 2, iy + ih / 2);
        }
    }


    fn park(&mut self, x0: i32, y0: i32, w: i32, h: i32) {
        for y in y0..y0 + h {
            for x in x0..x0 + w {
                self.m.set(x, y, Tile::Grass);
            }
        }
        for x in x0..x0 + w {
            self.m.set(x, y0 + h / 2, Tile::Gravel);
        }
        for y in y0..y0 + h {
            self.m.set(x0 + w / 2, y, Tile::Gravel);
        }
        self.m.add_prop(PKind::Fountain, x0 + w / 2 - 1, y0 + h / 2 - 1, 3, 3, None);
        for _ in 0..16 {
            let x = x0 + self.rng.range(0, w);
            let y = y0 + self.rng.range(0, h);
            if self.m.get(x, y) == Tile::Grass && !self.m.props.iter().any(|p| (p.x - x).abs() < 2 && (p.y - y).abs() < 2) {
                self.m.add_prop(PKind::Tree, x, y, 1, 1, None);
            }
        }
        for (bx, by) in [(x0 + 2, y0 + h / 2 - 1), (x0 + w - 4, y0 + h / 2 + 1), (x0 + w / 2 + 2, y0 + 1)] {
            self.m.add_prop(PKind::Bench, bx, by, 2, 1, None);
        }
        let name = match self.city {
            CityId::NewOrleans => "Praça Congo",
            CityId::London => "Victoria Park",
            CityId::Berlin => "Tiergarten",
            CityId::Bergen => "Byparken",
            CityId::NewYork => "Tompkins Square",
            CityId::LosAngeles => "MacArthur Park",
            CityId::SanFrancisco => "Golden Gate Panhandle",
            CityId::Chicago => "Washington Square",
            _ => "Parque Central",
        };
        self.m.districts.push(District { name: name.into(), x: x0, y: y0, w, h, wealth: 0.5, danger: 0.3 });
    }

    fn cemetery(&mut self, x0: i32, y0: i32, w: i32, h: i32) {
        for y in y0..y0 + h {
            for x in x0..x0 + w {
                let edge = x == x0 || y == y0 || x == x0 + w - 1 || y == y0 + h - 1;
                self.m.set(x, y, if edge { Tile::Fence } else { Tile::Gravel });
            }
        }
        self.m.set(x0 + w / 2, y0, Tile::Gravel);
        self.m.set(x0 + w / 2, y0 + h - 1, Tile::Gravel);
        for y in (y0 + 2..y0 + h - 2).step_by(3) {
            for x in (x0 + 2..x0 + w - 2).step_by(2) {
                if x != x0 + w / 2 {
                    self.m.add_prop(PKind::Grave, x, y, 1, 2, None);
                }
            }
        }
        self.m.add_prop(PKind::Tree, x0 + 1, y0 + 1, 1, 1, None);
        self.m.add_prop(PKind::Tree, x0 + w - 2, y0 + h - 2, 1, 1, None);
        let name = match self.city {
            CityId::NewOrleans => "Cemitério St. Louis Nº 1",
            CityId::Bavaria => "Friedhof",
            CityId::London => "Cemitério de Bow",
            _ => "Cemitério Municipal",
        };
        self.m.districts.push(District { name: name.into(), x: x0, y: y0, w, h, wealth: 0.2, danger: 0.5 });
    }

    fn plaza(&mut self, x0: i32, y0: i32, w: i32, h: i32) {
        for y in y0..y0 + h {
            for x in x0..x0 + w {
                self.m.set(x, y, Tile::Plaza);
            }
        }
        let id = self.m.buildings.len();
        let name = self.building_name(BKind::Market);
        self.m.buildings.push(Building {
            id,
            kind: BKind::Market,
            name,
            x: x0,
            y: y0,
            w,
            h,
            doors: vec![(x0 + w / 2, y0)],
            spots: Vec::new(),
            lights: vec![vec2(x0 as f32 + w as f32 / 2.0, y0 as f32 + h as f32 / 2.0)],
            district: self.m.district_at(vec2(x0 as f32, y0 as f32)).unwrap_or(0),
            address: if self.s.arch == Arch::Village { "Marktplatz".into() } else { "Praça do Mercado".into() },
            residents: Vec::new(),
            owner: None,
            closed_forever: false,
            locked: false,
        });
        if self.s.arch == Arch::Village {
            self.m.add_prop(PKind::Fountain, x0 + w / 2 - 1, y0 + h / 2 - 1, 3, 3, None);
            self.m.add_prop(PKind::Trough, x0 + 2, y0 + 2, 2, 1, None);
        }
        for y in (y0 + 1..y0 + h - 1).step_by(4) {
            for x in (x0 + 1..x0 + w - 2).step_by(4) {
                if self.s.arch == Arch::Village && (x - x0 - w / 2).abs() < 3 && (y - y0 - h / 2).abs() < 3 {
                    continue;
                }
                self.m.add_prop(PKind::Stall, x, y, 2, 1, Some(id));
                self.m.buildings[id].spots.push(Spot { kind: SpotKind::Work, pos: tile_center(x, y + 1), owner: None });
                self.m.buildings[id].spots.push(Spot { kind: SpotKind::Stand, pos: tile_center(x + 1, (y - 1).max(y0)), owner: None });
            }
        }
    }

    /// Bombed-out block (London Blitz, Berlin): broken walls, rubble, craters.
    fn ruins(&mut self, x0: i32, y0: i32, w: i32, h: i32) {
        for y in y0..y0 + h {
            for x in x0..x0 + w {
                self.m.set(x, y, Tile::Dirt);
            }
        }
        for _ in 0..(w * h / 6) {
            let x = x0 + self.rng.range(0, w);
            let y = y0 + self.rng.range(0, h);
            let r = self.rng.f();
            if r < 0.35 {
                self.m.set(x, y, Tile::Wall);
            } else if !self.m.props.iter().any(|p| p.x == x && p.y == y) {
                self.m.add_prop(PKind::Rubble, x, y, 1, 1, None);
            }
        }
        let name = if self.city == CityId::London { "Quarteirão bombardeado" } else { "Ruínas" };
        self.m.districts.push(District { name: name.into(), x: x0, y: y0, w, h, wealth: 0.0, danger: 0.8 });
    }

    fn outskirts(&mut self) {
        let ry = self.cy0() + self.g.py * (self.g.by / 2) + 1;
        let fx = 6;
        let fy = (ry - 10).max(RAIL + 1);
        let fid = self.building(BKind::Farmhouse, (fx, fy, 10, 8), false);
        let (dx, dy) = self.m.buildings[fid].doors[0];
        for y in dy + 1..ry {
            self.m.set(dx, y, Tile::Dirt);
        }
        self.building(BKind::Barn, (fx + 12, fy + 1, 8, 7), false);
        let (bdx, bdy) = self.m.buildings.last().unwrap().doors[0];
        for y in bdy + 1..ry {
            self.m.set(bdx, y, Tile::Dirt);
        }
        let wh = water_h(&self.s);
        let (n_trees, zone_name) = match self.s.outskirts {
            Outskirts::Forest | Outskirts::Snow => (320, "Floresta"),
            Outskirts::Hills => (140, "Colinas"),
            Outskirts::Beach => (40, "Praia"),
            Outskirts::Fields => (220, "Bosque"),
        };
        for _ in 0..n_trees {
            let x = self.rng.range(1, self.cx0() - 2);
            let y = self.rng.range(ry + 4, self.m.h - wh - 2);
            if matches!(self.m.get(x, y), Tile::Grass | Tile::Field) {
                self.m.set(x, y, Tile::Grass);
                if !self.m.props.iter().any(|p| p.kind == PKind::Tree && (p.x - x).abs() < 2 && (p.y - y).abs() < 2) {
                    self.m.add_prop(PKind::Tree, x, y, 1, 1, None);
                }
            }
        }
        for _ in 0..30 {
            let x = self.rng.range(1, self.cx0() - 2);
            let y = self.rng.range(RAIL + 1, ry - 1);
            if self.m.get(x, y) == Tile::Grass && self.m.building_at_tile(x, y).is_none() {
                self.m.add_prop(PKind::Tree, x, y, 1, 1, None);
            }
        }
        self.m.districts.push(District { name: zone_name.into(), x: 0, y: ry + 3, w: self.cx0(), h: self.m.h - wh - ry - 3, wealth: 0.1, danger: 0.6 });
        for i in 0..3 {
            let x = fx + 12 + i * 2;
            let y = fy + 9;
            if self.m.get(x, y) != Tile::Dirt {
                self.m.add_prop(PKind::Hay, x, y, 1, 1, None);
            }
        }
        if self.s.horses > 0.0 {
            self.m.add_prop(PKind::Trough, fx + 3, fy + 9, 2, 1, None);
        }
    }

    fn water_features(&mut self) {
        let wh = water_h(&self.s);
        match self.s.water {
            Water::Bottom | Water::Fjord => {
                let wy = self.m.h - wh;
                let step = if self.s.water == Water::Fjord { 12 } else { 26 };
                let mut i = 0;
                loop {
                    let x = self.cx0() + 6 + i * step;
                    if x + 3 >= self.m.w {
                        break;
                    }
                    let len = if self.s.water == Water::Fjord { 5 } else { 8 };
                    for y in wy..wy + len {
                        for dx in 0..3 {
                            self.m.set(x + dx, y, Tile::Dock);
                        }
                    }
                    self.m.add_prop(PKind::Crate, x, wy + len - 2, 1, 1, None);
                    self.m.add_prop(PKind::Barrel, x + 2, wy + len - 3, 1, 1, None);
                    i += 1;
                }
                let x_from = if self.s.outskirts == Outskirts::Beach { self.cx0() } else { self.cx0() };
                for x in x_from..self.m.w {
                    self.m.set(x, wy - 1, Tile::Dock);
                }
            }
            Water::Right => {
                // lake shore: a promenade along the water
                let cw = self.cx0() + self.city_w();
                for y in RAIL..self.m.h - wh {
                    self.m.set(cw + 2, y, Tile::Dock);
                }
                for y in (RAIL + 4..self.m.h - wh).step_by(18) {
                    for x in cw + 3..cw + 9 {
                        self.m.set(x, y, Tile::Dock);
                        self.m.set(x, y + 1, Tile::Dock);
                    }
                }
            }
            Water::None => {}
        }
    }

    fn special_features(&mut self) {
        if self.s.rail {
            for x in 0..self.m.w {
                if self.m.get(x, 1) != Tile::Water {
                    self.m.set(x, 1, Tile::Rail);
                    self.m.set(x, 2, Tile::Rail);
                }
            }
        }
        // Chicago's elevated train: pillars along one avenue
        if self.s.elevated {
            let x = self.cx0() + self.g.px * (self.g.bx / 2) + 1;
            for y in (RAIL + 2..self.m.h - water_h(&self.s) - 2).step_by(5) {
                if self.m.get(x, y) == Tile::Road {
                    self.m.add_prop(PKind::Pillar, x, y, 1, 1, None);
                    self.m.add_prop(PKind::Pillar, x + 2, y, 1, 1, None);
                }
            }
            self.m.elevated = Some(x);
        }
        // Berlin: the sector border with a checkpoint
        if self.s.wall {
            let x = self.cx0() + self.g.px * (self.g.bx / 2 + 1) + 2;
            let gate = self.cy0() + self.g.py * (self.g.by / 2) + 1;
            for y in RAIL..self.m.h - water_h(&self.s) {
                if (y - gate).abs() <= 1 {
                    continue;
                }
                if matches!(self.m.get(x, y), Tile::Road | Tile::Sidewalk | Tile::Alley | Tile::Gravel | Tile::Dirt | Tile::Grass) {
                    self.m.set(x, y, Tile::Fence);
                }
            }
            self.m.add_prop(PKind::Barrier, x - 1, gate - 1, 1, 1, None);
            self.m.add_prop(PKind::Barrier, x + 1, gate + 1, 1, 1, None);
            self.m.districts.push(District { name: "Checkpoint".into(), x: x - 3, y: gate - 3, w: 7, h: 7, wealth: 0.3, danger: 0.7 });
        }
        // London: sandbags in front of public buildings, air raid shelters
        if self.city == CityId::London && (1939..1946).contains(&self.year) {
            for b in self.m.buildings.clone().iter().filter(|b| matches!(b.kind, BKind::Police | BKind::Hospital | BKind::Station | BKind::Bank)) {
                let p = b.outside_px();
                let (x, y) = to_tile(p);
                for dx in [-2, 2] {
                    if matches!(self.m.get(x + dx, y), Tile::Sidewalk) {
                        self.m.add_prop(PKind::Sandbags, x + dx, y, 1, 1, None);
                    }
                }
            }
        }
    }

    fn street_furniture(&mut self) {
        let st = self.g.st;
        let wh = water_h(&self.s);
        for by in 0..self.g.by {
            for bx in 0..self.g.bx {
                let x0 = self.cx0() + bx * self.g.px + st;
                let y0 = self.cy0() + by * self.g.py + st;
                let x1 = x0 + self.g.px - st - 1;
                let y1 = y0 + self.g.py - st - 1;
                let lamp_ok = self.year >= 1880;
                let pts = [(x0, y0), (x1, y0), (x0, y1), (x1, y1), ((x0 + x1) / 2, y0), ((x0 + x1) / 2, y1)];
                for (x, y) in pts {
                    if lamp_ok && matches!(self.m.get(x, y), Tile::Sidewalk | Tile::Grass) && self.m.building_at_tile(x, y).is_none() && self.m.prop_at_slow(x, y) {
                        // villages have far fewer lamps
                        if self.s.arch == Arch::Village && (x + y) % 3 != 0 {
                            continue;
                        }
                        self.m.lamps.push(tile_center(x, y));
                        self.m.add_prop(PKind::LampPost, x, y, 1, 1, None);
                    }
                }
                // street trees: palms in LA, eucalyptus in Adelaide...
                if matches!(self.s.tree, TreeKind::Palm | TreeKind::Eucalyptus) || self.s.arch == Arch::Colonial {
                    for x in (x0 + 3..x1 - 1).step_by(5) {
                        if self.m.get(x, y1) == Tile::Sidewalk && self.m.prop_at_slow(x, y1) {
                            self.m.add_prop(PKind::Tree, x, y1, 1, 1, None);
                        }
                    }
                }
                if self.rng.chance(0.5) {
                    let (x, y) = (x0 + 1 + self.rng.range(0, self.g.px - st - 3), y0 + (self.g.py - st) / 2);
                    if self.m.get(x, y) == Tile::Alley {
                        self.m.add_prop(PKind::Dumpster, x, y, 1, 1, None);
                    }
                }
                let x = (x0 + x1) / 2 + 3;
                if self.rng.chance(0.3) && self.m.get(x, y1) == Tile::Sidewalk && self.m.prop_at_slow(x, y1) {
                    let k = match self.rng.idx(4) {
                        0 => PKind::NewsStand,
                        1 => PKind::Phone,
                        2 if self.year >= 1920 => PKind::Hydrant,
                        _ => {
                            if self.s.horses > 0.0 {
                                PKind::Trough
                            } else {
                                PKind::Hydrant
                            }
                        }
                    };
                    self.m.add_prop(k, x, y1, 1, 1, None);
                }
            }
        }
        if matches!(self.s.water, Water::Bottom | Water::Fjord) {
            let wy = self.m.h - wh - 1;
            for x in (self.cx0()..self.m.w).step_by(9) {
                self.m.lamps.push(tile_center(x, wy));
            }
        }
        let ry = self.cy0() + self.g.py * (self.g.by / 2) + 1;
        for x in (4..self.cx0()).step_by(10) {
            if self.m.get(x, ry - 1) == Tile::Grass || self.m.get(x, ry - 1) == Tile::Field {
                self.m.lamps.push(tile_center(x, ry - 1));
                self.m.add_prop(PKind::LampPost, x, ry - 1, 1, 1, None);
            }
        }
    }
}

impl Map {
    /// Slow check used during generation (the prop index is built at the end).
    pub fn prop_at_slow(&self, x: i32, y: i32) -> bool {
        !self.props.iter().any(|p| x >= p.x && y >= p.y && x < p.x + p.w && y < p.y + p.h)
    }
}
