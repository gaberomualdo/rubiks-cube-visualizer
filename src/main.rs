//! An example of generating julia fractals.
extern crate image;
extern crate num_complex;

use image::Pixels;
use image::Pixel;

fn main() {
    use image::GenericImageView;
    let image_path = "img/cube_edited_test.jpg";

    let mut img = image::open(image_path).unwrap();

    let img_width = img.dimensions().0;
    let img_height = img.dimensions().1;

    for p in img.pixels() {
        println!("pixel: {:?}", p);
    }

    /*
    // image dimension variables
    let imgx = 800;
    let imgy = 600;

    // create new image buffer with dimensions of imgx and imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // write blue image background
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([0 as u8, 0, 255]);
    }

    // write lines
    for x in 0..imgx {
        for y in 0..imgy {
            if ((x / 5 + 2) % 5 == 0) {
                let pixel = imgbuf.get_pixel_mut(x, y);
                *pixel = image::Rgb([0, 255, 0]);
            }
        }
    }

    // save image
    imgbuf.save("fractal.png").unwrap();*/
}