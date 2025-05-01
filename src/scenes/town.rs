use crate::{
    config::TILE_SIZE, 
    content::asset::AssetMan, 
    game::Inputs, 
    objects::{
        npc::NPC,
        player::Player, 
        statics::Static, 
        tilemap::{Tile, Tilemap}
        }, 
    renderer::Renderer, 
    ui::photo::Photo
};

pub struct Town<'a> {
    skybox: Photo<'a>,
    player: Player<'a>,
    steve: NPC<'a>,
    town: Static<'a>,
    tilemap: Tilemap<'a>,
    textbox: Photo<'a>,
}

impl<'a> Town<'a> {

    pub fn new(assets: &'a AssetMan, camera: &mut Renderer) -> Self {
        
        let mut tilemap = Tilemap::new(assets);
        let bubblet = assets.sprite_from_string("speech").unwrap();
        let skybox = Photo::new((0, 0), (0, 0), assets.sprite_from_string("Skybox").unwrap()); 
        let player = Player::new((100.0, 544.3), (2, 2), assets.sprite_from_string("player").unwrap());
        let steve = NPC::new((0.0, 544.0), (2, 2), assets.sprite_from_string("steve").unwrap(), bubblet);
        let town = Static::new((0.0, 0.0), (150, 45), assets.sprite_from_string("town").unwrap());
        let ground = Tile::new(TILE_SIZE, None, 1);
        let textbox = Photo::new((32, 100), ((448.0 * camera.scalar) as u32, (80.0 * camera.scalar) as u32), assets.sprite_from_string("textbox").unwrap());
        
        for i in 0..122 {
            tilemap.tilemap[36][i] = ground;
        }
        
        for i in 0..tilemap.tilemap.len() {
            tilemap.tilemap[i][0] = ground;
        }
    
        
        camera.camera_pos.0 = player.pos.0 + (player.size.0 as f32 * 16.0 / 2.0);
        camera.camera_pos.1 = player.pos.1;
        
        Self {
            player,
            steve,
            skybox,
            town,
            tilemap,
            textbox,
        }
        
    }

    pub fn update(&mut self, camera: &mut Renderer, input: &Inputs, windowsize: (u32, u32), time: f32) {
        
        self.player.movement(input);
        self.player.physics(&self.tilemap, time);
        self.steve.update(self.player.pos, input, &mut self.textbox);
        
        if windowsize.0 > windowsize.1 * 16 / 9 {
            self.skybox.size = (windowsize.1 * 256 / 144, windowsize.1);
        } else {
            self.skybox.size = (windowsize.0, windowsize.0 * 144 / 256);            
        }  
        
        let xchange = ((self.player.pos.0 + (self.player.size.0 as f32 * 16.0 / 2.0)) - camera.camera_pos.0) * 0.5;
        camera.camera_pos.0 = camera.camera_pos.0 + xchange;
                
        let ychange = (self.player.pos.1 - camera.camera_pos.1) * 0.2;
        camera.camera_pos.1 = camera.camera_pos.1 + ychange
          
    }

    pub fn render(&self, renderer: &mut Renderer) {

        self.skybox.draw(renderer);
        self.town.draw(renderer);
        self.tilemap.draw(renderer);
        self.steve.draw(renderer);
        self.player.draw(renderer); 
        self.textbox.draw(renderer);
        
    }

}
