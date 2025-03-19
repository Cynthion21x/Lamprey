use sdl2::{keyboard::Keycode, EventPump, event::Event};
use crate::{content::sprite::SpriteMan, rendering::{self, renderer::Renderer}};

pub enum State {
    MainMenu,
    Game
}

pub struct Inputs {
    pub mouse_pos: (u32, u32),
    pub mouse_pressed: bool,
    pub key_pressed: Option<Keycode>,
}

pub struct Game<'a> {
   pub running: bool,
   event_pump: EventPump,
   renderer: Renderer,
   sprites: SpriteMan<'a>,
   state: State,
   input: Inputs,
}

impl<'a> Game<'a> {
 
    pub fn new(event_pump: EventPump, sprites: SpriteMan<'a>, renderer: Renderer) -> Self {
        
        let input = Inputs {
            mouse_pos: (0, 0),
            mouse_pressed: false,
            key_pressed: None
        };
        
        Self { 
            running: true,
            event_pump,
            state: State::MainMenu,
            sprites,
            renderer,
            input
        }
    }
    
    pub fn update(&self) {
        
    }
    
    pub fn input(&mut self) {
       
        self.input.mouse_pressed = false;
        self.input.key_pressed = None;
        for event in self.event_pump.poll_iter() {
            
            match event {
                Event::Quit {..} => {
                    self.running = false
                },
                Event::KeyDown {keycode, ..} => {
                    self.input.key_pressed = Some(keycode).unwrap()
                },
                Event::MouseButtonDown {..} => {
                    self.input.mouse_pressed = true
                },
                Event::MouseMotion {x, y, .. } => {
                    self.input.mouse_pos = (x as u32, y as u32)
                }
                _ => {}               
            }
        } 
    }
    
    pub fn render(&mut self) {
        
        self.renderer.clear();
        
        let grass = self.sprites.sprite_from_string("grass").unwrap();
        self.renderer.draw((0, 0), (500, 500), grass);
        
        self.renderer.present();
        
    }
    
}
