use std::fs::File;
use std::io::Write;

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

pub struct BMPBitmap {
    pub bitmap: Vec<Vec<u32>>    
}

impl BMPBitmap {
    pub fn new(width: i32, height: i32) -> BMPBitmap {
        let mut bitmap: Vec<Vec<u32>> = vec![];
        
        for _ in 0..width {
            let mut row: Vec<u32> = vec![];
            for _ in 0..height {
                row.push(Color::WHITE);
            }
            
            bitmap.push(row);
        }
        
        BMPBitmap { bitmap }
    }
}

pub struct BMPImage {
    width: i32,
    height: i32,
    horizontal_ppm: i32,
    vertical_ppm: i32
}

impl BMPImage {
    pub fn new(
        width: i32,
        height: i32,
        horizontal_ppm: i32,
        vertical_ppm: i32
    ) -> BMPImage {
        BMPImage {
            width,
            height,
            horizontal_ppm,
            vertical_ppm
        }
    }

    pub fn init_headers(&self) -> std::io::Result<File> {
        let mut image = File::create("image.bmp")?;
        let mut header: Vec<u8> = vec![];

        // Header (14 bytes)
        header.extend_from_slice(&[0x42, 0x4d]); // "BM" signature
        header.extend_from_slice(&(54 + 32 * self.width * self.height).to_le_bytes()); // File size
        header.extend_from_slice(&0u16.to_le_bytes()); // Reserved (0)
        header.extend_from_slice(&0u16.to_le_bytes()); // Reserved (0)
        header.extend_from_slice(&54u32.to_le_bytes()); // Pixel array offset

        // DIB header (40 bytes)
        header.extend_from_slice(&56u32.to_le_bytes()); // DIB header size
        header.extend_from_slice(&self.width.to_le_bytes()); // Width
        header.extend_from_slice(&self.height.to_le_bytes()); // Height
        header.extend_from_slice(&1u16.to_le_bytes()); // Color planes (1)
        header.extend_from_slice(&32u16.to_le_bytes()); // Bits per pixel
        header.extend_from_slice(&0u32.to_le_bytes()); // Compression type
        header.extend_from_slice(&(self.width * self.height * 4).to_le_bytes()); // Image size in bytes
        header.extend_from_slice(&self.horizontal_ppm.to_le_bytes()); // Horizontal PPM
        header.extend_from_slice(&self.vertical_ppm.to_le_bytes()); // Vertical PPM
        header.extend_from_slice(&0u32.to_le_bytes()); // Number of colors in palette
        header.extend_from_slice(&0u32.to_le_bytes()); // Number of important colors
        header.extend_from_slice(&0u16.to_le_bytes()); // Units for horizontal and vertical resolution
        header.extend_from_slice(&0u16.to_le_bytes()); // Padding (0)
        header.extend_from_slice(&0u16.to_le_bytes()); // Fill direction (low-left corner)
        header.extend_from_slice(&0u16.to_le_bytes()); // Halftoning algorithm
        header.extend_from_slice(&0u32.to_le_bytes()); // Halftoning parameter 1
        header.extend_from_slice(&0u32.to_le_bytes()); // Halftoning parameter 2
        header.extend_from_slice(&0u32.to_le_bytes()); // Color encoding
    
        image.write_all(header.as_slice())?;
        
        Ok(image)
    }
    
    pub fn create_table(&self) -> BMPBitmap {
        BMPBitmap::new(self.width, self.height)
    }
    
    pub fn write_bitmap(&self, image: &mut File, bmp_bitmap: BMPBitmap) -> std::io::Result<()> {
        let bitmap: Vec<u8> = bmp_bitmap.bitmap.into_iter()
            .flat_map(|row| row.into_iter())
            .flat_map(|pixel| pixel.to_le_bytes().into_iter())
            .collect();
        
        image.write_all(bitmap.as_slice())
    }
}
