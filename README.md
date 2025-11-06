# Identicon

Un port moderno del algoritmo de [identicon](https://en.wikipedia.org/wiki/Identicon) de GitHub a Rust.

![ejemplo identicon](https://cloud.githubusercontent.com/assets/122102/5274078/62b57c18-7a4d-11e4-90fa-46edd2ff7084.png)

## Características

- ✨ Generación eficiente de identicons únicos
- 🎨 Colores personalizables y configuración flexible
- 📁 Soporte para entrada desde archivos, texto o stdin
- 🖼️ Salida PNG de alta calidad
- 🚀 CLI moderna con opciones avanzadas

## Instalación

```bash
cargo install identicon
```

## Uso

### Ejemplos básicos

```bash
# Generar desde texto
identicon -i "usuario123" -o avatar.png

# Generar desde archivo
identicon -f datos.txt -o identicon.png

# Generar desde stdin (compatible con versiones anteriores)
echo -n "480938" | identicon > octocat.png
```

### Opciones avanzadas

```bash
# Personalizar tamaño y colores
identicon -i "test" -s 600 -p 100 -b "f0f0f0" -o grande.png

# Mostrar hash MD5 calculado
identicon -i "test" --show-hash -o test.png
```

### Opciones de la CLI

- `-i, --input <TEXT>`: Texto de entrada para generar el identicon
- `-f, --file <FILE>`: Archivo de entrada para leer los datos
- `-o, --output <FILE>`: Archivo de salida PNG (stdout si no se especifica)
- `-s, --size <SIZE>`: Tamaño de la imagen en píxeles [default: 420]
- `-p, --pixel-size <PIXEL_SIZE>`: Tamaño de cada píxel del sprite [default: 70]
- `-b, --background <HEX_COLOR>`: Color de fondo en formato hex (ej: ff0000)
- `--show-hash`: Mostrar el hash MD5 calculado

## Como librería

```rust
use identicon::{Identicon, IdenticonConfig};
use image::Rgb;

// Configuración básica
let hash = b"datos del usuario";
let identicon = Identicon::new(hash);
let imagen = identicon.generate();
imagen.save("avatar.png").unwrap();

// Configuración personalizada
let config = IdenticonConfig {
    size: 600,
    pixel_size: 100,
    background: Rgb([240, 240, 240]),
};

let identicon = Identicon::with_config(hash, config);
let imagen = identicon.generate();
```

## Desarrollo

```bash
# Ejecutar tests
cargo test

# Construir el proyecto
cargo build --release
```

## Algoritmo

El identicon se genera usando el siguiente proceso:

1. **Hash MD5**: Los datos de entrada se procesan con MD5 para obtener un hash de 128 bits
2. **Colores**: Los últimos 4 bytes del hash determinan el color usando HSL
3. **Patrón**: Los primeros bytes generan un patrón simétrico de 5x5 píxeles
4. **Renderizado**: El patrón se dibuja con márgenes y escalado configurable

## Compatibilidad

- **Rust Edition**: 2024
- **MSRV**: 1.70+
- **Dependencias actualizadas**: image 0.25, clap 4.5, md-5 0.10

## Cambios desde v0.2.1

- ✅ Actualizado a Rust Edition 2024
- ✅ CLI moderna con `clap` 4.5 y manejo de errores con `anyhow`
- ✅ API mejorada con configuración flexible
- ✅ Mejor documentación y tests
- ✅ Soporte para colores personalizados
- ✅ Entrada desde archivos además de stdin

## Licencia

Identicon se distribuye bajo la licencia MIT. Consulta el archivo LICENSE para más detalles.
