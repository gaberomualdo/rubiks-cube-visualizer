//! An example of generating julia fractals.
extern crate image;

// use std::fs::OpenOptions;
// use std::io::prelude::*;

fn to_replace(looking_for: [i32; 3], actual: [i32; 3]) -> bool {
    let mut to_replace = true;

    for index in 0..3 {
        let looking_for_val = looking_for[index];
        let actual_val = actual[index];

        let difference = looking_for_val - actual_val;
        let difference_abs = difference.abs();

        if difference_abs > 4 {
            to_replace = false;
        }
    }

    return to_replace;
}

fn main() {
    use image::GenericImageView;

    let replace_values: [[[u8; 3]; 2]; 27] = [
        [[255, 160, 122], [0, 0, 73]],
        [[250, 128, 114], [0, 0, 80]],
        [[233, 150, 122], [0, 0, 87]],
        [[240, 128, 128], [0, 0, 94]],
        [[205, 92, 92], [0, 0, 101]],
        [[220, 20, 60], [0, 0, 108]],
        [[178, 34, 34], [0, 0, 115]],
        [[255, 0, 0], [0, 0, 122]],
        [[139, 0, 0], [0, 0, 129]],
        [[255, 127, 80], [0, 0, 136]],
        [[255, 99, 71], [0, 0, 143]],
        [[255, 69, 0], [0, 0, 150]],
        [[255, 215, 0], [0, 0, 157]],
        [[255, 165, 0], [0, 0, 164]],
        [[255, 140, 0], [0, 0, 171]],
        [[255, 255, 224], [0, 0, 178]],
        [[255, 250, 205], [0, 0, 185]],
        [[250, 250, 210], [0, 0, 192]],
        [[255, 239, 213], [0, 0, 199]],
        [[255, 228, 181], [0, 0, 206]],
        [[255, 218, 185], [0, 0, 213]],
        [[238, 232, 170], [0, 0, 220]],
        [[240, 230, 140], [0, 0, 227]],
        [[189, 183, 107], [0, 0, 234]],
        [[255, 255, 0], [0, 0, 241]],
        [[124, 252, 0], [0, 0, 248]],
        [[127, 255, 0], [0, 0, 255]],
    ];

    // let mut file = OpenOptions::new()
    //     .write(true)
    //     .append(true)
    //     .open("output.txt")
    //     .unwrap();

    let source_img_path = "img/cube_sourceimage.jpg";

    let img = image::open(source_img_path).unwrap();

    let img_width = img.dimensions().0;
    let img_height = img.dimensions().1;

    // create new image buffer with dimensions
    let mut new_img_buffer = image::ImageBuffer::new(img_width, img_height);

    for pixel in img.pixels() {
        let (x, y, rgba) = pixel;
        let rgb: [u8; 3] = [rgba[0], rgba[1], rgba[2]];
        let rgb_as_i32: [i32; 3] = [i32::from(rgba[0]), i32::from(rgba[1]), i32::from(rgba[2])];

        let mut new_pixel_value: [u8; 3] = rgb;

        for values in replace_values.iter() {
            let replace_value = values[0];
            let looking_for = [
                i32::from(values[1][0]),
                i32::from(values[1][1]),
                i32::from(values[1][2]),
            ];

            if to_replace(looking_for, rgb_as_i32) {
                new_pixel_value = replace_value;
            }
        }

        let new_img_pixel = new_img_buffer.get_pixel_mut(x, y);

        *new_img_pixel = image::Rgb(new_pixel_value);

        // if let Err(e) = writeln!(file, "{:?}" , (p)) {
        //     eprintln!("Couldn't write to file: {}", e);
        // }
    }

    // save image
    new_img_buffer.save("out.png").unwrap();
}
