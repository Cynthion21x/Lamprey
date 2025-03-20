use sdl2::{keyboard::Keycode, EventPump, event::Event};
use crate::{content::asset::AssetMan, rendering::{self, renderer::Renderer}};
use crate::ui::buttons;
use crate::scenes;

pub enum State {
    MainMenu,
    Game,
    Town
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
   assets: AssetMan<'a>,
   state: State,
   input: Inputs,

   main_menu: scenes::main_menu::MainMenu,
   town: scenes::town::Town,
   game: scenes::game::Game,

}

impl<'a> Game<'a> {
 
    pub fn new(event_pump: EventPump, assets: AssetMan<'a>, renderer: Renderer) -> Self {
        
        let input = Inputs {
            mouse_pos: (0, 0),
            mouse_pressed: false,
            key_pressed: None
        };
        
        Self { 
            running: true,
            event_pump,
            state: State::MainMenu,
            assets,
            renderer,
            input,
            main_menu: scenes::main_menu::MainMenu::new(),
            town: scenes::town::Town::new(),
            game: scenes::game::Game::new(),
        }
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
    
    pub fn update(&self) {

        if !self.running { return };

        match self.state {
            
            State::MainMenu => { self.main_menu.update(&self) },
            State::Game => { self.game.update(&self) },
            State::Town => { self.town.update(&self) },

        }

    }

    pub fn render(&mut self) {
        
        if !self.running { return };
        
        self.renderer.clear();
        
         match self.state {
            
            State::MainMenu => { self.main_menu.render(&self) },
            State::Game => { self.game.render(&self) },
            State::Town => { self.town.render(&self) },

        }       
        
        self.renderer.present();
        
    }

    pub fn close(&mut self) {

        self.assets.drop();
        
    }
    
}
