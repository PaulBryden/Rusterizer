
/// A 'Framebuffer' object
pub struct Framebuffer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl Framebuffer {
    /// Creates a new 'Framebuffer' object with a width and height.
    pub fn new(width: usize, height: usize) -> Self {
        let buffer: Vec<u32> = vec![0; width * height];

        Framebuffer {
            buffer,
            width,
            height,
        }
    }

    /// Returns the color at a particular x and y coord in the framebuffer.
    pub fn get_color_at(&self, x_coord: &u32, y_coord: &u32) -> &u32 {
        &self.buffer[(x_coord + (y_coord * self.width as u32)) as usize]
    }

    /// Sets the color at a particular x and y coord in the framebuffer.
    pub fn set_color_at(&mut self, x_coord: &u32, y_coord: &u32, color: u32) {
        if (*y_coord < self.height as u32 && *x_coord < self.width as u32)
            && (x_coord + (y_coord * self.width as u32)) < (self.buffer.len() as u32)
        {
            self.buffer[(x_coord + (y_coord * self.width as u32)) as usize] = color;
        }
    }

    /// Returns a reference to the internal framebuffer
    pub fn get_framebuffer(&self) -> &Vec<u32> {
        &self.buffer
    }

    /// Sets the entire framebuffer to 0.
    pub fn clear_buffer(&mut self) {
        for i in self.buffer.iter_mut() {
            *i = 0;
        }
    }
    
    /// Sets the entire framebuffer to 'color'.
    pub fn clear_buffer_color(&mut self, color: &u32) {
        for i in self.buffer.iter_mut() {
            *i = *color;
        }
    }
}
