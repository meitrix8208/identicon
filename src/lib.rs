use image::{ImageBuffer, Rgb, RgbImage};

mod hsl;
mod nibbler;

use hsl::Hsl;
use nibbler::Nibbler;

/// Configuración para generar identicons
#[derive(Debug, Clone)]
pub struct IdenticonConfig {
    /// Tamaño de la imagen en píxeles (ancho y alto)
    pub size: u32,
    /// Tamaño de cada píxel del sprite
    pub pixel_size: u32,
    /// Color de fondo
    pub background: Rgb<u8>,
}

impl Default for IdenticonConfig {
    fn default() -> Self {
        Self {
            size: 420,
            pixel_size: 70,
            background: Rgb([240, 240, 240]),
        }
    }
}

/// Generador de identicons basado en el algoritmo de GitHub
pub struct Identicon<'a> {
    hash: &'a [u8],
    config: IdenticonConfig,
}

impl<'a> Identicon<'a> {
    /// Crea un nuevo generador de identicons con la configuración por defecto
    pub fn new(hash: &'a [u8]) -> Self {
        Self {
            hash,
            config: IdenticonConfig::default(),
        }
    }

    /// Crea un nuevo generador de identicons con configuración personalizada
    pub fn with_config(hash: &'a [u8], config: IdenticonConfig) -> Self {
        Self { hash, config }
    }

    /// Mapea un valor de un rango a otro
    fn map_value(value: u32, from_min: u32, from_max: u32, to_min: u32, to_max: u32) -> f32 {
        let normalized = (value - from_min) as f32 / (from_max - from_min) as f32;
        to_min as f32 + normalized * (to_max - to_min) as f32
    }

    /// Calcula el color de primer plano basado en el hash
    fn calculate_foreground_color(&self) -> Rgb<u8> {
        // Usar los últimos 4 bytes del hash para determinar los valores HSL
        let hash_len = self.hash.len();
        if hash_len < 4 {
            return Rgb([100, 100, 100]); // Color por defecto si el hash es muy corto
        }

        let h_bytes = &self.hash[hash_len - 4..hash_len - 2];
        let s_byte = self.hash[hash_len - 2];
        let l_byte = self.hash[hash_len - 1];

        let hue_value = u16::from_be_bytes([h_bytes[0], h_bytes[1]]);
        let hue = Self::map_value(hue_value as u32, 0, u16::MAX as u32, 0, 360);
        let sat_offset = Self::map_value(s_byte as u32, 0, 255, 0, 20);
        let lum_offset = Self::map_value(l_byte as u32, 0, 255, 0, 20);

        Hsl::new(hue, 65.0 - sat_offset, 75.0 - lum_offset).to_rgb()
    }

    /// Dibuja un rectángulo relleno en la imagen
    fn draw_rectangle(
        image: &mut RgbImage,
        x0: u32,
        y0: u32,
        x1: u32,
        y1: u32,
        color: Rgb<u8>,
    ) {
        for x in x0..x1 {
            for y in y0..y1 {
                if x < image.width() && y < image.height() {
                    image.put_pixel(x, y, color);
                }
            }
        }
    }

    /// Genera el patrón de píxeles del identicon
    fn generate_pixel_pattern(&self) -> [bool; 25] {
        let mut nibbles = Nibbler::new(self.hash).map(|x| x % 2 == 0);
        let mut pixels = [false; 25];

        // Generar la mitad izquierda y reflejarla
        for col in 0..3 {
            for row in 0..5 {
                let left_idx = col + (row * 5);
                let right_idx = (4 - col) + (row * 5);
                let should_paint = nibbles.next().unwrap_or(false);
                
                pixels[left_idx] = should_paint;
                pixels[right_idx] = should_paint;
            }
        }

        pixels
    }

    /// Genera la imagen del identicon
    pub fn generate(&self) -> RgbImage {
        const SPRITE_SIZE: usize = 5;
        let margin = self.config.pixel_size / 2;
        let foreground = self.calculate_foreground_color();

        let mut image = ImageBuffer::from_pixel(
            self.config.size,
            self.config.size,
            self.config.background,
        );

        let pixels = self.generate_pixel_pattern();

        for (row, pixel_row) in pixels.chunks(SPRITE_SIZE).enumerate() {
            for (col, &should_paint) in pixel_row.iter().enumerate() {
                if should_paint {
                    let x = col as u32 * self.config.pixel_size + margin;
                    let y = row as u32 * self.config.pixel_size + margin;

                    Self::draw_rectangle(
                        &mut image,
                        x,
                        y,
                        x + self.config.pixel_size,
                        y + self.config.pixel_size,
                        foreground,
                    );
                }
            }
        }

        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_consistent_image() {
        let hash = b"test hash data for identicon generation";
        let identicon = Identicon::new(hash);
        let image1 = identicon.generate();
        let image2 = identicon.generate();
        assert_eq!(image1.dimensions(), image2.dimensions());
        assert_eq!(image1.as_raw(), image2.as_raw());
    }

    #[test]
    fn different_hashes_produce_different_images() {
        let hash1 = b"hash1";
        let hash2 = b"hash2";
        
        let image1 = Identicon::new(hash1).generate();
        let image2 = Identicon::new(hash2).generate();
        
        assert_ne!(image1.as_raw(), image2.as_raw());
    }

    #[test]
    fn respects_custom_config() {
        let hash = b"test";
        let config = IdenticonConfig {
            size: 200,
            pixel_size: 40,
            background: Rgb([255, 0, 0]),
        };
        
        let identicon = Identicon::with_config(hash, config);
        let image = identicon.generate();
        
        assert_eq!(image.dimensions(), (200, 200));
    }
}
