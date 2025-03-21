use sdl2::{render::Texture, pixels::Color, rect};
use crate::config::{GAME_HEIGHT, TILE_SIZE};
use crate::rendering::window; 
use crate::content::asset;
use crate::utils::in_range;

use super::window::WindowM;

pub struct Renderer {
    pub window: window::WindowM,
    pub camera_pos: (u32, u32),
    pub unitsize: u32,
    pub scalar: u32,
}

impl Renderer {
    
    pub fn new(windowm: WindowM) -> Self {
        
        let camera_pos = (0, 0);
        let unitsize = windowm.win_size().1 / GAME_HEIGHT;
        let scalar = unitsize / TILE_SIZE;
        Self{ 
            window: windowm,
            camera_pos,
            unitsize,
            scalar,
        }
        
        
    }
    
    pub fn draw(&mut self, pos: (u32, u32), size: (u32, u32), sprite: &Texture) {
        
        if in_range(pos.0, self.camera_pos.0 - size.0, self.camera_pos.0 + self.window.win_size().0)
        && in_range(pos.1, self.camera_pos.1 - size.1, self.camera_pos.1 + self.window.win_size().1) {
            let screenpos = (pos.0 - self.camera_pos.0, pos.1 - self.camera_pos.1);         
            let location = rect::Rect::new(screenpos.0 as i32, screenpos.1 as i32, size.0, size.1);
            self.window.canvas
                .copy(sprite, None, location)
                .expect("Get better at drawing bozo");
        }    
    }
    
    pub fn draw_gui(&mut self, pos: (u32, u32), size: (u32, u32), sprite: &Texture) {
        
        let location = rect::Rect::new(pos.0 as i32, pos.1 as i32, size.0, size.1);
        self.window.canvas
            .copy(sprite, None, location)
            .expect("Get better at drawing gui bozo");
        
    }

    pub fn draw_font(&mut self, pos: (u32, u32), size: u32) {

    }
    
    pub fn clear(&mut self) {
        self.window.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.window.canvas.clear();
    }
    
    pub fn present(&mut self) {
        self.window.canvas.present();
    }
}

