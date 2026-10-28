// Symmetry helpers: mirror and radial.

use glam::Vec2;

pub enum SymmetryMode {
    None,
    MirrorX,
    MirrorY,
    MirrorBoth,
    Radial { count: u32 },
}

pub fn mirror_points(p: Vec2, center: Vec2, mode: &SymmetryMode) -> Vec<Vec2> {
    let mut out = vec![p];
    match mode {
        SymmetryMode::None => {}
        SymmetryMode::MirrorX => out.push(Vec2::new(2.0 * center.x - p.x, p.y)),
        SymmetryMode::MirrorY => out.push(Vec2::new(p.x, 2.0 * center.y - p.y)),
        SymmetryMode::MirrorBoth => {
            out.push(Vec2::new(2.0 * center.x - p.x, p.y));
            out.push(Vec2::new(p.x, 2.0 * center.y - p.y));
            out.push(Vec2::new(2.0 * center.x - p.x, 2.0 * center.y - p.y));
        }
        SymmetryMode::Radial { count } => {
            let n = (*count).max(1);
            for i in 1..n {
                let a = (i as f32) * std::f32::consts::TAU / (n as f32);
                let (s, c) = a.sin_cos();
                let v = p - center;
                let r = Vec2::new(v.x * c - v.y * s, v.x * s + v.y * c);
                out.push(center + r);
            }
        }
    }
    out
}