use sdl2::rect;

use crate::{
    content::asset::AssetMan, 
    game::Inputs, 
    objects::player::Player, 
    renderer::Renderer,
    ui::photo::Photo
};

pub struct Town<'a> {
    background: Photo<'a>,
    player: Player<'a>
}

impl<'a> Town<'a> {

    pub fn new(assets: &'a AssetMan) -> Self {
        
        let background = Photo::new((0, 0), (0, 0), assets.sprite_from_string("titleScreen").unwrap()); 
        let player_sprite = assets.sprite_from_string("player").unwrap();
        let player = Player::new((100.0, 100.0), (2, 2), player_sprite);
        
        Self {
            player,
            background
        }
        
    }

    pub fn update(&mut self, input: &Inputs, windowsize: (u32, u32), time: f32) {
        
        self.player.movement(input, time);
        
        if windowsize.0 > windowsize.1 * 16 / 9 {
            self.background.size = (windowsize.1 * 256 / 144, windowsize.1);
        } else {
            self.background.size = (windowsize.0, windowsize.0 * 144 / 256);            
        }
        
    }

    pub fn render(&self, renderer: &mut Renderer) {

        self.background.draw(renderer);
        self.player.draw(renderer);

        
    }

}
