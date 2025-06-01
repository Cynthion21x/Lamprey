use sdl2::render::Texture;

use super::asset::AssetMan;

#[derive(PartialEq)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary
}

pub struct Weapon<'a> {
    
    pub name: &'a str,
    pub rarity: Rarity,
    pub sprite: &'a Texture<'a>,
    pub stats: &'a str,
    pub lore: &'a str

}
    
pub fn new<'a>(assets: &'a AssetMan<'a>) -> Vec<Weapon<'a>> {
        
    let mut weapon = Vec::new();
        
    weapon.push(
        Weapon {
            name: "Spear",
            rarity: Rarity::Common,
            sprite: assets.sfs("item").unwrap(),
            stats: "Damage: 10, Attack Speed: 4",
            lore: "the best spear in all the land"
        }
    );
    
    weapon.push(
        Weapon {
            name: "Grenade",
            rarity: Rarity::Uncommon,
            sprite: assets.sfs("item").unwrap(),
            stats: "Damage: 15, Attack Speed: 2",
            lore: "ball go boom boom"
        }
    );
    
    weapon.push(
        Weapon {
            name: "zenith",
            rarity: Rarity::Legendary,
            sprite: assets.sfs("item").unwrap(),
            stats: "Damage: 105, Attack Speed: 12",
            lore: "not a terraria reference"
        }
    );
    
    weapon
        
}
