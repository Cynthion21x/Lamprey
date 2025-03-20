use sdl2::{keyboard::Keycode, EventPump, event::Event};
use crate::{content::sprite::SpriteMan, rendering::{self, renderer::Renderer}};
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
   sprites: SpriteMan<'a>,
   state: State,
   input: Inputs,

   main_menu: scenes::main_menu::MainMenu,
   town: scenes::town::Town,
   game: scenes::game::Game,

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

        match self.state {
            
            State::MainMenu => { self.main_menu.update() },
            State::Game => { self.game.update() },
            State::Town => { self.town.update() },

        }

    }

    pub fn render(&mut self) {
        
        self.renderer.clear();
        
         match self.state {
            
            State::MainMenu => { self.main_menu.render() },
            State::Game => { self.game.render() },
            State::Town => { self.town.render() },

        }       
        
        self.renderer.present();
        
    }
    
}
