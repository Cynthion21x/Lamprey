use sdl2::{keyboard::Keycode, render::Texture};
use crate::{
    config::TILE_SIZE, game::Inputs, rendering::renderer::Renderer, ui::photo::Photo, utils::*
};

pub struct NPC<'a> {
    pos: (f32, f32),
    size: (u32, u32),
    sprite: &'a Texture<'a>,
    bubblet: &'a Texture<'a>,
    bubble: bool,
    //text: Vec<String>,
    visible: bool,
}

impl<'a> NPC<'a> {
    
    pub fn new(pos: (f32, f32), size: (u32, u32), sprite: &'a Texture<'a>, bubblet: &'a Texture<'a>) -> Self {
        
        Self{
            pos,
            size,
            sprite,
            visible: true,
            bubble: false,
            bubblet
        }
        
    }
    
    pub fn draw(&self, renderer: &mut Renderer) {
        
        if !self.visible { return () }
        
        let size = (self.size.0 * TILE_SIZE, self.size.1 * TILE_SIZE);
        renderer.draw(self.pos, size, self.sprite);
        
        if self.bubble {
            renderer.draw((self.pos.0 + 24.0, self.pos.1 - 10.0), (16, 16), self.bubblet);
        }
        
    }
    
    pub fn update(&mut self, player: (f32, f32), input: &Inputs, textbox: &mut Photo) {
        
        self.bubble = dist(player, self.pos) < 50.0;
        
        if input.key_pressed == Some(Keycode::E) {
            
            textbox.visible = true
            
        }
      
    }
    
}