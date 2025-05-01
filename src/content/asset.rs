use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::ffi::OsStr;
use std::fs;
use crate::config;

pub struct AssetMan<'a> {
    pub texture_maker: &'a TextureCreator<WindowContext>,
    pub sprite: Vec<(Texture<'a>, String)>,
}

impl<'a> AssetMan<'a> {
    
    pub fn new(tex_loader: &'a TextureCreator<WindowContext>) -> Self {
        
        let mut sprites = Vec::new();
        
        let paths = fs::read_dir(config::ASSET_PATH)
            .expect("Where did all my assets go :(");
        
        for i in paths {

            let y = i.unwrap().path();
            let ext = y.extension().unwrap_or(&OsStr::new(""));
            let location = y.to_str().unwrap();
            let name = y.file_stem().unwrap();
            
            match ext.to_str().unwrap() { 
               
                "png" => {

                    if let Ok(texture) = tex_loader.load_texture(location) {
                    
                        let file_name: String = name
                            .to_str()
                            .unwrap()
                            .into();

                        sprites.push((texture, file_name));
                    }
                },

                _ => { }

            }
        }
        
        Self { 
            texture_maker: tex_loader,
            sprite: sprites,
        }
        
    }
    
    pub fn sprite_from_string(&self, sprite: &str) -> Option<&Texture<'a>> {
        for i in 0..self.sprite.len() {
            if self.sprite[i].1 == sprite {
                return Some(&self.sprite[i].0);
            } 
        }
        None
    }

    pub fn drop(&mut self) {
        
        self.sprite.clear();

    }

}
