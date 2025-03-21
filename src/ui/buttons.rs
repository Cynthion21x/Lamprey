use sdl2::render::Texture;
use std::any::Any;
use crate::content::asset::AssetMan;
use crate::game::Inputs;
use crate::rendering::renderer::{self, Renderer};
use crate::utils;
use crate::game;

pub struct Button<'a> {
    pos: (u32, u32),
    size: (u32, u32),
    pressed: bool,
    hover: bool,
    texture: &'a Texture<'a>,
    visible: bool,
}

impl<'a> Button<'a> {
    pub fn new(pos: (u32, u32), size: (u32, u32), texture: &'a Texture<'a>) -> Self {
        
        Self { 
            pos,
            size,
            pressed: false, 
            hover: false, 
            texture,
            visible: true
        }
    }
    
    pub fn run(&mut self, input: &Inputs) {
        
        if !self.visible { return () }
        
        let xlb = self.pos.0;
        let ylb = self.pos.1;
        let xb = xlb + self.size.0;
        let yb = ylb + self.size.1;
        
        if utils::in_range(input.mouse_pos.0, xlb, xb) && utils::in_range(input.mouse_pos.1, ylb, yb){
            self.hover = true;
        }
        
        if self.hover && input.mouse_pressed  {
            self.pressed = true;
            println!("pressed bozo");
        } 
    }
    
    pub fn draw(&self, renderer: &mut Renderer) {
        
        if !self.visible { return () }
        
        renderer.draw_gui(self.pos, self.size, self.texture);
        
    }
}
