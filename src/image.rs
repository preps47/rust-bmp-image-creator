use std::fs::File;
use std::io::Write;

pub struct BMPImage {
    width: i32,
    height: i32,
    horizontal_ppm: i32,
    vertical_ppm: i32,
    bitmap: Vec<Vec<u32>>
}

impl BMPImage {
    pub fn new(
        width: i32,
        height: i32,
        horizontal_ppm: i32,
        vertical_ppm: i32,
        background_color: u32
    ) -> BMPImage {
        BMPImage {
            width,
            height,
            horizontal_ppm,
            vertical_ppm,
            bitmap: vec![vec![background_color; height as usize]; width as usize]
        }
    }

    pub fn init_headers(&self) -> std::io::Result<File> {
        let mut image = File::create("image.bmp")?;
        let mut header: Vec<u8> = vec![];

        // Header (14 bytes)
        header.extend_from_slice(&[0x42, 0x4d]); // "BM" signature
        header.extend_from_slice(&(54 + 4 * self.width * self.height).to_le_bytes()); // File size
        header.extend_from_slice(&0u16.to_le_bytes()); // Reserved (0)
        header.extend_from_slice(&0u16.to_le_bytes()); // Reserved (0)
        header.extend_from_slice(&54u32.to_le_bytes()); // Pixel array offset

        // DIB header (40 bytes)
        header.extend_from_slice(&40u32.to_le_bytes()); // DIB header size
        header.extend_from_slice(&self.width.to_le_bytes()); // Width
        header.extend_from_slice(&self.height.to_le_bytes()); // Height
        header.extend_from_slice(&1u16.to_le_bytes()); // Color planes (1)
        header.extend_from_slice(&32u16.to_le_bytes()); // Bits per pixel
        header.extend_from_slice(&0u32.to_le_bytes()); // Compression type
        header.extend_from_slice(&(4 * self.width * self.height).to_le_bytes()); // Image size in bytes
        header.extend_from_slice(&self.horizontal_ppm.to_le_bytes()); // Horizontal PPM
        header.extend_from_slice(&self.vertical_ppm.to_le_bytes()); // Vertical PPM
        header.extend_from_slice(&0u32.to_le_bytes()); // Number of colors in palette
        header.extend_from_slice(&0u32.to_le_bytes()); // Number of important colors

        image.write_all(header.as_slice())?;

        Ok(image)
    }

    pub fn write_bitmap(&self, image: &mut File) -> std::io::Result<()> {
        let bitmap: Vec<u8> = self.bitmap.iter()
            .flat_map(|row| row.into_iter())
            .flat_map(|pixel| pixel.to_le_bytes().into_iter())
            .collect();

        image.write_all(bitmap.as_slice())
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width as usize && y < self.height as usize {
            self.bitmap[x][y] = color
        }
    }

    // Bresenham's line algorithm
    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
        let mut x0 = x0 as isize;
        let mut y0 = y0 as isize;
        let x1 = x1 as isize;
        let y1 = y1 as isize;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let mut error = 2 * (dx + dy);

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        while error >= dy && x0 == x1 || error <= dx && y0 == y1 {
            self.set_pixel(x0 as usize, y0 as usize, color);

            if error >= dy {
                error += dy;
                x0 += sx;
            }
            if  error <= dx {
                error += dx;
                y0 += sy;
            }
        }
    }
}