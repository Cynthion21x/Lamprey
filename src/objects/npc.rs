use sdl2::{keyboard::Keycode, render::Texture};
use crate::{
    config::TILE_SIZE, game::Inputs, rendering::renderer::Renderer, ui::photo::Photo, utils::*
};

pub struct NPC<'a> {
    pos: (f32, f32),
    size: (u32, u32),
    sprite: &'a Texture<'a>,
    bubblet: &'a Texture<'a>,
    bubble: bool,
    text: &'a (Vec<String>, String),
    textvalue: (usize, usize),
    visible: bool,
    t: usize,
    pub question: bool,
    pub close: bool,
    pub textstate: u32,
    pub speaking: bool,
}

impl<'a> NPC<'a> {
    
    pub fn new(pos: (f32, f32), size: (u32, u32), sprite: &'a Texture<'a>, bubblet: &'a Texture<'a>, text: &'a (Vec<String>,String), textvalue: (usize, usize)) -> Self {
        
        let t = 0;
        
        Self{
            pos,
            size,
            sprite,
            visible: true,
            bubble: false,
            bubblet,
            text,
            t,
            question: false,
            close: false,
            textstate: 0,
            textvalue,
            speaking: false
        }
        
    }
    
    pub fn draw(&self, renderer: &mut Renderer) {
        
        if !self.visible { return () }
        
        let size = (self.size.0 * TILE_SIZE, self.size.1 * TILE_SIZE);
        renderer.draw(self.pos, size, self.sprite);
        
        if self.bubble {
            renderer.draw((self.pos.0 + 24.0, self.pos.1 - 10.0), (16, 16), self.bubblet);
        }
        
    }
    
    pub fn update(&mut self, player: (f32, f32), input: &Inputs, textbox: &mut Photo) -> Option<(String, String)> {
        
        if dist(player, self.pos) < 50.0 {
        
            self.bubble = true;
            
            if !self.speaking && self.textstate == 0{
                self.t = 0
            } else if !self.speaking && self.textstate == 1 {
                self.t = self.textvalue.0
            } else if !self.speaking && self.textstate == 2 {
                self.t = self.textvalue.1
            }
            
            if self.question && input.key_pressed == Some(Keycode::Z) {
                
                self.t += 1;
                self.textstate = 1;
                self.question = false;
                
                return self.speak(self.t, textbox)
                
            } else if self.question && input.key_pressed == Some(Keycode::X) {
                
                textbox.visible = false;
                self.speaking = false;
                self.question = false;
                
            }
            
            if !self.close && !self.question && input.key_pressed == Some(Keycode::E) {
                
                self.t += 1;
                self.speaking = true;
                return self.speak(self.t, textbox);
                
            } else if self.close && input.key_pressed == Some(Keycode::E) {
                
                textbox.visible = false;
                self.speaking = false;
                self.question = false;
                self.close = false;
                
            }
        
            None
        
        } else {None}
    }
    
    pub fn speak(&self, textnum: usize, textbox: &mut Photo) -> Option<(String, String)> {
        
        textbox.visible = true;
        if textnum - 1 < self.text.0.len() {
            Some((self.text.0[textnum - 1].to_owned(), self.text.1.to_owned()))
        } else {
            None
        }
    }
}