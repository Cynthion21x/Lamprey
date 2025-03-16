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
    pub sprite: Vec<Texture<'a>>
}

impl<'a> SpriteMan<'a> {
    
    pub fn new(loader: &'a TextureCreator<WindowContext>) -> Self {
        
        let mut sprites = Vec::new();
        
        let paths = fs::read_dir(config::ASSET_PATH)
            .expect("Where did all my assets go :(");
        
        for i in paths {
            
            let y = i.unwrap().path();
            let file: &str = y.to_str().unwrap();
           
            let ext = { 
                let split = file.char_indices().nth_back(2).unwrap().0;
                &file[split..]
            };
                
            if ext == "png" {
               
               if let Ok(texture) = loader.load_texture(file) {
                   sprites.push(texture);
               }                             
           }            
        }
        
        Self { 
            texture_maker: loader,
            sprite: sprites,
        }
        
    }    
}