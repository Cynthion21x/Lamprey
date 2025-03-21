use crate::scenes;
use crate::ui::buttons;
use crate::{
    content::asset::AssetMan,
    rendering::{self, renderer::Renderer},
};
use sdl2::{EventPump, event::Event, keyboard::Keycode, libc::scanf};

pub enum State {
    MainMenu,
    Game,
    Town,
}

pub struct Inputs {
    pub mouse_pos: (u32, u32),
    pub mouse_pressed: bool,
    pub key_pressed: Option<Keycode>,
}

pub struct Game<'a> {
    pub running: bool,
    pub event_pump: EventPump,
    pub renderer: &'a mut Renderer,
    pub assets: &'a AssetMan<'a>,
    pub state: State,
    pub input: Inputs,

    main_menu: scenes::main_menu::MainMenu<'a>,
    town: scenes::town::Town,
    game: scenes::game_scene::Game,
}

impl<'a> Game<'a> {
    pub fn new(event_pump: EventPump, assets: &'a AssetMan, renderer: &'a mut Renderer) -> Self {
        let input = Inputs {
            mouse_pos: (0, 0),
            mouse_pressed: false,
            key_pressed: None,
        };

        let main_menu = scenes::main_menu::MainMenu::new(&assets, &renderer);
        let town = scenes::town::Town::new();
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
        }
    }

    pub fn input(&mut self) {
        self.input.mouse_pressed = false;
        self.input.key_pressed = None;
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.running = false,
                Event::KeyDown { keycode, .. } => self.input.key_pressed = Some(keycode).unwrap(),
                Event::MouseButtonDown { .. } => self.input.mouse_pressed = true,
                Event::MouseMotion { x, y, .. } => self.input.mouse_pos = (x as u32, y as u32),
                _ => {}
            }
        }
    }

    pub fn update(&mut self) {
        if !self.running {
            return;
        };

        match self.state {
            State::MainMenu => self.main_menu.update(&self.input),
            State::Game => self.game.update(&self),
            State::Town => self.town.update(&self),
        }
    }

    pub fn render(&mut self) {
        if !self.running {
            return;
        };

        self.renderer.clear();

        match self.state {
            State::MainMenu => self.main_menu.render(&mut self.renderer, &self.assets),
            State::Game => self.game.render(&self),
            State::Town => self.town.render(&self),
        }

        self.renderer.present();
    }
}
