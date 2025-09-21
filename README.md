<h1>Rust BMP Image Creator Library</h1>
<p>A simple library used to create <i>.bmp</i> images from zero in rust.</p>
<hr>
<p>To create a new .bmp file you first need a BMPImage object:</p>
<pre>
use bmp_iamge::image::BMPImage;
use bmp_image::elements::Color;
<bl>
let width: i32 = 256;
let height: i32 = 256;
let horizontal_ppm: i32 = 1000; 
let vertical_ppm: i32 = 1000; 
<bl> 
let mut bmp_image = BMPImage::new( 
    width,
    height,
    horizontal_ppm,
    vertical_ppm,
    Color::WHITE
);
</pre>
<p>Now just create a new file with the specification you typed earlier:</p>
<pre>
let mut image = bmp_image.init_headers("image.bmp").unwrap();
</pre>
<p>And then you can finish the image by just typing:</p>
<pre>
bmp_image.write_bitmap(&mut image).unwrap();
</pre>
<hr>
<p>Here a list of the methods you can use to build shapes into your image:</p>
<table> 
<tr>
<th>Name</th>
<th>Parameters</th>
<th>Description</th>
</tr>
<tr>
<td>draw_line</td>
<td>x0: usize, y0: usize, x1: usize, y1: usize, color: u32</td>
<td>Draws a line between two points: p0(x0, y0) and p1(x1, y1)</td>
</tr>
<tr>
<td>draw_circle</td>
<td>cx: usize, cy: usize, radius: usize, color: u32</td>
<td>Draws a circle given a center (cx, cy) and a radius</td>
</tr>
</table>