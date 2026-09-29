//! A* pathfinding over the tile grid. Paths may only pass through the
//! interiors of the origin/destination buildings (no shortcuts through
//! other people's houses), and are smoothed with line-of-sight checks.

use crate::city::map::*;
use bevy::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, PartialEq)]
struct Node {
    f: f32,
    i: usize,
}
impl Eq for Node {}
impl Ord for Node {
    fn cmp(&self, o: &Self) -> Ordering {
        o.f.partial_cmp(&self.f).unwrap_or(Ordering::Equal)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

pub struct Pather {
    g: Vec<f32>,
    from: Vec<u32>,
    stamp: Vec<u32>,
    cur: u32,
}

impl Pather {
    pub fn new(m: &Map) -> Pather {
        let n = (m.w * m.h) as usize;
        Pather { g: vec![0.0; n], from: vec![u32::MAX; n], stamp: vec![0; n], cur: 0 }
    }

    fn passable(m: &Map, x: i32, y: i32, allow: [Option<usize>; 2]) -> Option<f32> {
        if !m.inb(x, y) || m.blocked(x, y) {
            return None;
        }
        let t = m.get(x, y);
        if matches!(t, Tile::Floor | Tile::Door) {
            if let Some(b) = m.building_at_tile(x, y) {
                if m.buildings[b].kind != BKind::Market && allow[0] != Some(b) && allow[1] != Some(b) {
                    return None;
                }
            }
        }
        Some(t.walk_cost())
    }

    pub fn find(&mut self, m: &Map, from: Vec2, to: Vec2, max_nodes: usize) -> Option<Vec<Vec2>> {
        let (sx, sy) = to_tile(from);
        let (tx, ty) = to_tile(to);
        if !m.inb(tx, ty) {
            return None;
        }
        let allow = [m.building_at_tile(sx, sy), m.building_at_tile(tx, ty)];
        if Self::passable(m, tx, ty, allow).is_none() {
            return None;
        }
        if self.g.len() != (m.w * m.h) as usize {
            *self = Pather::new(m);
        }
        self.cur = self.cur.wrapping_add(1);
        if self.cur == 0 {
            self.stamp.iter_mut().for_each(|s| *s = 0);
            self.cur = 1;
        }
        let w = m.w;
        let idx = |x: i32, y: i32| (y * w + x) as usize;
        let start = idx(sx.clamp(0, m.w - 1), sy.clamp(0, m.h - 1));
        let goal = idx(tx, ty);
        let h = |x: i32, y: i32| {
            let dx = (x - tx).abs() as f32;
            let dy = (y - ty).abs() as f32;
            dx.max(dy) + 0.41 * dx.min(dy)
        };
        let mut open = BinaryHeap::new();
        self.g[start] = 0.0;
        self.from[start] = u32::MAX;
        self.stamp[start] = self.cur;
        open.push(Node { f: h(sx, sy), i: start });
        let mut expanded = 0;
        let mut found = false;
        while let Some(Node { i, .. }) = open.pop() {
            if i == goal {
                found = true;
                break;
            }
            expanded += 1;
            if expanded > max_nodes {
                break;
            }
            let x = (i as i32) % w;
            let y = (i as i32) / w;
            let gi = self.g[i];
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)] {
                let nx = x + dx;
                let ny = y + dy;
                let cost = match Self::passable(m, nx, ny, allow) {
                    Some(c) => c,
                    None => continue,
                };
                if dx != 0 && dy != 0 {
                    // no corner cutting
                    if Self::passable(m, x + dx, y, allow).is_none() || Self::passable(m, x, y + dy, allow).is_none() {
                        continue;
                    }
                }
                let step = if dx != 0 && dy != 0 { 1.414 } else { 1.0 } * cost;
                let ni = idx(nx, ny);
                let ng = gi + step;
                if self.stamp[ni] != self.cur || ng < self.g[ni] {
                    self.stamp[ni] = self.cur;
                    self.g[ni] = ng;
                    self.from[ni] = i as u32;
                    open.push(Node { f: ng + h(nx, ny), i: ni });
                }
            }
        }
        if !found {
            return None;
        }
        let mut pts = Vec::new();
        let mut c = goal;
        while c != start && c as u32 != u32::MAX {
            let x = (c as i32) % w;
            let y = (c as i32) / w;
            pts.push(tile_center(x, y));
            let f = self.from[c];
            if f == u32::MAX {
                break;
            }
            c = f as usize;
        }
        pts.reverse();
        if let Some(l) = pts.last_mut() {
            *l = to;
        }
        Some(smooth(m, from, pts))
    }
}

fn smooth(m: &Map, from: Vec2, pts: Vec<Vec2>) -> Vec<Vec2> {
    if pts.len() < 3 {
        return pts;
    }
    let mut out = Vec::new();
    let mut anchor = from;
    let mut i = 0;
    while i < pts.len() {
        // advance as far as there is a clear straight line from anchor
        let mut j = i;
        while j + 1 < pts.len() && clear(m, anchor, pts[j + 1]) {
            j += 1;
        }
        out.push(pts[j]);
        anchor = pts[j];
        i = j + 1;
    }
    out
}

fn clear(m: &Map, a: Vec2, b: Vec2) -> bool {
    let d = b - a;
    let len = d.length();
    let steps = (len / 0.25).ceil() as i32;
    let n = d.perp().normalize_or_zero() * 0.28;
    for k in 1..=steps {
        let p = a + d * (k as f32 / steps as f32);
        for q in [p, p + n, p - n] {
            let (x, y) = to_tile(q);
            if m.blocked(x, y) {
                return false;
            }
            // don't smooth through doors diagonally or across different buildings
            if matches!(m.get(x, y), Tile::Floor | Tile::Door) != matches!(m.get(to_tile(a).0, to_tile(a).1), Tile::Floor | Tile::Door) && m.get(x, y) != Tile::Door {
                return false;
            }
        }
    }
    true
}
