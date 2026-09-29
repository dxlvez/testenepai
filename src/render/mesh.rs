//! Tiny procedural mesh builder: every building, prop, car and person in the
//! game is assembled from these primitives with vertex colours.

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

#[derive(Default, Clone)]
pub struct MB {
    pub pos: Vec<[f32; 3]>,
    pub nrm: Vec<[f32; 3]>,
    pub col: Vec<[f32; 4]>,
    pub uv: Vec<[f32; 2]>,
    pub idx: Vec<u32>,
}

pub fn c3(c: [f32; 3]) -> [f32; 4] {
    // vertex colours are linear; our palette values are authored in sRGB
    let l = |v: f32| if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) };
    [l(c[0]), l(c[1]), l(c[2]), 1.0]
}

impl MB {
    pub fn new() -> MB {
        MB::default()
    }

    pub fn is_empty(&self) -> bool {
        self.idx.is_empty()
    }

    /// Quad with explicit corners (counter-clockwise when seen from the front).
    pub fn quad(&mut self, a: Vec3, b: Vec3, c: Vec3, d: Vec3, col: [f32; 4]) {
        let n = (b - a).cross(c - a).normalize_or_zero();
        let base = self.pos.len() as u32;
        for (p, uv) in [(a, [0.0, 0.0]), (b, [1.0, 0.0]), (c, [1.0, 1.0]), (d, [0.0, 1.0])] {
            self.pos.push(p.into());
            self.nrm.push(n.into());
            self.col.push(col);
            self.uv.push(uv);
        }
        self.idx.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    /// Horizontal quad on the XZ plane facing up, with world-space UVs.
    pub fn floor(&mut self, x0: f32, z0: f32, x1: f32, z1: f32, y: f32, col: [f32; 4]) {
        let base = self.pos.len() as u32;
        for (x, z) in [(x0, z0), (x0, z1), (x1, z1), (x1, z0)] {
            self.pos.push([x, y, z]);
            self.nrm.push([0.0, 1.0, 0.0]);
            self.col.push(col);
            self.uv.push([x * 0.5, z * 0.5]);
        }
        self.idx.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    /// Axis aligned box. `skip_bottom` avoids useless faces for things on the ground.
    pub fn cuboid(&mut self, min: Vec3, max: Vec3, col: [f32; 4]) {
        self.cuboid_shaded(min, max, col, true);
    }

    pub fn cuboid_shaded(&mut self, min: Vec3, max: Vec3, col: [f32; 4], skip_bottom: bool) {
        let (x0, y0, z0) = (min.x, min.y, min.z);
        let (x1, y1, z1) = (max.x, max.y, max.z);
        let base = self.pos.len() as u32;
        let faces: [([f32; 3], [[f32; 3]; 4]); 6] = [
            ([0.0, 1.0, 0.0], [[x0, y1, z0], [x0, y1, z1], [x1, y1, z1], [x1, y1, z0]]),
            ([0.0, -1.0, 0.0], [[x0, y0, z0], [x1, y0, z0], [x1, y0, z1], [x0, y0, z1]]),
            ([0.0, 0.0, 1.0], [[x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1]]),
            ([0.0, 0.0, -1.0], [[x1, y0, z0], [x0, y0, z0], [x0, y1, z0], [x1, y1, z0]]),
            ([1.0, 0.0, 0.0], [[x1, y0, z1], [x1, y0, z0], [x1, y1, z0], [x1, y1, z1]]),
            ([-1.0, 0.0, 0.0], [[x0, y0, z0], [x0, y0, z1], [x0, y1, z1], [x0, y1, z0]]),
        ];
        let mut n = 0;
        for (i, (nrm, vs)) in faces.iter().enumerate() {
            if skip_bottom && i == 1 {
                continue;
            }
            // fake ambient occlusion: darken sides towards the bottom a little
            for (k, v) in vs.iter().enumerate() {
                self.pos.push(*v);
                self.nrm.push(*nrm);
                let shade = if nrm[1] == 0.0 && v[1] <= y0 + 0.001 { 0.78 } else { 1.0 };
                self.col.push([col[0] * shade, col[1] * shade, col[2] * shade, col[3]]);
                self.uv.push(match k {
                    0 => [0.0, 0.0],
                    1 => [1.0, 0.0],
                    2 => [1.0, 1.0],
                    _ => [0.0, 1.0],
                });
            }
            let b = base + n * 4;
            self.idx.extend_from_slice(&[b, b + 1, b + 2, b, b + 2, b + 3]);
            n += 1;
        }
    }

    /// Box centred at c with half extents h.
    pub fn bx(&mut self, c: Vec3, h: Vec3, col: [f32; 4]) {
        self.cuboid(c - h, c + h, col);
    }

    pub fn cylinder(&mut self, c: Vec3, r: f32, h: f32, seg: u32, col: [f32; 4]) {
        self.frustum(c, r, r, h, seg, col);
    }

    /// Truncated cone standing on c (base radius r0, top radius r1).
    pub fn frustum(&mut self, c: Vec3, r0: f32, r1: f32, h: f32, seg: u32, col: [f32; 4]) {
        let base = self.pos.len() as u32;
        for i in 0..=seg {
            let a = i as f32 / seg as f32 * std::f32::consts::TAU;
            let (s, co) = a.sin_cos();
            let n = Vec3::new(co, (r0 - r1) / h.max(0.001), s).normalize();
            self.pos.push([c.x + co * r0, c.y, c.z + s * r0]);
            self.nrm.push(n.into());
            self.col.push([col[0] * 0.8, col[1] * 0.8, col[2] * 0.8, col[3]]);
            self.uv.push([i as f32 / seg as f32, 0.0]);
            self.pos.push([c.x + co * r1, c.y + h, c.z + s * r1]);
            self.nrm.push(n.into());
            self.col.push(col);
            self.uv.push([i as f32 / seg as f32, 1.0]);
        }
        for i in 0..seg {
            let a = base + i * 2;
            self.idx.extend_from_slice(&[a, a + 1, a + 3, a, a + 3, a + 2]);
        }
        // top cap
        let top = self.pos.len() as u32;
        self.pos.push([c.x, c.y + h, c.z]);
        self.nrm.push([0.0, 1.0, 0.0]);
        self.col.push(col);
        self.uv.push([0.5, 0.5]);
        for i in 0..=seg {
            let a = i as f32 / seg as f32 * std::f32::consts::TAU;
            let (s, co) = a.sin_cos();
            self.pos.push([c.x + co * r1, c.y + h, c.z + s * r1]);
            self.nrm.push([0.0, 1.0, 0.0]);
            self.col.push(col);
            self.uv.push([0.5, 0.5]);
        }
        for i in 0..seg {
            self.idx.extend_from_slice(&[top, top + 2 + i, top + 1 + i]);
        }
    }

    pub fn sphere(&mut self, c: Vec3, r: Vec3, seg: u32, col: [f32; 4]) {
        let rings = (seg / 2).max(3);
        let base = self.pos.len() as u32;
        for j in 0..=rings {
            let v = j as f32 / rings as f32;
            let phi = v * std::f32::consts::PI;
            for i in 0..=seg {
                let u = i as f32 / seg as f32;
                let th = u * std::f32::consts::TAU;
                let n = Vec3::new(phi.sin() * th.cos(), phi.cos(), phi.sin() * th.sin());
                self.pos.push((c + n * r).into());
                self.nrm.push(n.into());
                let sh = 0.75 + 0.25 * n.y.max(0.0);
                self.col.push([col[0] * sh, col[1] * sh, col[2] * sh, col[3]]);
                self.uv.push([u, v]);
            }
        }
        for j in 0..rings {
            for i in 0..seg {
                let a = base + j * (seg + 1) + i;
                let b = a + seg + 1;
                self.idx.extend_from_slice(&[a, a + 1, b, a + 1, b + 1, b]);
            }
        }
    }

    /// Gable roof prism over the rect, ridge along the longer axis.
    pub fn gable(&mut self, x0: f32, z0: f32, x1: f32, z1: f32, y: f32, rise: f32, col: [f32; 4]) {
        let ridge_x = (x1 - x0) >= (z1 - z0);
        let dark = [col[0] * 0.7, col[1] * 0.7, col[2] * 0.7, col[3]];
        if ridge_x {
            let zm = (z0 + z1) / 2.0;
            let a = Vec3::new(x0, y, z1);
            let b = Vec3::new(x1, y, z1);
            let c = Vec3::new(x1, y + rise, zm);
            let d = Vec3::new(x0, y + rise, zm);
            self.quad(a, b, c, d, col);
            let a2 = Vec3::new(x1, y, z0);
            let b2 = Vec3::new(x0, y, z0);
            let c2 = Vec3::new(x0, y + rise, zm);
            let d2 = Vec3::new(x1, y + rise, zm);
            self.quad(a2, b2, c2, d2, dark);
            // gable ends
            self.tri(Vec3::new(x0, y, z0), Vec3::new(x0, y, z1), Vec3::new(x0, y + rise, zm), dark);
            self.tri(Vec3::new(x1, y, z1), Vec3::new(x1, y, z0), Vec3::new(x1, y + rise, zm), dark);
        } else {
            let xm = (x0 + x1) / 2.0;
            let a = Vec3::new(x1, y, z1);
            let b = Vec3::new(x1, y, z0);
            let c = Vec3::new(xm, y + rise, z0);
            let d = Vec3::new(xm, y + rise, z1);
            self.quad(a, b, c, d, col);
            let a2 = Vec3::new(x0, y, z0);
            let b2 = Vec3::new(x0, y, z1);
            let c2 = Vec3::new(xm, y + rise, z1);
            let d2 = Vec3::new(xm, y + rise, z0);
            self.quad(a2, b2, c2, d2, dark);
            self.tri(Vec3::new(x0, y, z1), Vec3::new(x1, y, z1), Vec3::new(xm, y + rise, z1), col);
            self.tri(Vec3::new(x1, y, z0), Vec3::new(x0, y, z0), Vec3::new(xm, y + rise, z0), dark);
        }
    }

    pub fn tri(&mut self, a: Vec3, b: Vec3, c: Vec3, col: [f32; 4]) {
        let n = (b - a).cross(c - a).normalize_or_zero();
        let base = self.pos.len() as u32;
        for p in [a, b, c] {
            self.pos.push(p.into());
            self.nrm.push(n.into());
            self.col.push(col);
            self.uv.push([0.0, 0.0]);
        }
        self.idx.extend_from_slice(&[base, base + 1, base + 2]);
    }

    pub fn append(&mut self, o: &MB, offset: Vec3) {
        let base = self.pos.len() as u32;
        for p in &o.pos {
            self.pos.push([p[0] + offset.x, p[1] + offset.y, p[2] + offset.z]);
        }
        self.nrm.extend_from_slice(&o.nrm);
        self.col.extend_from_slice(&o.col);
        self.uv.extend_from_slice(&o.uv);
        self.idx.extend(o.idx.iter().map(|i| i + base));
    }

    /// Append with a rotation around Y (radians) then offset.
    pub fn append_rot(&mut self, o: &MB, rot: f32, offset: Vec3) {
        let q = Quat::from_rotation_y(rot);
        let base = self.pos.len() as u32;
        for p in &o.pos {
            let v = q * Vec3::from(*p) + offset;
            self.pos.push(v.into());
        }
        for n in &o.nrm {
            self.nrm.push((q * Vec3::from(*n)).into());
        }
        self.col.extend_from_slice(&o.col);
        self.uv.extend_from_slice(&o.uv);
        self.idx.extend(o.idx.iter().map(|i| i + base));
    }

    pub fn build(self) -> Mesh {
        Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.pos)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, self.nrm)
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, self.col)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, self.uv)
            .with_inserted_indices(Indices::U32(self.idx))
    }
}
