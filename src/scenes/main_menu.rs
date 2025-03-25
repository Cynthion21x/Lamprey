use crate::{content::asset::AssetMan, game::{Inputs, State}, rendering::renderer::Renderer, ui::{photo, buttons}, utils::center_x, utils::tuple_add};

pub struct MainMenu<'a> {
    play_button: buttons::Button<'a>,
    play_button_down: buttons::Button<'a>,
    quit_button: buttons::Button<'a>,
    quit_button_d: buttons::Button::<'a>,
    title: photo::Photo<'a>,
    background: photo::Photo<'a>,
    pub new_state: State,
}

impl<'a> MainMenu<'a> {

    pub fn new(assets: &'a AssetMan) -> Self {
        
        let button = assets.sprite_from_string("rect_up").unwrap();
        let button_d = assets.sprite_from_string("rect_down").unwrap();
        
        let play_button = buttons::Button::new(button);
        let play_button_down = buttons::Button::new(button_d);
        
        let quit_button = buttons::Button::new( button);
        let quit_button_d = buttons::Button::new(button_d);
        
        let title = photo::Photo::new((0, 20), (223, 22), assets.sprite_from_string("Title").unwrap());
        let background = photo::Photo::new((0, 0), (0, 0), assets.sprite_from_string("titleScreen").unwrap()); 
               
        Self { 
            play_button,
            play_button_down,
            quit_button,
            quit_button_d,
            title,
            background,
            new_state: State::MainMenu,
        }
    }

    pub fn update(&mut self, input: &Inputs, windowsize: (u32, u32), scalar: f32) {
        

        self.title.size = ((223.0 * scalar) as u32, (22.0 * scalar) as u32);
        self.title.pos = center_x(self.title.size, windowsize, self.title.size.1);
        
        self.play_button.size = ((38.0 * scalar * 2.0) as u32, (15.0 * scalar * 2.0) as u32);
        self.play_button.pos = center_x(self.play_button.size, windowsize, windowsize.1 / 3);
        
        self.play_button_down.size = ((38.0 * scalar * 2.0) as u32, (12.0 * scalar * 2.0) as u32);
        self.play_button_down.pos = tuple_add(self.play_button.pos, (0, (6.0 * scalar) as u32));
        
        self.quit_button.size = ((38.0 * scalar * 2.0) as u32, (15.0 * scalar * 2.0) as u32);
        self.quit_button.pos = center_x(self.play_button.size, windowsize, windowsize.1 / 2);
        
        self.quit_button_d.size = ((38.0 * scalar * 2.0) as u32, (12.0 * scalar * 2.0) as u32);
        self.quit_button_d.pos = tuple_add(self.quit_button.pos, (0, (6.0 * scalar) as u32));
        
        if windowsize.0 > windowsize.1 * 16 / 9 {
            self.background.size = (windowsize.1 * 256 / 144, windowsize.1);
        } else {
            self.background.size = (windowsize.0, windowsize.0 * 144 / 256);            
        }

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
        
        if !self.play_button.pressed {
            self.play_button.draw(renderer);
        } else {
            self.play_button_down.draw(renderer);
        }
        
        if !self.quit_button.pressed {
            self.quit_button.draw(renderer);
        } else {
            self.quit_button_d.draw(renderer);
        }
        
        self.play_button.draw_text("PLAY", assets.sprite_from_string("black_font").unwrap(), renderer);
        self.quit_button.draw_text("QUIT", assets.sprite_from_string("black_font").unwrap(), renderer);
    }

}
