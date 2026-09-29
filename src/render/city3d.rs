//! Turns a `Map` into 3D geometry: merged ground chunks, unique buildings
//! (style per city: creole balconies, fire escapes, timber frames, verandas,
//! wooden houses, victorian bays, skyscrapers, stucco), cut-away walls,
//! removable roofs, furniture, street props, doors.

use super::mesh::{c3, MB};
use super::props;
use super::textures::Tex;
use crate::city::gen::CityId;
use crate::city::map::*;
use crate::city::style::{style, Arch, Style, Surface, TreeKind};
use crate::util::{hash2, hashf};
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Mats {
    pub road: Handle<StandardMaterial>,
    pub asphalt: Handle<StandardMaterial>,
    pub sidewalk: Handle<StandardMaterial>,
    pub stone: Handle<StandardMaterial>,
    pub matte: Handle<StandardMaterial>,
    pub floor: Handle<StandardMaterial>,
    pub wall: Handle<StandardMaterial>,
    pub plaster: Handle<StandardMaterial>,
    pub roof: Handle<StandardMaterial>,
    pub roof_flat: Handle<StandardMaterial>,
    pub brick_yellow: Handle<StandardMaterial>,
    pub stone_wall: Handle<StandardMaterial>,
    pub siding: Handle<StandardMaterial>,
    pub slate: Handle<StandardMaterial>,
    pub grass: Handle<StandardMaterial>,
    pub dirt: Handle<StandardMaterial>,
    pub sand: Handle<StandardMaterial>,
    pub gravel: Handle<StandardMaterial>,
    pub floor_tiles: Handle<StandardMaterial>,
    pub wallpaper: Handle<StandardMaterial>,
    pub leather: Handle<StandardMaterial>,
    pub furn: Handle<StandardMaterial>,
    pub fabric: Handle<StandardMaterial>,
    pub metal: Handle<StandardMaterial>,
    pub car_paint: Handle<StandardMaterial>,
    pub chrome: Handle<StandardMaterial>,
    pub lamp_unlit: Handle<StandardMaterial>,
    pub glow: Handle<StandardMaterial>,
    pub glass_lit: Handle<StandardMaterial>,
    pub glass_dark: Handle<StandardMaterial>,
    pub water: Handle<StandardMaterial>,
    pub puddle: Handle<StandardMaterial>,
    pub halo: Handle<StandardMaterial>,
    pub red: Handle<StandardMaterial>,
    pub plain: Handle<StandardMaterial>,
    pub skin: Handle<StandardMaterial>,
    pub neon: Handle<StandardMaterial>,
    pub ghost: Handle<StandardMaterial>,
}

pub fn make_mats(mats: &mut Assets<StandardMaterial>, tex: &Tex, real: &super::textures::Real) -> Mats {
    // scanned materials: colour + normal + occlusion/roughness from the ARM map
    let scan = |m: &mut Assets<StandardMaterial>, p: &super::textures::Pbr, rough: f32| {
        m.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(p.diff.clone()),
            normal_map_texture: Some(p.nor.clone()),
            metallic_roughness_texture: Some(p.arm.clone()),
            occlusion_texture: Some(p.arm.clone()),
            perceptual_roughness: rough,
            metallic: 1.0,
            reflectance: 0.5,
            cull_mode: None,
            ..default()
        })
    };
    let base = |m: &mut Assets<StandardMaterial>, t: &(Handle<Image>, Handle<Image>), rough: f32, refl: f32| {
        m.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(t.0.clone()),
            normal_map_texture: Some(t.1.clone()),
            perceptual_roughness: rough,
            reflectance: refl,
            cull_mode: None,
            ..default()
        })
    };
    let emissive = |m: &mut Assets<StandardMaterial>, e: LinearRgba| m.add(StandardMaterial { base_color: Color::WHITE, emissive: e, cull_mode: None, ..default() });
    Mats {
        road: scan(mats, &real.cobble, 1.0),
        asphalt: scan(mats, &real.asphalt, 1.0),
        sidewalk: scan(mats, &real.sidewalk, 1.0),
        stone: base(mats, &tex.grain, 0.6, 0.35),
        matte: base(mats, &tex.grain, 0.95, 0.2),
        floor: scan(mats, &real.wood_floor, 1.0),
        wall: scan(mats, &real.brick, 1.0),
        plaster: scan(mats, &real.plaster, 1.0),
        roof: scan(mats, &real.roof_tiles, 1.0),
        roof_flat: base(mats, &tex.asphalt, 0.85, 0.25),
        brick_yellow: scan(mats, &real.brick_yellow, 1.0),
        stone_wall: scan(mats, &real.stone, 1.0),
        siding: scan(mats, &real.siding, 1.0),
        slate: scan(mats, &real.slate, 1.0),
        grass: scan(mats, &real.grass, 1.0),
        dirt: scan(mats, &real.dirt, 1.0),
        sand: scan(mats, &real.sand, 1.0),
        gravel: scan(mats, &real.gravel, 1.0),
        floor_tiles: scan(mats, &real.floor_tiles, 1.0),
        wallpaper: base(mats, &tex.wallpaper, 0.85, 0.2),
        leather: base(mats, &tex.leather, 0.5, 0.4),
        furn: base(mats, &tex.grain, 0.6, 0.35),
        fabric: base(mats, &tex.fabric, 0.95, 0.15),
        metal: mats.add(StandardMaterial { base_color: Color::WHITE, metallic: 0.8, perceptual_roughness: 0.35, cull_mode: None, ..default() }),
        // glossy clear-coated car paint (colour from the vertices)
        car_paint: mats.add(StandardMaterial {
            base_color: Color::WHITE,
            metallic: 0.25,
            perceptual_roughness: 0.32,
            reflectance: 0.6,
            clearcoat: 1.0,
            clearcoat_perceptual_roughness: 0.06,
            cull_mode: None,
            ..default()
        }),
        chrome: mats.add(StandardMaterial { base_color: Color::WHITE, metallic: 1.0, perceptual_roughness: 0.12, cull_mode: None, ..default() }),
        lamp_unlit: mats.add(StandardMaterial { base_color: Color::WHITE, unlit: true, cull_mode: None, ..default() }),
        glow: emissive(mats, LinearRgba::rgb(6.0, 4.2, 2.4)),
        glass_lit: emissive(mats, LinearRgba::rgb(1.1, 0.72, 0.36)),
        glass_dark: mats.add(StandardMaterial { base_color: Color::srgb(0.05, 0.06, 0.09), perceptual_roughness: 0.05, reflectance: 0.8, cull_mode: None, ..default() }),
        water: mats.add(StandardMaterial { base_color: Color::srgb(0.02, 0.03, 0.05), perceptual_roughness: 0.04, reflectance: 0.9, cull_mode: None, ..default() }),
        puddle: mats.add(StandardMaterial {
            base_color: Color::srgba(0.05, 0.05, 0.06, 0.55),
            perceptual_roughness: 0.06,
            reflectance: 1.0,
            alpha_mode: AlphaMode::Blend,
            cull_mode: None,
            ..default()
        }),
        halo: mats.add(StandardMaterial {
            base_color: Color::srgba(0.0, 0.0, 0.0, 0.0),
            emissive: LinearRgba::rgb(0.12, 0.08, 0.035),
            alpha_mode: AlphaMode::Add,
            unlit: true,
            cull_mode: None,
            ..default()
        }),
        red: emissive(mats, LinearRgba::rgb(9.0, 0.3, 0.6)),
        plain: mats.add(StandardMaterial { base_color: Color::WHITE, perceptual_roughness: 0.75, reflectance: 0.3, cull_mode: None, ..default() }),
        skin: mats.add(StandardMaterial { base_color: Color::WHITE, perceptual_roughness: 0.55, reflectance: 0.35, cull_mode: None, ..default() }),
        neon: emissive(mats, LinearRgba::rgb(1.0, 1.0, 1.0)),
        ghost: mats.add(StandardMaterial {
            base_color: Color::srgba(0.6, 0.75, 1.0, 0.35),
            emissive: LinearRgba::rgb(0.8, 1.0, 1.6),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            cull_mode: None,
            ..default()
        }),
    }
}

#[derive(Component)]
pub struct CityRoot;

#[derive(Component)]
pub struct DoorPanel {
    pub tile: (i32, i32),
    pub building: usize,
    pub open: f32,
    pub hinge: Vec3,
    pub axis_x: bool,
}

pub struct BVis {
    pub full: Entity,
    pub low: Entity,
    pub roof: Entity,
    pub glass: Entity,
    pub cut: bool,
    pub lit: bool,
    pub height: f32,
}

/// Light halos and cones under the street lamps (visible at night).
#[derive(Component)]
pub struct LampHalos;

/// Real trees; hidden while they stand between the camera and Elias (Diablo-style see-through).
#[derive(Component)]
pub struct TreeCanopy;

pub fn fade_trees(
    game: Res<crate::state::Game>,
    cam: Query<&GlobalTransform, With<crate::camera::MainCam>>,
    mut q: Query<(&Transform, &mut Visibility), With<TreeCanopy>>,
) {
    let Ok(ct) = cam.single() else { return };
    let cp = ct.translation();
    let pp = Vec3::new(game.player.pos.x, 1.0, game.player.pos.y);
    let to_cam = Vec2::new(cp.x - pp.x, cp.z - pp.z).normalize_or_zero();
    for (tr, mut v) in q.iter_mut() {
        let d = Vec2::new(tr.translation.x - pp.x, tr.translation.z - pp.z);
        // a canopy is ~3 m wide and sits high, so it blocks the view on the camera side of Elias
        let along = d.dot(to_cam);
        let across = (d - to_cam * along).length();
        let block = along > -2.0 && along < 13.0 && across < 4.5 + along.max(0.0) * 0.15;
        let want = if block { Visibility::Hidden } else { Visibility::Inherited };
        if *v != want {
            *v = want;
        }
    }
}

/// Rain puddles on streets and sidewalks (visible when wet).
#[derive(Component)]
pub struct Puddles;

#[derive(Resource, Default)]
pub struct CityVis {
    pub buildings: Vec<BVis>,
    pub lamps: Vec<Vec3>,
    pub root: Option<Entity>,
}

pub const GROUND_H: f32 = 3.2;
/// inset of the thin walls inside their tile (wall thickness = 1 - 2*WIN)
pub const WIN: f32 = 0.39;
pub const FLOOR_H: f32 = 2.7;

/// Number of floors of a building (visual; only the ground floor is playable).
pub fn floors(b: &Building, s: &Style) -> i32 {
    let h = hash2(b.x, b.y, 71) as i32;
    let (lo, hi) = s.floors;
    let base = lo + h.rem_euclid((hi - lo + 1).max(1));
    match b.kind {
        BKind::Church => 1,
        BKind::Barn | BKind::Farmhouse | BKind::Market => 1,
        BKind::Warehouse | BKind::Factory => base.min(2),
        BKind::Police | BKind::Hospital | BKind::Hotel | BKind::Bank | BKind::Office | BKind::Newspaper => (base + 1).max(2),
        BKind::Abandoned => base.max(1),
        _ => base,
    }
}

pub fn wall_height(b: &Building, s: &Style) -> f32 {
    let f = floors(b, s);
    let church = if b.kind == BKind::Church { 3.0 } else { 0.0 };
    GROUND_H + (f - 1).max(0) as f32 * FLOOR_H + church
}

fn facade(b: &Building, s: &Style) -> [f32; 3] {
    let h = hash2(b.x, b.y, 77);
    let base = match b.kind {
        BKind::Police | BKind::Bank | BKind::Hospital | BKind::Station if !matches!(s.arch, Arch::Wooden | Arch::Village) => [0.5, 0.49, 0.47],
        BKind::Church => [0.62, 0.6, 0.56],
        BKind::Barn => [0.42, 0.18, 0.13],
        BKind::Abandoned => [0.25, 0.24, 0.23],
        _ => s.facades[(h as usize) % s.facades.len()],
    };
    // per-building weathering
    let k = 0.85 + hashf(b.x, b.y, 78) * 0.25;
    [base[0] * k, base[1] * k, base[2] * k]
}

#[derive(Clone, Copy, PartialEq)]
enum WallKind {
    Brick,
    YellowBrick,
    Stone,
    Siding,
    Plaster,
}

fn wall_kind(b: &Building, s: &Style) -> WallKind {
    let h = hash2(b.x, b.y, 81);
    if matches!(b.kind, BKind::Police | BKind::Bank | BKind::Church | BKind::Hospital | BKind::Station | BKind::Newspaper) && !matches!(s.arch, Arch::Wooden | Arch::Village) {
        return WallKind::Stone;
    }
    match s.arch {
        Arch::Terrace => {
            if h % 3 == 0 {
                WallKind::Brick
            } else {
                WallKind::YellowBrick
            }
        }
        Arch::Brick | Arch::HighRise => {
            if h % 5 == 0 {
                WallKind::Stone
            } else if h % 5 == 1 {
                WallKind::YellowBrick
            } else {
                WallKind::Brick
            }
        }
        Arch::Wooden | Arch::Victorian => WallKind::Siding,
        Arch::Creole => {
            if h % 4 == 0 {
                WallKind::Brick
            } else {
                WallKind::Plaster
            }
        }
        Arch::Stone => {
            if h % 3 == 0 {
                WallKind::Plaster
            } else {
                WallKind::Stone
            }
        }
        _ => WallKind::Plaster,
    }
}

pub fn spawn_city(c: &mut Commands, meshes: &mut Assets<Mesh>, mats: &Mats, m: &Map, year: i32, vis: &mut CityVis, lib: Option<&super::lib3d::Lib>) {
    let root = c.spawn((CityRoot, Transform::default(), Visibility::default(), Name::new("city"))).id();
    vis.buildings.clear();
    vis.lamps.clear();
    vis.root = Some(root);
    let city = m.city.unwrap_or(CityId::NewOrleans);
    let s = style(city, year);
    const CH: i32 = 16;
    let modern = year >= 1950 && s.street == Surface::Asphalt;
    let grass = s.grass;
    for cy in (0..m.h).step_by(CH as usize) {
        for cx in (0..m.w).step_by(CH as usize) {
            let mut road = MB::new();
            let mut stone = MB::new();
            let mut walk = MB::new();
            let mut grass_mb = MB::new();
            let mut dirt_mb = MB::new();
            let mut sand_mb = MB::new();
            let mut gravel_mb = MB::new();
            let mut tile_floor = MB::new();
            let mut matte = MB::new();
            let mut floor = MB::new();
            let mut water = MB::new();
            let mut street = MB::new();
            let mut glow = MB::new();
            let mut blades = MB::new();
            for y in cy..(cy + CH).min(m.h) {
                for x in cx..(cx + CH).min(m.w) {
                    let (fx, fz) = (x as f32, y as f32);
                    let n = hashf(x, y, 1) * 0.08;
                    match m.get(x, y) {
                        Tile::Road => {
                            let col = if modern { [0.95 + n, 0.95 + n, 0.95 + n] } else { [1.0 + n, 1.0 + n, 1.0 + n] };
                            road.floor(fx, fz, fx + 1.0, fz + 1.0, 0.0, c3(col));
                            if modern && hash2(x, y, 3) % 2 == 0 {
                                let gy = (y - crate::city::gen::RAIL) % s.py;
                                let gx = (x - crate::city::gen::OUT) % s.px;
                                if gy == s.st / 2 || gx == s.st / 2 {
                                    road.floor(fx + 0.45, fz + 0.1, fx + 0.55, fz + 0.9, 0.005, c3([0.6, 0.55, 0.3]));
                                }
                            }
                            // horse droppings & straw on old streets
                            if s.horses > 0.0 && hash2(x, y, 9) % 41 == 0 {
                                road.floor(fx + 0.3, fz + 0.3, fx + 0.5, fz + 0.45, 0.004, c3([0.2, 0.15, 0.08]));
                            }
                        }
                        Tile::Sidewalk | Tile::Plaza => {
                            let col = if m.get(x, y) == Tile::Plaza { [1.05 + n, 1.0 + n, 0.92 + n] } else if s.arch == Arch::Colonial { [1.15 + n, 1.05 + n, 0.9 + n] } else { [1.0 + n, 1.0 + n, 1.0 + n] };
                            walk.cuboid(Vec3::new(fx, -0.1, fz), Vec3::new(fx + 1.0, 0.12, fz + 1.0), c3(col));
                            // graffiti tags and trash in the 80s/90s
                            if s.graffiti && hash2(x, y, 13) % 29 == 0 {
                                stone.floor(fx + 0.1, fz + 0.2, fx + 0.5, fz + 0.5, 0.125, c3([0.8, 0.78, 0.7]));
                            }
                        }
                        Tile::Alley => matte.floor(fx, fz, fx + 1.0, fz + 1.0, 0.03, c3([0.2 + n, 0.19 + n, 0.18 + n])),
                        Tile::Grass => {
                            // the texture carries the colour; the city's grass tone nudges it (dry Adelaide, lush Bergen)
                            let k = [0.62 + grass[0] * 0.5 + n, 0.8 + grass[1] * 0.5 + n, 0.5 + grass[2] * 0.5 + n];
                            grass_mb.floor(fx, fz, fx + 1.0, fz + 1.0, 0.02, c3(if s.snow { [1.6, 1.6, 1.7] } else { k }));
                            if !s.snow {
                                // tufts of real blades so lawns read as grass, not a painted floor
                                let lush = 0.8 + hashf(x, y, 21) * 0.4;
                                // city tone blended towards a living green so lawns never look burnt
                                let g = [grass[0] * 0.5 + 0.16, grass[1] * 0.5 + 0.26, grass[2] * 0.5 + 0.07];
                                for t in 0..22 {
                                    let tx = fx + 0.03 + hashf(x * 7 + t, y, 22) * 0.94;
                                    let tz = fz + 0.03 + hashf(x, y * 7 + t, 23) * 0.94;
                                    let dry = hashf(x + t, y - t, 24);
                                    let tip = [(g[0] * 1.35 + dry * 0.1) * lush, (g[1] * 1.3 + dry * 0.04) * lush, g[2] * 1.1 * lush];
                                    let root = [g[0] * 0.6, g[1] * 0.65, g[2] * 0.55];
                                    for b in 0..4 {
                                        let yaw = hashf(x * 3 + t, y * 5 + b, 25) * 6.28;
                                        let lean = Vec2::new(yaw.sin(), yaw.cos()) * (0.02 + hashf(t, b, (x + y) as u32) * 0.06);
                                        let h = (0.07 + hashf(x + b, y + t, 26) * 0.13) * lush;
                                        blades.blade(Vec3::new(tx, 0.02, tz), lean, h, 0.022, yaw, c3(root), c3(tip));
                                    }
                                }
                            }
                        }
                        Tile::Sand => sand_mb.floor(fx, fz, fx + 1.0, fz + 1.0, 0.01, c3([0.95 + n, 0.95 + n, 0.95 + n])),
                        Tile::Dirt => {
                            let col = if s.snow && s.arch == Arch::Village { [1.5, 1.5, 1.55] } else { [0.95 + n, 0.95 + n, 0.95 + n] };
                            dirt_mb.floor(fx, fz, fx + 1.0, fz + 1.0, 0.02, c3(col));
                            // wheel ruts on village roads
                            if s.arch == Arch::Village && (x + y) % 2 == 0 {
                                matte.floor(fx + 0.2, fz, fx + 0.3, fz + 1.0, 0.025, c3([0.3, 0.26, 0.22]));
                            }
                        }
                        Tile::Gravel => gravel_mb.floor(fx, fz, fx + 1.0, fz + 1.0, 0.02, c3([0.95 + n, 0.95 + n, 0.95 + n])),
                        Tile::Field => {
                            let base = if s.snow { [0.7 + n, 0.72 + n, 0.75 + n] } else { [0.2 + n, 0.16 + n, 0.1 + n] };
                            matte.floor(fx, fz, fx + 1.0, fz + 1.0, 0.02, c3(base));
                            if !s.snow {
                                // a furrowed row of crop stalks (wheat-gold or green, by field)
                                let wheat = (x / 9 + y / 7) % 2 == 0;
                                let (root, tip) = if wheat { ([0.25, 0.2, 0.08], [0.75, 0.6, 0.28]) } else { ([0.08, 0.14, 0.04], [0.3, 0.45, 0.12]) };
                                matte.cuboid(Vec3::new(fx + 0.15, 0.0, fz + 0.35), Vec3::new(fx + 0.85, 0.06, fz + 0.65), c3([0.16, 0.12, 0.08]));
                                for t in 0..14 {
                                    let tx = fx + 0.05 + t as f32 * 0.066;
                                    let tz = fz + 0.5 + (hashf(x * 5 + t, y, 27) - 0.5) * 0.2;
                                    let yaw = hashf(x + t, y, 28) * 6.28;
                                    let h = 0.45 + hashf(x, y + t, 29) * 0.3 + n;
                                    blades.blade(Vec3::new(tx, 0.02, tz), Vec2::new(yaw.cos(), yaw.sin()) * 0.06, h, 0.04, yaw, c3(root), c3(tip));
                                    blades.blade(Vec3::new(tx + 0.02, 0.02, tz), Vec2::new(-yaw.sin(), yaw.cos()) * 0.08, h * 0.85, 0.03, yaw + 1.6, c3(root), c3(tip));
                                }
                            }
                        }
                        Tile::Water => {
                            water.floor(fx, fz, fx + 1.0, fz + 1.0, -0.35, [1.0, 1.0, 1.0, 1.0]);
                            if !matches!(m.get(x, y - 1), Tile::Water | Tile::Dock) {
                                stone.cuboid(Vec3::new(fx, -0.8, fz - 0.1), Vec3::new(fx + 1.0, 0.1, fz + 0.05), c3([0.28, 0.27, 0.26]));
                            }
                        }
                        Tile::Dock => {
                            floor.cuboid(Vec3::new(fx, 0.02, fz), Vec3::new(fx + 1.0, 0.15, fz + 1.0), c3([0.3 + n, 0.22 + n, 0.15 + n]));
                            if matches!(m.get(x, y + 1), Tile::Water) || (x % 3 == 0 && y % 3 == 0) {
                                street.cylinder(Vec3::new(fx + 0.5, -0.9, fz + 0.5), 0.1, 1.2, 6, c3([0.2, 0.15, 0.1]));
                            }
                        }
                        Tile::Rail => {
                            gravel_mb.floor(fx, fz, fx + 1.0, fz + 1.0, 0.02, c3([0.8, 0.78, 0.76]));
                            street.cuboid(Vec3::new(fx + 0.1, 0.02, fz + 0.1), Vec3::new(fx + 0.3, 0.1, fz + 0.9), c3([0.2, 0.14, 0.1]));
                            street.cuboid(Vec3::new(fx + 0.6, 0.02, fz + 0.1), Vec3::new(fx + 0.8, 0.1, fz + 0.9), c3([0.2, 0.14, 0.1]));
                            if y == 1 || y == 2 {
                                let zc = if y == 1 { fz + 0.75 } else { fz + 0.25 };
                                street.cuboid(Vec3::new(fx, 0.1, zc - 0.04), Vec3::new(fx + 1.0, 0.16, zc + 0.04), c3([0.35, 0.35, 0.38]));
                            }
                        }
                        Tile::Floor | Tile::Door => {
                            let bk = m.building_at_tile(x, y).map(|b| m.buildings[b].kind);
                            // tiled floors where they belong, oak boards elsewhere (tinted by the building's floor colour)
                            let tiled = matches!(bk, Some(BKind::Bar | BKind::Restaurant | BKind::Hospital | BKind::Pharmacy | BKind::Market | BKind::Police | BKind::Lab | BKind::Station | BKind::Cabaret));
                            if tiled {
                                tile_floor.floor(fx, fz, fx + 1.0, fz + 1.0, 0.1, c3([1.0, 1.0, 1.0]));
                            } else {
                                let col = bk.map(|k| k.floor_color()).unwrap_or([0.3, 0.25, 0.2]);
                                let l = (col[0] + col[1] + col[2]) / 3.0;
                                let k = [0.8 + (col[0] - l) + n, 0.8 + (col[1] - l) + n, 0.8 + (col[2] - l) + n];
                                floor.floor(fx, fz, fx + 1.0, fz + 1.0, 0.1, c3(k));
                            }
                        }
                        Tile::Fence => {
                            let berlin = city == CityId::Berlin && m.building_at_tile(x, y).is_none() && !m.districts.iter().any(|d| d.name.contains("Cemit") && x >= d.x && x < d.x + d.w);
                            if berlin {
                                // concrete sector barrier with barbed wire
                                street.cuboid(Vec3::new(fx, 0.0, fz + 0.3), Vec3::new(fx + 1.0, 2.4, fz + 0.7), c3([0.55, 0.54, 0.52]));
                                street.cuboid(Vec3::new(fx, 2.4, fz + 0.45), Vec3::new(fx + 1.0, 2.5, fz + 0.55), c3([0.2, 0.2, 0.2]));
                            } else {
                                street.cuboid(Vec3::new(fx, 0.0, fz), Vec3::new(fx + 1.0, 0.3, fz + 1.0), c3([0.3, 0.3, 0.3]));
                                for i in 0..4 {
                                    let px = fx + 0.12 + i as f32 * 0.25;
                                    street.cuboid(Vec3::new(px, 0.3, fz + 0.45), Vec3::new(px + 0.04, 1.5, fz + 0.55), c3([0.08, 0.08, 0.09]));
                                }
                            }
                        }
                        Tile::Wall => {
                            // free-standing ruin walls
                            if m.building_at_tile(x, y).is_none() {
                                let hh = 0.6 + hashf(x, y, 4) * 2.5;
                                street.cuboid(Vec3::new(fx, 0.0, fz), Vec3::new(fx + 1.0, hh, fz + 1.0), c3([0.3 + n, 0.25 + n, 0.22 + n]));
                            }
                        }
                        Tile::Window | Tile::Void => {}
                    }
                }
            }
            for p in m.props.iter().filter(|p| p.building.is_none() && p.x >= cx && p.x < cx + CH && p.y >= cy && p.y < cy + CH) {
                let base_y = if matches!(m.get(p.x, p.y), Tile::Sidewalk | Tile::Plaza) { 0.12 } else { 0.02 };
                if p.kind != PKind::LampPost && real_model(c, lib, p, year, base_y, root, None).is_some() {
                    continue;
                }
                let (gs, gg) = prop_geo(p, year, &s, s.snow);
                let off = Vec3::new(p.x as f32, base_y, p.y as f32);
                street.append(&gs, off);
                glow.append(&gg, off);
                if p.kind == PKind::LampPost {
                    vis.lamps.push(off + Vec3::new(0.5, 3.1, 0.5));
                }
            }
            for (mb, mat, shadow) in [
                (road, if s.street == Surface::Cobble { mats.road.clone() } else { mats.asphalt.clone() }, true),
                (stone, mats.stone.clone(), true),
                (walk, mats.sidewalk.clone(), true),
                (grass_mb, mats.grass.clone(), true),
                (dirt_mb, mats.dirt.clone(), true),
                (sand_mb, mats.sand.clone(), true),
                (gravel_mb, mats.gravel.clone(), true),
                (tile_floor, mats.floor_tiles.clone(), true),
                (matte, mats.matte.clone(), true),
                (floor, mats.floor.clone(), true),
                (water, mats.water.clone(), false),
                (street, mats.furn.clone(), true),
                (glow, mats.glow.clone(), false),
                (blades, mats.plain.clone(), false),
            ] {
                if mb.is_empty() {
                    continue;
                }
                let mut e = c.spawn((Mesh3d(meshes.add(mb.build())), MeshMaterial3d(mat), Transform::default()));
                if !shadow {
                    e.insert(NotShadowCaster);
                }
                let id = e.id();
                c.entity(root).add_child(id);
            }
        }
    }
    // real shrubs along lawn edges, ferns and wild flowers inside them
    if let (Some(lib), false) = (lib, s.snow) {
        let mut placed = 0;
        'nature: for y in 1..m.h - 1 {
            for x in 1..m.w - 1 {
                if placed >= 260 {
                    break 'nature;
                }
                if m.get(x, y) != Tile::Grass || m.blocked(x, y) {
                    continue;
                }
                let edge = [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().any(|(dx, dy)| m.get(x + dx, y + dy) != Tile::Grass);
                let hsh = hash2(x, y, 4242);
                let (cat, target) = if edge && hsh % 5 == 0 {
                    ("shrub", 0.7 + (hsh % 7) as f32 * 0.1)
                } else if !edge && hsh % 13 == 0 {
                    (["flower", "plant", "grass", "flower"][(hsh / 13 % 4) as usize], 0.3 + (hsh % 5) as f32 * 0.06)
                } else {
                    continue;
                };
                let cat = if cat == "plant" { "shrub" } else { cat };
                let Some((e, h)) = lib.pick(cat, year, hsh, None) else { continue };
                if e.id.starts_with("potted") || e.id.starts_with("planter") {
                    continue;
                }
                let k = (target / e.size[1].max(0.05)).clamp(0.5, 4.0);
                let at = Vec3::new(x as f32 + 0.3 + hashf(x, y, 43) * 0.4, 0.02, y as f32 + 0.3 + hashf(x, y, 44) * 0.4);
                let tr = Transform::from_translation(at).with_rotation(Quat::from_rotation_y((hsh % 32) as f32 * 0.2)).with_scale(Vec3::splat(k));
                let ent = c.spawn((SceneRoot(h), tr)).id();
                c.entity(root).add_child(ent);
                placed += 1;
            }
        }
    }
    for l in &m.lamps {
        let p = Vec3::new(l.x, 3.2, l.y);
        if !vis.lamps.iter().any(|q| q.distance_squared(p) < 1.0) {
            vis.lamps.push(p);
        }
    }
    // halos + soft light cones for every lamp, puddles on streets
    let mut halos = MB::new();
    for l in &vis.lamps {
        halos.sphere(*l, Vec3::splat(0.45), 10, [1.0, 1.0, 1.0, 1.0]);
    }
    if !halos.is_empty() {
        let e = c.spawn((Mesh3d(meshes.add(halos.build())), MeshMaterial3d(mats.halo.clone()), Transform::default(), NotShadowCaster, LampHalos, Visibility::Hidden)).id();
        c.entity(root).add_child(e);
    }
    let mut pud = MB::new();
    for y in 0..m.h {
        for x in 0..m.w {
            let t = m.get(x, y);
            if !matches!(t, Tile::Road | Tile::Sidewalk | Tile::Plaza | Tile::Alley | Tile::Dirt) || hash2(x, y, 88) % 13 != 0 {
                continue;
            }
            let h = if matches!(t, Tile::Sidewalk | Tile::Plaza) { 0.125 } else { 0.035 };
            let r = 0.25 + hashf(x, y, 89) * 0.55;
            pud.disc(Vec3::new(x as f32 + 0.5, h, y as f32 + 0.5), r, r * (0.5 + hashf(x, y, 90) * 0.6), 12, 0.5, hash2(x, y, 91), [1.0, 1.0, 1.0, 1.0]);
        }
    }
    if !pud.is_empty() {
        let e = c.spawn((Mesh3d(meshes.add(pud.build())), MeshMaterial3d(mats.puddle.clone()), Transform::default(), NotShadowCaster, Puddles, Visibility::Hidden)).id();
        c.entity(root).add_child(e);
    }
    // Chicago's elevated train track
    if let Some(ex) = m.elevated {
        let mut mb = MB::new();
        let x = ex as f32;
        let col = c3([0.12, 0.13, 0.12]);
        mb.cuboid(Vec3::new(x - 0.3, 5.0, crate::city::gen::RAIL as f32), Vec3::new(x + 3.3, 5.4, (m.h - 4) as f32), col);
        for k in 0..((m.h - 10) / 2) {
            let z = crate::city::gen::RAIL as f32 + k as f32 * 2.0;
            mb.cuboid(Vec3::new(x - 0.2, 5.4, z), Vec3::new(x + 3.2, 5.5, z + 0.25), c3([0.2, 0.15, 0.1]));
        }
        let e = c.spawn((Mesh3d(meshes.add(mb.build())), MeshMaterial3d(mats.furn.clone()), Transform::default())).id();
        c.entity(root).add_child(e);
    }
    for b in &m.buildings {
        let bv = spawn_building(c, meshes, mats, m, b, year, root, &s, lib);
        vis.buildings.push(bv);
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_building(c: &mut Commands, meshes: &mut Assets<Mesh>, mats: &Mats, m: &Map, b: &Building, year: i32, root: Entity, s: &Style, lib: Option<&super::lib3d::Lib>) -> BVis {
    let hgt = wall_height(b, s);
    let nfl = floors(b, s);
    let wall_kind = wall_kind(b, s);
    // textured masonry carries its own colour; painted surfaces use the facade palette
    let fcol = match wall_kind {
        WallKind::Brick | WallKind::YellowBrick | WallKind::Stone => {
            let k = 0.9 + hashf(b.x, b.y, 79) * 0.2;
            [k, k * 0.98, k * 0.96]
        }
        _ => facade(b, s),
    };
    let seed = hash2(b.x, b.y, 55);
    let low_h = 0.55;
    let mut full = MB::new();
    let mut back = MB::new();
    let mut low = MB::new();
    let mut glass = MB::new();
    let mut furn = MB::new();
    let mut glow = MB::new();
    let mut roof = MB::new();
    let mut gabled_roof = false;
    let trim = c3([fcol[0] * 0.6, fcol[1] * 0.6, fcol[2] * 0.6]);
    let white_trim = c3([0.82, 0.8, 0.76]);
    let wc = c3(fcol);
    let inner = c3([0.78, 0.72, 0.64]);
    let beam = c3([0.18, 0.12, 0.08]);
    let is_market = b.kind == BKind::Market;
    let front_top = b.doors.first().map(|d| d.1 == b.y).unwrap_or(true);
    // walls are thin (WT) and run along the middle of the wall tiles
    let (x0, z0, x1, z1) = (b.x as f32 + WIN, b.y as f32 + WIN, (b.x + b.w) as f32 - WIN, (b.y + b.h) as f32 - WIN);
    let front_z = if front_top { z0 } else { z1 };
    let out = if front_top { -1.0 } else { 1.0 };
    if !is_market {
        // ground floor, tile by tile (doors and windows are gameplay)
        for y in b.y..b.y + b.h {
            for x in b.x..b.x + b.w {
                let (fx, fz) = (x as f32, y as f32);
                let lo = Vec3::new(fx, 0.0, fz);
                // walls on the far sides (min x / min z) stay up when the building is cut away
                let is_back = (x == b.x || y == b.y) && x != b.x + b.w - 1 && y != b.y + b.h - 1;
                let wall_mb: &mut MB = if is_back { &mut back } else { &mut full };
                let wallish = |xx: i32, yy: i32| matches!(m.get(xx, yy), Tile::Wall | Tile::Window | Tile::Door) && m.building_at_tile(xx, yy) == Some(b.id);
                // thin wall piece: a post in the middle plus arms towards the neighbouring wall tiles
                let arms = |mb: &mut MB, y0: f32, y1: f32, col: [f32; 4]| {
                    let (a, c) = (WIN, 1.0 - WIN);
                    mb.cuboid(Vec3::new(fx + a, y0, fz + a), Vec3::new(fx + c, y1, fz + c), col);
                    if wallish(x + 1, y) {
                        mb.cuboid(Vec3::new(fx + c, y0, fz + a), Vec3::new(fx + 1.0, y1, fz + c), col);
                    }
                    if wallish(x - 1, y) {
                        mb.cuboid(Vec3::new(fx, y0, fz + a), Vec3::new(fx + a, y1, fz + c), col);
                    }
                    if wallish(x, y + 1) {
                        mb.cuboid(Vec3::new(fx + a, y0, fz + c), Vec3::new(fx + c, y1, fz + 1.0), col);
                    }
                    if wallish(x, y - 1) {
                        mb.cuboid(Vec3::new(fx + a, y0, fz), Vec3::new(fx + c, y1, fz + a), col);
                    }
                };
                let along_x = wallish(x - 1, y) || wallish(x + 1, y);
                // a slab of the wall line through this tile (for window sills / lintels)
                let band = |mb: &mut MB, y0: f32, y1: f32, u0: f32, u1: f32, col: [f32; 4]| {
                    if along_x {
                        mb.cuboid(Vec3::new(fx + u0, y0, fz + WIN), Vec3::new(fx + u1, y1, fz + 1.0 - WIN), col);
                    } else {
                        mb.cuboid(Vec3::new(fx + WIN, y0, fz + u0), Vec3::new(fx + 1.0 - WIN, y1, fz + u1), col);
                    }
                };
                match m.get(x, y) {
                    Tile::Wall => {
                        arms(&mut *wall_mb, 0.0, GROUND_H, wc);
                        if !is_back {
                            arms(&mut low, 0.0, low_h, inner);
                        }
                        // skirting board inside, cornice line outside
                        arms(&mut *wall_mb, 0.0, 0.12, trim);
                        if s.arch == Arch::Village && (x + y) % 2 == 0 {
                            // timber framing
                            let ez = if y == b.y { fz + WIN - 0.01 } else { fz + 1.0 - WIN + 0.01 };
                            if y == b.y || y == b.y + b.h - 1 {
                                wall_mb.cuboid(Vec3::new(fx + 0.45, 0.3, ez - 0.02), Vec3::new(fx + 0.55, GROUND_H, ez + 0.02), beam);
                            }
                        }
                    }
                    Tile::Window => {
                        band(&mut *wall_mb, 0.0, 0.9, 0.0, 1.0, wc);
                        band(&mut *wall_mb, 2.1, GROUND_H, 0.0, 1.0, wc);
                        band(&mut *wall_mb, 0.9, 2.1, 0.0, 0.12, trim);
                        band(&mut *wall_mb, 0.9, 2.1, 0.88, 1.0, trim);
                        // sill and mullion
                        band(&mut *wall_mb, 0.86, 0.93, 0.06, 0.94, white_trim);
                        band(&mut *wall_mb, 1.45, 1.5, 0.12, 0.88, trim);
                        if along_x {
                            glass.cuboid(Vec3::new(fx + 0.12, 0.9, fz + 0.47), Vec3::new(fx + 0.88, 2.1, fz + 0.53), [1.0, 1.0, 1.0, 1.0]);
                        } else {
                            glass.cuboid(Vec3::new(fx + 0.47, 0.9, fz + 0.12), Vec3::new(fx + 0.53, 2.1, fz + 0.88), [1.0, 1.0, 1.0, 1.0]);
                        }
                        let oz = if y == b.y { fz + WIN - 0.04 } else { fz + 1.0 - WIN };
                        match s.arch {
                            Arch::Creole | Arch::Village | Arch::Wooden if along_x => {
                                let sh = c3([fcol[1] * 0.5, fcol[2] * 0.7, fcol[0] * 0.5]);
                                wall_mb.cuboid(Vec3::new(fx - 0.25, 0.9, oz), Vec3::new(fx + 0.05, 2.1, oz + 0.04), sh);
                                wall_mb.cuboid(Vec3::new(fx + 0.95, 0.9, oz), Vec3::new(fx + 1.25, 2.1, oz + 0.04), sh);
                                // shutter slats
                                for k in 0..6 {
                                    let yy = 1.0 + k as f32 * 0.18;
                                    wall_mb.cuboid(Vec3::new(fx - 0.24, yy, oz - 0.01), Vec3::new(fx + 0.04, yy + 0.03, oz), c3([fcol[1] * 0.4, fcol[2] * 0.55, fcol[0] * 0.4]));
                                    wall_mb.cuboid(Vec3::new(fx + 0.96, yy, oz - 0.01), Vec3::new(fx + 1.24, yy + 0.03, oz), c3([fcol[1] * 0.4, fcol[2] * 0.55, fcol[0] * 0.4]));
                                }
                                if s.arch == Arch::Village {
                                    // flower box
                                    wall_mb.cuboid(Vec3::new(fx + 0.1, 0.8, oz - 0.15), Vec3::new(fx + 0.9, 0.95, oz + 0.05), beam);
                                    for k in 0..4 {
                                        let col = [[0.8, 0.1, 0.15], [0.95, 0.5, 0.6], [0.9, 0.8, 0.2], [0.8, 0.1, 0.15]][((seed as i32 + k + x) % 4) as usize];
                                        wall_mb.sphere(Vec3::new(fx + 0.2 + k as f32 * 0.2, 1.0, oz - 0.05), Vec3::splat(0.07), 6, c3(col));
                                    }
                                }
                            }
                            Arch::Stone if along_x => {
                                if seed % 4 == 0 {
                                    // boarded window
                                    wall_mb.cuboid(Vec3::new(fx + 0.1, 0.95, oz - 0.02), Vec3::new(fx + 0.9, 2.05, oz + 0.02), c3([0.3, 0.24, 0.16]));
                                }
                            }
                            _ => {}
                        }
                        if !is_back {
                            band(&mut low, 0.0, low_h, 0.0, 1.0, inner);
                        }
                    }
                    Tile::Door => {
                        band(&mut *wall_mb, 2.3, GROUND_H, 0.0, 1.0, wc);
                        band(&mut *wall_mb, 2.2, 2.35, -0.08, 1.08, trim);
                        band(&mut *wall_mb, 0.0, 2.3, 0.0, 0.07, trim);
                        band(&mut *wall_mb, 0.0, 2.3, 0.93, 1.0, trim);
                        // step
                        if along_x {
                            let sz = if y == b.y { fz + WIN - 0.3 } else { fz + 1.0 - WIN };
                            wall_mb.cuboid(Vec3::new(fx - 0.05, 0.0, sz), Vec3::new(fx + 1.05, 0.12, sz + 0.3), c3([0.5, 0.48, 0.45]));
                        }
                    }
                    _ => {}
                }
            }
        }
        // upper floors: one shell with a window grid
        if nfl > 1 {
            full.cuboid(Vec3::new(x0, GROUND_H, z0), Vec3::new(x1, hgt, z1), wc);
            for f in 1..nfl {
                let fy = GROUND_H + (f - 1) as f32 * FLOOR_H;
                // floor band
                full.cuboid(Vec3::new(x0 - 0.03, fy, z0 - 0.03), Vec3::new(x1 + 0.03, fy + 0.12, z1 + 0.03), trim);
                let step = if matches!(s.arch, Arch::HighRise) { 1.2 } else { 1.6 };
                let mut wx = x0 + 0.6;
                while wx + 0.7 < x1 - 0.3 {
                    for (zz, dz) in [(z0 - 0.02, -0.01), (z1 + 0.02, 0.01)] {
                        glass.cuboid(Vec3::new(wx, fy + 0.7, zz - 0.015 + dz), Vec3::new(wx + 0.7, fy + 2.0, zz + 0.015 + dz), [1.0, 1.0, 1.0, 1.0]);
                    }
                    wx += step;
                }
                // side windows
                let mut wz = z0 + 0.8;
                while wz + 0.7 < z1 - 0.3 {
                    for xx in [x0 - 0.02, x1 + 0.02] {
                        glass.cuboid(Vec3::new(xx - 0.015, fy + 0.7, wz), Vec3::new(xx + 0.015, fy + 2.0, wz + 0.7), [1.0, 1.0, 1.0, 1.0]);
                    }
                    wz += 2.4;
                }
            }
            full.cuboid(Vec3::new(x0 - 0.1, hgt - 0.25, z0 - 0.1), Vec3::new(x1 + 0.1, hgt, z1 + 0.1), trim);
        }
        // style details on the street facade
        match s.arch {
            Arch::Creole => {
                if matches!(b.kind, BKind::House | BKind::Apartment | BKind::Hotel | BKind::Bar | BKind::Restaurant | BKind::Club | BKind::Cabaret) && seed % 3 != 0 {
                    let iron = c3([0.07, 0.07, 0.08]);
                    let bz0 = front_z;
                    let bz1 = front_z + out * 0.8;
                    let (za, zb) = (bz0.min(bz1), bz0.max(bz1));
                    full.cuboid(Vec3::new(x0 + 0.3, 2.55, za), Vec3::new(x1 - 0.3, 2.65, zb), iron);
                    full.cuboid(Vec3::new(x0 + 0.3, 3.3, bz1 - 0.03), Vec3::new(x1 - 0.3, 3.35, bz1 + 0.03), iron);
                    let mut xx = x0 + 0.3;
                    while xx < x1 - 0.3 {
                        full.cuboid(Vec3::new(xx, 2.65, bz1 - 0.02), Vec3::new(xx + 0.03, 3.3, bz1 + 0.02), iron);
                        xx += 0.2;
                    }
                    for px in [x0 + 0.35, x1 - 0.4] {
                        full.cuboid(Vec3::new(px, 0.0, bz1 - 0.03), Vec3::new(px + 0.06, 2.55, bz1 + 0.03), iron);
                    }
                    // hanging ferns
                    for k in 0..((x1 - x0) as i32 / 3) {
                        let px = x0 + 1.5 + k as f32 * 3.0;
                        full.sphere(Vec3::new(px, 2.3, bz1), Vec3::new(0.25, 0.2, 0.25), 6, c3([0.15, 0.3, 0.12]));
                    }
                }
            }
            Arch::Brick | Arch::HighRise => {
                // fire escapes on tall buildings
                if nfl >= 3 {
                    let iron = c3([0.1, 0.1, 0.1]);
                    let fx = x0 + 1.0 + (seed % 3) as f32;
                    let ez = front_z + out * 0.02;
                    for f in 1..nfl {
                        let fy = GROUND_H + (f - 1) as f32 * FLOOR_H;
                        let (za, zb) = if front_top { (ez - 0.9, ez) } else { (ez, ez + 0.9) };
                        full.cuboid(Vec3::new(fx, fy + 0.1, za), Vec3::new(fx + 2.2, fy + 0.16, zb), iron);
                        full.cuboid(Vec3::new(fx, fy + 0.9, za + if front_top { 0.0 } else { 0.85 }), Vec3::new(fx + 2.2, fy + 0.95, za + if front_top { 0.05 } else { 0.9 }), iron);
                        // ladder diagonal
                        full.cuboid(Vec3::new(fx + 0.4 + (f % 2) as f32 * 1.0, fy - FLOOR_H + 0.4, za + 0.2), Vec3::new(fx + 0.5 + (f % 2) as f32 * 1.0, fy + 0.1, za + 0.3), iron);
                    }
                }
                if s.graffiti && seed % 2 == 0 {
                    let gz = front_z + out * 0.03;
                    let cols = [[0.9, 0.2, 0.3], [0.2, 0.7, 0.9], [0.95, 0.8, 0.2], [0.4, 0.9, 0.3], [0.8, 0.4, 0.9]];
                    for k in 0..3 {
                        let gx = x0 + 0.5 + hashf(b.x, k, 3) * (x1 - x0 - 2.0);
                        let col = cols[(seed as usize + k as usize) % cols.len()];
                        full.cuboid(Vec3::new(gx, 0.3, gz - 0.01), Vec3::new(gx + 1.2, 0.3 + 0.4 + hashf(b.y, k, 4) * 0.6, gz + 0.01), c3(col));
                    }
                }
            }
            Arch::Colonial => {
                // veranda roof over the footpath
                let vz = front_z + out * 1.6;
                let (za, zb) = (front_z.min(vz), front_z.max(vz));
                full.cuboid(Vec3::new(x0, 2.7, za), Vec3::new(x1, 2.8, zb), c3([0.62, 0.64, 0.66]));
                let mut px = x0 + 0.2;
                while px < x1 {
                    full.cuboid(Vec3::new(px, 0.0, vz - 0.05), Vec3::new(px + 0.1, 2.7, vz + 0.05), white_trim);
                    px += 2.0;
                }
                full.cuboid(Vec3::new(x0, 2.6, vz - 0.06), Vec3::new(x1, 2.7, vz + 0.06), c3([0.5, 0.3, 0.2]));
            }
            Arch::Wooden => {
                // horizontal cladding lines + white corner boards
                let mut yy = 0.4;
                while yy < hgt {
                    full.cuboid(Vec3::new(x0 - 0.02, yy, front_z - 0.02 * out.abs()), Vec3::new(x1 + 0.02, yy + 0.03, front_z + 0.02), c3([fcol[0] * 0.75, fcol[1] * 0.75, fcol[2] * 0.75]));
                    yy += 0.35;
                }
                for xx in [x0 - 0.05, x1 - 0.1] {
                    full.cuboid(Vec3::new(xx, 0.0, z0 - 0.05), Vec3::new(xx + 0.15, hgt, z0 + 0.1), white_trim);
                    full.cuboid(Vec3::new(xx, 0.0, z1 - 0.1), Vec3::new(xx + 0.15, hgt, z1 + 0.05), white_trim);
                }
            }
            Arch::Victorian => {
                // bay window tower on the upper floors
                if nfl >= 2 && b.w >= 5 {
                    let bx0 = x0 + 1.0;
                    let bz = front_z + out * 0.7;
                    let (za, zb) = (front_z.min(bz), front_z.max(bz));
                    full.cuboid(Vec3::new(bx0, GROUND_H, za), Vec3::new(bx0 + 2.0, hgt - 0.3, zb), wc);
                    for f in 1..nfl {
                        let fy = GROUND_H + (f - 1) as f32 * FLOOR_H;
                        glass.cuboid(Vec3::new(bx0 + 0.2, fy + 0.6, bz - 0.02), Vec3::new(bx0 + 1.8, fy + 2.0, bz + 0.02), [1.0, 1.0, 1.0, 1.0]);
                    }
                    roof.gable(bx0 - 0.1, za - 0.1, bx0 + 2.1, zb + 0.1, hgt - 0.3, 0.8, c3([fcol[2] * 0.7, fcol[0] * 0.6, fcol[1] * 0.6]));
                }
                // gingerbread trim
                full.cuboid(Vec3::new(x0, hgt - 0.4, front_z - 0.05), Vec3::new(x1, hgt - 0.3, front_z + 0.05), white_trim);
            }
            Arch::Stucco => {
                // awning
                let (dx, _) = b.doors[0];
                let az = front_z + out * 1.2;
                let (za, zb) = (front_z.min(az), front_z.max(az));
                let col = [[0.7, 0.15, 0.1], [0.1, 0.4, 0.5], [0.85, 0.6, 0.2], [0.2, 0.5, 0.25]][(seed % 4) as usize];
                full.cuboid(Vec3::new(dx as f32 - 1.5, 2.4, za), Vec3::new(dx as f32 + 2.5, 2.5, zb), c3(col));
            }
            Arch::Terrace | Arch::Stone | Arch::Village => {}
        }
        // roof
        let (rx0, rz0, rx1, rz1) = (x0 - 0.1, z0 - 0.1, x1 + 0.1, z1 + 0.1);
        let mut rc = b.kind.roof_color();
        if s.snow {
            rc = [0.78, 0.8, 0.84];
        } else if s.arch == Arch::Wooden || s.arch == Arch::Village {
            rc = [0.25, 0.17, 0.14];
        } else if s.arch == Arch::Colonial {
            rc = [0.55, 0.57, 0.6];
        } else if s.arch == Arch::Stucco {
            rc = [0.55, 0.3, 0.2];
        }
        let rc_raw = rc;
        let rc = c3(rc);
        // flat tar roofs: the dark asphalt texture already carries the tone, so lift the tint
        let rc_flat = c3([(rc_raw[0] * 2.6).clamp(0.55, 1.0), (rc_raw[1] * 2.6).clamp(0.55, 1.0), (rc_raw[2] * 2.6).clamp(0.55, 1.0)]);
        let gabled =matches!(b.kind, BKind::Church | BKind::Barn | BKind::Farmhouse)
            || match s.arch {
                Arch::Village | Arch::Wooden => true,
                Arch::Terrace => false,
                Arch::Victorian => seed % 2 == 0,
                Arch::Creole | Arch::Colonial => matches!(b.kind, BKind::House | BKind::Mansion) && seed % 2 == 0,
                Arch::Stucco => seed % 4 == 0,
                _ => false,
            };
        gabled_roof = gabled;
        if gabled {
            let steep = matches!(s.arch, Arch::Village | Arch::Wooden) || b.kind == BKind::Church;
            let rise = ((rz1 - rz0).min(rx1 - rx0) * if steep { 0.6 } else { 0.35 }).min(if steep { 4.0 } else { 2.5 });
            roof.cuboid(Vec3::new(rx0, hgt, rz0), Vec3::new(rx1, hgt + 0.1, rz1), rc);
            roof.gable(rx0, rz0, rx1, rz1, hgt + 0.1, rise, rc);
            if b.kind == BKind::Church {
                let tx = (b.x + b.w / 2) as f32;
                let tz = if front_top { z0 + 1.0 } else { z1 - 2.0 };
                let tower_col = if s.arch == Arch::Village { c3([0.85, 0.82, 0.74]) } else { c3([0.55, 0.53, 0.5]) };
                roof.cuboid(Vec3::new(tx - 1.0, hgt, tz - 0.5), Vec3::new(tx + 1.0, hgt + 5.0, tz + 1.5), tower_col);
                if s.arch == Arch::Village {
                    // onion dome
                    roof.sphere(Vec3::new(tx, hgt + 5.8, tz + 0.5), Vec3::new(1.0, 0.9, 1.0), 10, c3([0.2, 0.3, 0.25]));
                    roof.frustum(Vec3::new(tx, hgt + 6.5, tz + 0.5), 0.3, 0.02, 1.2, 8, c3([0.2, 0.3, 0.25]));
                } else {
                    roof.frustum(Vec3::new(tx, hgt + 5.0, tz + 0.5), 1.3, 0.05, 3.0, 4, rc);
                }
                roof.cuboid(Vec3::new(tx - 0.05, hgt + 7.8, tz + 0.45), Vec3::new(tx + 0.05, hgt + 8.8, tz + 0.55), c3([0.7, 0.6, 0.3]));
                roof.cuboid(Vec3::new(tx - 0.3, hgt + 8.4, tz + 0.45), Vec3::new(tx + 0.3, hgt + 8.5, tz + 0.55), c3([0.7, 0.6, 0.3]));
            }
        } else {
            roof.cuboid(Vec3::new(rx0, hgt, rz0), Vec3::new(rx1, hgt + 0.12, rz1), rc_flat);
            roof.cuboid(Vec3::new(rx0, hgt, rz0), Vec3::new(rx1, hgt + 0.45, rz0 + 0.15), trim);
            roof.cuboid(Vec3::new(rx0, hgt, rz1 - 0.15), Vec3::new(rx1, hgt + 0.45, rz1), trim);
            roof.cuboid(Vec3::new(rx0, hgt, rz0), Vec3::new(rx0 + 0.15, hgt + 0.45, rz1), trim);
            roof.cuboid(Vec3::new(rx1 - 0.15, hgt, rz0), Vec3::new(rx1, hgt + 0.45, rz1), trim);
            if matches!(s.arch, Arch::Brick | Arch::HighRise) && b.w >= 7 && seed % 2 == 0 {
                let tx = rx0 + 2.0;
                let tz = rz0 + 1.5;
                roof.cylinder(Vec3::new(tx, hgt + 0.1, tz), 0.6, 1.4, 10, c3([0.28, 0.2, 0.15]));
                roof.frustum(Vec3::new(tx, hgt + 1.5, tz), 0.65, 0.05, 0.5, 10, c3([0.2, 0.15, 0.12]));
                for (lx, lz) in [(-0.4, -0.4), (0.4, -0.4), (-0.4, 0.4), (0.4, 0.4)] {
                    roof.cuboid(Vec3::new(tx + lx - 0.04, hgt, tz + lz - 0.04), Vec3::new(tx + lx + 0.04, hgt + 0.2, tz + lz + 0.04), c3([0.1, 0.1, 0.1]));
                }
            }
            if s.arch == Arch::Stucco && seed % 3 == 0 {
                // rooftop sign
                let col = [[0.9, 0.9, 0.85], [0.95, 0.8, 0.2]][(seed % 2) as usize];
                roof.cuboid(Vec3::new(rx0 + 1.0, hgt + 0.4, rz0 + 0.3), Vec3::new(rx1 - 1.0, hgt + 1.6, rz0 + 0.4), c3(col));
            }
        }
        // chimneys: London terraces have rows of chimney pots
        let chim = match s.arch {
            Arch::Terrace => 2 + (b.w / 4),
            Arch::Brick | Arch::Village | Arch::Wooden => 1,
            Arch::Creole | Arch::Victorian => (seed % 2) as i32,
            _ => 0,
        };
        for k in 0..chim {
            let cx = rx0 + 1.0 + k as f32 * ((rx1 - rx0 - 2.0) / chim.max(1) as f32);
            let cz = if front_top { rz1 - 1.2 } else { rz0 + 0.7 };
            roof.cuboid(Vec3::new(cx, hgt, cz), Vec3::new(cx + 0.6, hgt + 1.6, cz + 0.5), c3([0.3, 0.18, 0.15]));
            if s.arch == Arch::Terrace {
                for p in 0..2 {
                    roof.cylinder(Vec3::new(cx + 0.15 + p as f32 * 0.3, hgt + 1.6, cz + 0.25), 0.07, 0.35, 6, c3([0.55, 0.35, 0.25]));
                }
            }
        }
        if b.kind == BKind::Factory {
            roof.cylinder(Vec3::new(rx1 - 1.5, hgt, rz0 + 1.5), 0.6, 9.0, 10, c3([0.35, 0.2, 0.16]));
        }
        // signs: painted boards before neon, neon after
        if matches!(b.kind, BKind::Bar | BKind::Club | BKind::Cabaret | BKind::Hotel | BKind::Restaurant | BKind::General | BKind::Pharmacy | BKind::Pawn | BKind::GunShop | BKind::Clothing) {
            let (dx, _) = b.doors[0];
            let sz = if front_top { z0 - 0.12 } else { z1 + 0.02 };
            let neon = match b.kind {
                BKind::Club => c3([1.0, 0.1, 0.6]),
                BKind::Cabaret => c3([1.0, 0.05, 0.1]),
                BKind::Hotel => c3([0.2, 0.6, 1.0]),
                BKind::Pharmacy => c3([0.2, 1.0, 0.4]),
                _ => c3([1.0, 0.6, 0.2]),
            };
            if s.neon && matches!(b.kind, BKind::Bar | BKind::Club | BKind::Cabaret | BKind::Hotel | BKind::Restaurant | BKind::Pharmacy) {
                glow.cuboid(Vec3::new(dx as f32 - 0.8, 2.5, sz), Vec3::new(dx as f32 + 1.8, 2.9, sz + 0.1), neon);
                if s.arch == Arch::HighRise || s.arch == Arch::Stucco {
                    // vertical blade sign
                    glow.cuboid(Vec3::new(dx as f32 + 2.0, 2.8, sz - 0.4 * out.abs()), Vec3::new(dx as f32 + 2.15, 5.0, sz + 0.1), neon);
                }
            } else {
                full.cuboid(Vec3::new(dx as f32 - 0.8, 2.45, sz), Vec3::new(dx as f32 + 1.8, 2.95, sz + 0.1), c3([0.2, 0.12, 0.08]));
                full.cuboid(Vec3::new(dx as f32 - 0.7, 2.55, sz - 0.01), Vec3::new(dx as f32 + 1.7, 2.85, sz), c3([0.75, 0.65, 0.4]));
                glow.cuboid(Vec3::new(dx as f32 - 0.1, 2.3, sz - 0.2), Vec3::new(dx as f32 + 0.1, 2.45, sz), c3([1.0, 0.75, 0.4]));
            }
        }
        // ruined / abandoned look
        if b.kind == BKind::Abandoned && s.ruins > 0.0 {
            roof.clear_to_ruin();
        }
    }
    // ---------------------------------------------------------------- interior finish: wallpaper, skirting, pictures, curtains
    let mut lining = MB::new();
    let mut lining_low = MB::new();
    // things hung on the walls disappear together with the walls when cut away
    let mut deco = MB::new();
    let mut lining_back = MB::new();
    let mut deco_back = MB::new();
    // real wall pieces go under these (the front one hides with the cut-away walls)
    let hang_front = c.spawn((Transform::default(), Visibility::Inherited)).id();
    let hang_back = c.spawn((Transform::default(), Visibility::Inherited)).id();
    c.entity(root).add_child(hang_back);
    let room_style = building_style(b, m, year);
    if !is_market && !matches!(b.kind, BKind::Barn | BKind::Warehouse | BKind::Abandoned | BKind::Factory) {
        let paper = room_paper(b, year);
        let skirting = c3([paper[0] * 0.35, paper[1] * 0.3, paper[2] * 0.28]);
        let pc = c3(paper);
        for y in b.y..b.y + b.h {
            for x in b.x..b.x + b.w {
                if m.get(x, y) != Tile::Floor || m.building_at_tile(x, y) != Some(b.id) {
                    continue;
                }
                let (fx, fz) = (x as f32, y as f32);
                for (dx, dy) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
                    let t = m.get(x + dx, y + dy);
                    if !matches!(t, Tile::Wall | Tile::Window | Tile::Door) {
                        continue;
                    }
                    let (nx, ny) = (x + dx, y + dy);
                    let nb = (nx == b.x || ny == b.y) && nx != b.x + b.w - 1 && ny != b.y + b.h - 1;
                    let (lin, dec): (&mut MB, &mut MB) = if nb { (&mut lining_back, &mut deco_back) } else { (&mut lining, &mut deco) };
                    let holder = if nb { hang_back } else { hang_front };
                    // the inner face of the thin wall in the neighbouring tile
                    let (p0, p1) = match (dx, dy) {
                        (1, 0) => (Vec3::new(fx + 1.0 + WIN - 0.02, 0.0, fz), Vec3::new(fx + 1.0 + WIN, 0.0, fz + 1.0)),
                        (-1, 0) => (Vec3::new(fx - WIN, 0.0, fz), Vec3::new(fx - WIN + 0.02, 0.0, fz + 1.0)),
                        (0, 1) => (Vec3::new(fx, 0.0, fz + 1.0 + WIN - 0.02), Vec3::new(fx + 1.0, 0.0, fz + 1.0 + WIN)),
                        _ => (Vec3::new(fx, 0.0, fz - WIN), Vec3::new(fx + 1.0, 0.0, fz - WIN + 0.02)),
                    };
                    let slab = |mb: &mut MB, y0: f32, y1: f32, col: [f32; 4]| mb.cuboid(Vec3::new(p0.x, y0, p0.z), Vec3::new(p1.x, y1, p1.z), col);
                    let into = Vec3::new(-dx as f32, 0.0, -dy as f32);
                    let mid = (p0 + p1) * 0.5 + into * 0.012;
                    match t {
                        Tile::Wall => {
                            slab(&mut *lin, 0.12, GROUND_H - 0.02, pc);
                            slab(&mut lining_low, 0.12, low_h + 0.01, pc);
                            slab(&mut furn, 0.0, 0.14, skirting);
                            // wall decoration
                            let r = hash2(x * 7 + dx, y * 5 + dy, 404) % 100;
                            let side = if dx != 0 { Vec3::Z } else { Vec3::X };
                            let deco_c = mid + into * 0.02;
                            match r {
                                0..=13 => {
                                    let wanted = if matches!(b.kind, BKind::Bar | BKind::Club) && r < 3 { "dartboard" } else { "picture" };
                                    if !hang(c, lib, wanted, year, hash2(x, y, 405), room_style, deco_c + Vec3::Y * 1.55, -into, holder) {
                                        picture(&mut *dec, deco_c + Vec3::Y * 1.55, side, into, hash2(x, y, 405), year);
                                    }
                                }
                                14..=17 if hang(c, lib, "clock_wall", year, hash2(x, y, 406), room_style, deco_c + Vec3::Y * 1.9, -into, holder) => {}
                                28..=29 if hang(c, lib, "mirror", year, hash2(x, y, 407), room_style, deco_c + Vec3::Y * 1.5, -into, holder) => {}
                                14..=17 if year >= 1900 => {
                                    // wall clock
                                    dec.bx(deco_c + Vec3::Y * 1.9, side * 0.16 + Vec3::Y * 0.16 + into * 0.03, c3([0.3, 0.18, 0.1]));
                                    dec.bx(deco_c + Vec3::Y * 1.9 + into * 0.035, side * 0.12 + Vec3::Y * 0.12 + into * 0.005, c3([0.92, 0.9, 0.82]));
                                    dec.bx(deco_c + Vec3::Y * 1.93 + into * 0.045, side * 0.008 + Vec3::Y * 0.07 + into * 0.004, c3([0.05, 0.05, 0.05]));
                                }
                                18..=24 => {
                                    // small wall shelf with objects
                                    dec.bx(deco_c + Vec3::Y * 1.45 + into * 0.1, side * 0.4 + Vec3::Y * 0.02 + into * 0.1, c3([0.35, 0.22, 0.13]));
                                    for k in 0..4 {
                                        let off = side * (-0.3 + k as f32 * 0.2) + into * 0.1 + Vec3::Y * 1.47;
                                        let hh = 0.08 + (hash2(x, k, 406) % 10) as f32 * 0.015;
                                        let col = [[0.6, 0.15, 0.12], [0.2, 0.3, 0.5], [0.85, 0.82, 0.7], [0.25, 0.4, 0.25]][(hash2(y, k, 407) % 4) as usize];
                                        dec.bx(deco_c + off + Vec3::Y * hh, side * 0.05 + Vec3::Y * hh + into * 0.05, c3(col));
                                    }
                                }
                                25..=27 => {
                                    // sconce lamp
                                    dec.bx(deco_c + Vec3::Y * 1.8 + into * 0.05, side * 0.04 + Vec3::Y * 0.06 + into * 0.05, c3([0.7, 0.55, 0.25]));
                                    dec.sphere(deco_c + Vec3::Y * 1.92 + into * 0.12, Vec3::new(0.07, 0.09, 0.07), 8, c3([1.0, 0.8, 0.5]));
                                }
                                28..=29 => {
                                    // mirror
                                    dec.bx(deco_c + Vec3::Y * 1.5, side * 0.3 + Vec3::Y * 0.4 + into * 0.02, c3([0.3, 0.2, 0.1]));
                                    glass.bx(deco_c + Vec3::Y * 1.5 + into * 0.022, side * 0.26 + Vec3::Y * 0.36 + into * 0.004, [1.0, 1.0, 1.0, 1.0]);
                                }
                                _ => {}
                            }
                        }
                        Tile::Window => {
                            slab(&mut *lin, 0.12, 0.9, pc);
                            slab(&mut *lin, 2.1, GROUND_H - 0.02, pc);
                            slab(&mut lining_low, 0.12, low_h + 0.01, pc);
                            slab(&mut furn, 0.0, 0.14, skirting);
                            // curtains, rod and a sill
                            let side = if dx != 0 { Vec3::Z } else { Vec3::X };
                            let cc = curtain_col(b, year);
                            let base = mid + into * 0.05;
                            for sgn in [-1.0f32, 1.0] {
                                let c0 = base + side * sgn * 0.42;
                                for k in 0..3 {
                                    let off = side * sgn * (k as f32 * 0.035) + into * ((k % 2) as f32 * 0.025);
                                    dec.bx(c0 + off + Vec3::Y * 1.45, side * 0.03 + Vec3::Y * 0.95 + into * 0.015, c3(cc));
                                }
                            }
                            dec.bx(base + Vec3::Y * 2.35, side * 0.55 + Vec3::Y * 0.012 + into * 0.012, c3([0.55, 0.45, 0.25]));
                            dec.bx(base + Vec3::Y * 2.3, side * 0.5 + Vec3::Y * 0.08 + into * 0.02, c3([cc[0] * 0.8, cc[1] * 0.8, cc[2] * 0.8]));
                            dec.bx(mid + Vec3::Y * 0.9 + into * 0.08, side * 0.5 + Vec3::Y * 0.02 + into * 0.08, c3([0.85, 0.83, 0.78]));
                        }
                        _ => {
                            slab(&mut *lin, 2.3, GROUND_H - 0.02, pc);
                        }
                    }
                }
            }
        }
    }
    for p in m.props.iter().filter(|p| p.building == Some(b.id)) {
        let floor_y = if is_market { 0.12 } else { 0.1 };
        let style = building_style(b, m, year);
        if let Some(e) = real_model(c, lib, p, year, floor_y, root, style) {
            let _ = e;
            dress_top(c, lib, p, year, floor_y, root, style, b);
            continue;
        }
        let (gs, gg) = prop_geo(p, year, s, false);
        let off = Vec3::new(p.x as f32, floor_y, p.y as f32);
        furn.append(&gs, off);
        glow.append(&gg, off);
    }
    let mut spawn = |mb: MB, mat: Handle<StandardMaterial>, visible: bool, shadow: bool| -> Entity {
        let has = !mb.is_empty();
        let mut e = c.spawn((Transform::default(), if visible { Visibility::Inherited } else { Visibility::Hidden }));
        if has {
            e.insert((Mesh3d(meshes.add(mb.build())), MeshMaterial3d(mat)));
        }
        if !shadow {
            e.insert(NotShadowCaster);
        }
        let id = e.id();
        c.entity(root).add_child(id);
        id
    };
    let wall_mat = match wall_kind {
        WallKind::Brick => mats.wall.clone(),
        WallKind::YellowBrick => mats.brick_yellow.clone(),
        WallKind::Stone => mats.stone_wall.clone(),
        WallKind::Siding => mats.siding.clone(),
        WallKind::Plaster => mats.plaster.clone(),
    };
    let full_e = spawn(full, wall_mat.clone(), true, true);
    let low_e = spawn(low, mats.plaster.clone(), false, false);
    let lining_e = spawn(lining, mats.wallpaper.clone(), true, false);
    let deco_e = spawn(deco, mats.furn.clone(), true, true);
    spawn(back, wall_mat.clone(), true, true);
    spawn(lining_back, mats.wallpaper.clone(), true, false);
    spawn(deco_back, mats.furn.clone(), true, true);
    let lining_low_e = spawn(lining_low, mats.wallpaper.clone(), true, false);
    let _ = &wall_mat;
    let slate_city = matches!(s.arch, Arch::Terrace | Arch::Stone | Arch::Wooden | Arch::Victorian);
    let roof_e = spawn(roof, if !gabled_roof { mats.roof_flat.clone() } else if slate_city { mats.slate.clone() } else { mats.roof.clone() }, true, true);
    let glass_e = spawn(glass, mats.glass_dark.clone(), true, false);
    spawn(furn, mats.furn.clone(), true, true);
    let glow_e = spawn(glow, mats.glow.clone(), true, false);
    // signs and window glass disappear together with the walls when cut away
    c.entity(full_e).add_children(&[glass_e, glow_e, lining_e, deco_e, hang_front]);
    c.entity(low_e).add_child(lining_low_e);
    if !is_market {
        for &(dx, dy) in &b.doors {
            let axis_x = dy == b.y || dy == b.y + b.h - 1;
            let hinge = if axis_x { Vec3::new(dx as f32 + 0.05, 0.1, dy as f32 + 0.5) } else { Vec3::new(dx as f32 + 0.5, 0.1, dy as f32 + 0.05) };
            let mut mb = MB::new();
            let dcol = match s.arch {
                Arch::Creole | Arch::Victorian => c3([fcol[2] * 0.6, fcol[0] * 0.5, fcol[1] * 0.5]),
                Arch::Wooden => c3([0.85, 0.85, 0.8]),
                _ => c3([0.22, 0.13, 0.09]),
            };
            if axis_x {
                mb.cuboid(Vec3::new(0.0, 0.0, -0.05), Vec3::new(0.9, 2.1, 0.05), dcol);
                mb.cuboid(Vec3::new(0.7, 1.0, -0.08), Vec3::new(0.78, 1.08, 0.08), c3([0.7, 0.6, 0.3]));
            } else {
                mb.cuboid(Vec3::new(-0.05, 0.0, 0.0), Vec3::new(0.05, 2.1, 0.9), dcol);
            }
            let e = c
                .spawn((Mesh3d(meshes.add(mb.build())), MeshMaterial3d(mats.furn.clone()), Transform::from_translation(hinge), DoorPanel { tile: (dx, dy), building: b.id, open: 0.0, hinge, axis_x }))
                .id();
            c.entity(root).add_child(e);
        }
    }
    BVis { full: full_e, low: low_e, roof: roof_e, glass: glass_e, cut: false, lit: false, height: hgt }
}


/// Build a prop's geometry in its own orientation and place it on its footprint.
fn prop_geo(p: &Prop, year: i32, s: &Style, snow: bool) -> (MB, MB) {
    let odd = p.rot % 2 == 1;
    let (lw, lh) = if odd { (p.h as f32, p.w as f32) } else { (p.w as f32, p.h as f32) };
    let geo = props::build(p.kind, lw, lh, p.tint, year, hash2(p.x, p.y, 5), s.tree, snow);
    if p.rot == 0 && !p.kind.back_neg_z() {
        return (geo.solid, geo.glow);
    }
    // local back is +z (or -z); turn it towards the wall side `rot`
    let base = [0.0, std::f32::consts::FRAC_PI_2, std::f32::consts::PI, -std::f32::consts::FRAC_PI_2][(p.rot % 4) as usize];
    let ang = base + if p.kind.back_neg_z() { std::f32::consts::PI } else { 0.0 };
    let centre = Vec3::new(lw / 2.0, 0.0, lh / 2.0);
    let mut solid = MB::new();
    let mut glow = MB::new();
    let mut shift = |src: &MB, dst: &mut MB| {
        let mut c = src.clone();
        for v in c.pos.iter_mut() {
            v[0] -= centre.x;
            v[2] -= centre.z;
        }
        dst.append_rot(&c, ang, Vec3::new(p.w as f32 / 2.0, 0.0, p.h as f32 / 2.0));
    };
    shift(&geo.solid, &mut solid);
    shift(&geo.glow, &mut glow);
    (solid, glow)
}

/// Wallpaper / paint colour of a building's rooms, by era.
fn room_paper(b: &Building, year: i32) -> [f32; 3] {
    let pal: &[[f32; 3]] = if year < 1935 {
        &[[0.55, 0.62, 0.5], [0.62, 0.35, 0.33], [0.8, 0.72, 0.55], [0.45, 0.5, 0.58], [0.72, 0.62, 0.5]]
    } else if year < 1960 {
        &[[0.82, 0.78, 0.62], [0.68, 0.78, 0.72], [0.85, 0.7, 0.68], [0.75, 0.75, 0.8], [0.88, 0.85, 0.75]]
    } else if year < 1980 {
        &[[0.8, 0.6, 0.35], [0.6, 0.55, 0.3], [0.85, 0.78, 0.55], [0.55, 0.4, 0.3], [0.7, 0.62, 0.45]]
    } else {
        &[[0.9, 0.88, 0.84], [0.85, 0.82, 0.75], [0.78, 0.82, 0.85], [0.88, 0.8, 0.78], [0.8, 0.85, 0.8]]
    };
    let mut c = pal[(hash2(b.x, b.y, 410) as usize) % pal.len()];
    if matches!(b.kind, BKind::Police | BKind::Hospital | BKind::Office | BKind::Station | BKind::Newspaper) {
        c = [0.78, 0.8, 0.76];
    }
    if matches!(b.kind, BKind::Bar | BKind::Club | BKind::Cabaret) {
        c = [c[0] * 0.6, c[1] * 0.45, c[2] * 0.4];
    }
    c
}

fn curtain_col(b: &Building, year: i32) -> [f32; 3] {
    let pal: &[[f32; 3]] = if year < 1950 { &[[0.55, 0.12, 0.12], [0.3, 0.4, 0.3], [0.85, 0.82, 0.75], [0.6, 0.5, 0.3]] } else { &[[0.9, 0.85, 0.7], [0.75, 0.45, 0.2], [0.4, 0.5, 0.6], [0.95, 0.95, 0.92]] };
    pal[(hash2(b.x, b.y, 411) as usize) % pal.len()]
}

/// A framed painting or photograph (landscape, portrait or poster by era).
fn picture(mb: &mut MB, c: Vec3, side: Vec3, into: Vec3, seed: u32, year: i32) {
    let (w, h) = if seed % 3 == 0 { (0.28, 0.36) } else { (0.42, 0.3) };
    let frame = [[0.45, 0.3, 0.12], [0.1, 0.07, 0.05], [0.75, 0.6, 0.25], [0.2, 0.2, 0.2]][(seed % 4) as usize];
    mb.bx(c, side * (w + 0.04) + Vec3::Y * (h + 0.04) + into * 0.015, c3(frame));
    // canvas: sky and ground bands (landscape) or a dark portrait
    let photo = year > 1945 && seed % 2 == 0;
    let (top, bot) = match seed % 5 {
        0 => ([0.55, 0.65, 0.75], [0.3, 0.4, 0.2]),
        1 => ([0.7, 0.55, 0.35], [0.35, 0.25, 0.15]),
        2 => ([0.2, 0.25, 0.35], [0.12, 0.12, 0.15]),
        3 => ([0.8, 0.75, 0.6], [0.5, 0.35, 0.3]),
        _ => ([0.4, 0.5, 0.6], [0.6, 0.55, 0.4]),
    };
    let (top, bot) = if photo { ([0.75, 0.73, 0.7], [0.3, 0.29, 0.28]) } else { (top, bot) };
    mb.bx(c + into * 0.017 + Vec3::Y * h * 0.4, side * w + Vec3::Y * h * 0.6 + into * 0.003, c3(top));
    mb.bx(c + into * 0.017 - Vec3::Y * h * 0.55, side * w + Vec3::Y * h * 0.45 + into * 0.003, c3(bot));
    if seed % 5 == 2 {
        // portrait: a figure in the middle
        mb.sphere(c + into * 0.022 + Vec3::Y * 0.05, Vec3::new(0.07, 0.09, 0.01), 8, c3([0.75, 0.6, 0.5]));
        mb.bx(c + into * 0.022 - Vec3::Y * 0.15, side * 0.12 + Vec3::Y * 0.1 + into * 0.005, c3([0.1, 0.1, 0.12]));
    }
}


/// Model category for a prop kind, and whether it keeps its natural size (trees, hydrants...).
fn model_category(k: PKind) -> Option<(&'static str, bool)> {
    use PKind::*;
    Some(match k {
        Bed => ("bed", false),
        Nightstand => ("nightstand", false),
        Wardrobe => ("wardrobe", false),
        Shelf => ("shelf", false),
        Chair => ("chair", false),
        Stool => ("stool", false),
        Armchair => ("armchair", false),
        Sofa => ("sofa", false),
        Table => ("table", false),
        Desk => ("desk", false),
        FloorLamp => ("lamp_floor", true),
        Plant => ("plant", true),
        Tv => ("tv", false),
        RadioSet => ("radio", true),
        Barrel => ("barrel", true),
        Crate => ("crate", true),
        Dumpster => ("trash_can", true),
        Hydrant => ("hydrant", true),
        Tree => ("tree", true),
        Bench => ("bench", false),
        _ => return None,
    })
}

/// Rich / period / regional style tag for picking furniture models.
fn building_style(b: &Building, m: &Map, year: i32) -> Option<&'static str> {
    let wealth = m.districts.get(b.district).map(|d| d.wealth).unwrap_or(0.5);
    if m.districts.get(b.district).map(|d| d.name.contains("Chinatown")).unwrap_or(false) {
        return Some("chinese");
    }
    if matches!(b.kind, BKind::Bar | BKind::Club | BKind::Cabaret | BKind::Restaurant) {
        return Some("bar");
    }
    if matches!(b.kind, BKind::Police | BKind::Office | BKind::Newspaper | BKind::Bank) {
        return Some("office");
    }
    if matches!(b.kind, BKind::Mansion) || wealth > 0.72 {
        return Some(if year < 1945 { "gothic" } else { "rich" });
    }
    if wealth < 0.35 {
        return Some("rustic");
    }
    None
}

/// Spawn the real photographed model for a prop if the library has one for this era.
fn real_model(c: &mut Commands, lib: Option<&super::lib3d::Lib>, p: &Prop, year: i32, base_y: f32, root: Entity, style: Option<&str>) -> Option<Entity> {
    let lib = lib?;
    let (cat, natural) = model_category(p.kind)?;
    let seed = hash2(p.x, p.y, 777);
    let (e, h) = lib.pick(cat, year, seed, style)?;
    let e = &e;
    let centre = Vec3::new(p.x as f32 + p.w as f32 / 2.0, base_y, p.y as f32 + p.h as f32 / 2.0);
    let rot = if p.kind.back_neg_z() { (p.rot + 2) % 4 } else { p.rot };
    // chairs/beds were authored with the back on -z in the procedural set; the models all have it on +z
    let rot = if p.kind.back_neg_z() { (rot + 2) % 4 } else { rot };
    let tr = if natural {
        let ang = (seed % 16) as f32 * 0.39;
        // trees: a believable street/park tree height, whatever the source size
        let k = if p.kind == PKind::Tree { (6.0 + (seed % 7) as f32 * 0.6) / e.size[1].max(0.5) } else { 1.0 };
        Transform::from_translation(centre).with_rotation(Quat::from_rotation_y(ang)).with_scale(Vec3::splat(k))
    } else {
        super::lib3d::fit(e, centre, p.w as f32 * 0.96, p.h as f32 * 0.96, rot, 1.0)
    };
    let ent = c.spawn((SceneRoot(h.clone()), tr)).id();
    if p.kind == PKind::Tree {
        c.entity(ent).insert(TreeCanopy);
    }
    c.entity(root).add_child(ent);
    Some(ent)
}


/// Put real objects on tables, desks, nightstands and counters.
#[allow(clippy::too_many_arguments)]
fn dress_top(c: &mut Commands, lib: Option<&super::lib3d::Lib>, p: &Prop, year: i32, floor_y: f32, root: Entity, style: Option<&str>, b: &Building) {
    let Some(lib) = lib else { return };
    let seed = hash2(p.x, p.y, 991);
    let (h, cats): (f32, &[&str]) = match p.kind {
        PKind::Nightstand => (0.6, &["lamp_table", "clock_table", "books"]),
        PKind::Table => {
            if matches!(b.kind, BKind::Bar | BKind::Restaurant | BKind::Club | BKind::Cabaret) {
                (0.76, &["bottles", "tableware", "lamp_table"])
            } else {
                (0.76, &["vase", "tableware", "books", "lamp_table", "kettle"])
            }
        }
        PKind::Desk => (0.76, &["lamp_table", "books", "radio"]),
        PKind::Counter => (1.08, &["cash_register", "bottles", "tableware"]),
        _ => return,
    };
    if seed % 3 == 0 && p.kind != PKind::Counter {
        return;
    }
    let cat = cats[(seed as usize / 3) % cats.len()];
    let Some((e, hnd)) = lib.pick(cat, year, seed, style) else { return };
    if e.size[0] > 0.8 || e.size[2] > 0.8 {
        return;
    }
    let jitter = Vec3::new(((seed >> 4) % 5) as f32 * 0.06 - 0.12, 0.0, ((seed >> 7) % 5) as f32 * 0.06 - 0.12);
    let pos = Vec3::new(p.x as f32 + p.w as f32 / 2.0, floor_y + h, p.y as f32 + p.h as f32 / 2.0) + jitter;
    let ent = c.spawn((SceneRoot(hnd), Transform::from_translation(pos).with_rotation(Quat::from_rotation_y((seed % 8) as f32 * 0.78)))).id();
    c.entity(root).add_child(ent);
}

/// A real wall piece (clock, picture, mirror, dartboard) hung at `pos` with its back towards `back`.
fn hang(c: &mut Commands, lib: Option<&super::lib3d::Lib>, cat: &str, year: i32, seed: u32, style: Option<&str>, pos: Vec3, back: Vec3, parent: Entity) -> bool {
    let Some(lib) = lib else { return false };
    let Some((e, hnd)) = lib.pick(cat, year, seed, style) else { return false };
    // models face -z with the back at +z: turn +z towards the wall
    let ang = back.x.atan2(back.z);
    let pos = pos - back * (e.size[2] * 0.5);
    let ent = c.spawn((SceneRoot(hnd), Transform::from_translation(pos).with_rotation(Quat::from_rotation_y(ang)))).id();
    c.entity(parent).add_child(ent);
    true
}

trait RuinRoof {
    fn clear_to_ruin(&mut self);
}
impl RuinRoof for MB {
    /// Bombed buildings lose their roof.
    fn clear_to_ruin(&mut self) {
        *self = MB::new();
    }
}

pub fn tree_kind_default() -> TreeKind {
    TreeKind::Oak
}
