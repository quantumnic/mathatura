//! Chaos theory — deterministic systems with extreme sensitivity to initial conditions.
//!
//! "Does the flap of a butterfly's wings in Brazil set off a tornado in Texas?"
//! — Edward Lorenz

/// A 3D point for Lorenz attractor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Parameters for the Lorenz system.
#[derive(Debug, Clone, Copy)]
pub struct LorenzParams {
    pub sigma: f64,
    pub rho: f64,
    pub beta: f64,
    pub dt: f64,
}

impl Default for LorenzParams {
    fn default() -> Self {
        Self {
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
            dt: 0.01,
        }
    }
}

/// Simulate the Lorenz attractor.
///
/// dx/dt = σ(y - x)
/// dy/dt = x(ρ - z) - y
/// dz/dt = xy - βz
pub fn lorenz_attractor(params: &LorenzParams, steps: usize, initial: Point3D) -> Vec<Point3D> {
    let mut points = Vec::with_capacity(steps);
    let mut p = initial;
    points.push(p);

    for _ in 1..steps {
        let dx = params.sigma * (p.y - p.x);
        let dy = p.x * (params.rho - p.z) - p.y;
        let dz = p.x * p.y - params.beta * p.z;
        p = Point3D {
            x: p.x + dx * params.dt,
            y: p.y + dy * params.dt,
            z: p.z + dz * params.dt,
        };
        points.push(p);
    }
    points
}

/// Logistic map: x_{n+1} = r × x_n × (1 - x_n)
///
/// This simple equation produces chaos for r > ~3.57.
pub fn logistic_map(r: f64, x0: f64, steps: usize) -> Vec<f64> {
    let mut values = Vec::with_capacity(steps);
    let mut x = x0;
    for _ in 0..steps {
        values.push(x);
        x = r * x * (1.0 - x);
    }
    values
}

/// Generate bifurcation diagram data.
///
/// For each r value, runs the logistic map and records the attractor values.
pub fn bifurcation_diagram(r_min: f64, r_max: f64, r_steps: usize, warmup: usize, samples: usize) -> Vec<(f64, f64)> {
    let mut data = Vec::new();
    for i in 0..r_steps {
        let r = r_min + (r_max - r_min) * (i as f64) / (r_steps as f64 - 1.0);
        let values = logistic_map(r, 0.5, warmup + samples);
        for &v in &values[warmup..] {
            data.push((r, v));
        }
    }
    data
}

/// Compute Lyapunov exponent for logistic map at given r.
///
/// Positive Lyapunov exponent → chaos.
pub fn lyapunov_exponent(r: f64, iterations: usize) -> f64 {
    let mut x = 0.5;
    let mut sum = 0.0;
    for _ in 0..iterations {
        let derivative = (r * (1.0 - 2.0 * x)).abs();
        if derivative > 0.0 {
            sum += derivative.ln();
        }
        x = r * x * (1.0 - x);
    }
    sum / iterations as f64
}

/// Demonstrate butterfly effect: two nearby starting points diverge.
pub fn butterfly_effect(params: &LorenzParams, steps: usize, epsilon: f64) -> (Vec<Point3D>, Vec<Point3D>) {
    let start1 = Point3D { x: 1.0, y: 1.0, z: 1.0 };
    let start2 = Point3D { x: 1.0 + epsilon, y: 1.0, z: 1.0 };
    let path1 = lorenz_attractor(params, steps, start1);
    let path2 = lorenz_attractor(params, steps, start2);
    (path1, path2)
}

/// Distance between two 3D points.
pub fn distance_3d(a: &Point3D, b: &Point3D) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)).sqrt()
}

/// Generate SVG of Lorenz attractor (XZ projection).
pub fn lorenz_to_svg(points: &[Point3D]) -> String {
    if points.is_empty() {
        return String::from(r##"<svg xmlns="http://www.w3.org/2000/svg" width="800" height="600"></svg>"##);
    }
    let w = 800;
    let h = 600;
    let min_x = points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
    let max_x = points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
    let min_z = points.iter().map(|p| p.z).fold(f64::INFINITY, f64::min);
    let max_z = points.iter().map(|p| p.z).fold(f64::NEG_INFINITY, f64::max);
    let sx = (w - 80) as f64 / (max_x - min_x).max(1.0);
    let sy = (h - 80) as f64 / (max_z - min_z).max(1.0);

    let mut svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}">
<rect width="{w}" height="{h}" fill="#0a0a1a"/>
<polyline points=""##
    );

    for p in points {
        let x = 40.0 + (p.x - min_x) * sx;
        let y = h as f64 - 40.0 - (p.z - min_z) * sy;
        svg.push_str(&format!("{:.1},{:.1} ", x, y));
    }

    svg.push_str(r##"" fill="none" stroke="#ff6b6b" stroke-width="0.5" opacity="0.8"/>
</svg>"##);
    svg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lorenz_attractor_length() {
        let params = LorenzParams::default();
        let points = lorenz_attractor(&params, 1000, Point3D { x: 1.0, y: 1.0, z: 1.0 });
        assert_eq!(points.len(), 1000);
    }

    #[test]
    fn test_lorenz_bounded() {
        let params = LorenzParams::default();
        let points = lorenz_attractor(&params, 5000, Point3D { x: 1.0, y: 1.0, z: 1.0 });
        for p in &points {
            assert!(p.x.abs() < 50.0, "x unbounded: {}", p.x);
            assert!(p.y.abs() < 60.0, "y unbounded: {}", p.y);
            assert!(p.z < 60.0 && p.z > -5.0, "z unbounded: {}", p.z);
        }
    }

    #[test]
    fn test_logistic_map_fixed_point() {
        // For r=2.0, logistic map converges to x = 1 - 1/r = 0.5
        let values = logistic_map(2.0, 0.1, 100);
        let last = values[99];
        assert!((last - 0.5).abs() < 1e-6, "Expected 0.5, got {}", last);
    }

    #[test]
    fn test_logistic_map_chaos() {
        // For r=3.9, system should be chaotic (no convergence)
        let values = logistic_map(3.9, 0.5, 200);
        let last_20 = &values[180..];
        let unique: std::collections::HashSet<u64> = last_20.iter()
            .map(|v| (v * 1e10) as u64)
            .collect();
        assert!(unique.len() > 10, "Should have many distinct values in chaos");
    }

    #[test]
    fn test_bifurcation_diagram() {
        let data = bifurcation_diagram(2.5, 4.0, 100, 100, 20);
        assert_eq!(data.len(), 100 * 20);
    }

    #[test]
    fn test_lyapunov_positive_for_chaos() {
        let le = lyapunov_exponent(3.9, 10000);
        assert!(le > 0.0, "Lyapunov exponent should be positive for r=3.9: {}", le);
    }

    #[test]
    fn test_lyapunov_negative_for_order() {
        let le = lyapunov_exponent(2.5, 10000);
        assert!(le < 0.0, "Lyapunov exponent should be negative for r=2.5: {}", le);
    }

    #[test]
    fn test_butterfly_effect_divergence() {
        let params = LorenzParams::default();
        let (path1, path2) = butterfly_effect(&params, 3000, 1e-10);
        // Initially very close
        let d_start = distance_3d(&path1[0], &path2[0]);
        assert!(d_start < 1e-9);
        // Eventually diverge significantly
        let d_end = distance_3d(&path1[2999], &path2[2999]);
        assert!(d_end > 1.0, "Paths should diverge: distance = {}", d_end);
    }

    #[test]
    fn test_lorenz_svg() {
        let params = LorenzParams::default();
        let points = lorenz_attractor(&params, 100, Point3D { x: 1.0, y: 1.0, z: 1.0 });
        let svg = lorenz_to_svg(&points);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("polyline"));
    }
}
