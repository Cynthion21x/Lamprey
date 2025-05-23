mod config;
pub mod content;
pub mod rendering;
pub mod scenes;
pub mod ui;
pub mod utils;
pub mod objects;
use content::asset;
use rendering::{renderer, window};
mod game;
use std::time::Instant;

fn main() -> Result<(), String> {
    
    let mut deltatime: u128 = 0;
    
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;
    let event_pump = sdl_context.event_pump()?;

    let window_m = window::WindowM::new(&video_subsystem);
    let texture_maker = window_m.canvas.texture_creator();
    let assets = asset::AssetMan::new(&texture_maker);
    let mut renderer = renderer::Renderer::new(window_m);

    let mut core = game::Game::new(event_pump, &assets, &mut renderer);

    while core.running {
        
        let now = Instant::now();
        
        core.input();
        core.update(deltatime);
        core.render();
        
        let time_elapsed = now.elapsed();
        deltatime = time_elapsed.as_millis();
        
    }

    Ok(())
}
