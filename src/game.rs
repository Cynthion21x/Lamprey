use sdl2::{keyboard::Keycode, EventPump, event::Event};

use crate::rendering;

pub enum State {
    MainMenu,
    Game
}

pub struct Inputs {
    pub mouse_pos: (u32, u32),
    pub mouse_pressed: bool,
    pub key_pressed: Option<Keycode>,
}

pub struct Game {
   pub running: bool,
   event_pump: EventPump,
   state: State,
   input: Inputs,
}

impl Game {
 
    pub fn new(event_pump: EventPump) -> Self {
        
        let input = Inputs {
            mouse_pos: (0, 0),
            mouse_pressed: false,
            key_pressed: None
        };
        
        Self { 
            running: true,
            event_pump,
            state: State::MainMenu,
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
    
    pub fn render(&self) {
        
    }
    
}
