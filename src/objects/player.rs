use sdl2::{
    render::Texture,
    keyboard::Keycode
};
use crate::{
    config::TILE_SIZE, game::Inputs, renderer::Renderer
};

pub struct Player<'a> {
    pos: (f32, f32),
    size: (u32, u32),
    sprite: &'a Texture<'a>,
    visible: bool,
}

impl<'a> Player<'a> {
    
    pub fn new(pos: (f32, f32), size: (u32, u32), sprite: &'a Texture<'a>) -> Self {
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
        let pos = (self.pos.0 as u32, self.pos.1 as u32);
        
        renderer.draw(pos, size, self.sprite);
        
    }
    
    pub fn movement(&mut self, input: &Inputs, time: f32) {
        
        for i in input.key_held.iter() {
            match *i {              
                Keycode::A => {
                    if self.pos.0 >= 0.0 {
                    self.pos.0 = self.pos.0 - 64.0 * time}
                },
                Keycode::D => self.pos.0 = self.pos.0 + 64.0 * time,
                _ => {}
            }
        }       
    }    
}