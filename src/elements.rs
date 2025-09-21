pub struct Color;

impl Color {
    pub const WHITE: u32 = 0xFFFFFFFF;
    pub const BLACK: u32 = 0xFF000000;
    pub const RED: u32 = 0xFFFF0000;
    pub const GREEN: u32 = 0xFF00FF00;
    pub const BLUE: u32 = 0xFF0000FF;
    pub const TRANSPARENT: u32 = 0x00000000;

    pub fn from_argb(alpha: u8, red: u8, green: u8, blue: u8) -> u32 {
        (alpha as u32) << 24 | (red as u32) << 16 | (green as u32) << 8 | (blue as u32)
    }

    pub fn from_rgb(red: u8, green: u8, blue: u8) -> u32 {
        Self::from_argb(255, red, green, blue)
    }
}