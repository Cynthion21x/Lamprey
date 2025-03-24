use sdl2::sys::WINT_MAX;

use crate::{content::asset::AssetMan, game::{Inputs, State}, rendering::renderer::{self, Renderer}, ui, utils};

pub struct MainMenu<'a> {
    play_button: ui::buttons::Button<'a>,
    play_button_down: ui::buttons::Button<'a>,
    title: ui::photo::Photo<'a>,
    background: ui::photo::Photo<'a>,
    pub new_state: State,
    click: u32
}

impl<'a> MainMenu<'a> {

    pub fn new(assets: &'a AssetMan) -> Self {
        
        let play_button = ui::buttons::Button::new((0, 200), (50,20), assets.sprite_from_string("rect_up").unwrap());
        let play_button_down = ui::buttons::Button::new((0, 200), (50,20), assets.sprite_from_string("rect_down").unwrap());
        let title = ui::photo::Photo::new((0, 20), (223, 22), assets.sprite_from_string("Title").unwrap());
        let background = ui::photo::Photo::new((0, 0), (0, 0), assets.sprite_from_string("titleScreen").unwrap()); 
        let new_state = State::MainMenu;
        
        
        Self { 
            play_button,
            play_button_down,
            title,
            background,
            new_state: State::MainMenu,
            click: 0
        }
    }

    pub fn update(&mut self, input: &Inputs, windowsize: (u32, u32), scalar: f32) {
        
        self.play_button.run(input);
        self.title.size = ((223.0 * scalar) as u32, (22.0 * scalar) as u32);
        self.title.pos = ((windowsize.0 - self.title.size.0) / 2, self.title.size.1);
        
        self.play_button.size = ((38.0 * scalar * 2.0) as u32, (15.0 * scalar * 2.0) as u32);
        self.play_button.pos = ((windowsize.0 - self.play_button.size.0) / 2, windowsize.1 / 3);
        
        self.play_button_down.size = ((38.0 * scalar * 2.0) as u32, (12.0 * scalar * 2.0) as u32);
        self.click = self.play_button.size.1 - self.play_button_down.size.1;
        self.play_button_down.pos = ((windowsize.0 - self.play_button.size.0) / 2, windowsize.1 / 3 + self.click);
        
        if windowsize.0 > windowsize.1 * 16 / 9 {
            self.background.size = (windowsize.1 * 256 / 144, windowsize.1);
        } else {
            self.background.size = (windowsize.0, windowsize.0 * 144 / 256);            
        }
        if self.play_button.released {
            //self.new_state = State::Town
        }

    }

    pub fn render(&self, renderer: &mut Renderer, assets: &AssetMan) {

        self.background.draw(renderer);
        self.title.draw(renderer);
        
        if !self.play_button.pressed {
            self.play_button.draw(renderer);
        } else {
            self.play_button_down.draw(renderer);
        }
        
        self.play_button.draw_text("PLAY", assets.sprite_from_string("black_font").unwrap(), renderer);
        
    }

}
