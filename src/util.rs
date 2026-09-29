//! Small shared utilities: deterministic RNG and helpers.

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Rng {
    s: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Rng {
        let mut r = Rng { s: seed ^ 0x9E37_79B9_7F4A_7C15 };
        r.next();
        r.next();
        r
    }
    pub fn next(&mut self) -> u64 {
        // splitmix64
        self.s = self.s.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.s;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    pub fn f(&mut self) -> f32 {
        (self.next() >> 40) as f32 / (1u64 << 24) as f32
    }
    pub fn range(&mut self, lo: i32, hi: i32) -> i32 {
        if hi <= lo {
            return lo;
        }
        lo + (self.next() % (hi - lo) as u64) as i32
    }
    pub fn rangef(&mut self, lo: f32, hi: f32) -> f32 {
        lo + self.f() * (hi - lo)
    }
    pub fn chance(&mut self, p: f32) -> bool {
        self.f() < p
    }
    pub fn pick<'a, T>(&mut self, v: &'a [T]) -> &'a T {
        &v[(self.next() % v.len() as u64) as usize]
    }
    pub fn idx(&mut self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            (self.next() % n as u64) as usize
        }
    }
    pub fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = (self.next() % (i as u64 + 1)) as usize;
            v.swap(i, j);
        }
    }
}

/// Stable hash for procedural variation.
pub fn hash2(x: i32, y: i32, salt: u32) -> u32 {
    let mut h = (x as u32).wrapping_mul(374761393) ^ (y as u32).wrapping_mul(668265263) ^ salt.wrapping_mul(2246822519);
    h = (h ^ (h >> 13)).wrapping_mul(1274126177);
    h ^ (h >> 16)
}

pub fn hashf(x: i32, y: i32, salt: u32) -> f32 {
    (hash2(x, y, salt) & 0xFFFF) as f32 / 65535.0
}

pub fn fmt_money(c: i32) -> String {
    if c < 0 {
        format!("-${}", -c)
    } else {
        format!("${}", c)
    }
}

pub fn clock_str(minutes: f32) -> String {
    let m = minutes.rem_euclid(1440.0) as i32;
    format!("{:02}:{:02}", m / 60, m % 60)
}

/// A process-wide mailbox between systems. (Bevy runs systems on several
/// threads, so `thread_local!` values written by one system may be invisible
/// to the next.) Same `with` / `borrow` / `borrow_mut` shape as a RefCell.
pub struct Shared<T>(std::sync::Mutex<T>);

impl<T> Shared<T> {
    pub const fn new(v: T) -> Self {
        Shared(std::sync::Mutex::new(v))
    }
    pub fn with<R>(&self, f: impl FnOnce(&Self) -> R) -> R {
        f(self)
    }
    pub fn borrow(&self) -> std::sync::MutexGuard<'_, T> {
        self.0.lock().unwrap_or_else(|e| e.into_inner())
    }
    pub fn borrow_mut(&self) -> std::sync::MutexGuard<'_, T> {
        self.borrow()
    }
}
