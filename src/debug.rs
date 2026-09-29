//! Debug commands (used by automated playtest scripts): `cmd=time:12`,
//! `cmd=rain:0`, `cmd=tp:x,y`, `cmd=zoom:20`, `cmd=yaw:1.57`, ...

use crate::camera::CamState;
use crate::keys::Script;
use crate::state::Game;
use bevy::prelude::*;

pub fn debug_cmds(mut s: ResMut<Script>, mut game: ResMut<Game>, mut cam: ResMut<CamState>, map: Option<Res<crate::world::CityMap>>, sim: Res<crate::sim::agents::Sim>) {
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
            "tpclue" => {
                let i: usize = v.parse().unwrap_or(0);
                let pos = game.cases.iter().find(|c| c.id == game.case_idx as u8).and_then(|c| c.clue_pos.get(i).copied().flatten());
                if let (Some((x, y)), Some(m)) = (pos, &map) {
                    game.player.pos = m.0.nearest_open(Vec2::new(x + 0.9, y));
                    cam.focus = Vec3::new(x, 0.8, y);
                }
            }
            "tpcast" => {
                let i: usize = v.parse().unwrap_or(0);
                let pid = game.cases.iter().find(|c| c.id == game.case_idx as u8).and_then(|c| c.cast.get(i).copied());
                if let Some(a) = pid.and_then(|p| sim.agent(p)) {
                    game.player.pos = map.as_ref().map(|m| m.0.nearest_open(a.pos + Vec2::new(0.7, 0.0))).unwrap_or(a.pos);
                    cam.focus = Vec3::new(a.pos.x, 0.8, a.pos.y);
                }
            }
            "give" => {
                use crate::items::*;
                let it = match v {
                    "revolver" => Some(Item::Weapon(Weapon::Revolver)),
                    "knife" => Some(Item::Weapon(Weapon::Knife)),
                    "ammo" => Some(Item::Ammo(24)),
                    "lockpick" => Some(Item::Tool(Tool::Lockpick)),
                    "chloroform" => Some(Item::Drug(Drug::Chloroform)),
                    "jewel" => Some(Item::Gift(Gift::Jewel)),
                    _ => None,
                };
                if let Some(it) = it {
                    match it {
                        Item::Ammo(n) => game.player.add_ammo(n),
                        other => game.player.inv.push(other),
                    }
                }
            }
            "skill" => {
                for sk in crate::state::Skill::ALL {
                    game.player.skills.insert(sk, v.parse().unwrap_or(5));
                }
            }
            "flag" => {
                game.flags.insert(v.to_string());
            }
            _ => warn!("unknown debug cmd {}", c),
        }
    }
}
