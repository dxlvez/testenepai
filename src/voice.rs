//! Voices. Every person has a unique voice (sex, age, personality → pitch,
//! speed, timbre). On Windows the system's Portuguese speech synthesizer
//! speaks the subtitles; everywhere (and for crowd murmur) a procedural
//! formant voice follows the rhythm and vowels of the text.

use crate::audio::{wav, SR};
use crate::sim::agents::Sim;
use crate::sim::people::{Person, Pid};
use crate::state::Game;
use crate::ui::dialogue::Dlg;
use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;

#[derive(Clone, Copy, Debug)]
pub struct VoiceParams {
    pub female: bool,
    /// fundamental frequency (Hz) for the procedural voice
    pub f0: f32,
    /// formant scaling (bigger = smaller vocal tract)
    pub formant: f32,
    /// speed multiplier
    pub rate: f32,
    pub breath: f32,
    pub jitter: f32,
    /// pitch for the system synthesizer (0.5..2)
    pub sys_pitch: f32,
    pub sys_voice: u32,
}

pub fn params_for(p: &Person, year: i32) -> VoiceParams {
    let s = p.seed;
    let h = |k: u32| ((s.wrapping_mul(2654435761).wrapping_add(k * 97)) >> 8 & 0xFFFF) as f32 / 65535.0;
    let age = p.age(year) as f32;
    let child = age < 13.0;
    let old = age > 60.0;
    let (f0, formant) = if child {
        (250.0 + h(1) * 60.0, 1.28 + h(2) * 0.1)
    } else if p.female {
        (180.0 + h(1) * 70.0 - if old { 25.0 } else { 0.0 }, 1.12 + h(2) * 0.12)
    } else {
        (88.0 + h(1) * 55.0 - if old { 10.0 } else { 0.0 }, 0.92 + h(2) * 0.12)
    };
    let soc = p.traits.sociability as f32 / 100.0;
    VoiceParams {
        female: p.female,
        f0,
        formant,
        rate: 0.85 + soc * 0.3 + h(3) * 0.1 - if old { 0.1 } else { 0.0 },
        breath: 0.05 + h(4) * 0.15 + if old { 0.1 } else { 0.0 },
        jitter: 0.01 + if old { 0.03 } else { 0.0 } + h(5) * 0.01,
        sys_pitch: if child { 1.5 } else if p.female { 0.95 + h(6) * 0.35 } else { 0.7 + h(6) * 0.35 },
        sys_voice: (h(7) * 1000.0) as u32,
    }
}

pub fn elias_voice() -> VoiceParams {
    VoiceParams { female: false, f0: 108.0, formant: 0.97, rate: 0.95, breath: 0.12, jitter: 0.012, sys_pitch: 0.85, sys_voice: 0 }
}

// ------------------------------------------------------------------ procedural voice

struct Res2 {
    a: f32,
    b: f32,
    c: f32,
    y1: f32,
    y2: f32,
}
impl Res2 {
    fn new(freq: f32, bw: f32, sr: f32) -> Res2 {
        let mut r = Res2 { a: 0.0, b: 0.0, c: 0.0, y1: 0.0, y2: 0.0 };
        r.set(freq, bw, sr);
        r
    }
    fn set(&mut self, freq: f32, bw: f32, sr: f32) {
        let t = 1.0 / sr;
        self.c = -(-2.0 * std::f32::consts::PI * bw * t).exp();
        self.b = 2.0 * (-std::f32::consts::PI * bw * t).exp() * (2.0 * std::f32::consts::PI * freq * t).cos();
        self.a = 1.0 - self.b - self.c;
    }
    fn tick(&mut self, x: f32) -> f32 {
        let y = self.a * x + self.b * self.y1 + self.c * self.y2;
        self.y2 = self.y1;
        self.y1 = y;
        y
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Ph {
    V(f32, f32, f32, bool),
    Plosive(bool),
    Fric(f32, bool),
    Nasal,
    Liquid,
    Pause(f32),
}

fn strip(c: char) -> char {
    match c {
        'á' | 'à' | 'â' | 'ã' | 'Á' | 'À' | 'Â' | 'Ã' => 'a',
        'é' | 'ê' | 'É' | 'Ê' => 'e',
        'í' | 'Í' => 'i',
        'ó' | 'ô' | 'õ' | 'Ó' | 'Ô' | 'Õ' => 'o',
        'ú' | 'ü' | 'Ú' => 'u',
        'ç' | 'Ç' => 's',
        c => c.to_ascii_lowercase(),
    }
}

fn phones(text: &str) -> Vec<Ph> {
    let mut v = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let raw = chars[i];
        let nasal = matches!(raw, 'ã' | 'õ' | 'Ã' | 'Õ');
        let c = strip(raw);
        let next = chars.get(i + 1).map(|c| strip(*c)).unwrap_or(' ');
        let p = match c {
            'a' => Some(Ph::V(760.0, 1320.0, 2500.0, nasal)),
            'e' => Some(Ph::V(480.0, 1900.0, 2600.0, nasal)),
            'i' | 'y' => Some(Ph::V(300.0, 2250.0, 2950.0, nasal)),
            'o' => Some(Ph::V(500.0, 900.0, 2450.0, nasal)),
            'u' => Some(Ph::V(330.0, 780.0, 2300.0, nasal)),
            'p' | 't' | 'k' | 'q' | 'c' => Some(Ph::Plosive(false)),
            'b' | 'd' | 'g' => Some(Ph::Plosive(true)),
            's' | 'x' | 'z' => Some(Ph::Fric(5200.0, c == 'z')),
            'f' => Some(Ph::Fric(3800.0, false)),
            'v' | 'j' => Some(Ph::Fric(3000.0, true)),
            'h' => None,
            'm' | 'n' => Some(Ph::Nasal),
            'l' | 'r' | 'w' => Some(Ph::Liquid),
            ' ' => Some(Ph::Pause(0.03)),
            ',' | ';' | ':' => Some(Ph::Pause(0.18)),
            '.' | '!' | '?' => Some(Ph::Pause(0.3)),
            '-' => Some(Ph::Pause(0.12)),
            _ => None,
        };
        if c == 'c' && next == 'h' {
            v.push(Ph::Fric(3200.0, false));
            i += 2;
            continue;
        }
        if let Some(p) = p {
            v.push(p);
        }
        i += 1;
    }
    v
}

/// Synthesize a line in a procedural voice (mono, SR Hz).
pub fn synth(text: &str, vp: &VoiceParams, emotion: f32) -> Vec<f32> {
    let sr = SR as f32;
    let ph = phones(text);
    let question = text.trim_end().ends_with('?');
    let exclaim = text.trim_end().ends_with('!');
    let mut out: Vec<f32> = Vec::new();
    let mut f1 = Res2::new(500.0, 80.0, sr);
    let mut f2 = Res2::new(1500.0, 110.0, sr);
    let mut f3 = Res2::new(2500.0, 160.0, sr);
    let mut fr = Res2::new(4000.0, 900.0, sr);
    let mut ph_glot = 0.0f32;
    let mut noise: u32 = 0x9E37_79B9 ^ vp.sys_voice;
    let mut rnd = || {
        noise ^= noise << 13;
        noise ^= noise >> 17;
        noise ^= noise << 5;
        (noise as f32 / u32::MAX as f32) * 2.0 - 1.0
    };
    let total_vowels = ph.iter().filter(|p| matches!(p, Ph::V(..))).count().max(1) as f32;
    let mut vowel_i = 0.0f32;
    let rate = vp.rate * (1.0 + emotion * 0.2);
    let mut cur = (500.0f32, 1500.0f32, 2500.0f32);
    for p in ph.iter() {
        let (dur, voiced, amp_noise, target, fric_f, gain) = match *p {
            Ph::V(a, b, c, _) => (0.085 / rate, true, 0.0, Some((a, b, c)), 0.0, 1.0),
            Ph::Plosive(v) => (0.05 / rate, v, 1.0, None, 3000.0, 0.5),
            Ph::Fric(f, v) => (0.07 / rate, v, 0.8, None, f, 0.4),
            Ph::Nasal => (0.055 / rate, true, 0.0, Some((280.0, 1100.0, 2400.0)), 0.0, 0.45),
            Ph::Liquid => (0.045 / rate, true, 0.0, Some((380.0, 1300.0, 2300.0)), 0.0, 0.6),
            Ph::Pause(s) => (s / rate, false, 0.0, None, 0.0, 0.0),
        };
        let n = (dur * sr) as usize;
        if let Some(t) = target {
            // glide towards the new formants
            cur = t;
        }
        if let Ph::V(..) = p {
            vowel_i += 1.0;
        }
        let prog = vowel_i / total_vowels;
        for k in 0..n {
            let tt = k as f32 / n.max(1) as f32;
            // intonation: declination, questions rise, emotion trembles
            let mut f0 = vp.f0 * (1.08 - prog * 0.16);
            if question && prog > 0.75 {
                f0 *= 1.0 + (prog - 0.75) * 1.2;
            }
            if exclaim {
                f0 *= 1.12;
            }
            f0 *= 1.0 + emotion * 0.18 + (out.len() as f32 / sr * (5.0 + emotion * 4.0)).sin() * (0.01 + emotion * 0.04) + rnd() * vp.jitter;
            let fs = vp.formant;
            f1.set(cur.0 * fs, 70.0, sr);
            f2.set(cur.1 * fs, 100.0, sr);
            f3.set(cur.2 * fs, 150.0, sr);
            ph_glot += f0 / sr;
            if ph_glot >= 1.0 {
                ph_glot -= 1.0;
            }
            // glottal pulse (Rosenberg-ish)
            let g = if ph_glot < 0.4 { (ph_glot / 0.4 * std::f32::consts::PI).sin() } else { 0.0 };
            let src = if voiced { g - 0.25 + rnd() * vp.breath } else { 0.0 };
            let env = (tt * 6.0).min(1.0) * ((1.0 - tt) * 6.0).min(1.0);
            let voiced_out = (f1.tick(src) * 1.0 + f2.tick(src) * 0.6 + f3.tick(src) * 0.3) * gain;
            let mut s = voiced_out * env;
            if amp_noise > 0.0 {
                fr.set(fric_f * fs.min(1.2), 1200.0, sr);
                let burst = if matches!(p, Ph::Plosive(_)) { if tt > 0.55 { (-(tt - 0.55) * 20.0).exp() } else { 0.0 } } else { env };
                s += fr.tick(rnd()) * amp_noise * burst * 0.5;
            }
            out.push(s);
        }
    }
    // normalize
    let m = out.iter().fold(0.0f32, |a, b| a.max(b.abs())).max(1e-4);
    for s in out.iter_mut() {
        *s = *s / m * 0.6;
    }
    // tail
    out.extend(std::iter::repeat(0.0).take((0.1 * sr) as usize));
    out
}

// ------------------------------------------------------------------ engine

pub struct Req {
    pub id: u64,
    pub text: String,
    pub vp: VoiceParams,
    pub emotion: f32,
    pub system: bool,
}

#[derive(Resource)]
pub struct VoiceEngine {
    tx: Mutex<Sender<Req>>,
    rx: Mutex<Receiver<(u64, Vec<u8>)>>,
    next: u64,
    pending: HashMap<u64, (Option<Pid>, Vec2, bool)>,
    heard: HashMap<Pid, String>,
    dlg_seen: usize,
    pub system_available: bool,
}

pub fn start_engine(mut c: Commands) {
    let (tx, rx_req) = channel::<Req>();
    let (tx_out, rx) = channel::<(u64, Vec<u8>)>();
    let (tx_avail, rx_avail) = channel::<bool>();
    std::thread::spawn(move || {
        let mut sys = system::SysVoice::new();
        let _ = tx_avail.send(sys.is_some());
        while let Ok(r) = rx_req.recv() {
            let mut bytes = None;
            if r.system {
                if let Some(s) = sys.as_mut() {
                    bytes = s.speak(&r.text, &r.vp, r.emotion);
                }
            }
            let bytes = bytes.unwrap_or_else(|| wav(&synth(&r.text, &r.vp, r.emotion), SR, false));
            if tx_out.send((r.id, bytes)).is_err() {
                break;
            }
        }
    });
    let avail = rx_avail.recv_timeout(std::time::Duration::from_secs(3)).unwrap_or(false);
    c.insert_resource(VoiceEngine { tx: Mutex::new(tx), rx: Mutex::new(rx), next: 1, pending: HashMap::new(), heard: HashMap::new(), dlg_seen: 0, system_available: avail });
}

impl VoiceEngine {
    pub fn say(&mut self, text: &str, vp: VoiceParams, emotion: f32, who: Option<Pid>, pos: Vec2, focus: bool) {
        let id = self.next;
        self.next += 1;
        let clean: String = text.chars().filter(|c| !matches!(c, '(' | ')' | '"' | '“' | '”' | '*')).collect();
        if clean.trim().is_empty() {
            return;
        }
        self.pending.insert(id, (who, pos, focus));
        if let Ok(tx) = self.tx.lock() {
            let _ = tx.send(Req { id, text: clean, vp, emotion, system: focus && self.system_available });
        }
    }
}

#[derive(Component)]
pub struct VoicePlayer {
    pub focus: bool,
}

fn emotion_of(text: &str) -> f32 {
    let t = text.to_lowercase();
    let mut e: f32 = 0.0;
    for w in ["por favor", "socorro", "não", "deus", "pelo amor", "morrer", "!"] {
        if t.contains(w) {
            e += 0.2;
        }
    }
    e.min(1.0)
}

/// Voice new speech bubbles near Elias and new lines in the dialogue window.
#[allow(clippy::too_many_arguments)]
pub fn voice_watch(
    mut ve: ResMut<VoiceEngine>,
    sim: Res<Sim>,
    game: Res<Game>,
    dlg: Res<Dlg>,
    ui: Res<crate::ui::UiState>,
    playing: Query<&VoicePlayer>,
) {
    let pp = game.player.pos;
    // dialogue lines (focus)
    if ui.mode == crate::ui::Mode::Dialogue {
        if dlg.log.len() < ve.dlg_seen {
            ve.dlg_seen = 0;
        }
        let new: Vec<(String, String)> = dlg.log.iter().skip(ve.dlg_seen).cloned().collect();
        ve.dlg_seen = dlg.log.len();
        for (who, text) in new {
            if who == "Elias" {
                let t = text.trim_start_matches("\"").to_string();
                // only voice actual speech (quoted options), not narration of actions
                if text.starts_with('"') || text.starts_with('“') {
                    ve.say(&t, elias_voice(), 0.0, None, pp, true);
                }
            } else if let Some(pid) = dlg.with {
                let p = game.pop.get(pid);
                let vp = params_for(p, game.year);
                ve.say(&text, vp, emotion_of(&text), Some(pid), pp, true);
                ve.heard.insert(pid, text);
            }
        }
    } else {
        ve.dlg_seen = 0;
    }
    // ambient bubbles
    let n_playing = playing.iter().filter(|p| !p.focus).count();
    if n_playing >= 5 {
        return;
    }
    let mut budget = 5 - n_playing;
    for a in sim.agents.iter() {
        if budget == 0 {
            break;
        }
        let Some((text, _)) = &a.bubble else { continue };
        if a.pos.distance_squared(pp) > 16.0 * 16.0 {
            continue;
        }
        if ve.heard.get(&a.pid) == Some(text) {
            continue;
        }
        ve.heard.insert(a.pid, text.clone());
        let p = game.pop.get(a.pid);
        let vp = params_for(p, game.year);
        let near = a.pos.distance(pp) < 5.0;
        let text = text.clone();
        ve.say(&text, vp, emotion_of(&text), Some(a.pid), a.pos, near);
        budget -= 1;
    }
}

pub fn voice_play(
    mut c: Commands,
    mut ve: ResMut<VoiceEngine>,
    mut assets: ResMut<Assets<AudioSource>>,
    game: Res<Game>,
    settings: Res<crate::keys::Settings>,
    sim: Res<Sim>,
) {
    let got: Vec<(u64, Vec<u8>)> = match ve.rx.lock() {
        Ok(rx) => rx.try_iter().collect(),
        Err(_) => return,
    };
    let pp = game.player.pos;
    for (id, bytes) in got {
        let Some((who, pos, focus)) = ve.pending.remove(&id) else { continue };
        let pos = who.and_then(|w| sim.agent(w).map(|a| a.pos)).unwrap_or(pos);
        let d = pos.distance(pp);
        let vol = if focus { 1.0 } else { (1.0 - d / 16.0).clamp(0.0, 1.0) * 0.7 };
        if vol <= 0.02 {
            continue;
        }
        let h = assets.add(AudioSource { bytes: bytes.into() });
        c.spawn((
            AudioPlayer(h),
            PlaybackSettings { mode: PlaybackMode::Despawn, volume: Volume::Linear(vol * settings.sfx.max(0.3) * settings.master * 1.2), ..default() },
            VoicePlayer { focus },
        ));
    }
}

// ------------------------------------------------------------------ Windows speech

#[cfg(windows)]
mod system {
    use super::VoiceParams;
    use windows::core::HSTRING;
    use windows::Media::SpeechSynthesis::{SpeechSynthesizer, VoiceGender, VoiceInformation};
    use windows::Storage::Streams::DataReader;

    pub struct SysVoice {
        synth: SpeechSynthesizer,
        male: Vec<VoiceInformation>,
        female: Vec<VoiceInformation>,
    }

    impl SysVoice {
        pub fn new() -> Option<SysVoice> {
            unsafe {
                let _ = windows::Win32::System::WinRT::RoInitialize(windows::Win32::System::WinRT::RO_INIT_MULTITHREADED);
            }
            let synth = SpeechSynthesizer::new().ok()?;
            let all = SpeechSynthesizer::AllVoices().ok()?;
            let mut male = Vec::new();
            let mut female = Vec::new();
            let n = all.Size().ok()?;
            for i in 0..n {
                let v = all.GetAt(i).ok()?;
                let lang = v.Language().ok()?.to_string().to_lowercase();
                if !lang.starts_with("pt") {
                    continue;
                }
                match v.Gender().ok()? {
                    g if g == VoiceGender::Female => female.push(v),
                    _ => male.push(v),
                }
            }
            if male.is_empty() && female.is_empty() {
                return None;
            }
            Some(SysVoice { synth, male, female })
        }

        pub fn speak(&mut self, text: &str, vp: &VoiceParams, emotion: f32) -> Option<Vec<u8>> {
            let pool = if vp.female { if self.female.is_empty() { &self.male } else { &self.female } } else if self.male.is_empty() { &self.female } else { &self.male };
            let v = pool.get(vp.sys_voice as usize % pool.len())?;
            self.synth.SetVoice(v).ok()?;
            if let Ok(o) = self.synth.Options() {
                let _ = o.SetAudioPitch((vp.sys_pitch as f64 + emotion as f64 * 0.2).clamp(0.3, 2.0));
                let _ = o.SetSpeakingRate((vp.rate as f64 * (1.0 + emotion as f64 * 0.15)).clamp(0.5, 3.0));
            }
            let esc = text.replace('&', "e").replace('<', " ").replace('>', " ");
            let lang = v.Language().ok()?.to_string();
            let ssml = format!("<speak version='1.0' xmlns='http://www.w3.org/2001/10/synthesis' xml:lang='{}'>{}</speak>", lang, esc);
            let stream = self.synth.SynthesizeSsmlToStreamAsync(&HSTRING::from(ssml)).ok()?.get().ok()?;
            let size = stream.Size().ok()? as u32;
            let input = stream.GetInputStreamAt(0).ok()?;
            let reader = DataReader::CreateDataReader(&input).ok()?;
            reader.LoadAsync(size).ok()?.get().ok()?;
            let mut buf = vec![0u8; size as usize];
            reader.ReadBytes(&mut buf).ok()?;
            Some(buf)
        }
    }
}

#[cfg(not(windows))]
mod system {
    use super::VoiceParams;
    pub struct SysVoice;
    impl SysVoice {
        pub fn new() -> Option<SysVoice> {
            None
        }
        pub fn speak(&mut self, _t: &str, _v: &VoiceParams, _e: f32) -> Option<Vec<u8>> {
            None
        }
    }
}

/// Debug: render a few voices to WAV files.
pub fn render_samples(dir: &str) {
    let _ = std::fs::create_dir_all(dir);
    let lines = ["Por favor, eu tenho dois filhos esperando em casa.", "Você soube? Encontraram o açougueiro morto!", "Boa noite, forasteiro. O que vai ser?"];
    for (i, l) in lines.iter().enumerate() {
        for (k, vp) in [elias_voice(), VoiceParams { female: true, f0: 215.0, formant: 1.18, rate: 1.0, breath: 0.1, jitter: 0.01, sys_pitch: 1.0, sys_voice: 1 }].iter().enumerate() {
            let s = synth(l, vp, if i == 0 { 0.7 } else { 0.0 });
            let _ = std::fs::write(format!("{}/voice_{}_{}.wav", dir, i, k), wav(&s, SR, false));
        }
    }
}
