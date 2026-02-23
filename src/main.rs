use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

use mathatura::categories::{phyllotaxis, fractals, spirals, chaos, lsystems, turing};

#[derive(Parser)]
#[command(name = "mathatura")]
#[command(about = "Mathematical beauty in nature — generate stunning visualizations")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Output file path
    #[arg(short, long, default_value = "output.svg")]
    output: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate phyllotaxis patterns (sunflower, rosette, pinecone)
    Phyllotaxis {
        /// Number of elements
        #[arg(short = 'n', long, default_value_t = 500)]
        count: usize,
        /// Divergence angle in degrees (golden angle ≈ 137.508)
        #[arg(short, long, default_value_t = 137.508)]
        angle: f64,
        /// Scaling factor
        #[arg(short, long, default_value_t = 8.0)]
        scale: f64,
        /// Pattern: sunflower, rosette, pinecone
        #[arg(short, long, default_value = "sunflower")]
        pattern: String,
    },
    /// Generate fractal visualizations
    Fractals {
        /// Type: fern, koch, sierpinski, mandelbrot
        #[arg(short = 't', long, default_value = "fern")]
        fractal_type: String,
        /// Iterations / detail level
        #[arg(short, long, default_value_t = 50000)]
        iterations: usize,
    },
    /// Generate spiral curves
    Spirals {
        /// Type: logarithmic, archimedean, fermat, golden, helix
        #[arg(short = 't', long, default_value = "golden")]
        spiral_type: String,
        /// Number of points
        #[arg(short = 'n', long, default_value_t = 1000)]
        points: usize,
        /// Maximum angle in turns (multiples of 2π)
        #[arg(long, default_value_t = 6.0)]
        turns: f64,
    },
    /// Generate chaos theory visualizations
    Chaos {
        /// Type: lorenz, logistic, bifurcation
        #[arg(short = 't', long, default_value = "lorenz")]
        chaos_type: String,
        /// Number of steps
        #[arg(short = 'n', long, default_value_t = 20000)]
        steps: usize,
    },
    /// Generate L-system patterns
    Lsystem {
        /// Type: tree, koch, sierpinski, dragon, plant
        #[arg(short = 't', long, default_value = "plant")]
        system_type: String,
        /// Number of iterations (careful: grows exponentially!)
        #[arg(short, long, default_value_t = 5)]
        iterations: usize,
    },
    /// Generate Turing reaction-diffusion patterns
    Turing {
        /// Preset: spots, stripes, coral, mitosis, worms
        #[arg(short, long, default_value = "spots")]
        preset: String,
        /// Grid size
        #[arg(short = 's', long, default_value_t = 80)]
        size: usize,
        /// Simulation steps
        #[arg(short = 'n', long, default_value_t = 5000)]
        steps: usize,
    },
    /// Generate the interactive web gallery
    Web {
        /// Output directory for web files
        #[arg(short, long, default_value = "web")]
        dir: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    let svg = match cli.command {
        Commands::Phyllotaxis { count, angle, scale, ref pattern } => {
            let params = phyllotaxis::Params { count, divergence_angle: angle, scale };
            match pattern.as_str() {
                "rosette" => {
                    let elements: Vec<_> = phyllotaxis::rosette(&params).into_iter().map(|(e, _)| e).collect();
                    phyllotaxis::to_svg(&elements, phyllotaxis::Pattern::Rosette)
                }
                "pinecone" => {
                    let elements = phyllotaxis::pinecone(&params);
                    phyllotaxis::to_svg(&elements, phyllotaxis::Pattern::Pinecone)
                }
                _ => {
                    let elements = phyllotaxis::vogel_spiral(&params);
                    phyllotaxis::to_svg(&elements, phyllotaxis::Pattern::Sunflower)
                }
            }
        }
        Commands::Fractals { ref fractal_type, iterations } => {
            match fractal_type.as_str() {
                "koch" => {
                    let points = fractals::koch_snowflake(iterations.min(6));
                    fractals::koch_to_svg(&points)
                }
                "sierpinski" => {
                    let points = fractals::sierpinski_triangle(iterations, 42);
                    // Reuse fern SVG with different scaling
                    let fern_pts: Vec<_> = points.iter().map(|p| fractals::Point { x: p.x / 100.0, y: (p.y + 200.0) / 60.0 }).collect();
                    fractals::fern_to_svg(&fern_pts)
                }
                _ => {
                    let points = fractals::barnsley_fern(iterations, 42);
                    fractals::fern_to_svg(&points)
                }
            }
        }
        Commands::Spirals { ref spiral_type, points, turns } => {
            let max_theta = turns * 2.0 * std::f64::consts::PI;
            let (spiral, color) = match spiral_type.as_str() {
                "logarithmic" => (spirals::SpiralType::Logarithmic { a: 0.5, b: 0.12 }, "#e91e63"),
                "archimedean" => (spirals::SpiralType::Archimedean { a: 0.0, b: 5.0 }, "#2196f3"),
                "fermat" => (spirals::SpiralType::Fermat { a: 5.0 }, "#4caf50"),
                "helix" => (spirals::SpiralType::Helix { radius: 50.0, pitch: 20.0 }, "#9c27b0"),
                _ => (spirals::SpiralType::Golden { a: 0.5 }, "#ffd700"),
            };
            let pts = spirals::generate_spiral(spiral, points, max_theta);
            spirals::to_svg(&pts, color)
        }
        Commands::Chaos { ref chaos_type, steps } => {
            match chaos_type.as_str() {
                _ => {
                    let params = chaos::LorenzParams::default();
                    let points = chaos::lorenz_attractor(&params, steps, chaos::Point3D { x: 1.0, y: 1.0, z: 1.0 });
                    chaos::lorenz_to_svg(&points)
                }
            }
        }
        Commands::Lsystem { ref system_type, iterations } => {
            let system = match system_type.as_str() {
                "tree" => lsystems::tree(),
                "koch" => lsystems::koch_curve(),
                "sierpinski" => lsystems::sierpinski_arrowhead(),
                "dragon" => lsystems::dragon_curve(),
                _ => lsystems::plant(),
            };
            let s = lsystems::generate(&system, iterations.min(8));
            let segments = lsystems::interpret(&system, &s);
            let md = lsystems::max_depth(&segments);
            lsystems::to_svg(&segments, md)
        }
        Commands::Turing { ref preset, size, steps } => {
            let p = match preset.as_str() {
                "stripes" => turing::Preset::Stripes,
                "coral" => turing::Preset::Coral,
                "mitosis" => turing::Preset::Mitosis,
                "worms" => turing::Preset::Worms,
                _ => turing::Preset::Spots,
            };
            let mut grid = turing::Grid::new_random(size, size, 42);
            grid.simulate(&p.params(), steps);
            turing::grid_to_svg(&grid)
        }
        Commands::Web { ref dir } => {
            println!("Web gallery files are in the '{}' directory.", dir.display());
            println!("Open web/index.html in a browser to explore!");
            return;
        }
    };

    if let Commands::Web { .. } = cli.command {
        return;
    }

    fs::write(&cli.output, &svg).expect("Failed to write output file");
    println!("✨ Generated {} ({} bytes)", cli.output.display(), svg.len());
}
