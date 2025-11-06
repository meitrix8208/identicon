use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use image::ImageFormat;
use md5::{Digest, Md5};

use identicon::{Identicon, IdenticonConfig};

/// Generador de identicons - Convierte texto o archivos en imágenes únicas
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Texto de entrada para generar el identicon
    #[arg(short, long, value_name = "TEXT")]
    input: Option<String>,

    /// Archivo de entrada para leer los datos
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    /// Archivo de salida PNG (si no se especifica, se usa stdout)
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Tamaño de la imagen en píxeles (ancho = alto)
    #[arg(short, long, default_value = "420", value_name = "SIZE")]
    size: u32,

    /// Tamaño de cada píxel del sprite
    #[arg(short, long, default_value = "70", value_name = "PIXEL_SIZE")]
    pixel_size: u32,

    /// Color de fondo en formato hex (ejemplo: ff0000 para rojo)
    #[arg(short, long, value_name = "HEX_COLOR")]
    background: Option<String>,

    /// Mostrar el hash MD5 calculado
    #[arg(long)]
    show_hash: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Obtener los datos de entrada
    let input_data = get_input_data(&args)?;
    
    // Calcular el hash MD5
    let hash = calculate_md5(&input_data);
    
    if args.show_hash {
        eprintln!("MD5: {}", format_hex(&hash));
    }

    // Crear configuración
    let config = create_config(&args)?;
    
    // Generar el identicon
    let identicon = Identicon::with_config(&hash, config);
    let image = identicon.generate();

    // Guardar la imagen
    save_image(&image, args.output.as_deref())?;

    Ok(())
}

fn get_input_data(args: &Args) -> Result<Vec<u8>> {
    match (&args.input, &args.file) {
        (Some(text), None) => Ok(text.as_bytes().to_vec()),
        (None, Some(file)) => {
            fs::read(file).with_context(|| format!("No se pudo leer el archivo: {}", file.display()))
        }
        (None, None) => {
            let mut buffer = Vec::new();
            io::stdin()
                .read_to_end(&mut buffer)
                .context("No se pudo leer desde stdin")?;
            Ok(buffer)
        }
        (Some(_), Some(_)) => {
            anyhow::bail!("No se pueden especificar tanto --input como --file al mismo tiempo")
        }
    }
}

fn calculate_md5(data: &[u8]) -> [u8; 16] {
    let mut hasher = Md5::new();
    hasher.update(data);
    hasher.finalize().into()
}

fn create_config(args: &Args) -> Result<IdenticonConfig> {
    let mut config = IdenticonConfig {
        size: args.size,
        pixel_size: args.pixel_size,
        ..Default::default()
    };

    if let Some(bg_color) = &args.background {
        config.background = parse_hex_color(bg_color)
            .with_context(|| format!("Color de fondo inválido: {}", bg_color))?;
    }

    Ok(config)
}

fn parse_hex_color(hex: &str) -> Result<image::Rgb<u8>> {
    let hex = hex.trim_start_matches('#');
    
    if hex.len() != 6 {
        anyhow::bail!("El color debe tener 6 caracteres hexadecimales");
    }

    let r = u8::from_str_radix(&hex[0..2], 16)
        .context("Valor rojo inválido")?;
    let g = u8::from_str_radix(&hex[2..4], 16)
        .context("Valor verde inválido")?;
    let b = u8::from_str_radix(&hex[4..6], 16)
        .context("Valor azul inválido")?;

    Ok(image::Rgb([r, g, b]))
}

fn format_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn save_image(image: &image::RgbImage, output_path: Option<&std::path::Path>) -> Result<()> {
    match output_path {
        Some(path) => {
            image
                .save_with_format(path, ImageFormat::Png)
                .with_context(|| format!("No se pudo guardar la imagen en: {}", path.display()))?;
            eprintln!("Identicon guardado en: {}", path.display());
        }
        None => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            let mut buffer = Vec::new();
            image
                .write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::Png)
                .context("No se pudo codificar la imagen PNG")?;
            handle
                .write_all(&buffer)
                .context("No se pudo escribir a stdout")?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hex_color_valid() {
        let color = parse_hex_color("ff0000").unwrap();
        assert_eq!(color, image::Rgb([255, 0, 0]));
    }

    #[test]
    fn parse_hex_color_with_hash() {
        let color = parse_hex_color("#00ff00").unwrap();
        assert_eq!(color, image::Rgb([0, 255, 0]));
    }

    #[test]
    fn parse_hex_color_invalid_length() {
        assert!(parse_hex_color("ff00").is_err());
    }

    #[test]
    fn calculate_md5_consistent() {
        let data = b"test";
        let hash1 = calculate_md5(data);
        let hash2 = calculate_md5(data);
        assert_eq!(hash1, hash2);
    }
}
