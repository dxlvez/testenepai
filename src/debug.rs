//! Debug commands (used by automated playtest scripts): `cmd=time:12`,
//! `cmd=rain:0`, `cmd=tp:x,y`, `cmd=zoom:20`, `cmd=yaw:1.57`, ...

use crate::camera::CamState;
use crate::keys::Script;
use crate::state::Game;
use bevy::prelude::*;

pub fn debug_cmds(mut s: ResMut<Script>, mut game: ResMut<Game>, mut cam: ResMut<CamState>, map: Option<Res<crate::world::CityMap>>) {
    let cmds: Vec<String> = s.cmds.drain(..).collect();
    for c in cmds {
        let (k, v) = c.split_once(':').unwrap_or((c.as_str(), ""));
        match k {
            "time" => {
                if let Ok(h) = v.parse::<f32>() {
                    game.minute = h * 60.0;
                }
            }
            "rain" => {
                if let Ok(r) = v.parse::<f32>() {
                    game.rain = r;
                    game.rain_target = r;
                }
            }
            "fog" => {
                if let Ok(r) = v.parse::<f32>() {
                    game.fog = r;
                }
            }
            "tp" => {
                let mut it = v.split(',');
                if let (Some(Ok(x)), Some(Ok(y))) = (it.next().map(|a| a.parse::<f32>()), it.next().map(|a| a.parse::<f32>())) {
                    game.player.pos = Vec2::new(x, y);
                    cam.focus = Vec3::new(x, 0.8, y);
                }
            }
            "tpb" => {
                // teleport in front of the first building of a kind
                if let Some(m) = &map {
                    let k = v.to_lowercase();
                    if let Some(b) = m.0.buildings.iter().find(|b| format!("{:?}", b.kind).to_lowercase() == k) {
                        game.player.pos = b.outside_px();
                        cam.focus = Vec3::new(game.player.pos.x, 0.8, game.player.pos.y);
                    }
                }
            }
            "tpin" => {
                if let Some(m) = &map {
                    let k = v.to_lowercase();
                    if let Some(b) = m.0.buildings.iter().find(|b| format!("{:?}", b.kind).to_lowercase() == k) {
                        game.player.pos = m.0.nearest_open(b.center_px());
                        cam.focus = Vec3::new(game.player.pos.x, 0.8, game.player.pos.y);
                    }
                }
            }
            "zoom" => {
                if let Ok(z) = v.parse::<f32>() {
                    cam.dist = z;
                    cam.dist_target = z;
                }
            }
            "yaw" => {
                if let Ok(z) = v.parse::<f32>() {
                    cam.yaw = z;
                    cam.yaw_target = z;
                }
            }
            "money" => {
                if let Ok(z) = v.parse::<i32>() {
                    game.player.money = z;
                }
            }
            "flag" => {
                game.flags.insert(v.to_string());
            }
            _ => warn!("unknown debug cmd {}", c),
        }
    }
}
