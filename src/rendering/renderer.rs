use sdl2::rect;
use sdl2::{render::Texture, pixels::Color, rect::Rect};
use crate::{
    content::letterpos, 
    rendering::window,
    utils::*,
    config::{GAME_HEIGHT, TILE_SIZE},
}; 

use super::window::WindowM;

pub struct Renderer {
    pub window: window::WindowM,
    pub camera_pos: (f32, f32),
    pub scalar: f32,
}

impl Renderer {
    
    pub fn new(windowm: WindowM) -> Self {
        
        let camera_pos = (0.0, 0.0);
        let unitsize = windowm.win_size().1 / GAME_HEIGHT;
        let scalar = (unitsize / TILE_SIZE) as f32;
        Self{ 
            window: windowm,
            camera_pos,
            scalar,
        }
        
        
    }

    pub fn calc_scalar(&mut self) {
        
        if self.window.win_size().0 < self.window.win_size().1 * 16 / 9 {

            self.scalar = self.window.win_size().0 as f32 / (256.0 * 2.0);

        } else {

            self.scalar = self.window.win_size().1 as f32 / (144.0 * 2.0);

        }

    }
    
    pub fn draw(&mut self, pos: (f32, f32), size: (u32, u32), sprite: &Texture) {
        
        let pos = ((pos.0 * self.scalar), (pos.1 * self.scalar));
        let size = ((size.0 as f32 * self.scalar) as u32, (size.1 as f32 * self.scalar) as u32);
        
        let camera_pos = tuple_times(self.camera_pos, self.scalar);
        let camera_pos = tuple_add_f32(
            camera_pos, (self.window.game_win_size().0 as f32 / - 2.0, 
            self.window.game_win_size().1 as f32 / -2.0)
        );
        
        if in_range_f32(pos.0, camera_pos.0 - size.0 as f32, camera_pos.0 + self.window.game_win_size().0 as f32)
        && in_range_f32(pos.1, camera_pos.1 - size.1 as f32, camera_pos.1 + self.window.game_win_size().1 as f32) {
            
            let screenpos = (pos.0 - camera_pos.0, pos.1 - camera_pos.1);         
            let mut location = rect::Rect::new(screenpos.0 as i32, screenpos.1 as i32, size.0, size.1);
            
            if self.window.win_size().0 > self.window.win_size().1 * 16 / 9 {
                let offset = (self.window.win_size().0 - self.window.game_win_size().0) / 2;
                location.x = location.x + offset as i32
            } else {
                let offset = (self.window.win_size().1 - self.window.game_win_size().1) / 2; 
                location.y = location.y + offset as i32 
            }
                      
            self.window.canvas
                .copy(sprite, None, location)
                .expect("Get better at drawing bozo");
        }    
    }
    
    pub fn draw_gui(&mut self, pos: (u32, u32), size: (u32, u32), sprite: &Texture) {
        
        let size = ((size.0 as f32 * self.scalar) as u32, (size.1 as f32 * self.scalar) as u32);
        let pos = ((pos.0 as f32 * self.scalar) as u32, (pos.1 as f32 * self.scalar) as u32);
        let mut location = rect::Rect::new(pos.0 as i32, pos.1 as i32, size.0, size.1);
        
        if self.window.win_size().0 > self.window.win_size().1 * 16 / 9 {
            let offset = (self.window.win_size().0 - self.window.game_win_size().0) / 2;
            location.x = location.x + offset as i32
        } else {
            let offset = (self.window.win_size().1 - self.window.game_win_size().1) / 2; 
            location.y = location.y + offset as i32 
        }
        
        self.window.canvas
            .copy(sprite, None, location)
            .expect("Get better at drawing gui bozo");
        
    }
    
    pub fn letterbox(&mut self) {
        
        if self.window.win_size().0 > self.window.win_size().1 * 16 / 9 {
            let size = (self.window.win_size().0 - self.window.game_win_size().0) / 2 + 1;
            
            self.window.canvas.fill_rect(
                rect::Rect::new(0, 0, size, self.window.win_size().1)
            ).unwrap();
            
            self.window.canvas.fill_rect(
                rect::Rect::new(size as i32 + self.window.game_win_size().0 as i32, 0, size, self.window.win_size().1)
            ).unwrap();
            
        } else {
            let size = (self.window.win_size().1 - self.window.game_win_size().1) / 2 + 1; 
            
            self.window.canvas.fill_rect(
                rect::Rect::new(0, 0, self.window.win_size().0, size)
            ).unwrap();
            
            self.window.canvas.fill_rect(
                rect::Rect::new(0, size as i32 + self.window.game_win_size().1 as i32, self.window.win_size().0, size)
            ).unwrap();
            
        } 
        
    }

    pub fn read_sheet(pos: (i32, i32)) -> Rect {
        
        rect::Rect::new(pos.0 * 7 + 1, pos.1 * 9 + 1, 7, 9)     
        
    }
    
    pub fn draw_font(&mut self, mut pos: (u32, u32), size: u32,  text: &str, sheet: &Texture) {
        
        let mut a = 0;
        let actsize = (((size as f32 * 7.0 / 9.0) * self.scalar) as u32, (size as f32 * self.scalar) as u32);
        pos = ((pos.0 as f32 * self.scalar) as u32, (pos.1 as f32 * self.scalar) as u32);
        
        for i in text.chars() {
            
            let letter = letterpos::letterpos(&i);
            let mut pos = Rect::new((pos.0 + a * actsize.0) as i32, (pos.1) as i32, actsize.0, actsize.1);
            
            if self.window.win_size().0 > self.window.win_size().1 * 16 / 9 {
                let offset = (self.window.win_size().0 - self.window.game_win_size().0) / 2;
                pos.x = pos.x + offset as i32
                
            } else {
                let offset = (self.window.win_size().1 - self.window.game_win_size().1) / 2; 
                pos.y = pos.y + offset as i32 
            }
            
            self.window.canvas.copy(sheet, letter, pos).expect("draw_font problems");
            
            a += 1
        }
        
        
    }

    pub fn draw_font_real(&mut self, mut pos: (u32, u32), size: u32,  text: &str, sheet: &Texture) {
        
        let mut a = 0;
        let actsize = (((size as f32 * 7.0 / 9.0) * self.scalar) as u32, (size as f32 * self.scalar) as u32);
        pos = ((pos.0 as f32 * self.scalar) as u32, (pos.1 as f32 * self.scalar) as u32);
        
        let camera_pos = tuple_times(self.camera_pos, self.scalar);
        let camera_pos = tuple_add_f32(
            camera_pos, (self.window.game_win_size().0 as f32 / - 2.0, 
            self.window.game_win_size().1 as f32 / -2.0)
        );
        
        let pos = (pos.0 as f32 - camera_pos.0, pos.1 as f32 - camera_pos.1);  
        
        for i in text.chars() {
            
            let letter = letterpos::letterpos(&i);
            let mut pos = Rect::new((pos.0 as u32 + a * actsize.0) as i32, (pos.1) as i32, actsize.0, actsize.1);
            
            if self.window.win_size().0 > self.window.win_size().1 * 16 / 9 {
                let offset = (self.window.win_size().0 - self.window.game_win_size().0) / 2;
                pos.x = pos.x + offset as i32
                
            } else {
                let offset = (self.window.win_size().1 - self.window.game_win_size().1) / 2; 
                pos.y = pos.y + offset as i32 
            }
            
            self.window.canvas.copy(sheet, letter, pos).expect("draw_font problems");
            
            a += 1
        }
        
        
    }
    
    pub fn clear(&mut self) {
        self.window.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.window.canvas.clear();
    }
    
    pub fn present(&mut self) {
        self.window.canvas.present();
    }
}

