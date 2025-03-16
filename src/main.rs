mod config;
use sdl2::{Sdl, video::Window};

fn main() -> Result<(), String> {

    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window(config::TITLE, config::GAME_WIDTH, config::GAME_HEIGHT)
        .position_centered()
        .build()
        .expect("Failed to build window");

    println!("Worked");

    Ok(())

}
