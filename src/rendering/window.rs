use sdl2::{render::Canvas, video::Window, VideoSubsystem};
use crate::config;

pub struct WindowM {
   pub canvas: Canvas<Window>,
}

impl WindowM {
    
    pub fn new(video_subsystem: &VideoSubsystem) -> Self {
        
        let win = video_subsystem.window(config::TITLE, config::WIN_WIDTH, config::WIN_HEIGHT)
            .position_centered()
            .resizable()
            .build()
            .expect("Failed to build window");
        
        let canvas = win.into_canvas()
            .present_vsync()
            .build()
            .expect("No canvas :c");
        
        Self { 
            canvas,
        }
        
    }
    
    pub fn win_size(&self) -> (u32, u32) {
        self.canvas.window().size()
    }
    
    pub fn game_win_size(&self) -> (u32, u32) {
        
        let windowsize = self.win_size();
        
        if windowsize.0 > windowsize.1 * 16 / 9 {
            (windowsize.1 * 256 / 144, windowsize.1)
        } else {
            (windowsize.0, windowsize.0 * 144 / 256)           
        }
        
    }
}
