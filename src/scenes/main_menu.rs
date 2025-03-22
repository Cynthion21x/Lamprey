use crate::{content::asset::AssetMan, game::{self, Game, Inputs}, rendering::renderer::{self, Renderer}, ui};

pub struct MainMenu<'a> {
    play_button: ui::buttons::Button<'a>,
    title: ui::photo::Photo<'a>,
    background: ui::photo::Photo<'a>,
}

impl<'a> MainMenu<'a> {

    pub fn new(assets: &'a AssetMan) -> Self {

        let play_button = ui::buttons::Button::new((0, 200), (50,20), assets.sprite_from_string("play").unwrap());
        let title = ui::photo::Photo::new((0, 20), (223, 22), assets.sprite_from_string("Title").unwrap());
        let background = ui::photo::Photo::new((0, 0), (0, 0), assets.sprite_from_string("titleScreen").unwrap()); 

        Self { 
            play_button,
            title,
            background
        }
    }

    pub fn update(&mut self, input: &Inputs, windowSize: (u32, u32), scalar: u32) {

        self.play_button.run(input);
        self.title.size = (windowSize.0 / 2, windowSize.0 / 2 * 22 / 223);
        self.title.pos = ((windowSize.0 - self.title.size.0) / 2, self.title.size.1);

        self.background.size = windowSize;

    }

    pub fn render(&self, renderer: &mut Renderer, assets: &AssetMan) {

        self.play_button.draw(renderer);
        self.background.draw(renderer);
        self.title.draw(renderer);

    }

}
