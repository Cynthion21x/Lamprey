use std::fmt::Pointer;

use sdl2::{
    render::Texture,
    keyboard::Keycode
};
use crate::{
    config::TILE_SIZE, 
    game::Inputs, 
    renderer::Renderer, 
    utils::*,
    objects::tilemap::Tilemap
};

pub struct Player<'a> {
    pub pos: (f32, f32),
    pub size: (u32, u32),
    sprite: &'a Texture<'a>,
    visible: bool,
    accelleration: (f32, f32),
    velocity: (f32, f32),
    floating: bool,
    jumping: bool,
    jumpheigh: f32,
}

impl<'a> Player<'a> {
    
    pub fn new(pos: (f32, f32), size: (u32, u32), sprite: &'a Texture<'a>) -> Self {
        
        Self{
            pos,
            size,
            sprite,  
            visible: true,
            velocity: (0.0, 0.0),
            floating: false,
            accelleration: (0.0, 0.0),
            jumping: false,
            jumpheigh: pos.1,   
        }
    }
    
    pub fn draw(&self, renderer: &mut Renderer) {
        
        if !self.visible { return () }
        
        let size = (self.size.0 * TILE_SIZE, self.size.1 * TILE_SIZE);
        
        renderer.draw(self.pos, size, self.sprite);
        
    }
    
    pub fn physics(&mut self, tilemap: &Tilemap) {
        
        let map_pos = tuple_times(self.pos, 1.0 / 16.0);
        
        if map_pos.1 + 3.0 < tilemap.tilemap.len() as f32 
        && tilemap.tilemap[map_pos.1 as usize + 2][map_pos.0 as usize].tiletype == 1 
        && collision((map_pos.0 * 16.0, (map_pos.1 + 3.0) * 16.0), (16, 16), self.pos, (16, 32)) {
            self.velocity.1 = 0.0;
            self.floating = false;
            self.jumping = false;
            self.pos.1 = map_pos.1 * 16.0 - 1.0             
        } 
    }
    
    pub fn movement(&mut self, input: &Inputs, time: f32) {
        
        self.velocity.0 = self.velocity.0 * 0.9;
        
        if self.floating && self.velocity.1 < 600.0{
            self.accelleration = (0.0, 1000.0);
        } else {
            self.accelleration = (0.0, 0.0);
        }
        
        for i in input.key_held.iter() {
            match *i {              
                Keycode::A => {
                    if self.velocity.0 > - 256.0 && self.pos.0 > 0.0 {
                        self.accelleration.0 = - 512.0
                    } 
                },
                Keycode::D => {
                    if self.velocity.0 < 256.0 {
                        self.accelleration.0 = 512.0
                    }
                },
                _ => {}
            }
        }    
        
        if !self.jumping && !self.floating && input.key_pressed == Some(Keycode::SPACE){
            self.jumping = true;
            self.floating = true;
            self.jumpheigh = self.pos.1
        }
        
        if input.key_up == Some(Keycode::Space) {
            self.jumping = false;
        }
        
        if input.key_held.contains(&Keycode::Space) && (self.jumpheigh - self.pos.1) <= 50.0 && self.jumping {
            self.velocity.1 = -150.0;
            self.jumping = true;
        } else if (self.jumpheigh - self.pos.1) > 50.0 {
            self.jumping = false;
        }
        
        let a = tuple_times(self.accelleration, time);
        self.velocity = tuple_add_f32(self.velocity, a);
        
        let v = tuple_times(self.velocity, time);
        self.pos = tuple_add_f32(self.pos, v);
        
    }   
}