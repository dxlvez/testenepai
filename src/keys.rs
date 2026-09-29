//! Rebindable controls + scripted input for automated playtests.

use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Run,
    Walk,
    Interact,
    Echo,
    RedSight,
    Attack,
    Weapon,
    Reload,
    Drag,
    CaseBoard,
    Journal,
    Map,
    Inventory,
    Network,
    Camera,
    Photo,
    CamLeft,
    CamRight,
    Pause,
    Hide,
    Surrender,
}

impl Action {
    pub const ALL: [Action; 25] = [
        Action::Up,
        Action::Down,
        Action::Left,
        Action::Right,
        Action::Run,
        Action::Walk,
        Action::Interact,
        Action::Echo,
        Action::RedSight,
        Action::Attack,
        Action::Weapon,
        Action::Reload,
        Action::Drag,
        Action::Hide,
        Action::Surrender,
        Action::CaseBoard,
        Action::Journal,
        Action::Map,
        Action::Inventory,
        Action::Network,
        Action::Camera,
        Action::Photo,
        Action::CamLeft,
        Action::CamRight,
        Action::Pause,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Action::Up => "Andar para cima",
            Action::Down => "Andar para baixo",
            Action::Left => "Andar para a esquerda",
            Action::Right => "Andar para a direita",
            Action::Run => "Correr (segurar)",
            Action::Walk => "Andar devagar (segurar)",
            Action::Interact => "Interagir / conversar",
            Action::Echo => "Eco temporal",
            Action::RedSight => "Visão Vermelha",
            Action::Attack => "Soco / ataque corpo a corpo",
            Action::Weapon => "Sacar / guardar arma",
            Action::Reload => "Recarregar",
            Action::Drag => "Arrastar / carregar corpo",
            Action::Hide => "Esconder corpo / esconder-se",
            Action::Surrender => "Render-se à polícia",
            Action::CaseBoard => "Quadro do caso",
            Action::Journal => "Diário",
            Action::Map => "Mapa",
            Action::Inventory => "Inventário",
            Action::Network => "Rede e relações",
            Action::Camera => "Câmera fotográfica",
            Action::Photo => "Tirar foto",
            Action::CamLeft => "Girar câmera (esquerda)",
            Action::CamRight => "Girar câmera (direita)",
            Action::Pause => "Menu (salvar/carregar/ajustes)",
        }
    }
    fn default_key(self) -> KeyCode {
        match self {
            Action::Up => KeyCode::KeyW,
            Action::Down => KeyCode::KeyS,
            Action::Left => KeyCode::KeyA,
            Action::Right => KeyCode::KeyD,
            Action::Run => KeyCode::ShiftLeft,
            Action::Walk => KeyCode::AltLeft,
            Action::Interact => KeyCode::KeyE,
            Action::Echo => KeyCode::KeyQ,
            Action::RedSight => KeyCode::KeyR,
            Action::Attack => KeyCode::KeyF,
            Action::Weapon => KeyCode::KeyG,
            Action::Reload => KeyCode::KeyT,
            Action::Drag => KeyCode::KeyC,
            Action::Hide => KeyCode::KeyH,
            Action::Surrender => KeyCode::KeyB,
            Action::CaseBoard => KeyCode::Tab,
            Action::Journal => KeyCode::KeyJ,
            Action::Map => KeyCode::KeyM,
            Action::Inventory => KeyCode::KeyI,
            Action::Network => KeyCode::KeyN,
            Action::Camera => KeyCode::KeyP,
            Action::Photo => KeyCode::KeyV,
            Action::CamLeft => KeyCode::KeyZ,
            Action::CamRight => KeyCode::KeyX,
            Action::Pause => KeyCode::Escape,
        }
    }
}

pub fn key_name(k: KeyCode) -> String {
    let s = format!("{:?}", k);
    let s = s.strip_prefix("Key").unwrap_or(&s).to_string();
    let s = s.strip_prefix("Digit").map(|d| d.to_string()).unwrap_or(s);
    match s.as_str() {
        "ShiftLeft" => "Shift Esq.".into(),
        "ShiftRight" => "Shift Dir.".into(),
        "AltLeft" => "Alt Esq.".into(),
        "AltRight" => "Alt Dir.".into(),
        "ControlLeft" => "Ctrl Esq.".into(),
        "ControlRight" => "Ctrl Dir.".into(),
        "Escape" => "Esc".into(),
        "Space" => "Espaço".into(),
        "ArrowUp" => "Seta ↑".into(),
        "ArrowDown" => "Seta ↓".into(),
        "ArrowLeft" => "Seta ←".into(),
        "ArrowRight" => "Seta →".into(),
        "Enter" => "Enter".into(),
        "Backspace" => "Backspace".into(),
        _ => s,
    }
}

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct Bindings {
    pub map: HashMap<Action, KeyCode>,
}

impl Default for Bindings {
    fn default() -> Self {
        Bindings { map: Action::ALL.iter().map(|a| (*a, a.default_key())).collect() }
    }
}

impl Bindings {
    pub fn key(&self, a: Action) -> KeyCode {
        *self.map.get(&a).unwrap_or(&a.default_key())
    }
    pub fn reset(&mut self) {
        *self = Bindings::default();
    }
}

/// Convenience wrapper used by gameplay systems.
#[derive(Resource, Default)]
pub struct Act {
    pressed: Vec<Action>,
    held: Vec<Action>,
    released: Vec<Action>,
}

impl Act {
    pub fn just(&self, a: Action) -> bool {
        self.pressed.contains(&a)
    }
    pub fn held(&self, a: Action) -> bool {
        self.held.contains(&a)
    }
    pub fn released(&self, a: Action) -> bool {
        self.released.contains(&a)
    }
    pub fn consume(&mut self, a: Action) {
        self.pressed.retain(|x| *x != a);
    }
}

pub fn update_act(keys: Res<ButtonInput<KeyCode>>, b: Res<Bindings>, mut act: ResMut<Act>) {
    act.pressed.clear();
    act.held.clear();
    act.released.clear();
    for a in Action::ALL {
        let k = b.key(a);
        if keys.just_pressed(k) {
            act.pressed.push(a);
        }
        if keys.pressed(k) {
            act.held.push(a);
        }
        if keys.just_released(k) {
            act.released.push(a);
        }
    }
}

// ------------------------------------------------------------------ scripting

/// Automated input script, from env RT_SCRIPT:
///   `frame:press=KeyE;frame:hold=KeyW,60;frame:shot=/tmp/a.png;frame:click=640,360;frame:cmd=...;frame:quit`
#[derive(Resource, Default)]
pub struct Script {
    pub frame: u64,
    events: Vec<(u64, String)>,
    holds: Vec<(KeyCode, u64)>,
    pub cmds: Vec<String>,
    pub shots: Vec<String>,
    pub quit: bool,
    pub click: Option<Vec2>,
    pub mouse: Option<Vec2>,
    pub mouse_held: bool,
    pub active: bool,
}

pub fn parse_key(n: &str) -> Option<KeyCode> {
    use KeyCode::*;
    Some(match n {
        "Enter" => Enter,
        "Escape" | "Esc" => Escape,
        "Space" => Space,
        "Tab" => Tab,
        "Up" => ArrowUp,
        "Down" => ArrowDown,
        "Left" => ArrowLeft,
        "Right" => ArrowRight,
        "Shift" => ShiftLeft,
        "Alt" => AltLeft,
        "Ctrl" => ControlLeft,
        "Backspace" => Backspace,
        "1" => Digit1,
        "2" => Digit2,
        "3" => Digit3,
        "4" => Digit4,
        "5" => Digit5,
        "6" => Digit6,
        "7" => Digit7,
        "8" => Digit8,
        "9" => Digit9,
        "0" => Digit0,
        s if s.len() == 1 => match s.chars().next()?.to_ascii_uppercase() {
            'A' => KeyA, 'B' => KeyB, 'C' => KeyC, 'D' => KeyD, 'E' => KeyE, 'F' => KeyF, 'G' => KeyG,
            'H' => KeyH, 'I' => KeyI, 'J' => KeyJ, 'K' => KeyK, 'L' => KeyL, 'M' => KeyM, 'N' => KeyN,
            'O' => KeyO, 'P' => KeyP, 'Q' => KeyQ, 'R' => KeyR, 'S' => KeyS, 'T' => KeyT, 'U' => KeyU,
            'V' => KeyV, 'W' => KeyW, 'X' => KeyX, 'Y' => KeyY, 'Z' => KeyZ,
            _ => return None,
        },
        _ => return None,
    })
}

impl Script {
    pub fn from_env() -> Script {
        let mut s = Script::default();
        if let Ok(v) = std::env::var("RT_SCRIPT") {
            s.active = true;
            for part in v.split(';') {
                if let Some((f, c)) = part.trim().split_once(':') {
                    if let Ok(f) = f.trim().parse::<u64>() {
                        s.events.push((f, c.trim().to_string()));
                    }
                }
            }
        }
        s
    }
}

pub fn run_script(mut s: ResMut<Script>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    if !s.active {
        return;
    }
    s.frame += 1;
    let f = s.frame;
    s.click = None;
    // release finished holds
    let mut keep = Vec::new();
    for (k, until) in s.holds.drain(..) {
        if until <= f {
            keys.release(k);
        } else {
            keys.press(k);
            keep.push((k, until));
        }
    }
    s.holds = keep;
    let evs: Vec<String> = s.events.iter().filter(|(ef, _)| *ef == f).map(|(_, c)| c.clone()).collect();
    for cmd in evs {
        if let Some(k) = cmd.strip_prefix("press=") {
            if let Some(k) = parse_key(k) {
                keys.press(k);
                s.holds.push((k, f + 1));
            }
        } else if let Some(h) = cmd.strip_prefix("hold=") {
            let mut it = h.split(',');
            let k = it.next().and_then(parse_key);
            let n: u64 = it.next().and_then(|n| n.parse().ok()).unwrap_or(10);
            if let Some(k) = k {
                keys.press(k);
                s.holds.push((k, f + n));
            }
        } else if let Some(c) = cmd.strip_prefix("click=") {
            let mut it = c.split(',');
            let x: f32 = it.next().and_then(|n| n.parse().ok()).unwrap_or(0.0);
            let y: f32 = it.next().and_then(|n| n.parse().ok()).unwrap_or(0.0);
            s.click = Some(vec2(x, y));
            s.mouse = Some(vec2(x, y));
        } else if let Some(c) = cmd.strip_prefix("mouse=") {
            let mut it = c.split(',');
            let x: f32 = it.next().and_then(|n| n.parse().ok()).unwrap_or(0.0);
            let y: f32 = it.next().and_then(|n| n.parse().ok()).unwrap_or(0.0);
            s.mouse = Some(vec2(x, y));
        } else if cmd == "mousedown" {
            s.mouse_held = true;
        } else if cmd == "mouseup" {
            s.mouse_held = false;
        } else if let Some(p) = cmd.strip_prefix("shot=") {
            s.shots.push(p.to_string());
        } else if let Some(c) = cmd.strip_prefix("cmd=") {
            s.cmds.push(c.to_string());
        } else if cmd == "quit" {
            s.quit = true;
        }
    }
}

pub fn script_effects(
    mut s: ResMut<Script>,
    mut c: Commands,
    mut exit: EventWriter<AppExit>,
) {
    use bevy::render::view::screenshot::{save_to_disk, Screenshot};
    for p in s.shots.drain(..) {
        c.spawn(Screenshot::primary_window()).observe(save_to_disk(p));
    }
    if s.quit {
        exit.write(AppExit::Success);
    }
}

pub fn settings_path() -> std::path::PathBuf {
    let base = std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.to_path_buf())).unwrap_or_default();
    base.join("red_thread_ajustes.json")
}

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub bindings: Bindings,
    pub master: f32,
    pub music: f32,
    pub sfx: f32,
    pub fullscreen: bool,
    pub shadows: bool,
    pub fog: bool,
    pub grain: bool,
    pub cam_speed: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            bindings: Bindings::default(),
            master: 0.8,
            music: 0.7,
            sfx: 0.8,
            fullscreen: false,
            shadows: true,
            fog: true,
            grain: true,
            cam_speed: 1.0,
        }
    }
}

impl Settings {
    pub fn load() -> Settings {
        std::fs::read_to_string(settings_path())
            .ok()
            .and_then(|s| serde_json::from_str::<Settings>(&s).ok())
            .map(|mut s| {
                // make sure new actions get their defaults
                let d = Bindings::default();
                for (a, k) in d.map {
                    s.bindings.map.entry(a).or_insert(k);
                }
                s
            })
            .unwrap_or_default()
    }
    pub fn save(&self) {
        if let Ok(j) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(settings_path(), j);
        }
    }
}
