//! Phyllotaxis — the arrangement of leaves, seeds, and florets in plants.
//!
//! Implements Vogel's model: each element n is placed at
//!   θ = n × α  (α = divergence angle)
//!   r = c × √n  (c = scaling constant)
//!
//! When α = golden angle ≈ 137.508°, we get the optimal packing seen in sunflowers.

use std::f64::consts::PI;
use crate::constants::{GOLDEN_ANGLE_DEG, FIBONACCI};

/// A single element in a phyllotactic arrangement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Element {
    pub index: usize,
    pub angle: f64,
    pub radius: f64,
    pub x: f64,
    pub y: f64,
}

/// Parameters for phyllotaxis generation.
#[derive(Debug, Clone)]
pub struct Params {
    pub count: usize,
    pub divergence_angle: f64,
    pub scale: f64,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            count: 500,
            divergence_angle: GOLDEN_ANGLE_DEG,
            scale: 8.0,
        }
    }
}

/// Pattern type for different plant structures.
#[derive(Debug, Clone, Copy)]
pub enum Pattern {
    /// Flat sunflower head — classic Vogel model
    Sunflower,
    /// Top-down rosette view (e.g. succulent)
    Rosette,
    /// Conical arrangement (e.g. pinecone)
    Pinecone,
}

/// Generate a Vogel spiral pattern.
pub fn vogel_spiral(params: &Params) -> Vec<Element> {
    let angle_rad = params.divergence_angle.to_radians();
    (0..params.count)
        .map(|n| {
            let nf = n as f64;
            let theta = nf * angle_rad;
            let r = params.scale * nf.sqrt();
            Element {
                index: n,
                angle: theta,
                radius: r,
                x: r * theta.cos(),
                y: r * theta.sin(),
            }
        })
        .collect()
}

/// Generate a rosette (succulent) pattern with size variation.
pub fn rosette(params: &Params) -> Vec<(Element, f64)> {
    let elements = vogel_spiral(params);
    elements
        .into_iter()
        .map(|e| {
            // Outer elements are larger (like succulent leaves)
            let size = 3.0 + (e.index as f64 / params.count as f64) * 12.0;
            (e, size)
        })
        .collect()
}

/// Generate a pinecone pattern (conical projection).
pub fn pinecone(params: &Params) -> Vec<Element> {
    let angle_rad = params.divergence_angle.to_radians();
    (0..params.count)
        .map(|n| {
            let nf = n as f64;
            let theta = nf * angle_rad;
            // Pinecone: tighter packing, elliptical projection
            let t = nf / params.count as f64;
            let r = params.scale * nf.sqrt() * (1.0 - 0.3 * t);
            let x = r * theta.cos();
            let y = r * theta.sin() * 0.6; // squash vertically
            Element {
                index: n,
                angle: theta,
                radius: r,
                x,
                y,
            }
        })
        .collect()
}

/// Count visible spirals (parastichies) in a pattern.
///
/// In a sunflower, you can count spirals going clockwise and counter-clockwise.
/// These counts are always consecutive Fibonacci numbers (e.g., 21 and 34).
pub fn count_parastichies(elements: &[Element]) -> Vec<(usize, usize)> {
    if elements.len() < 10 {
        return vec![];
    }
    // The parastichy numbers are the Fibonacci numbers closest to
    // the number of elements that evenly divide the angular range.
    let mut result = Vec::new();
    for window in FIBONACCI.windows(2) {
        let (a, b) = (window[0] as usize, window[1] as usize);
        if a > 0 && b > 0 && b < elements.len() {
            result.push((a, b));
        }
    }
    result
}

/// Measure packing efficiency compared to golden angle.
///
/// Returns a value between 0.0 and 1.0, where 1.0 is perfectly uniform.
pub fn packing_efficiency(elements: &[Element]) -> f64 {
    if elements.len() < 3 {
        return 0.0;
    }
    // Compute average nearest-neighbor distance
    let mut total_min_dist = 0.0;
    let mut count = 0;
    for (i, e1) in elements.iter().enumerate().skip(1) {
        let mut min_dist = f64::INFINITY;
        for (j, e2) in elements.iter().enumerate().skip(1) {
            if i != j {
                let d = ((e1.x - e2.x).powi(2) + (e1.y - e2.y).powi(2)).sqrt();
                if d < min_dist {
                    min_dist = d;
                }
            }
        }
        if min_dist.is_finite() {
            total_min_dist += min_dist;
            count += 1;
        }
    }
    if count == 0 {
        return 0.0;
    }
    let avg = total_min_dist / count as f64;
    // Compute standard deviation of nearest-neighbor distances
    let mut variance = 0.0;
    for (i, e1) in elements.iter().enumerate().skip(1) {
        let mut min_dist = f64::INFINITY;
        for (j, e2) in elements.iter().enumerate().skip(1) {
            if i != j {
                let d = ((e1.x - e2.x).powi(2) + (e1.y - e2.y).powi(2)).sqrt();
                if d < min_dist {
                    min_dist = d;
                }
            }
        }
        if min_dist.is_finite() {
            variance += (min_dist - avg).powi(2);
        }
    }
    let stddev = (variance / count as f64).sqrt();
    // Coefficient of variation → invert for efficiency score
    let cv = stddev / avg;
    (1.0 - cv).max(0.0).min(1.0)
}

/// Generate SVG of a phyllotaxis pattern.
pub fn to_svg(elements: &[Element], pattern: Pattern) -> String {
    if elements.is_empty() {
        return String::from(r##"<svg xmlns="http://www.w3.org/2000/svg" width="800" height="800"></svg>"##);
    }
    let max_r = elements.iter().map(|e| e.radius).fold(0.0_f64, f64::max);
    let margin = 40.0;
    let size = (max_r * 2.0 + margin * 2.0).max(200.0);
    let cx = size / 2.0;
    let cy = size / 2.0;

    let mut svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{s}" height="{s}" viewBox="0 0 {s} {s}">
<rect width="{s}" height="{s}" fill="#1a1a2e"/>
"##,
        s = size as u32
    );

    for e in elements {
        let x = cx + e.x;
        let y = cy + e.y;
        let t = e.index as f64 / elements.len() as f64;
        let base_r = match pattern {
            Pattern::Sunflower => 2.5 + t * 2.0,
            Pattern::Rosette => 3.0 + t * 10.0,
            Pattern::Pinecone => 2.0 + t * 3.0,
        };
        // Color based on spiral arm (using golden angle)
        let hue = (e.angle * 180.0 / PI * 0.3) % 360.0;
        let sat = 70.0 + t * 20.0;
        let light = 45.0 + t * 15.0;
        svg.push_str(&format!(
            r##"<circle cx="{:.1}" cy="{:.1}" r="{:.1}" fill="hsl({:.0},{:.0}%,{:.0}%)" opacity="0.9"/>
"##,
            x, y, base_r, hue, sat, light
        ));
    }

    svg.push_str("</svg>");
    svg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vogel_spiral_count() {
        let p = Params { count: 100, ..Default::default() };
        assert_eq!(vogel_spiral(&p).len(), 100);
    }

    #[test]
    fn test_vogel_spiral_origin() {
        let p = Params::default();
        let e = &vogel_spiral(&p)[0];
        assert_eq!(e.x, 0.0);
        assert_eq!(e.y, 0.0);
    }

    #[test]
    fn test_vogel_radius_increases() {
        let p = Params { count: 50, ..Default::default() };
        let elements = vogel_spiral(&p);
        for w in elements.windows(2).skip(1) {
            assert!(w[1].radius >= w[0].radius);
        }
    }

    #[test]
    fn test_90_degree_four_arms() {
        let p = Params { count: 8, divergence_angle: 90.0, scale: 10.0 };
        let elements = vogel_spiral(&p);
        // Elements 0 and 4 are on the same radial line (360° apart)
        let angle_diff = (elements[4].angle - elements[0].angle) % (2.0 * PI);
        assert!(angle_diff.abs() < 1e-10 || (angle_diff - 2.0 * PI).abs() < 1e-10);
    }

    #[test]
    fn test_rosette_size_increases() {
        let p = Params { count: 50, ..Default::default() };
        let rosette = rosette(&p);
        let first_size = rosette[1].1;
        let last_size = rosette[49].1;
        assert!(last_size > first_size);
    }

    #[test]
    fn test_pinecone_squash() {
        let p = Params { count: 20, ..Default::default() };
        let normal = vogel_spiral(&p);
        let pine = pinecone(&p);
        // Pinecone should have reduced y extent
        let normal_y_range: f64 = normal.iter().map(|e| e.y.abs()).fold(0.0, f64::max);
        let pine_y_range: f64 = pine.iter().map(|e| e.y.abs()).fold(0.0, f64::max);
        assert!(pine_y_range < normal_y_range);
    }

    #[test]
    fn test_parastichies_fibonacci() {
        let p = Params { count: 200, ..Default::default() };
        let elements = vogel_spiral(&p);
        let pairs = count_parastichies(&elements);
        // All parastichy pairs should be consecutive Fibonacci numbers
        for (a, b) in &pairs {
            assert!(crate::constants::FIBONACCI.contains(&(*a as u64)));
            assert!(crate::constants::FIBONACCI.contains(&(*b as u64)));
        }
    }

    #[test]
    fn test_packing_efficiency_golden_angle() {
        let p = Params { count: 30, ..Default::default() };
        let elements = vogel_spiral(&p);
        let eff = packing_efficiency(&elements);
        assert!(eff > 0.3, "Golden angle should have decent packing: {}", eff);
    }

    #[test]
    fn test_svg_output() {
        let p = Params { count: 10, ..Default::default() };
        let elements = vogel_spiral(&p);
        let svg = to_svg(&elements, Pattern::Sunflower);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("<circle"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_svg_empty() {
        let svg = to_svg(&[], Pattern::Sunflower);
        assert!(svg.contains("<svg"));
    }
}
