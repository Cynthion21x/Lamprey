pub mod content;
pub mod rendering;
pub mod ui;
pub mod scenes;
pub mod utils;
mod config;
use rendering::{renderer, window};
use content::asset;
mod game;
use sdl2::{keyboard::Keycode, pixels::Color, event::Event, ttf};

fn main() -> Result<(), String> {

    let sdl_context = sdl2::init()?;
    let font_maker = ttf::init().expect("Couldn't Load Fonts"); 

    let video_subsystem = sdl_context.video()?;
    let event_pump = sdl_context.event_pump()?;  
    
    let window_m = window::WindowM::new(&video_subsystem);    
    let texture_maker = window_m.canvas.texture_creator();
    let assets = asset::AssetMan::new(&texture_maker, &font_maker);  
    let renderer = renderer::Renderer::new(window_m);

    let mut core = game::Game::new(event_pump, assets, renderer);
    
    while core.running {
       core.input();
       core.update();
       core.render();
    }
    
    Ok(())

}
