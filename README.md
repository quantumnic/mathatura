# ✦ Mathatura

**The beauty of mathematics in nature — interactive simulations, visualizations, and deep explanations.**

> *"Mathematics is the language with which God has written the universe."* — Galileo Galilei

Mathatura is a collection of interactive mathematical simulations that reveal the hidden patterns governing the natural world. From the spiral of a nautilus shell to the spots on a leopard, from the chaos of weather to the fractal geometry of ferns — nature speaks mathematics.

## 🌐 Web Gallery

**Open `web/index.html` in any browser** for the full interactive experience:

- 🎨 12 interactive simulations with real-time Canvas rendering
- 🎛️ Sliders and controls to explore parameter spaces
- 📐 Mathematical formulas and explanations for each topic
- 🏷️ Difficulty levels: 🟢 Kids · 🟡 Teens · 🔴 University
- 🔍 Search and filter categories

## 📚 Categories

### 🌻 Phyllotaxis — *How sunflowers count*
Plants arrange leaves, seeds, and petals using the **golden angle** ≈ 137.508°.

```
θₙ = n × 137.508°
rₙ = c × √n

Golden angle = 360° / φ² where φ = (1+√5)/2 ≈ 1.618
```

The golden angle is the most irrational angle — it avoids creating lines or gaps, producing optimal packing. The visible spirals (parastichies) always come in consecutive Fibonacci numbers: 21/34, 55/89.

**In nature:** Sunflower heads, pine cones, pineapple scales, romanesco broccoli, succulent rosettes.

### 🌿 Fractals — *Self-similarity at every scale*
The **Barnsley fern** emerges from four affine transformations chosen randomly:

```
f₁(x,y) = (0, 0.16y)                          p = 1%   [stem]
f₂(x,y) = (0.85x+0.04y, -0.04x+0.85y+1.6)    p = 85%  [main leaflet]
f₃(x,y) = (0.2x-0.26y, 0.23x+0.22y+1.6)      p = 7%   [left]
f₄(x,y) = (-0.15x+0.28y, 0.26x+0.24y+0.44)   p = 7%   [right]
```

The **Koch snowflake** has infinite perimeter but finite area, with fractal dimension ln(4)/ln(3) ≈ 1.262.

The **Mandelbrot set** — iterate z → z² + c — contains infinite complexity at every scale.

**In nature:** Ferns, coastlines, lightning, blood vessels, romanesco, snowflakes.

### 🐚 Spirals — *The universe's favorite curve*

```
Logarithmic:  r = a × e^(bθ)     nautilus, galaxies, hurricanes
Archimedean:  r = a + bθ          watch springs, coiled rope
Fermat:       r = a × √θ          sunflower background curve
Golden:       r = a × φ^(2θ/π)    special logarithmic spiral
```

The golden spiral grows by φ every quarter turn — approximating the curves of nautilus shells, galaxy arms, and hurricane formations.

### 🦋 Chaos Theory — *Determinism without predictability*

The **Lorenz attractor** — a simplified weather model that revealed chaos:

```
dx/dt = σ(y - x)
dy/dt = x(ρ - z) - y
dz/dt = xy - βz

Classic: σ=10, ρ=28, β=8/3
```

Two points starting 10⁻¹⁰ apart diverge completely. The trajectory never repeats, yet stays confined to a strange attractor. The **logistic map** x → rx(1-x) shows how a single parameter drives the route from order to chaos through period-doubling cascades.

**Feigenbaum's constant** δ ≈ 4.6692 — universal across all period-doubling systems.

### 🌳 L-Systems — *Growing structures from grammars*

Lindenmayer systems use simple string rewriting rules + turtle graphics:

```
Plant axiom: X
Rules: X → F+[[X]-X]-F[-FX]+X
       F → FF

F = draw forward, + = turn left, - = turn right
[ = save state, ] = restore state
```

The same branching principle appears in trees, ferns, blood vessels, lungs, rivers, and lightning — nature reuses fractal branching because it optimizes distribution networks.

### 🐆 Turing Patterns — *How leopards get their spots*

Alan Turing's 1952 reaction-diffusion model:

```
∂A/∂t = Dₐ∇²A - AB² + f(1-A)
∂B/∂t = D_b∇²B + AB² - (k+f)B
```

Two chemicals diffusing at different rates spontaneously create patterns. Tuning feed rate (f) and kill rate (k) produces spots, stripes, labyrinths, and traveling waves — explaining leopard spots, zebra stripes, and seashell pigmentation.

### 🍯 Tessellations & Symmetry
Hexagons tile the plane with minimum perimeter per unit area (Honeycomb Conjecture, proven 1999). Voronoi diagrams appear in giraffe skin, dragonfly wings, and cracked mud.

### 🔢 Fibonacci Spiral
Golden rectangles with Fibonacci side lengths, connected by quarter-circle arcs. The ratio F(n)/F(n-1) converges to φ at the slowest possible rate — making it the "most irrational" number.

## 🖥️ CLI Usage

Generate SVG visualizations from the command line:

```bash
# Phyllotaxis patterns
cargo run -- phyllotaxis -n 1000 --angle 137.508 -o sunflower.svg
cargo run -- phyllotaxis --pattern rosette -n 300 -o rosette.svg

# Fractals
cargo run -- fractals -t fern --iterations 100000 -o fern.svg
cargo run -- fractals -t koch --iterations 5 -o koch.svg

# Spirals
cargo run -- spirals -t golden --turns 8 -o golden-spiral.svg
cargo run -- spirals -t logarithmic -o log-spiral.svg

# Chaos
cargo run -- chaos -t lorenz -n 30000 -o lorenz.svg

# L-Systems
cargo run -- lsystem -t plant --iterations 6 -o plant.svg
cargo run -- lsystem -t dragon --iterations 10 -o dragon.svg

# Turing patterns (takes a moment to simulate)
cargo run -- turing --preset spots -s 100 -n 8000 -o spots.svg
cargo run -- turing --preset stripes -o stripes.svg
```

## 🧪 Testing

```bash
cargo test        # Run all 65 tests
cargo test -- --nocapture   # See output
```

The test suite covers:
- Core mathematical constants (φ, golden angle)
- Fibonacci properties and convergence
- Fractal generation and bounds checking
- Chaos theory (Lyapunov exponents, butterfly effect)
- L-system string generation and interpretation
- Turing pattern simulation stability
- SVG output validity
- Numerical accuracy and determinism

## 📁 Project Structure

```
mathatura/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs              # CLI entry point (clap)
│   ├── lib.rs               # Library root + constants
│   ├── render.rs            # Shared SVG utilities
│   └── categories/
│       ├── mod.rs
│       ├── phyllotaxis.rs   # Golden angle, Vogel's model
│       ├── fractals.rs      # Barnsley fern, Koch, Mandelbrot
│       ├── spirals.rs       # Log, Archimedean, Fermat, Golden
│       ├── chaos.rs         # Lorenz attractor, logistic map
│       ├── lsystems.rs      # Lindenmayer systems
│       └── turing.rs        # Gray-Scott reaction-diffusion
└── web/
    └── index.html           # Interactive gallery (single-file)
```

## 🔮 Roadmap

Future categories planned:
- **Optimization** — Boids flocking, ant colony paths, fish schooling
- **Packing** — Foam, sphere packing, pomegranate seeds
- **Scaling Laws** — Allometry, Kleiber's law, metabolic scaling
- **Topology** — Möbius strips in nature, DNA knots
- **Minimal Surfaces** — Soap bubbles, why hexagons minimize surface area

## 🌿 Philosophy

Mathematics isn't something humans invented — it's something we discovered. The same equations govern the spiral of a galaxy and the curl of a fern frond. The Fibonacci sequence appears in flower petals not because flowers "know" math, but because mathematics describes the deepest patterns of growth and optimization.

Mathatura exists to make these connections visible, interactive, and beautiful.

## License

MIT

---

*Built with 🦀 Rust + vanilla JS/Canvas. No frameworks, no dependencies beyond `clap`.*
