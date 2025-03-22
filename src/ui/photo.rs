use sdl2::render::Texture;
use std::any::Any;
use crate::content::asset::AssetMan;
use crate::game::Inputs;
use crate::rendering::renderer::{self, Renderer};
use crate::utils;
use crate::game;

pub struct Photo<'a> {
    pub pos: (u32, u32),
    pub size: (u32, u32),
    texture: &'a Texture<'a>,
    visible: bool,
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
