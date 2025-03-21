use crate::{content::asset::AssetMan, game::{self, Game, Inputs}, rendering::renderer::{self, Renderer}, ui::buttons::{self, Button}};

pub struct MainMenu<'a> {
    play_button: Button<'a>,
}

impl<'a> MainMenu<'a> {

    pub fn new(assets: &'a AssetMan, windowSize: (u32, u32)) -> Self {
        
        let playbutton = buttons::Button::new((300, 300), (50,20), assets.sprite_from_string("play").unwrap());
        
        Self { 
            play_button: playbutton
        }
    }

    pub fn update(&mut self, input: &Inputs) {

        self.play_button.run(input);
        
    }

    pub fn render(&self, renderer: &mut Renderer, assets: &AssetMan) {
        
        let font = assets.sprite_from_string("font").unwrap();
        renderer.draw_font((12, 50), 25, "Hello World!", font);
        self.play_button.draw(renderer);
        
    }

}
