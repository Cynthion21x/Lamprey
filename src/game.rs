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
   pub mousePos: (u32, u32),
   pub mousePressed: bool,
   pub keyPressed: Option<Event>,
}

impl Game {
 
    pub fn new(event_pump: EventPump) -> Self {
        Self { 
            running: true,
            event_pump: event_pump,
            state: State::MainMenu,
            mousePos: (0, 0),
            mousePressed: false,
            keyPressed: None
        }
    }
    
    pub fn update(&self) {
        
    }
    
    pub fn input(&mut self) {
       
       for event in self.event_pump.poll_iter() {
           
           match event {
               Event::Quit {..} => {
                   self.running = false
               },
               Event::KeyDown {..} => {
                   self.keyPressed = Some(event)
               },
               _ => {}               
           }
       } 
    }
    
    pub fn render(&self) {
        
    }
    
}
