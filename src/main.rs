pub mod content;
pub mod rendering;
pub mod ui;
pub mod utils;
mod config;
use rendering::{renderer, window};
use content::sprite;
mod game;
use sdl2::{keyboard::Keycode, pixels::Color, event::Event};

fn main() -> Result<(), String> {

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let event_pump = sdl_context.event_pump()?;  
    let window_m = window::WindowM::new(&video_subsystem);    
    let texture_maker = window_m.canvas.texture_creator();
    let sprites = sprite::SpriteMan::new(&texture_maker);  
    let mut renderer = renderer::Renderer{window:window_m};
    let mut core = game::Game::new(event_pump);
    
    renderer.clear();
    renderer.window.canvas.present();
    
    while core.running {
       core.input();
       core.update();
       core.render();
    }
    
    Ok(())

}
