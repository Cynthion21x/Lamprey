mod config;
pub mod content;
pub mod rendering;
pub mod scenes;
pub mod ui;
pub mod utils;
use content::asset;
use rendering::{renderer, window};
mod game;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;
    let event_pump = sdl_context.event_pump()?;

    let window_m = window::WindowM::new(&video_subsystem);
    let texture_maker = window_m.canvas.texture_creator();
    let assets = asset::AssetMan::new(&texture_maker);
    let mut renderer = renderer::Renderer::new(window_m);

    let mut core = game::Game::new(event_pump, &assets, &mut renderer);

    while core.running {
        core.input();
        core.update();
        core.render();
    }

    Ok(())
}
