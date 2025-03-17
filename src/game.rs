use sdl2::{keyboard::Keycode, EventPump, event::Event};

use crate::rendering;

pub enum State {
    MainMenu,
    Game
}

pub struct Game {
   pub running: bool,
   event_pump: EventPump,
   state: State,
}

impl Game {
 
    pub fn new(event_pump: EventPump) -> Self {
        Self { 
            running: true,
            event_pump: event_pump,
            state: State::MainMenu,
        }
    }
    
    pub fn update(&self) {
        
    }
    
    pub fn input(&mut self) {
       
       for event in self.event_pump.poll_iter() {
           
           match event {
               Event::Quit {..} |
               Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                   self.running = false
               },
               _ => {}               
           }
       } 
    }
    
    pub fn render(&self) {
        
    }
    
}
