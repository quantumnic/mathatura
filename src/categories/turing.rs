//! Turing Patterns — reaction-diffusion systems that create animal markings.
//!
//! Alan Turing's 1952 paper "The Chemical Basis of Morphogenesis" showed how
//! two interacting chemicals (morphogens) can create stable patterns:
//! spots (leopard), stripes (zebra), and labyrinths (brain coral).

/// Grid cell containing two chemical concentrations.
#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub a: f64, // Activator concentration
    pub b: f64, // Inhibitor concentration
}

/// Parameters for the Gray-Scott reaction-diffusion model.
#[derive(Debug, Clone, Copy)]
pub struct GrayScottParams {
    /// Diffusion rate of chemical A
    pub da: f64,
    /// Diffusion rate of chemical B
    pub db: f64,
    /// Feed rate (replenishment of A)
    pub feed: f64,
    /// Kill rate (removal of B)
    pub kill: f64,
    /// Time step
    pub dt: f64,
}

/// Preset patterns for Gray-Scott model.
#[derive(Debug, Clone, Copy)]
pub enum Preset {
    /// Spots like a leopard
    Spots,
    /// Stripes like a zebra
    Stripes,
    /// Labyrinthine patterns like brain coral
    Coral,
    /// Mitosis-like cell division
    Mitosis,
    /// Worm-like solitons
    Worms,
}

impl Preset {
    pub fn params(self) -> GrayScottParams {
        match self {
            Preset::Spots => GrayScottParams { da: 1.0, db: 0.5, feed: 0.035, kill: 0.065, dt: 1.0 },
            Preset::Stripes => GrayScottParams { da: 1.0, db: 0.5, feed: 0.04, kill: 0.06, dt: 1.0 },
            Preset::Coral => GrayScottParams { da: 1.0, db: 0.5, feed: 0.06, kill: 0.062, dt: 1.0 },
            Preset::Mitosis => GrayScottParams { da: 1.0, db: 0.5, feed: 0.028, kill: 0.062, dt: 1.0 },
            Preset::Worms => GrayScottParams { da: 1.0, db: 0.5, feed: 0.058, kill: 0.065, dt: 1.0 },
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Preset::Spots => "Leopard Spots",
            Preset::Stripes => "Zebra Stripes",
            Preset::Coral => "Brain Coral",
            Preset::Mitosis => "Cell Division",
            Preset::Worms => "Worm Solitons",
        }
    }
}

/// A 2D grid for reaction-diffusion simulation.
#[derive(Debug, Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl Grid {
    /// Create a new grid initialized to the steady state (A=1, B=0)
    /// with a small seed region of B in the center.
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = vec![Cell { a: 1.0, b: 0.0 }; width * height];
        // Seed a small square of B in the center
        let cx = width / 2;
        let cy = height / 2;
        let seed_size = width.min(height) / 10;
        for dy in 0..seed_size {
            for dx in 0..seed_size {
                let x = cx - seed_size / 2 + dx;
                let y = cy - seed_size / 2 + dy;
                if x < width && y < height {
                    cells[y * width + x] = Cell { a: 0.0, b: 1.0 };
                }
            }
        }
        Grid { width, height, cells }
    }

    /// Create with random seed points for more interesting patterns.
    pub fn new_random(width: usize, height: usize, seed: u64) -> Self {
        let mut grid = Self::new(width, height);
        let mut rng = super::fractals::SimpleRng::new(seed);
        // Add several random seed points
        for _ in 0..5 {
            let cx = rng.next_usize(width);
            let cy = rng.next_usize(height);
            let size = 3 + rng.next_usize(8);
            for dy in 0..size {
                for dx in 0..size {
                    let x = (cx + dx).min(width - 1);
                    let y = (cy + dy).min(height - 1);
                    grid.cells[y * width + x] = Cell { a: 0.0, b: 1.0 };
                }
            }
        }
        grid
    }

    /// Get cell at (x, y) with wrapping boundary conditions.
    pub fn get(&self, x: isize, y: isize) -> Cell {
        let wx = ((x % self.width as isize) + self.width as isize) as usize % self.width;
        let wy = ((y % self.height as isize) + self.height as isize) as usize % self.height;
        self.cells[wy * self.width + wx]
    }

    /// Compute Laplacian of chemical concentrations at (x, y).
    fn laplacian(&self, x: usize, y: usize) -> (f64, f64) {
        let xi = x as isize;
        let yi = y as isize;
        let center = self.get(xi, yi);
        // 5-point stencil
        let neighbors = [
            self.get(xi - 1, yi),
            self.get(xi + 1, yi),
            self.get(xi, yi - 1),
            self.get(xi, yi + 1),
        ];
        let la = neighbors.iter().map(|c| c.a).sum::<f64>() - 4.0 * center.a;
        let lb = neighbors.iter().map(|c| c.b).sum::<f64>() - 4.0 * center.b;
        (la, lb)
    }

    /// Advance simulation by one time step using Gray-Scott model.
    ///
    /// ∂A/∂t = Dₐ∇²A - AB² + f(1-A)
    /// ∂B/∂t = D_b∇²B + AB² - (k+f)B
    pub fn step(&mut self, params: &GrayScottParams) {
        let mut new_cells = self.cells.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.cells[y * self.width + x];
                let (la, lb) = self.laplacian(x, y);
                let ab2 = cell.a * cell.b * cell.b;
                let new_a = cell.a + params.dt * (params.da * la - ab2 + params.feed * (1.0 - cell.a));
                let new_b = cell.b + params.dt * (params.db * lb + ab2 - (params.kill + params.feed) * cell.b);
                new_cells[y * self.width + x] = Cell {
                    a: new_a.clamp(0.0, 1.0),
                    b: new_b.clamp(0.0, 1.0),
                };
            }
        }
        self.cells = new_cells;
    }

    /// Run simulation for n steps.
    pub fn simulate(&mut self, params: &GrayScottParams, steps: usize) {
        for _ in 0..steps {
            self.step(params);
        }
    }

    /// Calculate average concentrations.
    pub fn averages(&self) -> (f64, f64) {
        let n = self.cells.len() as f64;
        let avg_a = self.cells.iter().map(|c| c.a).sum::<f64>() / n;
        let avg_b = self.cells.iter().map(|c| c.b).sum::<f64>() / n;
        (avg_a, avg_b)
    }

    /// Calculate pattern contrast (variance of B).
    pub fn contrast(&self) -> f64 {
        let (_, avg_b) = self.averages();
        let variance = self.cells.iter().map(|c| (c.b - avg_b).powi(2)).sum::<f64>() / self.cells.len() as f64;
        variance.sqrt()
    }
}

/// Generate a simple SVG heatmap of the grid's B chemical.
pub fn grid_to_svg(grid: &Grid) -> String {
    let scale = 4;
    let w = grid.width * scale;
    let h = grid.height * scale;
    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">
"#
    );
    for y in 0..grid.height {
        for x in 0..grid.width {
            let cell = &grid.cells[y * grid.width + x];
            let v = (cell.b * 255.0).clamp(0.0, 255.0) as u8;
            let r = v;
            let g = (v as f64 * 0.6) as u8;
            let b_col = 50 + v / 2;
            svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{scale}" height="{scale}" fill="rgb({r},{g},{b_col})"/>
"#,
                x * scale, y * scale
            ));
        }
    }
    svg.push_str("</svg>");
    svg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(50, 50);
        assert_eq!(grid.cells.len(), 2500);
    }

    #[test]
    fn test_grid_has_seed() {
        let grid = Grid::new(50, 50);
        // Center should have B > 0
        let center = &grid.cells[25 * 50 + 25];
        assert!(center.b > 0.0);
    }

    #[test]
    fn test_grid_wrapping() {
        let grid = Grid::new(10, 10);
        let c1 = grid.get(-1, 0);
        let c2 = grid.get(9, 0);
        assert_eq!(c1.a, c2.a);
        assert_eq!(c1.b, c2.b);
    }

    #[test]
    fn test_simulation_step() {
        let mut grid = Grid::new(20, 20);
        let params = Preset::Spots.params();
        let before = grid.cells.clone();
        grid.step(&params);
        // Grid should change after a step
        let changed = grid.cells.iter().zip(before.iter())
            .any(|(a, b)| (a.a - b.a).abs() > 1e-10 || (a.b - b.b).abs() > 1e-10);
        assert!(changed, "Grid should change after simulation step");
    }

    #[test]
    fn test_concentrations_bounded() {
        let mut grid = Grid::new(20, 20);
        let params = Preset::Spots.params();
        grid.simulate(&params, 100);
        for cell in &grid.cells {
            assert!(cell.a >= 0.0 && cell.a <= 1.0, "A out of bounds: {}", cell.a);
            assert!(cell.b >= 0.0 && cell.b <= 1.0, "B out of bounds: {}", cell.b);
        }
    }

    #[test]
    fn test_averages() {
        let grid = Grid::new(10, 10);
        let (avg_a, avg_b) = grid.averages();
        assert!(avg_a > 0.0);
        assert!(avg_b >= 0.0);
    }

    #[test]
    fn test_contrast() {
        let grid = Grid::new(20, 20);
        let c = grid.contrast();
        assert!(c >= 0.0);
    }

    #[test]
    fn test_presets() {
        for preset in [Preset::Spots, Preset::Stripes, Preset::Coral, Preset::Mitosis, Preset::Worms] {
            let params = preset.params();
            assert!(params.da > 0.0);
            assert!(params.db > 0.0);
            assert!(!preset.name().is_empty());
        }
    }

    #[test]
    fn test_random_grid() {
        let grid = Grid::new_random(30, 30, 42);
        // Should have some B seeded
        let total_b: f64 = grid.cells.iter().map(|c| c.b).sum();
        assert!(total_b > 0.0);
    }

    #[test]
    fn test_grid_svg() {
        let grid = Grid::new(10, 10);
        let svg = grid_to_svg(&grid);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("<rect"));
    }
}
