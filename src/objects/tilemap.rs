use sdl2::render::Texture;
use crate::{config::TILE_SIZE, 
    rendering::renderer::Renderer}
;

#[derive(Copy, Clone)]
pub struct Tile<'a> {
    size: u32,
    texture: Option<&'a Texture<'a>>,  
    pub tiletype: u32,
}

pub struct Tilemap<'a> {
    pub tilemap: [[Tile<'a>; 200]; 101]
}

impl<'a> Tilemap<'a> {
    
    pub fn new() -> Self{
        
        let air = Tile::new(TILE_SIZE, None, 0);
        let tilemap: [[Tile<'a>; 200]; 101] = [[air; 200]; 101];
        
        Self {
            tilemap,
        }
    } 
    
    pub fn draw(&self, renderer: &'a mut Renderer) {
        let mut x = 0;
        for i in self.tilemap.iter() {
            let mut y = 0;
            for j in i.iter() {
                j.draw(renderer, (y * 16, x * 16));
                 y = y + 1
            }
            x = x + 1;
        }
    }
}

impl<'a> Tile <'a> {
    
    pub fn new(size: u32, texture: Option<&'a Texture<'a>>, tiletype: u32) -> Self {
        
        Self {
            size,
            texture,
            tiletype,
        }
    }
    
    pub fn draw(&self, renderer: &'a mut Renderer, pos: (usize, usize)) {
        
        if !Option::is_none(&self.texture) {
            renderer.draw((pos.0 as f32, pos.1 as f32), (self.size, self.size), self.texture.unwrap());
        }
    }
    
}