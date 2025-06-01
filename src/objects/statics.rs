use sdl2::render::Texture;

use crate::{
    config::TILE_SIZE,
    renderer::Renderer
};

pub struct Static<'a> {
        pub pos: (f32, f32),
        pub size: (u32, u32),
        pub sprite: &'a Texture<'a>,
}


impl<'a> Static<'a> {
    
    pub fn new(pos: (f32, f32), size: (u32, u32), sprite: &'a Texture<'a>) -> Self {
        Self{
            pos,
            size,
            sprite,
        }
    }
    
    pub fn draw(&self, renderer: &mut Renderer) {
        
        let size = (self.size.0 * TILE_SIZE, self.size.1 * TILE_SIZE);
        
        renderer.draw(self.pos, size, self.sprite);
        
    }
}