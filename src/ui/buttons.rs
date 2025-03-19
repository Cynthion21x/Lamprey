use sdl2::render::Texture;
use std::any::Any;
use crate::content::sprite::SpriteMan;
use crate::game::Inputs;
use crate::utils;

pub struct Button<'a> {
    pos: (u32, u32),
    size: (u32, u32),
    pressed: bool,
    hover: bool,
    texture: &'a Texture<'a>,
}

impl<'a> Button<'a> {
    fn new(pos: (u32, u32), size: (u32, u32), sprite: &str, spriteman: &'a SpriteMan) -> Self {
        let tex = spriteman.sprite_from_string(sprite).expect("sprite not found");
        Self { 
            pos,
            size,
            pressed: false, 
            hover: false, 
            texture: tex 
        }
    }
    
    fn run(&mut self, input: Inputs) {
        
        let xlb = self.pos.0;
        let ylb = self.pos.1;
        let xb = xlb + self.size.0;
        let yb = ylb + self.size.1;
        
        if utils::in_range(input.mouse_pos.0, xlb, xb) && utils::in_range(input.mouse_pos.1, ylb, yb){
            self.hover = true;
        }
        
        if self.hover && input.mouse_pressed  {
            self.pressed = true
        } 
    }
    
    fn draw(&self) {
        
    }
}
