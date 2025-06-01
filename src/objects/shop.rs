use crate::{
    content::{asset::AssetMan, weapon::{self, Weapon}}, 
    objects::statics::Static, 
    renderer::Renderer, 
    utils::dist,
    game::Inputs,
};

use sdl2::keyboard::Keycode;

pub struct Shop<'a> {
    model: Static<'a>,
    items: (usize, usize, usize),
    weapons: Vec<Weapon<'a>>,
    inrange: bool,
}

impl<'a> Shop<'a> {

    
    pub fn new(model: Static<'a>, assets: &'a AssetMan<'a>) -> Self {
        
        let weapons = weapon::new(assets);
        
        
        
        Self {
           model,
           items: (
               rand::random_range(0..weapons.len() - 1), 
               rand::random_range(0..weapons.len() - 1), 
               rand::random_range(0..weapons.len() - 1), 
           ),
           weapons,
           inrange: false,
        }
        
    }
    
    pub fn update(&mut self, input: &Inputs, playerpos:(f32, f32)) {
        
        if dist((self.model.pos.0 + 33.0, self.model.pos.1 + 38.0), playerpos) <= 8.0 {
            
            self.inrange = true;
            
            if input.key_pressed == Some(Keycode::E) {
                
                
                
            }
            
        }
        
        if dist((self.model.pos.0 + 40.0, self.model.pos.1 + 38.0), playerpos) <= 8.0 {
            
            if input.key_pressed == Some(Keycode::E) {
                
            }
            
        }
        
        if dist((self.model.pos.0 + 86.0, self.model.pos.1 + 38.0), playerpos) <= 8.0 {
            
            if input.key_pressed == Some(Keycode::E) {
                
            }
            
        }
        
    }
    
    pub fn draw(&mut self, renderer: &mut Renderer) {
        
        self.model.draw(renderer);
        renderer.draw((self.model.pos.0 + 25.0, self.model.pos.1 + 30.0), (16, 16), self.weapons[self.items.0].sprite);
        renderer.draw((self.model.pos.0 + 52.0, self.model.pos.1 + 30.0), (16, 16), self.weapons[self.items.1].sprite);
        renderer.draw((self.model.pos.0 + 78.0, self.model.pos.1 + 30.0), (16, 16), self.weapons[self.items.2].sprite);
        
        if self.inrange {
            //renderer.draw_font((renderer.window.game_win_size().0 - 20), 15, "E to Interact", );
        }
        
    }
    
}