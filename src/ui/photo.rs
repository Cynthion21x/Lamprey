use sdl2::render::Texture;
use crate::rendering::renderer::Renderer;

pub struct Photo<'a> {
    pub pos: (u32, u32),
    pub size: (u32, u32),
    texture: &'a Texture<'a>,
    pub visible: bool,
}

impl<'a> Photo<'a> {
    pub fn new(pos: (u32, u32), size: (u32, u32), texture: &'a Texture<'a>) -> Self {
        
        Self { 
            pos,
            size,
            texture,
            visible: true
        }
    }
    
    pub fn draw(&self, renderer: &mut Renderer) {
        
        if !self.visible { return () }
        
        renderer.draw_gui(self.pos, self.size, self.texture);
        
    }
}
