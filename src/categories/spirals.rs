//! Spirals — the universe's favorite curve.
//!
//! From nautilus shells to galaxies, hurricanes to DNA helices,
//! spirals appear wherever growth meets rotation.

use std::f64::consts::PI;

/// A point on a spiral curve.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpiralPoint {
    pub x: f64,
    pub y: f64,
    pub theta: f64,
    pub r: f64,
}

/// Type of spiral.
#[derive(Debug, Clone, Copy)]
pub enum SpiralType {
    /// r = a × e^(b×θ) — nautilus, galaxies, hurricanes
    Logarithmic { a: f64, b: f64 },
    /// r = a + b×θ — watch springs, coiled rope
    Archimedean { a: f64, b: f64 },
    /// r = a × √θ — sunflower, phyllotaxis background
    Fermat { a: f64 },
    /// Golden spiral: logarithmic with b = ln(φ)/(π/2)
    Golden { a: f64 },
    /// 3D helix projected to 2D — DNA, vines, horns
    Helix { radius: f64, pitch: f64 },
}

/// Generate points along a spiral.
pub fn generate_spiral(spiral_type: SpiralType, num_points: usize, max_theta: f64) -> Vec<SpiralPoint> {
    let phi = crate::constants::PHI;
    (0..num_points)
        .map(|i| {
            let t = i as f64 / num_points as f64;
            let theta = t * max_theta;
            let r = match spiral_type {
                SpiralType::Logarithmic { a, b } => a * (b * theta).exp(),
                SpiralType::Archimedean { a, b } => a + b * theta,
                SpiralType::Fermat { a } => a * theta.sqrt(),
                SpiralType::Golden { a } => a * ((phi.ln() / (PI / 2.0)) * theta).exp(),
                SpiralType::Helix { radius, .. } => radius,
            };
            let (x, y) = match spiral_type {
                SpiralType::Helix { radius, pitch } => {
                    (radius * theta.cos(), radius * theta.sin() + pitch * theta / (2.0 * PI))
                }
                _ => (r * theta.cos(), r * theta.sin()),
            };
            SpiralPoint { x, y, theta, r }
        })
        .collect()
}

/// Measure how closely a spiral matches the golden spiral.
pub fn golden_spiral_fitness(points: &[SpiralPoint]) -> f64 {
    if points.len() < 2 {
        return 0.0;
    }
    let phi = crate::constants::PHI;
    let golden_b = phi.ln() / (PI / 2.0);

    // For a golden spiral, consecutive quarter-turn radii should have ratio φ
    let mut total_error = 0.0;
    let mut count = 0;
    for w in points.windows(2) {
        if w[0].r > 0.01 && w[1].r > 0.01 {
            let dtheta = w[1].theta - w[0].theta;
            if dtheta > 0.0 {
                let expected_ratio = (golden_b * dtheta).exp();
                let actual_ratio = w[1].r / w[0].r;
                total_error += (actual_ratio - expected_ratio).abs() / expected_ratio;
                count += 1;
            }
        }
    }
    if count == 0 {
        return 0.0;
    }
    (1.0 - total_error / count as f64).max(0.0)
}

/// Calculate arc length of a spiral numerically.
pub fn arc_length(points: &[SpiralPoint]) -> f64 {
    points.windows(2).map(|w| {
        let dx = w[1].x - w[0].x;
        let dy = w[1].y - w[0].y;
        (dx * dx + dy * dy).sqrt()
    }).sum()
}

/// Calculate curvature at each point.
pub fn curvature(points: &[SpiralPoint]) -> Vec<f64> {
    if points.len() < 3 {
        return vec![];
    }
    points.windows(3).map(|w| {
        let (x1, y1) = (w[0].x, w[0].y);
        let (x2, y2) = (w[1].x, w[1].y);
        let (x3, y3) = (w[2].x, w[2].y);
        // Curvature via the Menger curvature formula
        let area = ((x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)).abs();
        let d12 = ((x2-x1).powi(2) + (y2-y1).powi(2)).sqrt();
        let d23 = ((x3-x2).powi(2) + (y3-y2).powi(2)).sqrt();
        let d13 = ((x3-x1).powi(2) + (y3-y1).powi(2)).sqrt();
        let product = d12 * d23 * d13;
        if product > 1e-10 { 4.0 * area / product } else { 0.0 }
    }).collect()
}

/// Generate SVG for a spiral.
pub fn to_svg(points: &[SpiralPoint], color: &str) -> String {
    if points.is_empty() {
        return String::from(r#"<svg xmlns="http://www.w3.org/2000/svg" width="800" height="800"></svg>"#);
    }
    let max_extent = points.iter().map(|p| p.x.abs().max(p.y.abs())).fold(0.0_f64, f64::max);
    let size = (max_extent * 2.2).max(100.0);
    let _cx = size / 2.0;
    let _cy = size / 2.0;

    let hs = size / 2.0;
    let sw = size / 400.0;
    let mut svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"800\" height=\"800\" viewBox=\"{} {} {} {}\">\
         <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#0a0a1a\"/>\
         <polyline points=\"",
        -hs, -hs, size, size,
        -hs, -hs, size, size,
    );

    for p in points {
        svg.push_str(&format!("{:.2},{:.2} ", p.x, p.y));
    }
    svg.push_str(&format!(
        "\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\" stroke-linecap=\"round\" opacity=\"0.9\"/>\
         </svg>",
        color, sw
    ));
    svg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logarithmic_spiral_grows() {
        let points = generate_spiral(
            SpiralType::Logarithmic { a: 1.0, b: 0.15 },
            100, 6.0 * PI,
        );
        assert!(points.last().unwrap().r > points[1].r);
    }

    #[test]
    fn test_archimedean_linear_growth() {
        let points = generate_spiral(
            SpiralType::Archimedean { a: 0.0, b: 1.0 },
            100, 4.0 * PI,
        );
        // Radius should grow linearly with theta
        let mid = &points[50];
        let end = &points[99];
        let ratio = end.r / mid.r;
        let theta_ratio = end.theta / mid.theta;
        assert!((ratio - theta_ratio).abs() < 0.1);
    }

    #[test]
    fn test_fermat_sqrt_growth() {
        let points = generate_spiral(
            SpiralType::Fermat { a: 1.0 },
            100, 16.0 * PI,
        );
        // r should be proportional to √θ
        let p = &points[50];
        assert!((p.r - p.theta.sqrt()).abs() < 0.1);
    }

    #[test]
    fn test_golden_spiral_ratio() {
        let points = generate_spiral(
            SpiralType::Golden { a: 1.0 },
            1000, 4.0 * PI,
        );
        let fitness = golden_spiral_fitness(&points);
        assert!(fitness > 0.95, "Golden spiral should match itself: {}", fitness);
    }

    #[test]
    fn test_helix_constant_radius() {
        let points = generate_spiral(
            SpiralType::Helix { radius: 5.0, pitch: 1.0 },
            100, 4.0 * PI,
        );
        for p in &points {
            assert!((p.r - 5.0).abs() < 1e-10);
        }
    }

    #[test]
    fn test_arc_length_positive() {
        let points = generate_spiral(
            SpiralType::Logarithmic { a: 1.0, b: 0.1 },
            100, 4.0 * PI,
        );
        assert!(arc_length(&points) > 0.0);
    }

    #[test]
    fn test_curvature_length() {
        let points = generate_spiral(
            SpiralType::Logarithmic { a: 1.0, b: 0.1 },
            100, 4.0 * PI,
        );
        let k = curvature(&points);
        assert_eq!(k.len(), 98); // n-2 points
    }

    #[test]
    fn test_spiral_svg() {
        let points = generate_spiral(
            SpiralType::Golden { a: 1.0 },
            50, 4.0 * PI,
        );
        let svg = to_svg(&points, "#ffd700");
        assert!(svg.contains("<svg"));
        assert!(svg.contains("polyline"));
    }
}
