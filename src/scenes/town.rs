use crate::{
    content::asset::AssetMan, 
    game::Inputs, 
    objects::{player::Player, 
        statics::Static, 
        tilemap::{Tilemap, Tile}
    }, 
    renderer::Renderer,
    ui::photo::Photo,
    config::TILE_SIZE
};

pub struct Town<'a> {
    skybox: Photo<'a>,
    player: Player<'a>,
    town: Static<'a>,
    tilemap: Tilemap<'a>
}

impl<'a> Town<'a> {

    pub fn new(assets: &'a AssetMan) -> Self {
        
        let mut tilemap = Tilemap::new();
        let skybox = Photo::new((0, 0), (0, 0), assets.sprite_from_string("Skybox").unwrap()); 
        let player = Player::new((100.0, 100.0), (2, 2), assets.sprite_from_string("player").unwrap());
        let town = Static::new((0.0, 0.0), (150, 45), assets.sprite_from_string("town").unwrap());
        let ground = Tile::new(TILE_SIZE, None, 1);
        
        for i in 0..122 {
            tilemap.tilemap[36][i] = ground;
        }
        
        Self {
            player,
            skybox,
            town,
            tilemap,
        }
        
    }

    pub fn update(&mut self, camera: &mut Renderer, input: &Inputs, windowsize: (u32, u32), time: f32) {
        
        self.player.movement(input, time);
        self.player.physics(&self.tilemap);
        
        if windowsize.0 > windowsize.1 * 16 / 9 {
            self.skybox.size = (windowsize.1 * 256 / 144, windowsize.1);
        } else {
            self.skybox.size = (windowsize.0, windowsize.0 * 144 / 256);            
        }        
        
        camera.camera_pos = (self.player.pos.0 + (self.player.size.0 * 16) as f32 / 2.0, self.player.pos.1 + (self.player.size.1 * 16) as f32 / 2.0);    
    }

    pub fn render(&self, renderer: &mut Renderer) {

        self.skybox.draw(renderer);
        self.town.draw(renderer);
        self.tilemap.draw(renderer);
        self.player.draw(renderer);        
    }

}
