//! # Mathatura
//!
//! A comprehensive collection of mathematical beauty found in nature.
//!
//! Each category module provides:
//! - Core mathematical functions and models
//! - SVG generation for static output
//! - Parameter structures for interactive exploration
//!
//! ## Categories
//!
//! - **Phyllotaxis**: Leaf spirals, sunflowers, pinecones (golden angle, Vogel's model)
//! - **Fractals**: Barnsley fern, Koch snowflake, Mandelbrot set, Sierpinski triangle
//! - **Spirals**: Logarithmic, Archimedean, Fermat spirals found in shells and galaxies
//! - **Chaos**: Lorenz attractor, logistic map, strange attractors
//! - **L-Systems**: Lindenmayer systems for trees, ferns, branching structures
//! - **Turing Patterns**: Reaction-diffusion systems creating animal markings
//! - **Symmetry**: Bilateral, radial, and rotational symmetry in nature
//! - **Tessellations**: Honeycombs, Voronoi diagrams, natural tilings

pub mod categories;
pub mod render;

/// Mathematical constants used throughout the library.
pub mod constants {
    /// The golden ratio φ = (1 + √5) / 2
    pub const PHI: f64 = 1.618_033_988_749_895;

    /// The golden angle in degrees ≈ 137.508°
    pub const GOLDEN_ANGLE_DEG: f64 = 137.507_764_050_332_64;

    /// The golden angle in radians
    pub const GOLDEN_ANGLE_RAD: f64 = 2.399_963_229_728_653;

    /// Fibonacci numbers up to F(20)
    pub const FIBONACCI: [u64; 21] = [
        0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
        6765,
    ];

    /// Common Fibonacci numbers found in flower petals
    pub const PETAL_COUNTS: [u64; 8] = [1, 2, 3, 5, 8, 13, 21, 34];

    /// Generate Fibonacci sequence of n terms
    pub fn fibonacci_sequence(n: usize) -> Vec<u64> {
        if n == 0 {
            return vec![];
        }
        let mut seq = Vec::with_capacity(n);
        seq.push(0);
        if n >= 2 {
            seq.push(1);
        }
        for i in 2..n {
            seq.push(seq[i - 1] + seq[i - 2]);
        }
        seq
    }

    /// Check if a number is a Fibonacci number
    pub fn is_fibonacci(n: u64) -> bool {
        // A number is Fibonacci if 5n² + 4 or 5n² - 4 is a perfect square
        let check = |x: u64| -> bool {
            let s = (x as f64).sqrt() as u64;
            s * s == x
        };
        let n2 = n.saturating_mul(n).saturating_mul(5);
        check(n2 + 4) || check(n2.saturating_sub(4))
    }

    /// Fibonacci ratios converging to φ
    pub fn fibonacci_ratios(n: usize) -> Vec<f64> {
        let seq = fibonacci_sequence(n);
        seq.windows(2)
            .skip(1)
            .map(|w| w[1] as f64 / w[0] as f64)
            .collect()
    }
}
