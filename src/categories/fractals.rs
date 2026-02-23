//! Fractals — self-similar structures at every scale.
//!
//! Nature is full of fractals: ferns, coastlines, blood vessels, lightning,
//! romanesco broccoli, and snowflakes.

use std::f64::consts::PI;

/// A 2D point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Affine transformation for IFS (Iterated Function Systems).
#[derive(Debug, Clone, Copy)]
pub struct AffineTransform {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
    pub probability: f64,
}

impl AffineTransform {
    pub fn apply(&self, p: Point) -> Point {
        Point {
            x: self.a * p.x + self.b * p.y + self.e,
            y: self.c * p.x + self.d * p.y + self.f,
        }
    }
}

/// Barnsley fern IFS transforms.
pub fn barnsley_fern_transforms() -> Vec<AffineTransform> {
    vec![
        // Stem
        AffineTransform { a: 0.0, b: 0.0, c: 0.0, d: 0.16, e: 0.0, f: 0.0, probability: 0.01 },
        // Largest leaflet
        AffineTransform { a: 0.85, b: 0.04, c: -0.04, d: 0.85, e: 0.0, f: 1.6, probability: 0.85 },
        // Left leaflet
        AffineTransform { a: 0.2, b: -0.26, c: 0.23, d: 0.22, e: 0.0, f: 1.6, probability: 0.07 },
        // Right leaflet
        AffineTransform { a: -0.15, b: 0.28, c: 0.26, d: 0.24, e: 0.0, f: 0.44, probability: 0.07 },
    ]
}

/// Generate Barnsley fern points using the chaos game.
pub fn barnsley_fern(iterations: usize, seed: u64) -> Vec<Point> {
    let transforms = barnsley_fern_transforms();
    let mut points = Vec::with_capacity(iterations);
    let mut p = Point { x: 0.0, y: 0.0 };
    let mut rng = SimpleRng::new(seed);

    for _ in 0..iterations {
        let r = rng.next_f64();
        let mut cumulative = 0.0;
        let mut transform = &transforms[0];
        for t in &transforms {
            cumulative += t.probability;
            if r < cumulative {
                transform = t;
                break;
            }
        }
        p = transform.apply(p);
        points.push(p);
    }
    points
}

/// Koch snowflake: recursive line subdivision.
pub fn koch_snowflake(iterations: usize) -> Vec<Point> {
    // Start with an equilateral triangle
    let s = 300.0;
    let h = s * (3.0_f64).sqrt() / 2.0;
    let mut points = vec![
        Point { x: 0.0, y: h * 2.0 / 3.0 },
        Point { x: s / 2.0, y: -h / 3.0 },
        Point { x: -s / 2.0, y: -h / 3.0 },
        Point { x: 0.0, y: h * 2.0 / 3.0 }, // close
    ];

    for _ in 0..iterations {
        let mut new_points = Vec::new();
        for window in points.windows(2) {
            let (p1, p2) = (window[0], window[1]);
            let dx = p2.x - p1.x;
            let dy = p2.y - p1.y;
            // Divide segment into thirds
            let a = p1;
            let b = Point { x: p1.x + dx / 3.0, y: p1.y + dy / 3.0 };
            let d = Point { x: p1.x + 2.0 * dx / 3.0, y: p1.y + 2.0 * dy / 3.0 };
            // Peak of equilateral triangle
            let c = Point {
                x: b.x + dx / 3.0 * (PI / 3.0).cos() - dy / 3.0 * (PI / 3.0).sin(),
                y: b.y + dx / 3.0 * (PI / 3.0).sin() + dy / 3.0 * (PI / 3.0).cos(),
            };
            new_points.push(a);
            new_points.push(b);
            new_points.push(c);
            new_points.push(d);
        }
        new_points.push(*points.last().unwrap());
        points = new_points;
    }
    points
}

/// Sierpinski triangle via chaos game.
pub fn sierpinski_triangle(iterations: usize, seed: u64) -> Vec<Point> {
    let vertices = [
        Point { x: 0.0, y: 300.0 },
        Point { x: -260.0, y: -150.0 },
        Point { x: 260.0, y: -150.0 },
    ];
    let mut p = Point { x: 0.0, y: 0.0 };
    let mut points = Vec::with_capacity(iterations);
    let mut rng = SimpleRng::new(seed);

    for _ in 0..iterations {
        let v = &vertices[rng.next_usize(3)];
        p = Point {
            x: (p.x + v.x) / 2.0,
            y: (p.y + v.y) / 2.0,
        };
        points.push(p);
    }
    points
}

/// Mandelbrot set: test if point c = (cx, cy) is in the set.
/// Returns iteration count (0 = in set, >0 = escaped at that iteration).
pub fn mandelbrot_escape(cx: f64, cy: f64, max_iter: u32) -> u32 {
    let mut zx = 0.0;
    let mut zy = 0.0;
    for i in 0..max_iter {
        let zx2 = zx * zx;
        let zy2 = zy * zy;
        if zx2 + zy2 > 4.0 {
            return i;
        }
        zy = 2.0 * zx * zy + cy;
        zx = zx2 - zy2 + cx;
    }
    0
}

/// Calculate fractal dimension estimate using box-counting.
pub fn box_counting_dimension(points: &[Point], box_sizes: &[f64]) -> Vec<(f64, f64)> {
    let mut results = Vec::new();
    for &size in box_sizes {
        let mut boxes = std::collections::HashSet::new();
        for p in points {
            let bx = (p.x / size).floor() as i64;
            let by = (p.y / size).floor() as i64;
            boxes.insert((bx, by));
        }
        if !boxes.is_empty() {
            results.push((size.ln(), (boxes.len() as f64).ln()));
        }
    }
    results
}

/// Estimate fractal dimension from box-counting data.
pub fn estimate_dimension(data: &[(f64, f64)]) -> f64 {
    if data.len() < 2 {
        return 0.0;
    }
    // Linear regression on log-log data (negative slope = dimension)
    let n = data.len() as f64;
    let sum_x: f64 = data.iter().map(|(x, _)| x).sum();
    let sum_y: f64 = data.iter().map(|(_, y)| y).sum();
    let sum_xy: f64 = data.iter().map(|(x, y)| x * y).sum();
    let sum_x2: f64 = data.iter().map(|(x, _)| x * x).sum();
    let denom = n * sum_x2 - sum_x * sum_x;
    if denom.abs() < 1e-10 {
        return 0.0;
    }
    -((n * sum_xy - sum_x * sum_y) / denom)
}

/// Generate SVG for Barnsley fern.
pub fn fern_to_svg(points: &[Point]) -> String {
    if points.is_empty() {
        return String::from(r##"<svg xmlns="http://www.w3.org/2000/svg" width="600" height="800"></svg>"##);
    }
    let w = 600;
    let h = 800;
    // Fern coords: x in [-2.5, 2.5], y in [0, 10]
    let scale_x = w as f64 / 5.5;
    let scale_y = h as f64 / 11.0;

    let mut svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">
<rect width="{w}" height="{h}" fill="#0a0a1a"/>
"##
    );

    for p in points {
        let sx = (p.x + 2.75) * scale_x;
        let sy = h as f64 - (p.y * scale_y);
        let green = 100 + ((p.y / 10.0) * 155.0) as u8;
        svg.push_str(&format!(
            r##"<circle cx="{:.1}" cy="{:.1}" r="0.5" fill="rgb(30,{green},50)" opacity="0.7"/>
"##,
            sx, sy
        ));
    }
    svg.push_str("</svg>");
    svg
}

/// Generate SVG for Koch snowflake.
pub fn koch_to_svg(points: &[Point]) -> String {
    let w = 700;
    let h = 700;
    let cx = w as f64 / 2.0;
    let cy = h as f64 / 2.0;

    let mut svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">
<rect width="{w}" height="{h}" fill="#0a0a2e"/>
<polygon points=""##
    );

    for p in points {
        svg.push_str(&format!("{:.1},{:.1} ", cx + p.x, cy - p.y));
    }

    svg.push_str(r##"" fill="none" stroke="#4fc3f7" stroke-width="1.5"/>
</svg>"##);
    svg
}

/// Simple deterministic RNG (xorshift64) for reproducible fractals.
pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed.max(1) }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() as f64) / (u64::MAX as f64)
    }

    pub fn next_usize(&mut self, bound: usize) -> usize {
        (self.next_u64() % bound as u64) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_barnsley_fern_bounds() {
        let points = barnsley_fern(10000, 42);
        assert_eq!(points.len(), 10000);
        // Fern should be roughly in [-2.5, 2.5] x [0, 10]
        for p in &points {
            assert!(p.x > -3.0 && p.x < 3.0, "x out of range: {}", p.x);
            assert!(p.y > -0.5 && p.y < 11.0, "y out of range: {}", p.y);
        }
    }

    #[test]
    fn test_barnsley_fern_deterministic() {
        let a = barnsley_fern(100, 42);
        let b = barnsley_fern(100, 42);
        assert_eq!(a.len(), b.len());
        for (pa, pb) in a.iter().zip(b.iter()) {
            assert_eq!(pa.x, pb.x);
            assert_eq!(pa.y, pb.y);
        }
    }

    #[test]
    fn test_koch_snowflake_initial() {
        let points = koch_snowflake(0);
        assert_eq!(points.len(), 4); // triangle + closing point
    }

    #[test]
    fn test_koch_snowflake_growth() {
        let p0 = koch_snowflake(0);
        let p1 = koch_snowflake(1);
        let p2 = koch_snowflake(2);
        assert!(p1.len() > p0.len());
        assert!(p2.len() > p1.len());
    }

    #[test]
    fn test_sierpinski_triangle() {
        let points = sierpinski_triangle(1000, 42);
        assert_eq!(points.len(), 1000);
    }

    #[test]
    fn test_mandelbrot_in_set() {
        // Origin is in the set
        assert_eq!(mandelbrot_escape(0.0, 0.0, 100), 0);
        // -1,0 is in the set (period-2 cycle)
        assert_eq!(mandelbrot_escape(-1.0, 0.0, 100), 0);
    }

    #[test]
    fn test_mandelbrot_outside() {
        // 2,0 escapes immediately
        assert!(mandelbrot_escape(2.0, 0.0, 100) > 0);
        // Far away escapes fast
        assert!(mandelbrot_escape(5.0, 5.0, 100) > 0);
    }

    #[test]
    fn test_box_counting() {
        let points: Vec<Point> = (0..100).map(|i| {
            let t = i as f64 / 100.0;
            Point { x: t * 100.0, y: t * 100.0 }
        }).collect();
        let sizes = vec![50.0, 25.0, 10.0, 5.0];
        let result = box_counting_dimension(&points, &sizes);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_estimate_dimension_line() {
        // A line should have dimension ≈ 1.0
        let points: Vec<Point> = (0..1000).map(|i| {
            let t = i as f64 / 1000.0;
            Point { x: t * 500.0, y: t * 500.0 }
        }).collect();
        let sizes = vec![100.0, 50.0, 25.0, 10.0, 5.0, 2.0, 1.0];
        let data = box_counting_dimension(&points, &sizes);
        let dim = estimate_dimension(&data);
        assert!(dim > 0.8 && dim < 1.3, "Line dimension should be ~1.0, got {}", dim);
    }

    #[test]
    fn test_affine_transform_apply() {
        let t = AffineTransform { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 5.0, f: 3.0, probability: 1.0 };
        let p = t.apply(Point { x: 1.0, y: 2.0 });
        assert!((p.x - 6.0).abs() < 1e-10);
        assert!((p.y - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_fern_svg() {
        let points = barnsley_fern(100, 42);
        let svg = fern_to_svg(&points);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_koch_svg() {
        let points = koch_snowflake(2);
        let svg = koch_to_svg(&points);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("<polygon"));
    }

    #[test]
    fn test_simple_rng_deterministic() {
        let mut a = SimpleRng::new(42);
        let mut b = SimpleRng::new(42);
        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }
}
