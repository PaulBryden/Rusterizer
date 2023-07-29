use embedded_graphics::{image::Image, prelude::*};
use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use tinybmp::Bmp;

#[derive(Default, Debug, Clone)]
pub struct Texture
{
  pub pixels: Vec<u32>,
  pub width: u32,
  pub height: u32
}

impl Texture
{
  pub fn get_color_at_normalized_coord(&self, x_coord: &f32, y_coord: &f32) -> &u32
  {
      let texture_coord_x = (x_coord*((self.width-1)as f32)).round() as u32;
      let texture_coord_y = (self.height-1)-(y_coord*((self.height-1)as f32)).round() as u32;
      &self.pixels[((texture_coord_x+(texture_coord_y*(self.width))) as usize)]
  }
}

pub fn get_texture_from_bmp(bmp_bytes: &[u8]) -> Texture
{    let mut texture: Texture;
     let bmp = Bmp::<Rgb888>::from_slice(bmp_bytes).unwrap();
     texture = Texture{width: 0, height: 0, pixels: Vec::new()};
     let mut max_x = 0;
     let mut max_y = 0;
     for Pixel(position, color) in bmp.pixels() {
        let (r, g, b) = (color.r() as u32, color.g() as u32, color.b() as u32);
        texture.pixels.push((r << 16) | (g << 8) | b);
        max_x=max_x.max(position.x);
        max_y=max_y.max(position.y);
    }
    texture.height=(max_y+1 )as u32;
    texture.width= (max_x+1) as u32;
    texture
}