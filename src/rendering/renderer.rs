use sdl2::{render::Texture, pixels::Color, rect};
use crate::rendering::window; 
use crate::content::sprite;

pub struct Renderer {
    pub window: window::WindowM
}

impl Renderer {
    
    pub fn draw(&mut self, pos: (u32, u32), size: (u32, u32), sprite: &Texture) {
        
        let location = rect::Rect::new(pos.0 as i32, pos.1 as i32, size.0, size.1);
        
        self.window.canvas
            .copy(sprite, None, location)
            .expect("Get better at drawing bozo");
        
    }
    
    pub fn clear(&mut self) {
        self.window.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.window.canvas.clear();
    }
    
    pub fn present(&mut self) {
        self.window.canvas.present();
    }
}