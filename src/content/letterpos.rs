use sdl2::rect::Rect;

const ABCD: char = char::from_u32(27).unwrap();
const BACKSLASH: char = char::from_u32(92).unwrap();

pub fn letterpos(letter: &char) -> Rect {
    match letter {
        ' ' => {Rect::new(1, 1, 7, 9)},
        '!' => {Rect::new(8, 1, 7, 9)},
        '"' => {Rect::new(15, 1, 7, 9)},
        '#' => {Rect::new(22, 1, 7, 9)},
        '$' => {Rect::new(29, 1, 7, 9)},
        '%' => {Rect::new(36, 1, 7, 9)},
        '&' => {Rect::new(43, 1, 7, 9)},
        &ABCD => {Rect::new(50, 1, 7, 9)},
        '(' => {Rect::new(57, 1, 7, 9)},
        ')' => {Rect::new(64, 1, 7, 9)},
        '*' => {Rect::new(71, 1, 7, 9)},
        '+' => {Rect::new(78, 1, 7, 9)},
        ',' => {Rect::new(85, 1, 7, 9)},
        '-' => {Rect::new(92, 1, 7, 9)},
        '.' => {Rect::new(99, 1, 7, 9)},
        '/' => {Rect::new(106, 1, 7, 9)},
        '0' => {Rect::new(113, 1, 7, 9)},
        '1' => {Rect::new(120, 1, 7, 9)},
        '2' => {Rect::new(1, 10, 7, 9)},
        '3' => {Rect::new(8, 10, 7, 9)},
        '4' => {Rect::new(15, 10, 7, 9)},
        '5' => {Rect::new(22, 10, 7, 9)},
        '6' => {Rect::new(29, 10, 7, 9)},
        '7' => {Rect::new(36, 10, 7, 9)},
        '8' => {Rect::new(43, 10, 7, 9)},
        '9' => {Rect::new(50, 10, 7, 9)},
        ':' => {Rect::new(57, 10, 7, 9)},
        ';' => {Rect::new(64, 10, 7, 9)},
        '<' => {Rect::new(71, 10, 7, 9)},
        '=' => {Rect::new(78, 10, 7, 9)},
        '>' => {Rect::new(85, 10, 7, 9)},
        '?' => {Rect::new(92, 10, 7, 9)},
        '@' => {Rect::new(99, 10, 7, 9)},
        'A' => {Rect::new(106, 10, 7, 9)},
        'B' => {Rect::new(113, 10, 7, 9)},
        'C' => {Rect::new(120, 10, 7, 9)},        
        'D' => {Rect::new(1, 19, 7, 9)},
        'E' => {Rect::new(8, 19, 7, 9)},
        'F' => {Rect::new(15, 19, 7, 9)},
        'G' => {Rect::new(22, 19, 7, 9)},
        'H' => {Rect::new(29, 19, 7, 9)},
        'I' => {Rect::new(36, 19, 7, 9)},
        'J' => {Rect::new(43, 19, 7, 9)},
        'K' => {Rect::new(50, 19, 7, 9)},
        'L' => {Rect::new(57, 19, 7, 9)},
        'M' => {Rect::new(64, 19, 7, 9)},
        'N' => {Rect::new(71, 19, 7, 9)},
        'O' => {Rect::new(78, 19, 7, 9)},
        'P' => {Rect::new(85, 19, 7, 9)},
        'Q' => {Rect::new(92, 19, 7, 9)},
        'R' => {Rect::new(99, 19, 7, 9)},
        'S' => {Rect::new(106, 19, 7, 9)},
        'T' => {Rect::new(113, 19, 7, 9)},
        'U' => {Rect::new(120, 19, 7, 9)},
        'V' => {Rect::new(1, 28, 7, 9)},
        'W' => {Rect::new(8, 28, 7, 9)},
        'X' => {Rect::new(15, 28, 7, 9)},
        'Y' => {Rect::new(22, 28, 7, 9)},
        'Z' => {Rect::new(29, 28, 7, 9)},
        '[' => {Rect::new(36, 28, 7, 9)},
        &BACKSLASH => {Rect::new(43, 28, 7, 9)},
        ']' => {Rect::new(50, 28, 7, 9)},
        '^' => {Rect::new(57, 28, 7, 9)},
        '_' => {Rect::new(64, 28, 7, 9)},
        '`' => {Rect::new(71, 28, 7, 9)},
        'a' => {Rect::new(78, 28, 7, 9)},
        'b' => {Rect::new(85, 28, 7, 9)},
        'c' => {Rect::new(92, 28, 7, 9)},
        'd' => {Rect::new(99, 28, 7, 9)},
        'e' => {Rect::new(106, 28, 7, 9)},
        'f' => {Rect::new(113, 28, 7, 9)},
        'g' => {Rect::new(120, 28, 7, 9)},
        'h' => {Rect::new(1, 37, 7, 9)},
        'i' => {Rect::new(8, 37, 7, 9)},
        'j' => {Rect::new(15, 37, 7, 9)},
        'k' => {Rect::new(22, 37, 7, 9)},
        'l' => {Rect::new(29, 37, 7, 9)},
        'm' => {Rect::new(36, 37, 7, 9)},
        'n' => {Rect::new(43, 37, 7, 9)},
        'o' => {Rect::new(50, 37, 7, 9)},
        'p' => {Rect::new(57, 37, 7, 9)},
        'q' => {Rect::new(64, 37, 7, 9)},
        'r' => {Rect::new(71, 37, 7, 9)},
        's' => {Rect::new(78, 37, 7, 9)},
        't' => {Rect::new(85, 37, 7, 9)},
        'u' => {Rect::new(92, 37, 7, 9)},
        'v' => {Rect::new(99, 37, 7, 9)},
        'w' => {Rect::new(106, 37, 7, 9)},
        'x' => {Rect::new(113, 37, 7, 9)},
        'y' => {Rect::new(120, 37, 7, 9)},
        'z' => {Rect::new(1, 46, 7, 9)},
        '{' => {Rect::new(8, 46, 7, 9)},
        '|' => {Rect::new(15, 46, 7, 9)},
        '}' => {Rect::new(22, 46, 7, 9)},
        '~' => {Rect::new(29, 46, 7, 9)},
        _ => {Rect::new(1, 1, 7, 9)},
    }
}