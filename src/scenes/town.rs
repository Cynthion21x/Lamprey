use sdl2::render::Texture;

use crate::{
    config::TILE_SIZE, 
    content::asset::AssetMan, 
    game::Inputs, 
    objects::{
        npc::NPC,
        player::Player, 
        statics::Static, 
        tilemap::{Tile, Tilemap},
        shop::Shop,
        }, 
    renderer::Renderer, 
    ui::photo::Photo,
    
};

pub struct Town<'a> {
    skybox: Photo<'a>,
    player: Player<'a>,
    steve: NPC<'a>,
    town: Static<'a>,
    text_name: String,
    text_words: String,
    tilemap: Tilemap<'a>,
    textbox: Photo<'a>,
    font: &'a Texture<'a>,
    money: Photo<'a>,
    shop: Shop<'a>,
}

impl<'a> Town<'a> {

    pub fn new(assets: &'a AssetMan, camera: &mut Renderer) -> Self {
        
        let mut tilemap = Tilemap::new(assets);
        let bubblet = assets.sfs("speech").unwrap();
        let font = assets.sfs("black_font").unwrap();
        let skybox = Photo::new((0, 0), (0, 0), assets.sfs("Skybox").unwrap()); 
        let player = Player::new((100.0, 544.6), (2, 2), assets.sfs("player").unwrap());
        let money = Photo::new((10, 10), (24, 24), assets.sfs("coin").unwrap());
        let steve = NPC::new((0.0, 545.0), (2, 2), assets.sfs("steve").unwrap(), bubblet, assets.tfs("Steve").unwrap(), (4, 5));
        let town = Static::new((0.0, 0.0), (150, 45), assets.sfs("town").unwrap());
        let ground = Tile::new(TILE_SIZE, None, 1);
        let shopsprite = Static::new((40.0, 513.0), (7, 4), assets.sfs("market").unwrap());
        let shop = Shop::new(shopsprite, assets);
        let mut textbox = Photo::new((32, 190), (448, 80), assets.sfs("textbox").unwrap());
        textbox.visible = false;
        
        for i in 0..122 {
            tilemap.tilemap[36][i] = ground;
        }
        
        for i in 0..tilemap.tilemap.len() {
            tilemap.tilemap[i][0] = ground;
        }
        
        let text_name = String::new();
        let text_words = String::new();
        
        camera.camera_pos.0 = player.pos.0 + (player.size.0 as f32 * 16.0 / 2.0);
        camera.camera_pos.1 = player.pos.1;
        
        Self {
            player,
            steve,
            skybox,
            town,
            tilemap,
            textbox,
            font,
            text_name,
            text_words,
            money,
            shop,
        }
        
    }

    pub fn update(&mut self, camera: &mut Renderer, input: &Inputs, windowsize: (u32, u32), time: f32) {
        
        if !self.textbox.visible {
            
            self.player.movement(input);
            self.player.physics(&self.tilemap, time);
            self.shop.update(input, self.player.pos);
            
        }
        let speech = self.steve.update(self.player.pos, input, &mut self.textbox);
        if speech.is_some() {
            let s = speech.unwrap();
            self.text_name = s.1;
            self.text_words = s.0;
        }
        
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

    pub fn render(&mut self, renderer: &mut Renderer) {

        self.skybox.draw(renderer);
        self.town.draw(renderer);
        self.tilemap.draw(renderer);
        self.steve.draw(renderer);
        self.shop.draw(renderer);
        self.player.draw(renderer); 
        self.textbox.draw(renderer);
        self.money.draw(renderer);
        let mon = format!("{}", self.player.money);
        renderer.draw_font((38, 16), 15, &mon, self.font);
        
        if self.textbox.visible {
            
            renderer.draw_font((45, 175), 15, &self.text_name, &self.font);
            
            let mut h = 203;
            let mut temp = String::new();
            
            for i in self.text_words.chars() {
                

                if !(i == char::from_u32(10).unwrap() || i == ';') {
                    
                    temp = format!("{}{}", temp, i);
                    
                } else if temp != "" {
                    
                    renderer.draw_font((45, h), 12, &temp, &self.font);
                    h += 13;
                    temp = String::new();
                }
                
                if i == ';' {
                    self.steve.question = true;
                }
                
                if i == '§' {
                    self.steve.close = true;
                }
            }
        }
    }
}
