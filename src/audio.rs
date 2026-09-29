//! Procedural audio: every piece of music and every sound effect is
//! synthesized at runtime. The Elias theme is re-orchestrated per era.

use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;
use std::f32::consts::TAU;
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};

pub const SR: u32 = 22050;

// ------------------------------------------------------------------ wav

pub fn wav(samples: &[f32], sr: u32, stereo: bool) -> Vec<u8> {
    let ch: u16 = if stereo { 2 } else { 1 };
    let n = samples.len() as u32;
    let mut v = Vec::with_capacity(44 + samples.len() * 2);
    v.extend(b"RIFF");
    v.extend((36 + n * 2).to_le_bytes());
    v.extend(b"WAVEfmt ");
    v.extend(16u32.to_le_bytes());
    v.extend(1u16.to_le_bytes());
    v.extend(ch.to_le_bytes());
    v.extend(sr.to_le_bytes());
    v.extend((sr * 2 * ch as u32).to_le_bytes());
    v.extend((2 * ch).to_le_bytes());
    v.extend(16u16.to_le_bytes());
    v.extend(b"data");
    v.extend((n * 2).to_le_bytes());
    for s in samples {
        v.extend(((s.clamp(-1.0, 1.0) * 32000.0) as i16).to_le_bytes());
    }
    v
}

// ------------------------------------------------------------------ dsp

struct Noise(u32);
impl Noise {
    fn next(&mut self) -> f32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        (self.0 as f32 / u32::MAX as f32) * 2.0 - 1.0
    }
}

fn midi(n: f32) -> f32 {
    440.0 * 2f32.powf((n - 69.0) / 12.0)
}

#[derive(Clone, Copy)]
enum Inst {
    Bass,
    Piano,
    Horn,
    Strings,
    Violin,
    EPiano,
    SynthLead,
    SynthPad,
    Arp,
    Clarinet,
    Bell,
    Organ,
}

fn env_adsr(t: f32, dur: f32, a: f32, d: f32, s: f32, r: f32) -> f32 {
    if t < a {
        t / a
    } else if t < a + d {
        1.0 - (1.0 - s) * (t - a) / d
    } else if t < dur {
        s
    } else if t < dur + r {
        s * (1.0 - (t - dur) / r)
    } else {
        0.0
    }
}

/// Render one note into buf at sample offset.
fn note(buf: &mut [f32], start: usize, dur: f32, pitch: f32, vel: f32, inst: Inst, sr: f32, nz: &mut Noise) {
    let f = midi(pitch);
    let rel = match inst {
        Inst::Piano | Inst::EPiano | Inst::Bell => 1.2,
        Inst::Strings | Inst::SynthPad => 1.0,
        Inst::Bass => 0.15,
        _ => 0.25,
    };
    let len = ((dur + rel) * sr) as usize;
    let mut ph = 0.0f32;
    let mut ph2 = 0.0f32;
    let mut lp = 0.0f32;
    for i in 0..len {
        let idx = start + i;
        if idx >= buf.len() {
            break;
        }
        let t = i as f32 / sr;
        let vib = match inst {
            Inst::Violin | Inst::Horn | Inst::Clarinet => 1.0 + (t * 5.5 * TAU).sin() * 0.004 * (t * 2.0).min(1.0),
            Inst::SynthLead => 1.0 + (t * 5.0 * TAU).sin() * 0.003,
            _ => 1.0,
        };
        ph = (ph + f * vib / sr).fract();
        ph2 = (ph2 + f * 1.004 / sr).fract();
        let s = match inst {
            Inst::Bass => {
                let e = (-t * 5.0).exp() * env_adsr(t, dur, 0.005, 0.1, 0.6, 0.1);
                ((ph * TAU).sin() + 0.3 * (ph * 2.0 * TAU).sin()) * e
            }
            Inst::Piano => {
                let e = (-t * 2.2).exp() * env_adsr(t, dur, 0.002, 0.05, 0.8, 0.4);
                let h = (ph * TAU).sin() + 0.45 * (ph * 2.0 * TAU).sin() * (-t * 4.0).exp() + 0.2 * (ph * 3.0 * TAU).sin() * (-t * 6.0).exp() + 0.1 * (ph * 4.01 * TAU).sin() * (-t * 8.0).exp();
                h * e * 0.6
            }
            Inst::Horn => {
                let e = env_adsr(t, dur, 0.04, 0.1, 0.8, 0.12);
                let saw = ph * 2.0 - 1.0;
                lp += (saw - lp) * (0.08 + 0.1 * e);
                lp * e * 1.3
            }
            Inst::Clarinet => {
                let e = env_adsr(t, dur, 0.05, 0.1, 0.85, 0.15);
                let sq = if ph < 0.5 { 1.0 } else { -1.0 };
                lp += (sq - lp) * 0.07;
                lp * e
            }
            Inst::Strings => {
                let e = env_adsr(t, dur, 0.35, 0.2, 0.9, 0.8);
                let saw = (ph * 2.0 - 1.0) + (ph2 * 2.0 - 1.0);
                lp += (saw - lp) * 0.05;
                lp * e * 0.5
            }
            Inst::Violin => {
                let e = env_adsr(t, dur, 0.12, 0.1, 0.9, 0.3);
                let saw = ph * 2.0 - 1.0;
                lp += (saw - lp) * 0.12;
                (lp + nz.next() * 0.02) * e * 0.9
            }
            Inst::EPiano => {
                let e = (-t * 1.6).exp() * env_adsr(t, dur, 0.003, 0.05, 0.85, 0.5);
                let m = (ph * TAU).sin() * 1.8 * (-t * 3.0).exp();
                ((ph * TAU + m).sin() + 0.2 * (ph * 2.0 * TAU).sin()) * e * 0.55
            }
            Inst::SynthLead => {
                let e = env_adsr(t, dur, 0.01, 0.1, 0.7, 0.2);
                let sq = if ph < 0.5 { 1.0 } else { -1.0 };
                let saw = ph2 * 2.0 - 1.0;
                lp += ((sq + saw) * 0.5 - lp) * (0.05 + 0.2 * (-t * 3.0).exp());
                lp * e * 0.7
            }
            Inst::SynthPad => {
                let e = env_adsr(t, dur, 0.8, 0.3, 0.9, 1.0);
                let saw = (ph * 2.0 - 1.0) + (ph2 * 2.0 - 1.0) + ((ph * 0.5).fract() * 2.0 - 1.0) * 0.5;
                lp += (saw - lp) * 0.02;
                lp * e * 0.5
            }
            Inst::Arp => {
                let e = (-t * 12.0).exp();
                (if ph < 0.3 { 1.0 } else { -1.0 }) * e * 0.35
            }
            Inst::Bell => {
                let e = (-t * 1.5).exp();
                ((ph * TAU).sin() + 0.5 * (ph * 2.76 * TAU).sin() * (-t * 3.0).exp() + 0.3 * (ph * 5.4 * TAU).sin() * (-t * 5.0).exp()) * e * 0.5
            }
            Inst::Organ => {
                let e = env_adsr(t, dur, 0.05, 0.0, 1.0, 0.2);
                ((ph * TAU).sin() + 0.5 * (ph * 2.0 * TAU).sin() + 0.25 * (ph * 4.0 * TAU).sin()) * e * 0.4
            }
        };
        buf[idx] += s * vel;
    }
}

fn kick(buf: &mut [f32], start: usize, vel: f32, sr: f32) {
    let len = (0.35 * sr) as usize;
    let mut ph = 0.0f32;
    for i in 0..len {
        if start + i >= buf.len() {
            break;
        }
        let t = i as f32 / sr;
        let f = 50.0 + 90.0 * (-t * 30.0).exp();
        ph += f / sr;
        buf[start + i] += (ph * TAU).sin() * (-t * 9.0).exp() * vel;
    }
}

fn snare(buf: &mut [f32], start: usize, vel: f32, brush: bool, sr: f32, nz: &mut Noise) {
    let len = ((if brush { 0.25 } else { 0.2 }) * sr) as usize;
    let mut lp = 0.0;
    for i in 0..len {
        if start + i >= buf.len() {
            break;
        }
        let t = i as f32 / sr;
        let n = nz.next();
        lp += (n - lp) * if brush { 0.25 } else { 0.6 };
        let e = if brush { (-t * 14.0).exp() * (t * 60.0).min(1.0) } else { (-t * 18.0).exp() };
        let tone = if brush { 0.0 } else { (t * 190.0 * TAU).sin() * (-t * 30.0).exp() * 0.5 };
        buf[start + i] += (if brush { lp } else { n * 0.8 } + tone) * e * vel;
    }
}

fn hat(buf: &mut [f32], start: usize, vel: f32, sr: f32, nz: &mut Noise) {
    let len = (0.05 * sr) as usize;
    let mut prev = 0.0;
    for i in 0..len {
        if start + i >= buf.len() {
            break;
        }
        let t = i as f32 / sr;
        let n = nz.next();
        let hp = n - prev;
        prev = n;
        buf[start + i] += hp * (-t * 60.0).exp() * vel * 0.5;
    }
}

fn reverb(buf: &mut Vec<f32>, sr: f32, mix: f32, size: f32) {
    let combs = [1557, 1617, 1491, 1422].map(|d| ((d as f32 * size * sr / 44100.0) as usize).max(10));
    let mut out = vec![0.0f32; buf.len()];
    for d in combs {
        let mut line = vec![0.0f32; d];
        let mut k = 0;
        let mut lp = 0.0;
        for i in 0..buf.len() {
            let y = line[k];
            lp = y * 0.7 + lp * 0.3;
            line[k] = buf[i] + lp * 0.8;
            k = (k + 1) % d;
            out[i] += y * 0.25;
        }
    }
    for i in 0..buf.len() {
        buf[i] = buf[i] * (1.0 - mix) + out[i] * mix;
    }
}

fn lowpass(buf: &mut [f32], k: f32) {
    let mut lp = 0.0;
    for s in buf.iter_mut() {
        lp += (*s - lp) * k;
        *s = lp;
    }
}

fn normalize(buf: &mut [f32], peak: f32) {
    let m = buf.iter().fold(0.0f32, |a, b| a.max(b.abs())).max(1e-4);
    for s in buf.iter_mut() {
        *s = *s / m * peak;
    }
}

// ------------------------------------------------------------------ the theme

/// Elias' theme: (beat offset, length in beats, midi pitch relative to A minor root 57)
const THEME: &[(f32, f32, f32)] = &[
    (0.0, 1.5, 12.0), (1.5, 0.5, 15.0), (2.0, 1.0, 19.0), (3.0, 1.0, 17.0),
    (4.0, 2.0, 15.0), (6.0, 1.0, 14.0), (7.0, 1.0, 12.0),
    (8.0, 1.5, 10.0), (9.5, 0.5, 12.0), (10.0, 1.0, 14.0), (11.0, 1.0, 7.0),
    (12.0, 3.0, 12.0), (15.0, 1.0, 11.0),
    (16.0, 1.5, 12.0), (17.5, 0.5, 15.0), (18.0, 1.0, 19.0), (19.0, 1.0, 22.0),
    (20.0, 2.0, 20.0), (22.0, 1.0, 19.0), (23.0, 1.0, 17.0),
    (24.0, 1.0, 15.0), (25.0, 1.0, 14.0), (26.0, 1.0, 12.0), (27.0, 1.0, 11.0),
    (28.0, 4.0, 12.0),
];

/// Chord roots per bar (4 beats): Am, F, Dm, E, Am, F, E, Am
const CHORDS: &[[f32; 3]] = &[
    [57.0, 60.0, 64.0],
    [53.0, 57.0, 60.0],
    [50.0, 53.0, 57.0],
    [52.0, 56.0, 59.0],
    [57.0, 60.0, 64.0],
    [53.0, 57.0, 60.0],
    [52.0, 56.0, 59.0],
    [57.0, 60.0, 64.0],
];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Track {
    Menu,
    Limbo,
    Era1920,
    Era1934,
    Era1948,
    Era1969,
    Era1986,
    Era2001,
    Finale,
    Tension,
}

pub fn track_for_year(y: i32) -> Track {
    if y < 1930 {
        Track::Era1920
    } else if y < 1945 {
        Track::Era1934
    } else if y < 1960 {
        Track::Era1948
    } else if y < 1976 {
        Track::Era1969
    } else if y < 2000 {
        Track::Era1986
    } else {
        Track::Era2001
    }
}

fn render_era(tr: Track, nz: &mut Noise) -> Vec<f32> {
    let sr = SR as f32;
    let (bpm, swing) = match tr {
        Track::Era1920 => (96.0, 0.66),
        Track::Era1934 => (88.0, 0.62),
        Track::Era1948 => (70.0, 0.5),
        Track::Era1969 => (84.0, 0.5),
        Track::Era1986 => (104.0, 0.5),
        Track::Era2001 | Track::Limbo | Track::Menu => (64.0, 0.5),
        Track::Finale => (72.0, 0.5),
        Track::Tension => (120.0, 0.5),
    };
    let beat = 60.0 / bpm;
    let loops = 2;
    let beats = 32.0 * loops as f32;
    let total = ((beats * beat + 3.0) * sr) as usize;
    let mut buf = vec![0.0f32; total];
    let pos = |b: f32| -> usize {
        // swing the off-beats
        let whole = b.floor();
        let frac = b - whole;
        let sw = if (frac - 0.5).abs() < 0.01 { swing } else { frac };
        ((whole + sw) * beat * sr) as usize
    };
    for lp in 0..loops {
        let base = lp as f32 * 32.0;
        let second = lp == 1;
        for (bar, ch) in CHORDS.iter().enumerate() {
            let b0 = base + bar as f32 * 4.0;
            match tr {
                Track::Era1920 | Track::Era1934 => {
                    // walking bass
                    let walk = [ch[0] - 12.0, ch[1] - 12.0, ch[2] - 12.0, ch[1] - 12.0 + 2.0];
                    for (k, n) in walk.iter().enumerate() {
                        note(&mut buf, pos(b0 + k as f32), beat * 0.9, *n, 0.55, Inst::Bass, sr, nz);
                    }
                    // piano comping on 2 and 4
                    for k in [1.0, 3.0] {
                        for n in ch {
                            note(&mut buf, pos(b0 + k), beat * 0.6, *n + 12.0, 0.16, Inst::Piano, sr, nz);
                        }
                    }
                    // brushes
                    for k in 0..4 {
                        snare(&mut buf, pos(b0 + k as f32), if k % 2 == 1 { 0.18 } else { 0.08 }, true, sr, nz);
                        snare(&mut buf, pos(b0 + k as f32 + 0.5), 0.05, true, sr, nz);
                    }
                    if tr == Track::Era1934 {
                        for n in ch {
                            note(&mut buf, pos(b0), beat * 3.8, *n, 0.1, Inst::Strings, sr, nz);
                        }
                    }
                }
                Track::Era1948 => {
                    note(&mut buf, pos(b0), beat * 3.9, ch[0] - 12.0, 0.4, Inst::Strings, sr, nz);
                    for (k, n) in ch.iter().enumerate() {
                        note(&mut buf, pos(b0 + k as f32 * 0.66), beat * 2.0, *n, 0.12, Inst::Piano, sr, nz);
                    }
                }
                Track::Era1969 => {
                    note(&mut buf, pos(b0), beat * 1.5, ch[0] - 12.0, 0.5, Inst::Bass, sr, nz);
                    note(&mut buf, pos(b0 + 1.5), beat * 0.5, ch[0] - 12.0, 0.4, Inst::Bass, sr, nz);
                    note(&mut buf, pos(b0 + 2.0), beat * 1.5, ch[2] - 12.0, 0.45, Inst::Bass, sr, nz);
                    for k in [0.0, 1.5, 2.5] {
                        for n in ch {
                            note(&mut buf, pos(b0 + k), beat * 0.9, *n + 12.0, 0.12, Inst::EPiano, sr, nz);
                        }
                    }
                    kick(&mut buf, pos(b0), 0.5, sr);
                    kick(&mut buf, pos(b0 + 2.5), 0.35, sr);
                    snare(&mut buf, pos(b0 + 1.0), 0.25, false, sr, nz);
                    snare(&mut buf, pos(b0 + 3.0), 0.25, false, sr, nz);
                    for k in 0..8 {
                        hat(&mut buf, pos(b0 + k as f32 * 0.5), 0.2, sr, nz);
                    }
                }
                Track::Era1986 | Track::Tension => {
                    for n in ch {
                        note(&mut buf, pos(b0), beat * 3.8, *n, 0.12, Inst::SynthPad, sr, nz);
                    }
                    for k in 0..16 {
                        let n = ch[k % 3] + if k % 6 >= 3 { 12.0 } else { 0.0 } + 12.0;
                        note(&mut buf, pos(b0 + k as f32 * 0.25), beat * 0.2, n, 0.14, Inst::Arp, sr, nz);
                    }
                    for k in 0..4 {
                        note(&mut buf, pos(b0 + k as f32), beat * 0.8, ch[0] - 12.0, 0.45, Inst::Bass, sr, nz);
                        kick(&mut buf, pos(b0 + k as f32), 0.45, sr);
                    }
                    snare(&mut buf, pos(b0 + 1.0), 0.3, false, sr, nz);
                    snare(&mut buf, pos(b0 + 3.0), 0.3, false, sr, nz);
                }
                Track::Era2001 | Track::Finale | Track::Menu | Track::Limbo => {
                    for n in ch {
                        note(&mut buf, pos(b0), beat * 3.9, *n - 12.0, 0.1, Inst::SynthPad, sr, nz);
                    }
                    if tr == Track::Finale {
                        let walk = [ch[0] - 12.0, ch[1] - 12.0, ch[2] - 12.0, ch[1] - 12.0];
                        for (k, n) in walk.iter().enumerate() {
                            note(&mut buf, pos(b0 + k as f32), beat * 0.9, *n, 0.35, Inst::Bass, sr, nz);
                        }
                        note(&mut buf, pos(b0), beat * 3.9, ch[0], 0.2, Inst::Strings, sr, nz);
                        for k in [1.0, 3.0] {
                            for n in ch {
                                note(&mut buf, pos(b0 + k), beat * 0.6, *n + 12.0, 0.1, Inst::EPiano, sr, nz);
                            }
                        }
                    }
                }
            }
        }
        // melody
        let (inst, oct, vel) = match tr {
            Track::Era1920 => (Inst::Horn, 0.0, 0.32),
            Track::Era1934 => (Inst::Clarinet, 0.0, 0.3),
            Track::Era1948 => (Inst::Violin, 0.0, 0.34),
            Track::Era1969 => (Inst::EPiano, 12.0, 0.3),
            Track::Era1986 => (Inst::SynthLead, 0.0, 0.26),
            Track::Era2001 | Track::Menu => (Inst::Piano, 12.0, 0.35),
            Track::Finale => (Inst::Violin, 0.0, 0.3),
            Track::Limbo => (Inst::Bell, 12.0, 0.18),
            Track::Tension => (Inst::SynthLead, -12.0, 0.2),
        };
        let play_melody = !(tr == Track::Limbo && !second);
        if play_melody {
            for (b, l, n) in THEME {
                if tr == Track::Limbo && (*b as i32) % 3 != 0 {
                    continue;
                }
                let p = 57.0 + n + oct - 12.0 + if second && tr == Track::Era1920 { 12.0 } else { 0.0 };
                note(&mut buf, pos(base + b), beat * l * 0.95, p, vel, inst, sr, nz);
                if tr == Track::Finale {
                    note(&mut buf, pos(base + b), beat * l * 0.95, p + 12.0, 0.18, Inst::Piano, sr, nz);
                    note(&mut buf, pos(base + b), beat * l * 0.95, p - 12.0, 0.12, Inst::Horn, sr, nz);
                    note(&mut buf, pos(base + b), beat * l * 0.95, p, 0.1, Inst::SynthLead, sr, nz);
                }
            }
        }
    }
    // era colouring
    match tr {
        Track::Era1920 | Track::Era1934 => {
            lowpass(&mut buf, 0.35);
            // vinyl crackle + hiss
            for (i, s) in buf.iter_mut().enumerate() {
                let n = nz.next();
                *s += n * 0.012;
                if n > 0.9993 {
                    *s += 0.35 * nz.next();
                }
                // slight wobble
                *s *= 1.0 + (i as f32 / sr * 0.7 * TAU).sin() * 0.03;
            }
            reverb(&mut buf, sr, 0.18, 0.8);
        }
        Track::Era1948 => reverb(&mut buf, sr, 0.35, 1.2),
        Track::Era1969 => {
            reverb(&mut buf, sr, 0.2, 0.9);
            for (i, s) in buf.iter_mut().enumerate() {
                *s += nz.next() * 0.004;
                *s *= 1.0 + (i as f32 / sr * 0.4 * TAU).sin() * 0.02;
            }
        }
        Track::Era1986 | Track::Tension => reverb(&mut buf, sr, 0.25, 1.0),
        Track::Era2001 | Track::Menu | Track::Finale => reverb(&mut buf, sr, 0.45, 1.5),
        Track::Limbo => {
            // radio static bursts and a low drone
            for (i, s) in buf.iter_mut().enumerate() {
                let t = i as f32 / sr;
                *s += (t * 55.0 * TAU).sin() * 0.08 + (t * 55.3 * TAU).sin() * 0.08;
                let burst = ((t * 0.37).sin() * (t * 1.13).cos()).max(0.0).powf(8.0);
                *s += nz.next() * 0.25 * burst;
            }
            reverb(&mut buf, sr, 0.6, 1.8);
        }
    }
    if tr == Track::Menu {
        for (i, s) in buf.iter_mut().enumerate() {
            *s += nz.next() * 0.008 * (1.0 + (i as f32 / sr * 0.2).sin());
        }
    }
    normalize(&mut buf, 0.7);
    buf
}

// ------------------------------------------------------------------ sfx

#[derive(Event, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Sfx {
    Click,
    Tick,
    Type,
    Gunshot,
    Shotgun,
    Punch,
    Door,
    Scream,
    Thunder,
    Static,
    Heartbeat,
    Echo,
    RedSight,
    Paper,
    Cash,
    Lockpick,
    Glass,
    Engine,
    Hum,
    Whisper,
    Bell,
    Whistle,
    Siren,
    Shutter,
    Step,
    Alter,
    Evidence,
    Hurt,
}

fn render_sfx(s: Sfx, nz: &mut Noise) -> Vec<f32> {
    let sr = SR as f32;
    let len = |secs: f32| (secs * sr) as usize;
    let mut b: Vec<f32>;
    match s {
        Sfx::Click => {
            b = vec![0.0; len(0.05)];
            for (i, x) in b.iter_mut().enumerate() {
                let t = i as f32 / sr;
                *x = nz.next() * (-t * 200.0).exp() * 0.6 + (t * 1800.0 * TAU).sin() * (-t * 120.0).exp() * 0.4;
            }
        }
        Sfx::Tick => {
            b = vec![0.0; len(0.03)];
            for (i, x) in b.iter_mut().enumerate() {
                let t = i as f32 / sr;
                *x = (t * 3000.0 * TAU).sin() * (-t * 300.0).exp() * 0.25;
            }
        }
        Sfx::Type => {
            b = vec![0.0; len(0.12)];
            for (i, x) in b.iter_mut().enumerate() {
                let t = i as f32 / sr;
                *x = nz.next() * (-t * 90.0).exp() * 0.8 + (t * 900.0 * TAU).sin() * (-t * 60.0).exp() * 0.3;
            }
        }
        Sfx::Gunshot | Sfx::Shotgun => {
            b = vec![0.0; len(1.2)];
            let mut lp = 0.0;
            for (i, x) in b.iter_mut().enumerate() {
                let t = i as f32 / sr;
                let n = nz.next();
                lp += (n - lp) * 0.3;
                *x = (n * (-t * 40.0).exp() + lp * (-t * 5.0).exp() * 0.6) + (t * 60.0 * TAU).sin() * (-t * 20.0).exp();
            }
            reverb(&mut b, sr, 0.35, 1.3);
        }
        Sfx::Punch | Sfx::Hurt => {
            b = vec![0.0; len(0.25)];
            let mut lp = 0.0;
            for (i, x) in b.iter_mut().enumerate() {
                let t = i as f32 / sr;
                lp += (nz.next() - lp) * 0.15;
                *x = lp * (-t * 25.0).exp() * 2.0 + (t * 90.0 * TAU).sin() * (-t * 30.0).exp();
            }
        }
        Sfx::Door => {
            b = vec![0.0; len(0.6)];
            let mut lp = 0.0;
            for (i, x) in b.iter_mut().enumerate() {
                let t = i as f32 / sr;
                let f = 300.0 + (t * 7.0).sin() * 120.0;
                lp += (nz.next() - lp) * 0.05;
                *x = (t * f * TAU).sin() * 0.15 * (t * 6.0).min(1.0) * (1.0 - t / 0.6) + lp * 0.3;
            }
        }
        Sfx::Scream => {
            b = vec![0.0; len(1.1)];
            let mut ph = 0.0;
            for (i, x) in b.iter_mut().enumerate() {
                let t = i as f32 / sr;
                let f = 700.0 + (t * 30.0).sin() * 40.0 - t * 150.0;
                ph += f / sr;
                let s = (ph * TAU).sin() + 0.5 * (ph * 2.0 * TAU).sin() + 0.3 * nz.next();
                *x = s * env_adsr(t, 0.8, 0.05, 0.1, 0.8, 0.3) * 0.4;
            }
            reverb(&mut b, sr, 0.3, 1.0);
        }
        Sfx::Thunder => {
            b = vec![0.0; len(3.5)];
            let mut lp = 0.0;
            for (i, x) in b.iter_mut().enumerate() {
                let t = i as f32 / sr;
                lp += (nz.next() - lp) * 0.02;
                *x = lp * 4.0 * (-t * 0.9).exp() * (1.0 + (t * 3.0).sin() * 0.4);
            }
        }
        Sfx::Static | Sfx::Whisper => {
            b = vec![0.0; len(1.5)];
            let mut lp = 0.0;
            for (i, x) in b.iter_mut().enumerate() {
                let t = i as f32 / sr;
                let n = nz.next();
                lp += (n - lp) * if s == Sfx::Whisper { 0.2 } else { 0.8 };
                let am = if s == Sfx::Whisper { ((t * 9.0).sin() * (t * 3.1).sin()).abs() } else { 0.6 + 0.4 * (t * 23.0).sin() };
                *x = lp * am * 0.4 * (1.0 - t / 1.5);
            }
        }
        Sfx::Heartbeat => {
            b = vec![0.0; len(0.9)];
            kick(&mut b, 0, 0.8, sr);
            kick(&mut b, len(0.25), 0.55, sr);
            lowpass(&mut b, 0.2);
        }
        Sfx::Echo | Sfx::Alter => {
            b = vec![0.0; len(2.5)];
            for (i, x) in b.iter_mut().enumerate() {
                let t = i as f32 / sr;
                let f = if s == Sfx::Echo { 200.0 + t * 400.0 } else { 800.0 - t * 300.0 };
                *x = ((t * f * TAU).sin() * 0.3 + nz.next() * 0.15) * (t * 2.0).min(1.0) * (1.0 - t / 2.5);
            }
            reverb(&mut b, sr, 0.6, 1.8);
        }
        Sfx::RedSight | Sfx::Hum => {
            b = vec![0.0; len(2.0)];
            for (i, x) in b.iter_mut().enumerate() {
                let t = i as f32 / sr;
                *x = ((t * 55.0 * TAU).sin() + (t * 82.5 * TAU).sin() * 0.5 + (t * 110.3 * TAU).sin() * 0.3) * 0.3 * (t * 3.0).min(1.0) * (1.0 - t / 2.0);
            }
        }
        Sfx::Paper => {
            b = vec![0.0; len(0.35)];
            for (i, x) in b.iter_mut().enumerate() {
                let t = i as f32 / sr;
                *x = nz.next() * 0.4 * ((t * 40.0).sin().abs()) * (1.0 - t / 0.35);
            }
        }
        Sfx::Cash => {
            b = vec![0.0; len(0.6)];
            note(&mut b, 0, 0.1, 88.0, 0.5, Inst::Bell, sr, nz);
            note(&mut b, len(0.08), 0.3, 93.0, 0.5, Inst::Bell, sr, nz);
        }
        Sfx::Lockpick => {
            b = vec![0.0; len(0.15)];
            for (i, x) in b.iter_mut().enumerate() {
                let t = i as f32 / sr;
                *x = (t * 4200.0 * TAU).sin() * (-t * 80.0).exp() * 0.5;
            }
        }
        Sfx::Glass => {
            b = vec![0.0; len(0.8)];
            for k in 0..6 {
                note(&mut b, len(k as f32 * 0.03), 0.05, 96.0 + k as f32 * 3.0, 0.3, Inst::Bell, sr, nz);
            }
            for (i, x) in b.iter_mut().enumerate() {
                let t = i as f32 / sr;
                *x += nz.next() * (-t * 20.0).exp() * 0.5;
            }
        }
        Sfx::Engine => {
            b = vec![0.0; len(1.5)];
            let mut ph = 0.0;
            for (i, x) in b.iter_mut().enumerate() {
                let t = i as f32 / sr;
                ph += (40.0 + t * 20.0) / sr;
                *x = ((ph * TAU).sin().signum() * 0.3 + nz.next() * 0.1) * (t * 4.0).min(1.0) * (1.0 - t / 1.5);
            }
            lowpass(&mut b, 0.1);
        }
        Sfx::Bell => {
            b = vec![0.0; len(4.0)];
            note(&mut b, 0, 2.0, 50.0, 0.8, Inst::Bell, sr, nz);
            reverb(&mut b, sr, 0.4, 1.5);
        }
        Sfx::Whistle => {
            b = vec![0.0; len(0.9)];
            for (i, x) in b.iter_mut().enumerate() {
                let t = i as f32 / sr;
                let f = 2600.0 + (t * 40.0 * TAU).sin() * 200.0;
                *x = (t * f * TAU).sin() * 0.3 * env_adsr(t, 0.7, 0.02, 0.05, 0.9, 0.1);
            }
        }
        Sfx::Siren => {
            b = vec![0.0; len(3.0)];
            let mut ph = 0.0;
            for (i, x) in b.iter_mut().enumerate() {
                let t = i as f32 / sr;
                let f = 700.0 + (t * 0.8 * TAU).sin() * 300.0;
                ph += f / sr;
                *x = (ph * TAU).sin() * 0.25;
            }
        }
        Sfx::Shutter => {
            b = vec![0.0; len(0.2)];
            for (i, x) in b.iter_mut().enumerate() {
                let t = i as f32 / sr;
                *x = nz.next() * ((-t * 80.0).exp() + (-(t - 0.08).abs() * 200.0).exp()) * 0.6;
            }
        }
        Sfx::Step => {
            b = vec![0.0; len(0.08)];
            let mut lp = 0.0;
            for (i, x) in b.iter_mut().enumerate() {
                let t = i as f32 / sr;
                lp += (nz.next() - lp) * 0.3;
                *x = lp * (-t * 70.0).exp() * 0.5;
            }
        }
        Sfx::Evidence => {
            b = vec![0.0; len(1.2)];
            note(&mut b, 0, 0.3, 69.0, 0.4, Inst::Bell, sr, nz);
            note(&mut b, len(0.15), 0.3, 72.0, 0.4, Inst::Bell, sr, nz);
            note(&mut b, len(0.3), 0.6, 76.0, 0.4, Inst::Bell, sr, nz);
            reverb(&mut b, sr, 0.4, 1.2);
        }
    }
    normalize(&mut b, 0.8);
    b
}

// ------------------------------------------------------------------ runtime

#[derive(Resource, Default)]
pub struct AudioBank {
    pub sfx: std::collections::HashMap<Sfx, Handle<AudioSource>>,
    pub music: std::collections::HashMap<Track, Handle<AudioSource>>,
    pub ambience: Option<Handle<AudioSource>>,
    pub pending: Option<Arc<Mutex<Receiver<(Track, Vec<u8>)>>>>,
}

#[derive(Resource)]
pub struct MusicState {
    pub want: Track,
    pub playing: Option<(Track, Entity)>,
    pub fading: Vec<(Entity, f32)>,
    pub vol: f32,
    pub duck: f32,
}

impl Default for MusicState {
    fn default() -> Self {
        MusicState { want: Track::Menu, playing: None, fading: Vec::new(), vol: 1.0, duck: 1.0 }
    }
}

#[derive(Component)]
pub struct MusicPlayer;

#[derive(Component)]
pub struct RainAmbience;

pub fn setup_audio(mut bank: ResMut<AudioBank>, mut assets: ResMut<Assets<AudioSource>>) {
    let mut nz = Noise(0x1234_5678);
    use Sfx::*;
    for s in [Click, Tick, Type, Gunshot, Shotgun, Punch, Door, Scream, Thunder, Static, Heartbeat, Echo, RedSight, Paper, Cash, Lockpick, Glass, Engine, Hum, Whisper, Bell, Whistle, Siren, Shutter, Step, Alter, Evidence, Hurt] {
        let b = render_sfx(s, &mut nz);
        bank.sfx.insert(s, assets.add(AudioSource { bytes: wav(&b, SR, false).into() }));
    }
    // rain ambience loop
    let mut rain = vec![0.0f32; (SR * 6) as usize];
    let mut lp = 0.0;
    let mut lp2 = 0.0;
    for (i, x) in rain.iter_mut().enumerate() {
        let n = nz.next();
        lp += (n - lp) * 0.35;
        lp2 += (n - lp2) * 0.02;
        let drip = if nz.next() > 0.9985 { nz.next() * 0.6 } else { 0.0 };
        *x = lp * 0.35 + lp2 * 0.9 + drip;
        let fade = (i as f32 / 2000.0).min(1.0) * ((rain_len() - i) as f32 / 2000.0).min(1.0);
        *x *= fade.max(0.0);
    }
    normalize(&mut rain, 0.5);
    bank.ambience = Some(assets.add(AudioSource { bytes: wav(&rain, SR, false).into() }));
    // render music in the background
    let (tx, rx) = channel();
    std::thread::spawn(move || {
        let order = [Track::Menu, Track::Limbo, Track::Era1920, Track::Tension, Track::Era1934, Track::Era1948, Track::Era1969, Track::Era1986, Track::Era2001, Track::Finale];
        for (k, t) in order.iter().enumerate() {
            let mut nz = Noise(0xABCD_0000 + k as u32 * 7919);
            let b = render_era(*t, &mut nz);
            if tx.send((*t, wav(&b, SR, false))).is_err() {
                break;
            }
        }
    });
    bank.pending = Some(Arc::new(Mutex::new(rx)));
}

fn rain_len() -> usize {
    (SR * 6) as usize
}

pub fn receive_music(mut bank: ResMut<AudioBank>, mut assets: ResMut<Assets<AudioSource>>) {
    let Some(p) = bank.pending.clone() else { return };
    let Ok(rx) = p.lock() else { return };
    while let Ok((t, bytes)) = rx.try_recv() {
        let h = assets.add(AudioSource { bytes: bytes.into() });
        bank.music.insert(t, h);
    }
}

pub fn play_sfx(mut c: Commands, mut ev: EventReader<Sfx>, bank: Res<AudioBank>, settings: Res<crate::keys::Settings>) {
    let mut seen = Vec::new();
    for s in ev.read() {
        if seen.contains(s) {
            continue;
        }
        seen.push(*s);
        if let Some(h) = bank.sfx.get(s) {
            let base = match s {
                Sfx::Tick => 0.25,
                Sfx::Step => 0.25,
                Sfx::Thunder => 0.9,
                Sfx::Gunshot | Sfx::Shotgun => 0.8,
                _ => 0.6,
            };
            c.spawn((AudioPlayer(h.clone()), PlaybackSettings { mode: PlaybackMode::Despawn, volume: Volume::Linear(base * settings.sfx * settings.master), ..default() }));
        }
    }
}

pub fn music_system(
    mut c: Commands,
    bank: Res<AudioBank>,
    mut ms: ResMut<MusicState>,
    time: Res<Time>,
    mut sinks: Query<&mut AudioSink, (With<MusicPlayer>, Without<RainAmbience>)>,
    settings: Res<crate::keys::Settings>,
    game: Res<crate::state::Game>,
    mut rain: Query<(Entity, &mut AudioSink), (With<RainAmbience>, Without<MusicPlayer>)>,
    ui: Res<crate::ui::UiState>,
) {
    let dt = time.delta_secs();
    let target_vol = settings.music * settings.master * ms.duck * 0.55;
    // switch tracks
    let need_switch = ms.playing.map(|(t, _)| t != ms.want).unwrap_or(true);
    if need_switch {
        if let Some(h) = bank.music.get(&ms.want) {
            if let Some((_, e)) = ms.playing.take() {
                ms.fading.push((e, 1.0));
            }
            let e = c
                .spawn((AudioPlayer(h.clone()), PlaybackSettings { mode: PlaybackMode::Loop, volume: Volume::Linear(0.0), ..default() }, MusicPlayer))
                .id();
            ms.playing = Some((ms.want, e));
            ms.vol = 0.0;
        }
    }
    ms.vol = (ms.vol + dt * 0.4).min(1.0);
    if let Some((_, e)) = ms.playing {
        if let Ok(mut s) = sinks.get_mut(e) {
            s.set_volume(Volume::Linear(target_vol * ms.vol));
        }
    }
    let mut keep = Vec::new();
    for (e, v) in ms.fading.drain(..).collect::<Vec<_>>() {
        let nv = v - dt * 0.6;
        if nv <= 0.0 {
            c.entity(e).despawn();
        } else {
            if let Ok(mut s) = sinks.get_mut(e) {
                s.set_volume(Volume::Linear(target_vol * nv));
            }
            keep.push((e, nv));
        }
    }
    ms.fading = keep;
    // rain ambience follows the weather
    let want_rain = game.rain * settings.sfx * settings.master * 0.5 * if ui.mode == crate::ui::Mode::Title { 0.3 } else { 1.0 };
    match rain.single_mut() {
        Ok((_, mut s)) => s.set_volume(Volume::Linear(want_rain)),
        Err(_) => {
            if let Some(h) = &bank.ambience {
                c.spawn((AudioPlayer(h.clone()), PlaybackSettings { mode: PlaybackMode::Loop, volume: Volume::Linear(0.0), ..default() }, RainAmbience));
            }
        }
    }
}

/// Dump music to WAV files (debug helper: `--render-music DIR`).
pub fn render_to_dir(dir: &str) {
    let _ = std::fs::create_dir_all(dir);
    let order = [Track::Menu, Track::Limbo, Track::Era1920, Track::Era1934, Track::Era1948, Track::Era1969, Track::Era1986, Track::Era2001, Track::Finale, Track::Tension];
    for t in order {
        let mut nz = Noise(42);
        let b = render_era(t, &mut nz);
        let _ = std::fs::write(format!("{}/{:?}.wav", dir, t), wav(&b, SR, false));
    }
}

// ------------------------------------------------------------------ venue music (never the same)

/// Improvised piece for bars/clubs/hotels/radios: random key, tempo,
/// progression and melody, orchestrated for the era. Every call differs.
pub fn render_venue(year: i32, seed: u32) -> Vec<f32> {
    let sr = SR as f32;
    let mut nz = Noise(seed.wrapping_mul(2654435761) | 1);
    let mut rnd = |n: u32| -> u32 {
        let v = nz.next();
        ((v * 0.5 + 0.5) * n as f32) as u32 % n.max(1)
    };
    let root = 45.0 + rnd(12) as f32;
    let minor = rnd(3) != 0;
    let bpm = 70.0 + rnd(60) as f32;
    let beat = 60.0 / bpm;
    let scale: [f32; 7] = if minor { [0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0] } else { [0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 11.0] };
    let degrees = [0usize, 3, 4, 5, 1, 6];
    let bars = 16;
    let mut prog = Vec::new();
    for _ in 0..bars {
        prog.push(degrees[rnd(degrees.len() as u32) as usize]);
    }
    prog[0] = 0;
    prog[bars - 1] = 0;
    let total = ((bars as f32 * 4.0 * beat + 2.5) * sr) as usize;
    let mut buf = vec![0.0f32; total];
    let mut n2 = Noise(seed ^ 0xBEEF);
    let (bass, chord_i, lead, drums_brush, electric) = if year < 1935 {
        (Inst::Bass, Inst::Piano, if rnd(2) == 0 { Inst::Horn } else { Inst::Clarinet }, true, false)
    } else if year < 1960 {
        (Inst::Bass, Inst::Piano, if rnd(2) == 0 { Inst::Violin } else { Inst::Horn }, true, false)
    } else if year < 1978 {
        (Inst::Bass, Inst::EPiano, if rnd(2) == 0 { Inst::Organ } else { Inst::EPiano }, false, true)
    } else {
        (Inst::Bass, Inst::SynthPad, Inst::SynthLead, false, true)
    };
    let swing = if drums_brush { 0.64 } else { 0.5 };
    let pos = |b: f32| -> usize {
        let w = b.floor();
        let f = b - w;
        let sw = if (f - 0.5).abs() < 0.01 { swing } else { f };
        ((w + sw) * beat * sr) as usize
    };
    let mut prev_note = 7i32;
    for (bar, deg) in prog.iter().enumerate() {
        let b0 = bar as f32 * 4.0;
        let chord: Vec<f32> = (0..3).map(|k| root + scale[(deg + k * 2) % 7] + if deg + k * 2 >= 7 { 12.0 } else { 0.0 }).collect();
        for k in 0..4 {
            let n = chord[(k + rnd(2) as usize) % 3] - 12.0;
            note(&mut buf, pos(b0 + k as f32), beat * 0.9, n, 0.45, bass, sr, &mut n2);
        }
        let comp: &[f32] = if rnd(2) == 0 { &[1.0, 3.0] } else { &[0.0, 1.5, 2.5] };
        for k in comp {
            for n in &chord {
                note(&mut buf, pos(b0 + k), beat * 0.7, *n + 12.0, 0.12, chord_i, sr, &mut n2);
            }
        }
        for k in 0..4 {
            if drums_brush {
                snare(&mut buf, pos(b0 + k as f32), if k % 2 == 1 { 0.15 } else { 0.06 }, true, sr, &mut n2);
            } else {
                if k % 2 == 0 {
                    kick(&mut buf, pos(b0 + k as f32), 0.4, sr);
                } else {
                    snare(&mut buf, pos(b0 + k as f32), 0.22, false, sr, &mut n2);
                }
                hat(&mut buf, pos(b0 + k as f32 + 0.5), 0.15, sr, &mut n2);
            }
        }
        // improvised melody: a random walk on the scale, resting sometimes
        let mut b = 0.0;
        while b < 4.0 {
            let len = [0.5, 0.5, 1.0, 1.0, 1.5, 2.0][rnd(6) as usize];
            if rnd(5) != 0 {
                prev_note = (prev_note + rnd(5) as i32 - 2).clamp(0, 13);
                let deg_n = prev_note as usize % 7;
                let oct = (prev_note / 7) as f32 * 12.0;
                let p = root + 12.0 + scale[deg_n] + oct;
                note(&mut buf, pos(b0 + b), beat * len * 0.9, p, 0.26, lead, sr, &mut n2);
            }
            b += len;
        }
    }
    if year < 1950 {
        lowpass(&mut buf, 0.35);
        for s in buf.iter_mut() {
            let n = n2.next();
            *s += n * 0.01;
            if n > 0.9994 {
                *s += 0.3 * n2.next();
            }
        }
    }
    let _ = electric;
    reverb(&mut buf, sr, 0.25, 1.0);
    normalize(&mut buf, 0.6);
    buf
}

#[derive(Resource)]
pub struct VenueGen {
    tx: std::sync::Mutex<std::sync::mpsc::Sender<(i32, u32)>>,
    rx: std::sync::Mutex<std::sync::mpsc::Receiver<(u32, Vec<u8>)>>,
    pub ready: Vec<(u32, Handle<AudioSource>)>,
    pub requested: Vec<u32>,
}

pub fn start_venue_gen(mut c: Commands) {
    let (tx, rx_req) = channel::<(i32, u32)>();
    let (tx_out, rx) = channel::<(u32, Vec<u8>)>();
    std::thread::spawn(move || {
        while let Ok((year, seed)) = rx_req.recv() {
            let b = render_venue(year, seed);
            if tx_out.send((seed, wav(&b, SR, false))).is_err() {
                break;
            }
        }
    });
    c.insert_resource(VenueGen { tx: std::sync::Mutex::new(tx), rx: std::sync::Mutex::new(rx), ready: Vec::new(), requested: Vec::new() });
}

impl VenueGen {
    pub fn request(&mut self, year: i32, seed: u32) {
        if self.requested.contains(&seed) {
            return;
        }
        self.requested.push(seed);
        if let Ok(tx) = self.tx.lock() {
            let _ = tx.send((year, seed));
        }
    }
    pub fn poll(&mut self, assets: &mut Assets<AudioSource>) {
        let got: Vec<(u32, Vec<u8>)> = match self.rx.lock() {
            Ok(r) => r.try_iter().collect(),
            Err(_) => return,
        };
        for (s, b) in got {
            self.ready.push((s, assets.add(AudioSource { bytes: b.into() })));
        }
    }
}
