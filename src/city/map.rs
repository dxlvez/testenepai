//! Tile world: tiles, buildings, props, spots. Shared by the procedural cities
//! and the handcrafted places (Elias' apartment, the laboratory).

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub const TS: f32 = 1.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Tile {
    Void,
    Water,
    Road,
    Sidewalk,
    Alley,
    Grass,
    Dirt,
    Dock,
    Rail,
    Field,
    Floor,
    Wall,
    Window,
    Door,
    Fence,
    Gravel,
    Plaza,
    Sand,
}

impl Tile {
    pub fn solid(self) -> bool {
        matches!(self, Tile::Void | Tile::Water | Tile::Wall | Tile::Window | Tile::Fence)
    }
    pub fn walk_cost(self) -> f32 {
        match self {
            Tile::Sidewalk | Tile::Floor | Tile::Door | Tile::Plaza => 1.0,
            Tile::Road => 1.6,
            Tile::Alley | Tile::Dirt | Tile::Gravel | Tile::Dock => 1.3,
            Tile::Grass | Tile::Field | Tile::Sand => 1.5,
            Tile::Rail => 2.0,
            _ => 99.0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Hash)]
pub enum BKind {
    House,
    Apartment,
    Bar,
    Club,
    Restaurant,
    Police,
    Hospital,
    Church,
    Hotel,
    Market,
    Clothing,
    General,
    GunShop,
    Pharmacy,
    Newspaper,
    Bank,
    Warehouse,
    Mansion,
    Cabaret,
    Station,
    Radio,
    Factory,
    Pawn,
    Safehouse,
    Lab,
    Farmhouse,
    Barn,
    Abandoned,
    Office,
}

impl BKind {
    pub fn label(self) -> &'static str {
        use BKind::*;
        match self {
            House => "Casa",
            Apartment => "Cortiço",
            Bar => "Bar",
            Club => "Clube",
            Restaurant => "Restaurante",
            Police => "Delegacia",
            Hospital => "Hospital",
            Church => "Igreja",
            Hotel => "Hotel",
            Market => "Mercado",
            Clothing => "Alfaiataria",
            General => "Armazém",
            GunShop => "Armaria",
            Pharmacy => "Farmácia",
            Newspaper => "Jornal",
            Bank => "Banco",
            Warehouse => "Depósito",
            Mansion => "Mansão",
            Cabaret => "Casa Noturna",
            Station => "Estação",
            Radio => "Rádio",
            Factory => "Fábrica",
            Pawn => "Casa de Penhores",
            Safehouse => "Esconderijo",
            Lab => "Laboratório",
            Farmhouse => "Fazenda",
            Barn => "Celeiro",
            Abandoned => "Prédio Abandonado",
            Office => "Escritório",
        }
    }
    /// Public places anyone may enter during opening hours.
    pub fn public(self) -> bool {
        use BKind::*;
        matches!(
            self,
            Bar | Club | Restaurant | Police | Hospital | Church | Hotel | Market | Clothing | General | GunShop
                | Pharmacy | Newspaper | Bank | Cabaret | Station | Radio | Pawn | Office
        )
    }
    pub fn open_hours(self) -> (i32, i32) {
        use BKind::*;
        match self {
            Bar => (11, 26),
            Club | Cabaret => (19, 28),
            Restaurant => (7, 23),
            Police | Hospital | Station | Hotel => (0, 24),
            Church => (6, 21),
            Market => (5, 17),
            _ => (8, 19),
        }
    }
    pub fn is_open(self, hour: f32) -> bool {
        let (o, c) = self.open_hours();
        let h = hour as i32;
        if c > 24 {
            h >= o || h < c - 24
        } else {
            h >= o && h < c
        }
    }
    pub fn floor_color(self) -> [f32; 3] {
        use BKind::*;
        match self {
            Police | Hospital | Lab | Station | Bank | Office => [0.30, 0.31, 0.32],
            Church => [0.30, 0.27, 0.25],
            Warehouse | Factory | Barn | Abandoned => [0.24, 0.22, 0.20],
            Club | Cabaret => [0.20, 0.10, 0.14],
            Mansion | Hotel => [0.33, 0.20, 0.16],
            Market => [0.30, 0.28, 0.24],
            _ => [0.33, 0.24, 0.17],
        }
    }
    pub fn roof_color(self) -> [f32; 3] {
        use BKind::*;
        match self {
            Police => [0.16, 0.18, 0.24],
            Church => [0.22, 0.20, 0.23],
            Hospital => [0.24, 0.24, 0.26],
            Mansion => [0.20, 0.13, 0.14],
            Warehouse | Factory | Barn => [0.19, 0.17, 0.15],
            Club | Cabaret | Bar => [0.16, 0.11, 0.16],
            Abandoned => [0.12, 0.12, 0.12],
            _ => [0.18, 0.15, 0.17],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum PKind {
    Bed,
    Table,
    Chair,
    Stool,
    Counter,
    Shelf,
    Wardrobe,
    Stove,
    Piano,
    Pew,
    Altar,
    Desk,
    CellBars,
    Crate,
    Barrel,
    Stall,
    Sofa,
    RadioSet,
    Bathtub,
    Slab,
    LampPost,
    Tree,
    Bench,
    Grave,
    Fountain,
    Dumpster,
    Phone,
    NewsStand,
    Hay,
    Rug,
    Plant,
    Safe,
    Board,
    Sphere,
    Typewriter,
    Mirror,
    Car,
    Rubble,
    Trough,
    Pillar,
    Hydrant,
    Barrier,
    Sandbags,
    Toilet,
    Sink,
    Nightstand,
    Armchair,
    Tv,
    Fridge,
    FloorLamp,
}

impl PKind {
    /// Pieces whose model has the back on the local -z side (beds: headboard, chairs: backrest).
    pub fn back_neg_z(self) -> bool {
        matches!(self, PKind::Bed | PKind::Chair | PKind::Bathtub | PKind::Armchair | PKind::Pew | PKind::Piano)
    }
    pub fn solid(self) -> bool {
        use PKind::*;
        !matches!(self, Rug | Chair | Stool | Grave | Sphere | Car)
    }
    pub fn container(self) -> bool {
        use PKind::*;
        matches!(self, Wardrobe | Dumpster | Crate | Barrel | Bed | Hay | Safe | Desk | Shelf | Slab | Nightstand | Fridge | Sink)
    }
    pub fn hides_body(self) -> bool {
        use PKind::*;
        matches!(self, Wardrobe | Dumpster | Crate | Bed | Hay | Slab | Bathtub)
    }
    pub fn label(self) -> &'static str {
        use PKind::*;
        match self {
            Bed => "cama",
            Table => "mesa",
            Chair => "cadeira",
            Stool => "banqueta",
            Counter => "balcão",
            Shelf => "estante",
            Wardrobe => "armário",
            Stove => "fogão",
            Piano => "piano",
            Pew => "banco da igreja",
            Altar => "altar",
            Desk => "escrivaninha",
            CellBars => "cela",
            Crate => "caixote",
            Barrel => "barril",
            Stall => "barraca",
            Sofa => "sofá",
            RadioSet => "rádio",
            Bathtub => "banheira",
            Slab => "mesa do necrotério",
            LampPost => "poste",
            Tree => "árvore",
            Bench => "banco",
            Grave => "túmulo",
            Fountain => "chafariz",
            Dumpster => "caçamba de lixo",
            Phone => "telefone",
            NewsStand => "banca de jornal",
            Hay => "feno",
            Rug => "tapete",
            Plant => "planta",
            Safe => "cofre",
            Board => "quadro de cortiça",
            Sphere => "esfera",
            Typewriter => "máquina de escrever",
            Mirror => "espelho",
            Car => "carro",
            Rubble => "escombros",
            Trough => "bebedouro de cavalos",
            Pillar => "pilar do trem elevado",
            Hydrant => "hidrante",
            Barrier => "cancela",
            Sandbags => "sacos de areia",
            Toilet => "vaso sanitário",
            Sink => "pia",
            Nightstand => "criado-mudo",
            Armchair => "poltrona",
            Tv => "televisão",
            Fridge => "geladeira",
            FloorLamp => "abajur",
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Prop {
    pub kind: PKind,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub building: Option<usize>,
    pub tint: f32,
    /// Items stored inside (item ids) — containers only.
    pub items: Vec<u32>,
    /// Bodies hidden inside (person ids).
    pub bodies: Vec<u32>,
    /// Which way the back of the piece faces (0 = +z, 1 = +x, 2 = -z, 3 = -x):
    /// furniture stands with its back against a wall.
    #[serde(default)]
    pub rot: u8,
}

impl Prop {
    pub fn center(&self) -> Vec2 {
        vec2((self.x as f32 + self.w as f32 / 2.0) * TS, (self.y as f32 + self.h as f32 / 2.0) * TS)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum SpotKind {
    Work,
    Seat,
    Bed,
    Stand,
    Pray,
    Stage,
    Guard,
    /// at home: stove, toilet, sink/tub, sofa
    Cook,
    Toilet,
    Wash,
    Lounge,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Spot {
    pub kind: SpotKind,
    pub pos: Vec2,
    /// Index of the person who "owns" this spot (bed of a resident, etc.)
    pub owner: Option<u32>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Building {
    pub id: usize,
    pub kind: BKind,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub doors: Vec<(i32, i32)>,
    pub spots: Vec<Spot>,
    pub lights: Vec<Vec2>,
    pub district: usize,
    pub address: String,
    /// residents (person ids)
    pub residents: Vec<u32>,
    pub owner: Option<u32>,
    pub closed_forever: bool,
    pub locked: bool,
}

impl Building {
    pub fn rect_px(&self) -> Rect {
        Rect::new(self.x as f32 * TS, self.y as f32 * TS, (self.x + self.w) as f32 * TS, (self.y + self.h) as f32 * TS)
    }
    pub fn contains_tile(&self, x: i32, y: i32) -> bool {
        x >= self.x && y >= self.y && x < self.x + self.w && y < self.y + self.h
    }
    pub fn inner_contains(&self, p: Vec2) -> bool {
        let r = self.rect_px();
        p.x > r.min.x + 0.06 && p.y > r.min.y + 0.06 && p.x < r.max.x - 0.06 && p.y < r.max.y - 0.06
    }
    pub fn door_px(&self) -> Vec2 {
        let (dx, dy) = self.doors[0];
        vec2(dx as f32 * TS + TS / 2.0, dy as f32 * TS + TS / 2.0)
    }
    /// A tile just outside the main door.
    pub fn outside_px(&self) -> Vec2 {
        let (dx, dy) = self.doors[0];
        let (ox, oy) = if dy == self.y {
            (0, -1)
        } else if dy == self.y + self.h - 1 {
            (0, 1)
        } else if dx == self.x {
            (-1, 0)
        } else {
            (1, 0)
        };
        vec2((dx + ox) as f32 * TS + TS / 2.0, (dy + oy) as f32 * TS + TS / 2.0)
    }
    pub fn center_px(&self) -> Vec2 {
        self.rect_px().center()
    }
    pub fn spots_of(&self, k: SpotKind) -> impl Iterator<Item = &Spot> {
        self.spots.iter().filter(move |s| s.kind == k)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct District {
    pub name: String,
    /// rect in tile coordinates
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub wealth: f32,
    pub danger: f32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Map {
    pub w: i32,
    pub h: i32,
    pub tiles: Vec<Tile>,
    /// which building a tile belongs to (usize::MAX = none)
    pub owner: Vec<u32>,
    pub buildings: Vec<Building>,
    pub props: Vec<Prop>,
    pub lamps: Vec<Vec2>,
    pub districts: Vec<District>,
    pub streets: Vec<(String, bool, i32)>, // name, horizontal?, coordinate
    pub name: String,
    pub spawn: Vec2,
    #[serde(default)]
    pub city: Option<crate::city::gen::CityId>,
    #[serde(default)]
    pub year: i32,
    #[serde(default)]
    pub elevated: Option<i32>,
    /// prop index per tile for fast collision (u32::MAX = none)
    #[serde(skip)]
    pub prop_at: Vec<u32>,
}

pub const NONE: u32 = u32::MAX;

impl Map {
    pub fn new(w: i32, h: i32, fill: Tile) -> Map {
        Map {
            w,
            h,
            tiles: vec![fill; (w * h) as usize],
            owner: vec![NONE; (w * h) as usize],
            buildings: Vec::new(),
            props: Vec::new(),
            lamps: Vec::new(),
            districts: Vec::new(),
            streets: Vec::new(),
            name: String::new(),
            spawn: Vec2::ZERO,
            city: None,
            year: 2025,
            elevated: None,
            prop_at: Vec::new(),
        }
    }

    #[inline]
    pub fn inb(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.w && y < self.h
    }
    #[inline]
    pub fn get(&self, x: i32, y: i32) -> Tile {
        if self.inb(x, y) {
            self.tiles[(y * self.w + x) as usize]
        } else {
            Tile::Void
        }
    }
    #[inline]
    pub fn set(&mut self, x: i32, y: i32, t: Tile) {
        if self.inb(x, y) {
            self.tiles[(y * self.w + x) as usize] = t;
        }
    }
    pub fn building_at_tile(&self, x: i32, y: i32) -> Option<usize> {
        if !self.inb(x, y) {
            return None;
        }
        let o = self.owner[(y * self.w + x) as usize];
        if o == NONE {
            None
        } else {
            Some(o as usize)
        }
    }
    pub fn building_at(&self, p: Vec2) -> Option<usize> {
        let (x, y) = to_tile(p);
        let b = self.building_at_tile(x, y)?;
        // only count as inside when on an interior (non wall) tile
        match self.get(x, y) {
            Tile::Floor | Tile::Door => Some(b),
            _ => None,
        }
    }

    pub fn rebuild_prop_index(&mut self) {
        self.prop_at = vec![NONE; (self.w * self.h) as usize];
        for (i, p) in self.props.iter().enumerate() {
            for yy in p.y..p.y + p.h {
                for xx in p.x..p.x + p.w {
                    if self.inb(xx, yy) {
                        self.prop_at[(yy * self.w + xx) as usize] = i as u32;
                    }
                }
            }
        }
    }

    pub fn prop_at_tile(&self, x: i32, y: i32) -> Option<usize> {
        if !self.inb(x, y) || self.prop_at.is_empty() {
            return None;
        }
        let v = self.prop_at[(y * self.w + x) as usize];
        if v == NONE {
            None
        } else {
            Some(v as usize)
        }
    }

    pub fn blocked(&self, x: i32, y: i32) -> bool {
        if self.get(x, y).solid() {
            return true;
        }
        if let Some(p) = self.prop_at_tile(x, y) {
            return self.props[p].kind.solid();
        }
        false
    }

    /// Circle vs tile collision; returns the corrected position.
    pub fn collide(&self, pos: Vec2, r: f32) -> Vec2 {
        self.collide_with(pos, r, |x, y| self.blocked(x, y))
    }

    /// Collision with a custom notion of what is solid (Elias: locked doors
    /// are walls, opened windows are holes).
    pub fn collide_with(&self, pos: Vec2, r: f32, blocked: impl Fn(i32, i32) -> bool) -> Vec2 {
        let mut p = pos;
        let (tx, ty) = to_tile(p);
        for yy in ty - 1..=ty + 1 {
            for xx in tx - 1..=tx + 1 {
                if !blocked(xx, yy) {
                    continue;
                }
                let rx = xx as f32 * TS;
                let ry = yy as f32 * TS;
                let cx = p.x.clamp(rx, rx + TS);
                let cy = p.y.clamp(ry, ry + TS);
                let d = vec2(p.x - cx, p.y - cy);
                let dl = d.length();
                if dl < r {
                    if dl > 0.0001 {
                        p += d / dl * (r - dl);
                    } else {
                        // centre inside the tile: push out along smallest axis
                        let left = p.x - rx;
                        let right = rx + TS - p.x;
                        let up = p.y - ry;
                        let down = ry + TS - p.y;
                        let m = left.min(right).min(up).min(down);
                        if m == left {
                            p.x = rx - r;
                        } else if m == right {
                            p.x = rx + TS + r;
                        } else if m == up {
                            p.y = ry - r;
                        } else {
                            p.y = ry + TS + r;
                        }
                    }
                }
            }
        }
        p
    }

    /// Straight line walkability test (for line of sight / sight lines).
    pub fn line_clear(&self, a: Vec2, b: Vec2, walls_only: bool) -> bool {
        let d = b - a;
        let steps = (d.length() / (TS * 0.4)).ceil().max(1.0) as i32;
        for i in 1..steps {
            let p = a + d * (i as f32 / steps as f32);
            let (x, y) = to_tile(p);
            let t = self.get(x, y);
            if walls_only {
                if matches!(t, Tile::Wall | Tile::Void) {
                    return false;
                }
            } else if self.blocked(x, y) && !matches!(t, Tile::Window | Tile::Water) {
                return false;
            }
        }
        true
    }

    pub fn street_name_near(&self, p: Vec2) -> String {
        let (x, y) = to_tile(p);
        let mut best: Option<(i32, &String)> = None;
        for (n, horiz, c) in &self.streets {
            let d = if *horiz { (y - c).abs() } else { (x - c).abs() };
            if best.map(|b| d < b.0).unwrap_or(true) {
                best = Some((d, n));
            }
        }
        best.map(|b| b.1.clone()).unwrap_or_default()
    }

    pub fn district_at(&self, p: Vec2) -> Option<usize> {
        let (x, y) = to_tile(p);
        self.districts
            .iter()
            .rposition(|d| x >= d.x && y >= d.y && x < d.x + d.w && y < d.y + d.h)
    }

    pub fn find_building(&self, k: BKind) -> Option<usize> {
        self.buildings.iter().position(|b| b.kind == k && !b.closed_forever)
    }

    pub fn buildings_of(&self, k: BKind) -> Vec<usize> {
        self.buildings.iter().filter(|b| b.kind == k && !b.closed_forever).map(|b| b.id).collect()
    }

    /// Nearest walkable tile centre to p.
    pub fn nearest_open(&self, p: Vec2) -> Vec2 {
        let (tx, ty) = to_tile(p);
        for r in 0..8 {
            for yy in ty - r..=ty + r {
                for xx in tx - r..=tx + r {
                    if !self.blocked(xx, yy) && self.inb(xx, yy) {
                        return tile_center(xx, yy);
                    }
                }
            }
        }
        p
    }

    /// Build a handcrafted map from ASCII art.
    /// Legend: '#' wall, '.' floor, 'D' door, 'W' window, ',' sidewalk, '=' road,
    /// '"' grass, '~' water, ' ' void, ':' dirt, 'g' gravel.
    pub fn from_ascii(rows: &[&str], floor_building: BKind, name: &str) -> Map {
        let h = rows.len() as i32;
        let w = rows.iter().map(|r| r.chars().count()).max().unwrap_or(0) as i32;
        let mut m = Map::new(w, h, Tile::Void);
        m.name = name.to_string();
        let mut minx = w;
        let mut miny = h;
        let mut maxx = 0;
        let mut maxy = 0;
        let mut doors = Vec::new();
        for (y, row) in rows.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let (x, y) = (x as i32, y as i32);
                let t = match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Floor,
                    'D' => Tile::Door,
                    'W' => Tile::Window,
                    ',' => Tile::Sidewalk,
                    '=' => Tile::Road,
                    '"' => Tile::Grass,
                    '~' => Tile::Water,
                    ':' => Tile::Dirt,
                    'g' => Tile::Gravel,
                    _ => Tile::Void,
                };
                m.set(x, y, t);
                if matches!(t, Tile::Wall | Tile::Floor | Tile::Door | Tile::Window) {
                    minx = minx.min(x);
                    miny = miny.min(y);
                    maxx = maxx.max(x);
                    maxy = maxy.max(y);
                }
                if t == Tile::Door {
                    doors.push((x, y));
                }
            }
        }
        if maxx >= minx {
            let b = Building {
                id: 0,
                kind: floor_building,
                name: name.to_string(),
                x: minx,
                y: miny,
                w: maxx - minx + 1,
                h: maxy - miny + 1,
                doors: if doors.is_empty() { vec![(minx, miny)] } else { doors },
                spots: Vec::new(),
                lights: Vec::new(),
                district: 0,
                address: String::new(),
                residents: Vec::new(),
                owner: None,
                closed_forever: false,
                locked: false,
            };
            for yy in b.y..b.y + b.h {
                for xx in b.x..b.x + b.w {
                    if matches!(m.get(xx, yy), Tile::Wall | Tile::Floor | Tile::Door | Tile::Window) {
                        m.owner[(yy * w + xx) as usize] = 0;
                    }
                }
            }
            m.buildings.push(b);
        }
        m
    }

    pub fn add_prop(&mut self, kind: PKind, x: i32, y: i32, w: i32, h: i32, building: Option<usize>) -> usize {
        self.props.push(Prop { kind, x, y, w, h, building, tint: 1.0, items: Vec::new(), bodies: Vec::new(), rot: 0 });
        self.props.len() - 1
    }
}

#[inline]
pub fn to_tile(p: Vec2) -> (i32, i32) {
    ((p.x / TS).floor() as i32, (p.y / TS).floor() as i32)
}

#[inline]
pub fn tile_center(x: i32, y: i32) -> Vec2 {
    vec2(x as f32 * TS + TS / 2.0, y as f32 * TS + TS / 2.0)
}
