use sdl2::render::Texture;
use crate::game::Inputs;
use crate::rendering::renderer::Renderer;
use crate::utils;

pub struct Button<'a> {
    pub pos: (u32, u32),
    pub size: (u32, u32),
    pub pressed: bool,
    pub released: bool,
    pub hover: bool,
    pub texture: &'a Texture<'a>,
    pub visible: bool,
    pub centered: bool
}

impl<'a> Button<'a> {
    pub fn new(pos: (u32, u32), size: (u32, u32), texture: &'a Texture<'a>) -> Self {
        
        Self { 
            pos,
            size,
            pressed: false, 
            released: false,
            hover: false, 
            texture,
            visible: true,
            centered: false
        }
    }
    
    pub fn run(&mut self, input: &Inputs) {
        
        if !self.visible { return () }
        
        let xlb = self.pos.0;
        let ylb = self.pos.1;
        let xb = xlb + self.size.0;
        let yb = ylb + self.size.1;
        
        if utils::in_range(input.mouse_pos.0, xlb, xb) && utils::in_range(input.mouse_pos.1, ylb, yb){
            self.hover = true
        } else {
            self.hover = false
        }
        
        if self.hover && input.mouse_pressed  {
            self.pressed = true
        } 
        
        if self.pressed && !input.mouse_pressed {
            self.released = true;
            self.pressed = false
        }
    }
    
    pub fn draw(&self, renderer: &mut Renderer) {
        
        if !self.visible { return () }
        
        renderer.draw_gui(self.pos, self.size, self.texture);
        
    }
    
    pub fn draw_text(&self, text: &str, sheet: &Texture, renderer: &mut Renderer) {
        
        let size = self.size.1 - (12.0 * renderer.scalar) as u32;
        
        let pos; 
        let length = text.chars()
                         .count()
                         as u32;
        
        if self.pressed {

            pos = utils::tuple_add(
                self.pos,
                ((self.size.0 - (size * ((length * 7) - 2) / 9)) / 2, (9.0 * renderer.scalar) as u32)
            );

        } else {

            pos = utils::tuple_add(
                self.pos,
                ((self.size.0 - (size * ((length * 7) - 2) / 9)) / 2, (4.0 * renderer.scalar) as u32)
            );

        }
        
        renderer.draw_font(pos, size, text, sheet); 
        
    }
    
}


