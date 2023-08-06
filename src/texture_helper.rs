use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use tinybmp::Bmp;

use crate::texture::Texture;

/// Creates a new Texture object from a BMP file.
pub fn get_texture_from_bmp(bmp_bytes: &[u8]) -> Texture {
    let mut texture: Texture;
    let bmp = Bmp::<Rgb888>::from_slice(bmp_bytes).unwrap();
    texture = Texture {
        width: 0,
        height: 0,
        pixels: Vec::new(),
    };
    let mut max_x = 0;
    let mut max_y = 0;
    for Pixel(position, color) in bmp.pixels() {
        let (r, g, b) = (color.r() as u32, color.g() as u32, color.b() as u32);
        #[cfg(feature = "web")]
        {
            texture.pixels.push((0xff) << 24 | (b << 16) | (g << 8) | r); //ABGR little endian going from WASM to JS
        }
        #[cfg(not(feature = "web"))]
        {
            texture.pixels.push((0xff) << 24 | (r << 16) | (g << 8) | b); //ARGB
        }
        max_x = max_x.max(position.x);
        max_y = max_y.max(position.y);
    }
    texture.height = (max_y + 1) as u32;
    texture.width = (max_x + 1) as u32;
    texture
}
