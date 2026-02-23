//! L-Systems (Lindenmayer Systems) — formal grammars that generate branching structures.
//!
//! Trees, ferns, rivers, lungs, blood vessels, and lightning all share
//! fractal branching patterns that can be described by simple rewriting rules.

use std::f64::consts::PI;

/// A turtle graphics command produced by interpreting an L-system string.
#[derive(Debug, Clone, Copy)]
pub enum TurtleCommand {
    Forward(f64),
    TurnLeft(f64),
    TurnRight(f64),
    Push,
    Pop,
}

/// An L-system rule: character → replacement string.
#[derive(Debug, Clone)]
pub struct Rule {
    pub from: char,
    pub to: String,
}

/// An L-system definition.
#[derive(Debug, Clone)]
pub struct LSystem {
    pub name: String,
    pub axiom: String,
    pub rules: Vec<Rule>,
    pub angle: f64,
    pub step_length: f64,
    pub length_factor: f64,
}

/// A line segment produced by turtle interpretation.
#[derive(Debug, Clone, Copy)]
pub struct Segment {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub depth: usize,
}

/// Predefined L-systems.
pub fn tree() -> LSystem {
    LSystem {
        name: "Fractal Tree".to_string(),
        axiom: "0".to_string(),
        rules: vec![
            Rule { from: '1', to: "11".to_string() },
            Rule { from: '0', to: "1[0]0".to_string() },
        ],
        angle: 45.0,
        step_length: 8.0,
        length_factor: 0.7,
    }
}

pub fn koch_curve() -> LSystem {
    LSystem {
        name: "Koch Curve".to_string(),
        axiom: "F".to_string(),
        rules: vec![
            Rule { from: 'F', to: "F+F-F-F+F".to_string() },
        ],
        angle: 90.0,
        step_length: 4.0,
        length_factor: 1.0,
    }
}

pub fn sierpinski_arrowhead() -> LSystem {
    LSystem {
        name: "Sierpinski Arrowhead".to_string(),
        axiom: "A".to_string(),
        rules: vec![
            Rule { from: 'A', to: "B-A-B".to_string() },
            Rule { from: 'B', to: "A+B+A".to_string() },
        ],
        angle: 60.0,
        step_length: 4.0,
        length_factor: 1.0,
    }
}

pub fn dragon_curve() -> LSystem {
    LSystem {
        name: "Dragon Curve".to_string(),
        axiom: "FX".to_string(),
        rules: vec![
            Rule { from: 'X', to: "X+YF+".to_string() },
            Rule { from: 'Y', to: "-FX-Y".to_string() },
        ],
        angle: 90.0,
        step_length: 5.0,
        length_factor: 1.0,
    }
}

pub fn plant() -> LSystem {
    LSystem {
        name: "Plant".to_string(),
        axiom: "X".to_string(),
        rules: vec![
            Rule { from: 'X', to: "F+[[X]-X]-F[-FX]+X".to_string() },
            Rule { from: 'F', to: "FF".to_string() },
        ],
        angle: 25.0,
        step_length: 4.0,
        length_factor: 0.5,
    }
}

/// Apply L-system rules for n iterations.
pub fn generate(system: &LSystem, iterations: usize) -> String {
    let mut current = system.axiom.clone();
    for _ in 0..iterations {
        let mut next = String::with_capacity(current.len() * 2);
        for ch in current.chars() {
            let mut matched = false;
            for rule in &system.rules {
                if ch == rule.from {
                    next.push_str(&rule.to);
                    matched = true;
                    break;
                }
            }
            if !matched {
                next.push(ch);
            }
        }
        current = next;
    }
    current
}

/// Interpret an L-system string using turtle graphics.
pub fn interpret(system: &LSystem, lstring: &str) -> Vec<Segment> {
    let mut segments = Vec::new();
    let mut x = 0.0_f64;
    let mut y = 0.0_f64;
    let mut angle = -PI / 2.0; // Start pointing up
    let step = system.step_length;
    let turn = system.angle.to_radians();
    let mut stack: Vec<(f64, f64, f64, usize)> = Vec::new();
    let mut depth: usize = 0;

    for ch in lstring.chars() {
        match ch {
            'F' | '0' | '1' | 'A' | 'B' => {
                let nx = x + step * angle.cos();
                let ny = y + step * angle.sin();
                segments.push(Segment { x1: x, y1: y, x2: nx, y2: ny, depth });
                x = nx;
                y = ny;
            }
            '+' => angle += turn,
            '-' => angle -= turn,
            '[' => {
                stack.push((x, y, angle, depth));
                depth += 1;
            }
            ']' => {
                if let Some((px, py, pa, pd)) = stack.pop() {
                    x = px;
                    y = py;
                    angle = pa;
                    depth = pd;
                }
            }
            _ => {} // Skip non-drawing characters (X, Y, etc.)
        }
    }
    segments
}

/// Calculate total length of all segments.
pub fn total_length(segments: &[Segment]) -> f64 {
    segments.iter().map(|s| {
        ((s.x2 - s.x1).powi(2) + (s.y2 - s.y1).powi(2)).sqrt()
    }).sum()
}

/// Count branching points (where depth increases).
pub fn count_branches(segments: &[Segment]) -> usize {
    if segments.is_empty() { return 0; }
    segments.windows(2).filter(|w| w[1].depth > w[0].depth).count()
}

/// Maximum depth of branching.
pub fn max_depth(segments: &[Segment]) -> usize {
    segments.iter().map(|s| s.depth).max().unwrap_or(0)
}

/// Generate SVG of L-system segments.
pub fn to_svg(segments: &[Segment], max_depth_val: usize) -> String {
    if segments.is_empty() {
        return String::from(r##"<svg xmlns="http://www.w3.org/2000/svg" width="800" height="800"></svg>"##);
    }
    let min_x = segments.iter().map(|s| s.x1.min(s.x2)).fold(f64::INFINITY, f64::min);
    let max_x = segments.iter().map(|s| s.x1.max(s.x2)).fold(f64::NEG_INFINITY, f64::max);
    let min_y = segments.iter().map(|s| s.y1.min(s.y2)).fold(f64::INFINITY, f64::min);
    let max_y = segments.iter().map(|s| s.y1.max(s.y2)).fold(f64::NEG_INFINITY, f64::max);

    let margin = 40.0;
    let data_w = (max_x - min_x).max(1.0);
    let data_h = (max_y - min_y).max(1.0);
    let scale = (720.0 / data_w).min(720.0 / data_h);
    let w = (data_w * scale + margin * 2.0) as u32;
    let h = (data_h * scale + margin * 2.0) as u32;

    let mut svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}">
<rect width="{w}" height="{h}" fill="#0a0a1a"/>
"##
    );

    let md = max_depth_val.max(1) as f64;
    for s in segments {
        let x1 = margin + (s.x1 - min_x) * scale;
        let y1 = margin + (s.y1 - min_y) * scale;
        let x2 = margin + (s.x2 - min_x) * scale;
        let y2 = margin + (s.y2 - min_y) * scale;
        let t = s.depth as f64 / md;
        let hue = 90.0 + t * 40.0;
        let width = 3.0 - t * 2.5;
        svg.push_str(&format!(
            r##"<line x1="{x1:.1}" y1="{y1:.1}" x2="{x2:.1}" y2="{y2:.1}" stroke="hsl({hue:.0},60%,40%)" stroke-width="{width:.1}" stroke-linecap="round"/>
"##
        ));
    }
    svg.push_str("</svg>");
    svg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_tree() {
        let sys = tree();
        let s0 = generate(&sys, 0);
        assert_eq!(s0, "0");
        let s1 = generate(&sys, 1);
        assert_eq!(s1, "1[0]0");
        let s2 = generate(&sys, 2);
        assert_eq!(s2, "11[1[0]0]1[0]0");
    }

    #[test]
    fn test_generate_koch() {
        let sys = koch_curve();
        let s1 = generate(&sys, 1);
        assert_eq!(s1, "F+F-F-F+F");
    }

    #[test]
    fn test_string_growth() {
        let sys = plant();
        let lengths: Vec<usize> = (0..5).map(|i| generate(&sys, i).len()).collect();
        for w in lengths.windows(2) {
            assert!(w[1] > w[0], "String should grow each iteration");
        }
    }

    #[test]
    fn test_interpret_produces_segments() {
        let sys = tree();
        let s = generate(&sys, 3);
        let segments = interpret(&sys, &s);
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_interpret_plant() {
        let sys = plant();
        let s = generate(&sys, 3);
        let segments = interpret(&sys, &s);
        assert!(segments.len() > 10);
    }

    #[test]
    fn test_total_length() {
        let sys = tree();
        let s = generate(&sys, 3);
        let segments = interpret(&sys, &s);
        assert!(total_length(&segments) > 0.0);
    }

    #[test]
    fn test_branching_depth() {
        let sys = tree();
        let s = generate(&sys, 4);
        let segments = interpret(&sys, &s);
        assert!(max_depth(&segments) >= 2);
    }

    #[test]
    fn test_count_branches() {
        let sys = tree();
        let s = generate(&sys, 3);
        let segments = interpret(&sys, &s);
        assert!(count_branches(&segments) > 0);
    }

    #[test]
    fn test_svg_output() {
        let sys = tree();
        let s = generate(&sys, 3);
        let segments = interpret(&sys, &s);
        let md = max_depth(&segments);
        let svg = to_svg(&segments, md);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("<line"));
    }

    #[test]
    fn test_dragon_curve() {
        let sys = dragon_curve();
        let s = generate(&sys, 5);
        let segments = interpret(&sys, &s);
        assert!(!segments.is_empty());
    }
}
