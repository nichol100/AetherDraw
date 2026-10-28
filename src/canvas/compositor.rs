// Layer compositor: flattens the layer stack into a single RGBA image.
//
// Phase 1 uses a simple CPU compositor. Later we move this to a GPU render pass.

use crate::canvas::layer::{BlendMode, Layer};
use glam::Vec3;

/// Blend two pixels (src over dst) using the given blend mode and opacity.
pub fn blend_pixel(dst: [u8; 4], src: [u8; 4], mode: BlendMode, opacity: f32) -> [u8; 4] {
    // Normalize to 0..1
    let (dr, dg, db, da) = to_f(dst);
    let (sr, sg, sb, sa) = to_f(src);

    let sa = sa * opacity.clamp(0.0, 1.0);

    // Blend RGB according to mode.
    let (br, bg, bb) = match mode {
        BlendMode::Normal => (sr, sg, sb),
        BlendMode::Multiply => (dr * sr, dg * sg, db * sb),
        BlendMode::Screen => (1.0 - (1.0 - dr) * (1.0 - sr),
                            1.0 - (1.0 - dg) * (1.0 - sg),
                            1.0 - (1.0 - db) * (1.0 - sb)),
        BlendMode::Overlay => (
            overlay(dr, sr),
            overlay(dg, sg),
            overlay(db, sb),
        ),
        BlendMode::Add => ((dr + sr).min(1.0), (dg + sg).min(1.0), (db + sb).min(1.0)),
        BlendMode::Darken => (dr.min(sr), dg.min(sg), db.min(sb)),
        BlendMode::Lighten => (dr.max(sr), dg.max(sg), db.max(sb)),
        BlendMode::Difference => ((dr - sr).abs(), (dg - sg).abs(), (db - sb).abs()),
        // The remaining modes fall back to normal for the Phase-1 scaffold.
        _ => (sr, sg, sb),
    };

    // Alpha compositing (src over dst).
    let out_a = sa + da * (1.0 - sa);
    if out_a <= 0.0 {
        return [0, 0, 0, 0];
    }
    let out_r = (br * sa + dr * da * (1.0 - sa)) / out_a;
    let out_g = (bg * sa + dg * da * (1.0 - sa)) / out_a;
    let out_b = (bb * sa + db * da * (1.0 - sa)) / out_a;

    [
        (out_r * 255.0) as u8,
        (out_g * 255.0) as u8,
        (out_b * 255.0) as u8,
        (out_a * 255.0) as u8,
    ]
}

fn to_f(p: [u8; 4]) -> (f32, f32, f32, f32) {
    (
        p[0] as f32 / 255.0,
        p[1] as f32 / 255.0,
        p[2] as f32 / 255.0,
        p[3] as f32 / 255.0,
    )
}

fn overlay(d: f32, s: f32) -> f32 {
    if d < 0.5 {
        2.0 * d * s
    } else {
        1.0 - 2.0 * (1.0 - d) * (1.0 - s)
    }
}

/// Flatten a list of layers into one RGBA buffer.
pub fn composite(layers: &[Layer], width: u32, height: u32) -> Vec<u8> {
    let mut out = vec![0u8; (width * height * 4) as usize];

    for layer in layers {
        let (visible, opacity, blend, pixels, lw, lh) = match layer {
            Layer::Raster(r) => (
                r.visible,
                r.opacity,
                r.blend,
                &r.pixels,
                r.width,
                r.height,
            ),
            _ => continue, // Phase 1: only raster layers are composited.
        };
        if !visible {
            continue;
        }
        if lw != width || lh != height {
            continue; // size mismatch; skip in scaffold
        }

        for y in 0..height {
            for x in 0..width {
                let i = ((y * width + x) * 4) as usize;
                let src = [pixels[i], pixels[i + 1], pixels[i + 2], pixels[i + 3]];
                let dst = [out[i], out[i + 1], out[i + 2], out[i + 3]];
                let res = blend_pixel(dst, src, blend, opacity);
                out[i..i + 4].copy_from_slice(&res);
            }
        }
    }

    // Keep Vec3 import used (placeholder for future 3D compositing).
    let _ = Vec3::ZERO;

    out
}

// Helper to convert color to f32 unused warning suppress.
#[allow(dead_code)]
fn _suppress() {}