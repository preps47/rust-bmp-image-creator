use std::fs::File;
use std::io::Write;

/// Base struct to create a .bmp image.
/// It will incorporate the width and the height of the image, as well as the horizontal and the vertical pixel per meter.
pub struct BMPImage {
    width: i32,
    height: i32,
    horizontal_ppm: i32,
    vertical_ppm: i32,
    bitmap: Vec<Vec<u32>>
}

impl BMPImage {
    /// Creates a new BMPImage struct with a single color background.
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
            bitmap: vec![vec![background_color; width as usize]; height as usize]
        }
    }

    /// Creates a file with the necessary headers for a standard .bmp image with 32 bits pixel's color information.
    pub fn init_headers(&self, path: &str) -> std::io::Result<File> {
        let mut image = File::create(path)?;
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

    /// Writes the bitmap stored in a file.
    /// Better used with ```init_header()``` function so that in result you'll have a complete .bmp image.
    pub fn write_bitmap(&self, image: &mut File) -> std::io::Result<()> {
        let bitmap: Vec<u8> = self.bitmap.iter()
            .flat_map(|row| row.into_iter())
            .flat_map(|pixel| pixel.to_le_bytes().into_iter())
            .collect();

        image.write_all(bitmap.as_slice())
    }

    /// Sets a pixel in the bitmap to a color
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width as usize && y < self.height as usize {
            self.bitmap[y][x] = color
        }
    }

    /// Draws a line from two points in the bitmap using the Bresenham's line algorithm.
    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
        if (x0 + x1) < self.width as usize && (y0 + y1) < self.height as usize {
            let mut x0 = x0 as isize;
            let mut y0 = y0 as isize;
            let x1 = x1 as isize;
            let y1 = y1 as isize;

            let dx = (x1 - x0).abs();
            let dy = -(y1 - y0).abs();
            let mut error = dx + dy;

            let sx = if x0 < x1 { 1 } else { -1 };
            let sy = if y0 < y1 { 1 } else { -1 };

            loop {
                self.set_pixel(x0 as usize, y0 as usize, color);

                let e2 = 2 * error;
                if e2 >= dy {
                    if x0 == x1 {
                        break;
                    }
                    error += dy;
                    x0 += sx;
                }
                if e2 <= dx {
                    if y0 == y1 {
                        break;
                    }
                    error += dx;
                    y0 += sy;
                }
            }
        }
    }

    /// Draws a circle from a center and a radius using the midpoint circle algorithm.
    pub fn draw_circle(&mut self, cx: usize, cy: usize, radius: usize, color: u32) {
        if radius == 0 {
            self.set_pixel(cx, cy, color);
        } else {
            let mut x = 0;
            let mut y = radius as isize;
            let mut d = 3 - 2 * radius as isize;

            while x <= y as usize {
                self.draw_eight_points(cx, cy, x, y as usize, color);

                if d > 0 {
                    y -= 1;
                    d += 4 * (x as isize - y) + 10;
                } else {
                    d += 4 * x as isize + 6;
                }

                x += 1
            }
        }
    }

    /// Helper method for the ```draw_circle()``` function.
    /// Draws eight points for each iteration, reducing the processing time.
    fn draw_eight_points(&mut self, cx: usize, cy: usize, x: usize, y: usize, color: u32) {
        let points = [
            (cx + x, cy + y), (cx - x, cy - y),
            (cx + x, cy - y), (cx - x, cy + y),
            (cx + y, cy + x), (cx - y, cy - x),
            (cx + y, cy - x), (cx - y, cy + x)
        ];

        for (px, py) in points {
            self.set_pixel(px, py, color)
        }
    }

    pub fn apply_on_x<I>(&mut self, f: impl Fn(usize) -> I)
    where
        I: IntoIterator<Item = (usize, u32)>
    {
        for x in 0..self.width as usize {
            for (y, color) in f(x) {
                if y < self.height as usize {
                    self.set_pixel(x, y, color);
                }
            }
        }
    }

    pub fn apply_on_y<I>(&mut self, f: impl Fn(usize) -> I)
    where
        I: IntoIterator<Item = (usize, u32)>
    {
        for y in 0..self.height as usize {
            for (x, color) in f(y) {
                if x < self.width as usize {
                    self.set_pixel(x, y, color);
                }
            }
        }
    }
}
