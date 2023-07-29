
pub struct Framebuffer
{
    buffer: Vec<u32>,
    width: u64,
    height: u64
}

impl Framebuffer
{
    pub fn new(buffer: Vec<u32>, width: u64, height: u64) -> Self
    {
        Framebuffer{buffer, width, height}
    }

    pub fn get_color_at(&self, x_coord: &u64, y_coord: &u64) -> &u32
    {
        &self.buffer[((x_coord+(y_coord*self.width) ) as usize)]
    }
    
    pub fn set_color_at(&mut self, x_coord: &u64, y_coord: &u64, color: u32)
    {
        if(*y_coord<self.height && *x_coord<self.width) && ((x_coord+(y_coord*self.width) )) < (self.buffer.len() as u64)
        {
            self.buffer[(x_coord+(y_coord*self.width) ) as usize] = color;
        }
    }
    
    pub fn get_framebuffer(&self) -> &Vec<u32>
    {
        &self.buffer
    }
    pub fn clear_buffer(&mut self)
    {
        for i in self.buffer.iter_mut()
        {
            *i=0;
        }
    }
    pub fn clear_buffer_color(&mut self, color: &u32)
    {
        for i in self.buffer.iter_mut()
        {
            *i=*color;
        }
    }
}