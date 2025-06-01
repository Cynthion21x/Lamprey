use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::ffi::OsStr;
use std::fs;

pub struct AssetMan<'a> {
    pub texture_maker: &'a TextureCreator<WindowContext>,
    pub sprite: Vec<(Texture<'a>, String)>,
    pub npctext: Vec<(Vec<String>, String)>,
}

impl<'a> AssetMan<'a> {
    
    pub fn new(tex_loader: &'a TextureCreator<WindowContext>) -> Self {
        
        let mut sprites = Vec::new();
        let mut npctext = Vec::new();
        
        let textpath = fs::read_dir("./assets/text/")
            .expect("Where did all my text go :(");
        
        for i in textpath {
            
            let y = i.unwrap().path();
            let ext = y.extension().unwrap_or(&OsStr::new(""));
            let location = y.to_str().unwrap();
            let name = y.file_stem().unwrap();
            
            match ext.to_str().unwrap() { 
               
                "txt" => {
                    
                    let mut tempv: Vec<String> = Vec::new();
                    let mut temp = String::new();
                    let contents = fs::read_to_string(location)
                            .expect("Where did all my text go :(");
                    for j in contents.chars() {
                        
                        if j != '/' {
                            temp = format!("{}{}", temp, j);
                        } else if temp != "" {
                            
                            tempv.push(temp.clone());
                            temp = String::new();
                            
                        }                         
                    }
                    let file_name: String = name
                        .to_str()
                        .unwrap()
                        .into();
                    
                    npctext.push((tempv, file_name));
                },

                _ => {}
            }
        }
        
        let assetpath = fs::read_dir("./assets/sprites/")
            .expect("Where did all my assets go :(");
        
        for i in assetpath {

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
            npctext,
        }
        
    }
    
    pub fn sfs(&self, sprite: &str) -> Option<&Texture<'a>> {
        for i in 0..self.sprite.len() {
            if self.sprite[i].1 == sprite {
                return Some(&self.sprite[i].0);
            } 
        }
        None
    }
    
    pub fn tfs(&self, text: &str) -> Option<&(Vec<String>, String)> {
        for i in 0..self.npctext.len() {
            if self.npctext[i].1 == text {
                return Some(&self.npctext[i]);
            } 
        }
        None
    }

    pub fn drop(&mut self) {
        
        self.sprite.clear();

    }

}
