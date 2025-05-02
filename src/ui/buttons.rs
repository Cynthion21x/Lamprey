use sdl2::render::Texture;
use crate::game::Inputs;
use crate::rendering::renderer::Renderer;
use crate::utils::*;
use crate::content::asset::AssetMan;

pub struct Button<'a> {
    pub pos: (u32, u32),
    pub size: (u32, u32),
    pub pressed: bool,
    pub released: bool,
    pub hover: bool,
    pub texture: &'a Texture<'a>,
    pub texture_down: &'a Texture<'a>,
    pub visible: bool,
    pub centered: bool,
    pub textheight: u32,
    pub text: &'a str,
}

impl<'a> Button<'a> {
    pub fn new(pos: (u32, u32), size: (u32, u32), texture: &'a Texture<'a>, texture_down: &'a Texture<'a>, text: &'a str, textheight: u32) -> Self {
        
        Self { 
            pos,
            size,
            pressed: false, 
            released: false,
            hover: false, 
            texture,
            texture_down,
            visible: true,
            centered: false,
            textheight,
            text,
        }
    }
    
    pub fn run(&mut self, input: &Inputs) {
        
        if !self.visible { return () }
        
        let xlb = self.pos.0;
        let ylb = self.pos.1;
        let xb = xlb + self.size.0;
        let yb = ylb + self.size.1;
        
        if in_range(input.mouse_pos.0, xlb, xb) && in_range(input.mouse_pos.1, ylb, yb){
            self.hover = true;
        } else {
            self.hover = false;
        }
        
        if self.hover && input.mouse_pressed  {
            self.pressed = true
        } 
        
        if self.pressed && !input.mouse_pressed {
            self.released = true;
            self.pressed = false
        }
    }
    
    pub fn draw(&self, renderer: &mut Renderer, assets: &AssetMan) {
        
        if !self.visible { return () }
        
        if !self.pressed {
            renderer.draw_gui(self.pos, self.size, self.texture);
        } else {
            renderer.draw_gui((self.pos.0, self.pos.1 + self.textheight - 3), self.size, self.texture_down);
        }
        
        self.draw_text(self.text, assets.sfs("black_font").unwrap(), renderer);
    }
    
    pub fn draw_text(&self, text: &str, sheet: &Texture, renderer: &mut Renderer) {
        
        let length = text.chars().count() as u32;
        let mut size = (7 * length, 9);
        let y_pos;
        let x_pos;
        
        if self.size.0 >= self.size.1 / size.1 * size.0 {
            
            let multiplyer = (self.size.1 * 5 / 6) / size.1;
            size = tuple_times_u32(size, multiplyer);
            
        } else {
            
            let multiplyer = (self.size.0 * 5 / 6) / size.0; 
            size = tuple_times_u32(size, multiplyer);
        }
        
        if !self.pressed {

            let n = self.pos.1 - self.textheight;
            
            y_pos = n + center_y(size, self.size, 0).1;
            x_pos = self.pos.0 + center_x(size, self.size, 0).0;
            
        } else {

            y_pos = self.pos.1 + center_y(size, self.size, 0).1;
            x_pos = self.pos.0 + center_x(size, self.size, 0).0;
            
        }
        
        renderer.draw_font((x_pos + 2, y_pos + 2), size.1, text, sheet); 
        
    }
    
}


