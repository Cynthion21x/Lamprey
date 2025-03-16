use crate::rendering::window; 
use crate::config::{self, GRID_WIDTH};
use crate::content::sprite;

pub struct Renderer {
    window: window::WindowM
}

impl Renderer {
    
    pub fn draw(&self, x: u64, y: u64, id: u64) {
       
        let size = self.window.get_win_size();
        
        let tile_size = size.1 / config::GRID_HEIGHT as u32;
        
        let offset = (size.0 - GRID_WIDTH as u32 * tile_size) / 2 as u32;
        
    }
}