use crate::{
    content::asset::AssetMan, game::Inputs, objects::player::Player, renderer::Renderer
};

pub struct Town<'a> {
    player: Player<'a>
}

impl<'a> Town<'a> {

    pub fn new(assets: &'a AssetMan) -> Self {
        
        let player_sprite = assets.sprite_from_string("player").unwrap();
        let player = Player::new((100, 100), (2, 2), player_sprite);
        
        Self {
            player,
        }
        
    }

    pub fn update(&mut self, input: &Inputs) {
        
        self.player.movement(input)
        
    }

    pub fn render(&self, renderer: &mut Renderer) {

        self.player.draw(renderer);
        
    }

}
