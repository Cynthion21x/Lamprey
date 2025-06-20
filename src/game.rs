use crate::scenes;
use crate::{
    content::asset::AssetMan,
    rendering::renderer::Renderer,
    config
};
use sdl2::{EventPump, event::Event, keyboard::Keycode};

#[derive(PartialEq)]
pub enum State {
    MainMenu,
    Game,
    Town,
    Quit
}

pub struct Inputs {
    pub mouse_pos: (u32, u32),
    pub mouse_pressed: bool,
    pub key_pressed: Option<Keycode>,
    pub key_up: Option<Keycode>,
    pub key_held: Vec<Keycode>
}

pub struct Game<'a> {
    pub running: bool,
    pub event_pump: EventPump,
    pub renderer: &'a mut Renderer,
    pub assets: &'a AssetMan<'a>,
    pub state: State,
    pub input: Inputs,
    pub old_state: State,

    main_menu: scenes::main_menu::MainMenu<'a>,
    town: scenes::town::Town<'a>,
    game: scenes::game_scene::Game,
    clock: f32,
}

impl<'a> Game<'a> {
    pub fn new(event_pump: EventPump, assets: &'a AssetMan, renderer: &'a mut Renderer) -> Self {
        let input = Inputs {
            mouse_pos: (0, 0),
            mouse_pressed: false,
            key_pressed: None,
            key_held: vec![],
            key_up: None
        };

        let main_menu = scenes::main_menu::MainMenu::new(&assets);
        let town = scenes::town::Town::new(&assets, renderer);
        let game = scenes::game_scene::Game::new();

        Self {
            running: true,
            event_pump,
            state: State::MainMenu,
            assets,
            renderer,
            input,
            main_menu,
            town,
            game,
            clock: 1.0 / config::FPS,
            old_state: State::MainMenu
        }
    }

    pub fn input(&mut self) {
        self.input.key_pressed = None;
        self.input.key_up = None;
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.running = false,
                Event::KeyDown { keycode, .. } => {
                    if !self.input.key_held.contains(&keycode.unwrap()) {
                        self.input.key_held.push(keycode.unwrap());
                    }
                    self.input.key_pressed = Some(keycode).unwrap();
                },
                Event::KeyUp { keycode, .. } => {
                    
                    let index = self.input.key_held.iter().position(|x| *x == keycode.unwrap()).unwrap();
                    self.input.key_held.remove(index);
                    self.input.key_up = Some(keycode).unwrap()
                }
                Event::MouseButtonDown { .. } => self.input.mouse_pressed = true,
                Event::MouseButtonUp { .. } => self.input.mouse_pressed = false,
                Event::MouseMotion { x, y, .. } =>  { 
                    
                    self.input.mouse_pos = (x as u32, y as u32);
                    
                    let mut xoffset = 0 as u32;
                    let mut yoffset = 0 as u32;
                     
                    if self.renderer.window.win_size().0 > self.renderer.window.win_size().1 * 16 / 9 {
                        xoffset = (self.renderer.window.win_size().0 - self.renderer.window.game_win_size().0) / 2;
                    } else {
                        yoffset = (self.renderer.window.win_size().1 - self.renderer.window.game_win_size().1) / 2; 
                    }
                    
                    if xoffset < x as u32 {
                    self.input.mouse_pos.0 = self.input.mouse_pos.0 - xoffset;
                    }
                    
                    if yoffset < y as u32 {
                    self.input.mouse_pos.1 = self.input.mouse_pos.1 - yoffset;
                    }
                    
                    self.input.mouse_pos = ((self.input.mouse_pos.0 as f32 / self.renderer.scalar) as u32, (self.input.mouse_pos.1 as f32 / self.renderer.scalar) as u32);
                    
                },
                _ => {}
            }
        }
    }

    pub fn update(&mut self, time: u128) {
        if !self.running {
            return;
        };
        
        let time = time as f32 / 1000.0;
        
        self.clock = self.clock + time;
        if self.clock >= 1.0 / config::FPS{
            self.clock = 0.0
        } else {
            return;
        }

        self.renderer.calc_scalar();

        match self.state {
            State::MainMenu => {
                self.main_menu.update(&self.input);
                if self.main_menu.new_state == State::Town{
                    self.old_state = State::MainMenu;
                    self.state = State::Town
                } else if self.main_menu.new_state == State::Quit {
                    self.state = State::Quit
                }
            },
            State::Game => self.game.update(),
            State::Town => self.town.update(self.renderer, &self.input, self.renderer.window.game_win_size(), time as f32),
            State::Quit => {
                self.renderer.window.canvas.window_mut().set_bordered(false);
                if self.renderer.window.win_size().0 >= 20 && self.renderer.window.win_size().1 >= 100 
                && !self.renderer.window.canvas.window().is_maximized()  {
                    self.renderer.window.resize(
                        (self.renderer.window.win_size().0 - 25, self.renderer.window.win_size().1 - 25)
                    );
                } else {
                    self.running = false
                }   
            }
        }
    }

    pub fn render(&mut self) {
        if !self.running {
            return;
        };

        self.renderer.clear();

        match self.state {
            State::MainMenu => self.main_menu.render(&mut self.renderer, &self.assets),
            State::Game => self.game.render(),
            State::Town => self.town.render(&mut self.renderer),
            _ => {}
        }

        self.renderer.letterbox();
        self.renderer.present();
    }

}
