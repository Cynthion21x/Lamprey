use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::sys::LockMask;
use sdl2::video::{Window, WindowContext};
use sdl2::render::Canvas;
use std::{fs, env};
use std::fmt::Display;
use crate::config;

pub struct SpriteMan<'a> {
    pub texture_maker: &'a TextureCreator<WindowContext>,
    pub sprite: Vec<(Texture<'a>, String)>
}

impl<'a> SpriteMan<'a> {
    
    pub fn new(loader: &'a TextureCreator<WindowContext>) -> Self {
        
        let mut sprites = Vec::new();
        
        let paths = fs::read_dir(config::ASSET_PATH)
            .expect("Where did all my assets go :(");
        
        for i in paths {
            
            let y = i.unwrap().path();
            let ext = y.extension().unwrap();
            let location = y.to_str().unwrap();
            let name = y.file_stem().unwrap();
           
            if ext == "png" {
               
               if let Ok(texture) = loader.load_texture(location) {

                   let file_name: String = *name
                       .to_str()
                       .unwrap()
                       .into();

                   sprites.push((texture, file_name));
               }
           }            
        }
        
        Self { 
            texture_maker: loader,
            sprite: sprites,
        }
        
    }    
}
