mod config;

use tcod::colors::*;
use tcod::console::*;

fn main() {
    tcod::system::set_fps(config::FPS);

    let mut root = Root::initializer()
        .font(config::FONT, FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(config::GAME_WIDTH, config::GAME_HEIGHT)
        .title(config::TITLE)
        .init();

    while !root.window_closed() {
        root.set_default_foreground(WHITE);
        root.clear();
        root.put_char(1, 1, '@', BackgroundFlag::None);
        root.flush();
        root.wait_for_keypress(true);
    }
}

