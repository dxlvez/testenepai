//! Each city has its own look: layout, architecture, vegetation, weather,
//! street surface, vehicles (horses!), police and names — varying by year.

use super::gen::CityId;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Arch {
    /// New Orleans: pastel creole townhouses, iron balconies, shutters
    Creole,
    /// Chicago / Portland: red brick, fire escapes, water tanks
    Brick,
    /// Bavaria: timber-framed farmhouses, steep roofs, church spire
    Village,
    /// London: sooty brick terraces, chimney pots, bomb damage
    Terrace,
    /// Adelaide: low colonial bungalows with verandas over the sidewalk
    Colonial,
    /// Berlin: grey stone blocks, ruins, the wall
    Stone,
    /// Bergen: colourful wooden houses, steep roofs, wharf
    Wooden,
    /// San Francisco: painted victorians with bay windows
    Victorian,
    /// New York: tall brick & glass, graffiti, fire escapes
    HighRise,
    /// Los Angeles: stucco, flat roofs, palm trees, parking lots
    Stucco,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Water {
    Bottom,
    Right,
    Fjord,
    None,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Outskirts {
    Fields,
    Forest,
    Beach,
    Hills,
    Snow,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TreeKind {
    Oak,
    Pine,
    Palm,
    Eucalyptus,
    Birch,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Surface {
    Cobble,
    Asphalt,
    Dirt,
}

#[derive(Clone, Copy, Debug)]
pub struct Style {
    pub arch: Arch,
    pub bx: i32,
    pub by: i32,
    pub px: i32,
    pub py: i32,
    pub st: i32,
    pub water: Water,
    pub outskirts: Outskirts,
    pub rail: bool,
    pub elevated: bool,
    pub wall: bool,
    pub ruins: f32,
    pub snow: bool,
    pub tree: TreeKind,
    pub floors: (i32, i32),
    pub street: Surface,
    pub facades: &'static [[f32; 3]],
    pub horses: f32,
    pub neon: bool,
    pub graffiti: bool,
    pub grass: [f32; 3],
    pub fog: f32,
    pub rain: f32,
}

const CREOLE: &[[f32; 3]] = &[[0.62, 0.42, 0.40], [0.58, 0.52, 0.34], [0.34, 0.50, 0.46], [0.62, 0.57, 0.48], [0.46, 0.36, 0.50], [0.56, 0.38, 0.30], [0.40, 0.48, 0.58]];
const BRICK: &[[f32; 3]] = &[[0.45, 0.22, 0.17], [0.38, 0.19, 0.15], [0.52, 0.30, 0.22], [0.33, 0.25, 0.22], [0.42, 0.28, 0.20]];
const VILLAGE: &[[f32; 3]] = &[[0.85, 0.82, 0.74], [0.80, 0.76, 0.66], [0.78, 0.72, 0.60], [0.86, 0.80, 0.70]];
const TERRACE: &[[f32; 3]] = &[[0.32, 0.24, 0.20], [0.36, 0.30, 0.26], [0.28, 0.22, 0.19], [0.55, 0.52, 0.48]];
const COLONIAL: &[[f32; 3]] = &[[0.78, 0.72, 0.60], [0.70, 0.64, 0.52], [0.82, 0.80, 0.74], [0.62, 0.48, 0.36]];
const STONE: &[[f32; 3]] = &[[0.45, 0.44, 0.42], [0.38, 0.37, 0.36], [0.52, 0.50, 0.46], [0.33, 0.32, 0.31]];
const WOODEN: &[[f32; 3]] = &[[0.62, 0.15, 0.12], [0.85, 0.80, 0.70], [0.78, 0.60, 0.20], [0.22, 0.30, 0.45], [0.55, 0.22, 0.18], [0.85, 0.85, 0.82]];
const VICTORIAN: &[[f32; 3]] = &[[0.55, 0.30, 0.45], [0.30, 0.50, 0.55], [0.75, 0.62, 0.30], [0.45, 0.55, 0.35], [0.80, 0.55, 0.50], [0.35, 0.35, 0.55]];
const HIGHRISE: &[[f32; 3]] = &[[0.42, 0.25, 0.20], [0.50, 0.48, 0.45], [0.35, 0.33, 0.32], [0.55, 0.40, 0.30], [0.28, 0.28, 0.30]];
const STUCCO: &[[f32; 3]] = &[[0.85, 0.78, 0.65], [0.80, 0.62, 0.55], [0.72, 0.80, 0.78], [0.88, 0.85, 0.78], [0.80, 0.70, 0.50]];

pub fn style(city: CityId, year: i32) -> Style {
    let base = Style {
        arch: Arch::Creole,
        bx: 7,
        by: 5,
        px: 22,
        py: 22,
        st: 4,
        water: Water::Bottom,
        outskirts: Outskirts::Fields,
        rail: true,
        elevated: false,
        wall: false,
        ruins: 0.0,
        snow: false,
        tree: TreeKind::Oak,
        floors: (1, 3),
        street: if year < 1935 { Surface::Cobble } else { Surface::Asphalt },
        facades: CREOLE,
        horses: if year < 1925 { 0.45 } else if year < 1935 { 0.2 } else { 0.0 },
        neon: year >= 1950,
        graffiti: false,
        grass: [0.12, 0.16, 0.10],
        fog: 0.3,
        rain: 0.5,
    };
    match city {
        CityId::NewOrleans => Style { floors: if year >= 2000 { (1, 4) } else { (1, 3) }, graffiti: year >= 1990, ..base },
        CityId::Chicago => Style { arch: Arch::Brick, px: 20, py: 20, bx: 8, water: Water::Right, elevated: year >= 1900, floors: (2, 6), facades: BRICK, snow: true, tree: TreeKind::Birch, grass: [0.2, 0.2, 0.18], ..base },
        CityId::Bavaria => Style {
            arch: Arch::Village,
            bx: 4,
            by: 3,
            px: 26,
            py: 24,
            st: 3,
            water: Water::None,
            outskirts: Outskirts::Snow,
            rail: true,
            floors: (1, 2),
            street: Surface::Dirt,
            facades: VILLAGE,
            horses: 0.75,
            neon: false,
            snow: true,
            tree: TreeKind::Pine,
            grass: [0.75, 0.77, 0.8],
            fog: 0.5,
            rain: 0.3,
            ..base
        },
        CityId::London => Style { arch: Arch::Terrace, bx: 7, by: 5, px: 20, py: 21, ruins: 0.18, floors: (2, 4), facades: TERRACE, horses: 0.15, neon: false, fog: 0.7, rain: 0.8, grass: [0.12, 0.17, 0.11], ..base },
        CityId::Adelaide => Style { arch: Arch::Colonial, px: 24, py: 23, st: 5, floors: (1, 2), facades: COLONIAL, outskirts: Outskirts::Beach, tree: TreeKind::Eucalyptus, grass: [0.30, 0.30, 0.16], fog: 0.1, rain: 0.2, ..base },
        CityId::Berlin => Style { arch: Arch::Stone, px: 22, py: 22, st: 5, wall: true, ruins: 0.22, floors: (3, 5), facades: STONE, water: Water::None, snow: true, tree: TreeKind::Birch, grass: [0.2, 0.22, 0.18], fog: 0.5, ..base },
        CityId::Bergen => Style { arch: Arch::Wooden, bx: 6, by: 5, px: 20, py: 20, st: 3, water: Water::Fjord, outskirts: Outskirts::Forest, floors: (2, 3), street: Surface::Cobble, facades: WOODEN, tree: TreeKind::Pine, fog: 0.6, rain: 0.9, grass: [0.12, 0.2, 0.12], ..base },
        CityId::SanFrancisco => Style { arch: Arch::Victorian, px: 20, py: 22, floors: (2, 3), facades: VICTORIAN, outskirts: Outskirts::Hills, tree: TreeKind::Eucalyptus, fog: 0.8, rain: 0.3, grass: [0.2, 0.26, 0.14], ..base },
        CityId::Portland => Style { arch: Arch::Brick, bx: 6, by: 4, px: 22, py: 22, outskirts: Outskirts::Forest, floors: (1, 4), facades: BRICK, tree: TreeKind::Pine, rain: 0.9, fog: 0.5, grass: [0.1, 0.2, 0.1], ..base },
        CityId::NewYork => Style { arch: Arch::HighRise, bx: 8, by: 5, px: 18, py: 24, st: 5, water: Water::Right, floors: (4, 12), facades: HIGHRISE, graffiti: true, tree: TreeKind::Oak, grass: [0.14, 0.18, 0.12], ..base },
        CityId::LosAngeles => Style { arch: Arch::Stucco, px: 26, py: 24, st: 6, water: Water::Bottom, outskirts: Outskirts::Hills, floors: (1, 3), facades: STUCCO, tree: TreeKind::Palm, fog: 0.15, rain: 0.05, grass: [0.3, 0.32, 0.18], graffiti: true, ..base },
    }
}

impl CityId {
    pub fn currency(self, year: i32) -> &'static str {
        match self {
            CityId::Adelaide | CityId::London => "£",
            CityId::Bavaria => "RM ",
            CityId::Berlin => {
                if year >= 1948 {
                    "DM "
                } else {
                    "RM "
                }
            }
            CityId::Bergen => "kr ",
            _ => "$",
        }
    }
    /// price multiplier relative to the US dollar of the same year
    pub fn currency_factor(self) -> f32 {
        match self {
            CityId::Adelaide | CityId::London => 0.3,
            CityId::Bavaria | CityId::Berlin => 3.0,
            CityId::Bergen => 5.0,
            _ => 1.0,
        }
    }
}
