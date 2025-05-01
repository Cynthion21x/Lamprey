use crate::{config, content::asset::AssetMan, game::{Inputs, State}, rendering::renderer::Renderer, ui::{buttons, photo}, utils::{center_x, tuple_add}};

pub struct MainMenu<'a> {
    play_button: buttons::Button<'a>,
    play_button_down: buttons::Button<'a>,
    quit_button: buttons::Button<'a>,
    quit_button_d: buttons::Button::<'a>,
    title: photo::Photo<'a>,
    background: photo::Photo<'a>,
    pub new_state: State,
}

impl<'a> MainMenu<'a> {

    pub fn new(assets: &'a AssetMan) -> Self {
        
        let button = assets.sprite_from_string("rect_up").unwrap();
        let button_d = assets.sprite_from_string("rect_down").unwrap();
        
        let mut play_button = buttons::Button::new((0, 0), (76, 30), button, 5);
        let mut play_button_down = buttons::Button::new((0, 0), (76, 30), button_d, 5);
        play_button.pos = center_x(play_button.size, (config::WIN_WIDTH, config::WIN_HEIGHT), config::WIN_HEIGHT / 3);
        play_button_down.pos = tuple_add(play_button.pos, (0, 6));
        
        let mut quit_button = buttons::Button::new((0, 0), (76, 30), button, 5);
        let mut quit_button_d = buttons::Button::new((0, 0), (76, 30), button_d, 5);
        quit_button.pos = center_x(quit_button.size, (config::WIN_WIDTH, config::WIN_HEIGHT), config::WIN_HEIGHT / 2);
        quit_button_d.pos = tuple_add(quit_button.pos, (0, 6));
        
        let mut title = photo::Photo::new((0, 20), (223, 22), assets.sprite_from_string("Title").unwrap());
        title.pos = center_x(title.size, (config::WIN_WIDTH, config::WIN_HEIGHT), title.size.1);
        let background = photo::Photo::new((0, 0), (512, 288), assets.sprite_from_string("titleScreen").unwrap()); 
               
        Self { 
            play_button,
            play_button_down,
            quit_button,
            quit_button_d,
            title,
            background,
            new_state: State::MainMenu,
        }
    }

    pub fn update(&mut self, input: &Inputs) {

        self.play_button.run(input);
        self.quit_button.run(input);


        if self.play_button.released {
            self.new_state = State::Town
        }
        
        if self.quit_button.released {
            self.new_state = State::Quit
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
        
        if !self.quit_button.pressed {
            self.quit_button.draw(renderer);
        } else {
            self.quit_button_d.draw(renderer);
        }
        
        self.play_button.draw_text("PLAY", assets.sprite_from_string("font").unwrap(), renderer);
        self.quit_button.draw_text("QUIT", assets.sprite_from_string("font").unwrap(), renderer);
    }

}
