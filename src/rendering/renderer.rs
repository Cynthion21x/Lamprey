use sdl2::{render::Texture, pixels::Color};
use crate::rendering::window; 
use crate::content::sprite;

pub struct Renderer {
    pub window: window::WindowM
}

impl Renderer {
    
    pub fn draw(&self, pos: (u32, u32), size: (u32, u32), sprite: Texture) {
        
        
        
    }
    
    pub fn clear(&mut self) {
        self.window.canvas.set_draw_color(Color::RGB(0, 255, 0));
        self.window.canvas.clear();
    }
}