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
  let mut image = bmp_image.init_headers().unwrap();
</pre>
<p>And then you can finish the image by just typing:</p>
<pre>
  bmp_image.write_bitmap(&mut image).unwrap();
</pre>
