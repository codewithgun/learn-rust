enum Color {
    Red,
    Blue,
    Green,
    RgbColor(u8, u8, u8), //tuple
    CmkyColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    }, // struct
}

pub fn test_enumeration() {
    // let my_color_choice: Color = Color::RgbColor(255, 255, 255);
    let my_color_choice: Color = Color::CmkyColor {
        cyan: 0,
        magenta: 0,
        yellow: 0,
        black: 255,
    };
    match my_color_choice {
        Color::Red => print!("Red"),
        Color::RgbColor(255, 255, 255) => println!("Black rgb"),
        Color::CmkyColor {
            cyan: _,
            magenta: _,
            yellow: _,
            black: 255,
        } => println!("Black cmky while don't care other color"),
        _ => print!("Unknown color"),
    }
}
