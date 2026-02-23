//! Shared SVG rendering utilities.

/// Wrap content in an SVG document.
pub fn svg_document(width: u32, height: u32, content: &str) -> String {
    format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">
<rect width="{width}" height="{height}" fill="#0a0a1a"/>
{content}
</svg>"##
    )
}

/// Generate an HSL color string.
pub fn hsl(h: f64, s: f64, l: f64) -> String {
    format!("hsl({:.0},{:.0}%,{:.0}%)", h % 360.0, s.clamp(0.0, 100.0), l.clamp(0.0, 100.0))
}

/// Map a value 0..1 to a viridis-like color.
pub fn viridis(t: f64) -> String {
    let t = t.clamp(0.0, 1.0);
    let r = (68.0 + t * 187.0).min(255.0) as u8;
    let g = (1.0 + t * 180.0 + (1.0 - t) * 40.0).min(255.0) as u8;
    let b = (84.0 + (1.0 - t) * 140.0 + t * 20.0).min(255.0) as u8;
    format!("rgb({r},{g},{b})")
}

/// Map a value 0..1 to a magma-like color.
pub fn magma(t: f64) -> String {
    let t = t.clamp(0.0, 1.0);
    let r = (t * 255.0).min(255.0) as u8;
    let g = (t * t * 180.0).min(255.0) as u8;
    let b = (80.0 + t * 100.0).min(255.0) as u8;
    format!("rgb({r},{g},{b})")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_document() {
        let svg = svg_document(800, 600, "<circle cx='400' cy='300' r='50'/>");
        assert!(svg.contains("width=\"800\""));
        assert!(svg.contains("height=\"600\""));
        assert!(svg.contains("<circle"));
    }

    #[test]
    fn test_hsl() {
        assert_eq!(hsl(120.0, 50.0, 50.0), "hsl(120,50%,50%)");
    }

    #[test]
    fn test_viridis_bounds() {
        let c0 = viridis(0.0);
        let c1 = viridis(1.0);
        assert!(c0.starts_with("rgb("));
        assert!(c1.starts_with("rgb("));
    }

    #[test]
    fn test_magma_bounds() {
        let c = magma(0.5);
        assert!(c.starts_with("rgb("));
    }

    #[test]
    fn test_viridis_clamping() {
        let _ = viridis(-1.0);
        let _ = viridis(2.0);
        // Should not panic
    }
}
