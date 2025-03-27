use sdl2::{
    render::Texture,
    keyboard::Keycode
};
use crate::{
    config::TILE_SIZE, game::Inputs, renderer::Renderer
};

pub struct Player<'a> {
    pos: (u32, u32),
    size: (u32, u32),
    sprite: &'a Texture<'a>,
    visible: bool,
}

impl<'a> Player<'a> {
    
    pub fn new(pos: (u32, u32), size: (u32, u32), sprite: &'a Texture<'a>) -> Self {
        Self{
            pos,
            size,
            sprite,
            visible: true
        }
    }
    
    pub fn draw(&self, renderer: &mut Renderer) {
        
        if !self.visible { return () }
        
        let size = (self.size.0 * TILE_SIZE, self.size.1 * TILE_SIZE);
        
        renderer.draw(self.pos, size, self.sprite);
        
    }
    
    pub fn movement(&mut self, input: &Inputs) {
        
        match input.key_pressed {
            
            Some(Keycode::A) => {
                if self.pos.0 != 0 {
                self.pos.0 = self.pos.0 - 1}
            },
            Some(Keycode::D) => self.pos.0 = self.pos.0  + 1,
            _ => {}
        }       
    }    
}