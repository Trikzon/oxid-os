#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,          // §0
    Blue = 1,           // §1
    Green = 2,          // §2
    Cyan = 3,           // §3
    Red = 4,            // §4
    Magenta = 5,        // §5
    Brown = 6,          // §6
    LightGray = 7,      // §7
    DarkGray = 8,       // §8
    LightBlue = 9,      // §9
    LightGreen = 10,    // §a
    LightCyan = 11,     // §b
    LightRed = 12,      // §c
    Pink = 13,          // §d
    Yellow = 14,        // §e
    White = 15,         // §f
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CharColor(u8);

impl CharColor {
    pub fn new(foreground: Color, background: Color) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }

    pub fn set_foreground(&mut self, foreground: Color) {
        self.0 = (self.0 & 0xF0) | (foreground as u8);
    }

    pub fn set_background(&mut self, background: Color) {
        self.0 = (background as u8) << 4 | self.0;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Char {
    pub ibm437_character: u8,
    pub char_color: CharColor,
}

impl Char {
    pub fn new(ibm437_character: u8, char_color: CharColor) -> Self {
        Self { ibm437_character, char_color }
    }
}
