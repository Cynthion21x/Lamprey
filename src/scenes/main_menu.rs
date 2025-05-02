use crate::{
    config, 
    content::asset::AssetMan, 
    game::{Inputs, State}, 
    rendering::renderer::Renderer, 
    ui::{buttons::Button, photo::Photo}, 
    utils::*,
};

pub struct MainMenu<'a> {
    play_button: Button<'a>,
    quit_button: Button<'a>,
    title: Photo<'a>,
    background: Photo<'a>,
    pub new_state: State,
}

impl<'a> MainMenu<'a> {

    pub fn new(assets: &'a AssetMan) -> Self {
        
        let button = assets.sfs("rect_up").unwrap();
        let button_d = assets.sfs("rect_down").unwrap();
        
        let mut play_button = Button::new((0, 0), (76, 30), button, button_d, "PLAY", 5);
        play_button.pos = center_x(play_button.size, (config::WIN_WIDTH, config::WIN_HEIGHT), config::WIN_HEIGHT / 3);
        
        let mut quit_button = Button::new((0, 0), (76, 30), button, button_d, "QUIT", 5);
        quit_button.pos = center_x(quit_button.size, (config::WIN_WIDTH, config::WIN_HEIGHT), config::WIN_HEIGHT / 2);
        
        let mut title = Photo::new((0, 20), (223, 22), assets.sfs("Title").unwrap());
        title.pos = center_x(title.size, (config::WIN_WIDTH, config::WIN_HEIGHT), title.size.1);
        let background = Photo::new((0, 0), (512, 288), assets.sfs("titleScreen").unwrap()); 
               
        Self { 
            play_button,
            quit_button,
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
        self.play_button.draw(renderer, assets);   
        self.quit_button.draw(renderer, assets);
    
    }

}
