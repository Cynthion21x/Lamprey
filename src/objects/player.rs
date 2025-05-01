use sdl2::{
    keyboard::Keycode,
    render::Texture
};
use crate::{
    config::TILE_SIZE, 
    game::Inputs, 
    objects::tilemap::Tilemap, 
    renderer::Renderer, utils::*
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
            floating: true,
            accelleration: (0.0, 0.0),
            jumping: true,
            jumpheigh: pos.1,   
        }
    }
    
    pub fn draw(&self, renderer: &mut Renderer) {
        
        if !self.visible { return () }
        
        let size = (self.size.0 * TILE_SIZE, self.size.1 * TILE_SIZE);
        
        let dpos = (self.pos.0 - 7.5, self.pos.1);
        renderer.draw(dpos, size, self.sprite);
        
    }
    
    pub fn physics(&mut self, tilemap: &Tilemap, time: f32) {
        
        let _map_pos = tuple_times(self.pos, 1.0 / 16.0);
        let newx = ((self.pos.0 + (self.velocity.0 * time)) / 16.0) as usize;
        let newy = ((self.pos.1 + (self.velocity.1 * time)) / 16.0) as usize;
        
        let oldy = (self.pos.1 / 16.0) as usize; 
        let oldx = (self.pos.0 / 16.0) as usize;
        
        if tilemap.tilemap[newy + 2][oldx].tiletype == 1 
        || tilemap.tilemap[newy - 1][oldx].tiletype == 1 
        || tilemap.tilemap[newy + 2][oldx + 1].tiletype == 1 
        || tilemap.tilemap[newy - 1][oldx + 1].tiletype == 1 {
            self.velocity.1 = 0.0;
            self.floating = false;
            self.jumping = false;
        } else {
            self.floating = true;
        }
        
        if tilemap.tilemap[oldy][newx].tiletype == 1
        || tilemap.tilemap[oldy + 1][newx].tiletype == 1
        || tilemap.tilemap[oldy][newx + 1].tiletype == 1
        || tilemap.tilemap[oldy + 1][newx + 1].tiletype == 1{
            self.velocity.0 *= -1.0;
            self.accelleration.0 *= -1.0;
        }
            
        let a = tuple_times(self.accelleration, time);
        self.velocity = tuple_add_f32(self.velocity, a);
            
        let v = tuple_times(self.velocity, time);
        self.pos = tuple_add_f32(self.pos, v);
        
    }
    
    pub fn movement(&mut self, input: &Inputs) {
        
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
        
        if input.key_held.contains(&Keycode::Space) && (self.jumpheigh - self.pos.1) <= 40.0 && self.jumping {
            self.velocity.1 = -160.0;
            self.jumping = true;
        } else if (self.jumpheigh - self.pos.1) > 40.0 {
            self.jumping = false;
        }
        
    }   
}