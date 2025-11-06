use image::Rgb;

/// Representa un color en formato HSL (Hue, Saturation, Lightness)
#[derive(Debug, Clone, Copy)]
pub struct Hsl {
    /// Matiz en grados (0-360)
    pub hue: f32,
    /// Saturación en porcentaje (0-100)
    pub saturation: f32,
    /// Luminosidad en porcentaje (0-100)
    pub lightness: f32,
}

impl Hsl {
    /// Crea un nuevo color HSL
    pub const fn new(hue: f32, saturation: f32, lightness: f32) -> Self {
        Self {
            hue,
            saturation,
            lightness,
        }
    }

    /// Convierte HSL a RGB según la especificación W3C CSS3
    /// http://www.w3.org/TR/css3-color/#hsl-color
    pub fn to_rgb(&self) -> Rgb<u8> {
        let hue = self.hue / 360.0;
        let sat = self.saturation / 100.0;
        let lum = self.lightness / 100.0;

        let b = if lum <= 0.5 {
            lum * (sat + 1.0)
        } else {
            lum + sat - lum * sat
        };
        let a = lum * 2.0 - b;

        let r = Self::hue_to_rgb(a, b, hue + 1.0 / 3.0);
        let g = Self::hue_to_rgb(a, b, hue);
        let b = Self::hue_to_rgb(a, b, hue - 1.0 / 3.0);

        Rgb([
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
        ])
    }

    fn hue_to_rgb(a: f32, b: f32, hue: f32) -> f32 {
        let h = match hue {
            h if h < 0.0 => h + 1.0,
            h if h > 1.0 => h - 1.0,
            h => h,
        };

        match h {
            h if h < 1.0 / 6.0 => a + (b - a) * 6.0 * h,
            h if h < 1.0 / 2.0 => b,
            h if h < 2.0 / 3.0 => a + (b - a) * (2.0 / 3.0 - h) * 6.0,
            _ => a,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Hsl;
    use image::Rgb;

    #[test]
    fn converts_black() {
        let expected = Rgb([0, 0, 0]);
        let actual = Hsl::new(0.0, 0.0, 0.0).to_rgb();
        assert_eq!(expected, actual);
    }

    #[test]
    fn converts_white() {
        let expected = Rgb([255, 255, 255]);
        let actual = Hsl::new(0.0, 0.0, 100.0).to_rgb();
        assert_eq!(expected, actual);
    }

    #[test]
    fn converts_red() {
        let expected = Rgb([255, 0, 0]);
        let actual = Hsl::new(0.0, 100.0, 50.0).to_rgb();
        assert_eq!(expected, actual);
    }

    #[test]
    fn converts_green() {
        let expected = Rgb([0, 255, 0]);
        let actual = Hsl::new(120.0, 100.0, 50.0).to_rgb();
        assert_eq!(expected, actual);
    }

    #[test]
    fn converts_blue() {
        let expected = Rgb([0, 0, 255]);
        let actual = Hsl::new(240.0, 100.0, 50.0).to_rgb();
        assert_eq!(expected, actual);
    }
}
