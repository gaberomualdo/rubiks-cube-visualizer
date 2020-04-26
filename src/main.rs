extern crate image;

use std::time::{Instant};
use std::thread::spawn;

fn to_replace(looking_for: [i32; 3], actual: [i32; 3]) -> bool {
    for index in 0..3 {
        let looking_for_val = looking_for[index];
        let actual_val = actual[index];
        
        let difference = looking_for_val - actual_val;
        let difference_abs = difference.abs();
        
        if difference_abs > 3 {
            return false;
        }
    }
    
    return true;
}

fn main() {
    use std::env;
    use image::GenericImageView;

    let start_time = Instant::now();

    let green: [u8; 3] = [99, 250, 119];
    let orange: [u8; 3] = [249, 128, 42];
    let blue: [u8; 3] = [58, 194, 254];
    let red: [u8; 3] = [236, 97, 100];
    let white: [u8; 3] = [227, 227, 219];
    let yellow: [u8; 3] = [203, 247, 76];

    let none: [u8; 3] = [0, 0, 0];

    let mut replace_values: [[[u8; 3]; 2]; 27] = [
        [none, [0, 0, 115]],
        [none, [0, 0, 108]],
        [none, [0, 0, 73]],
        [none, [0, 0, 122]],
        [none, [0, 0, 101]],
        [none, [0, 0, 80]],
        [none, [0, 0, 129]],
        [none, [0, 0, 94]],
        [none, [0, 0, 87]],

        [none, [0, 0, 136]],
        [none, [0, 0, 157]],
        [none, [0, 0, 178]],
        [none, [0, 0, 143]],
        [none, [0, 0, 164]],
        [none, [0, 0, 185]],
        [none, [0, 0, 150]],
        [none, [0, 0, 171]],
        [none, [0, 0, 192]],
        
        [none, [0, 0, 199]],
        [none, [0, 0, 206]],
        [none, [0, 0, 213]],
        [none, [0, 0, 220]],
        [none, [0, 0, 227]],
        [none, [0, 0, 234]],
        [none, [0, 0, 241]],
        [none, [0, 0, 248]],
        [none, [0, 0, 255]],
    ];

    eprintln!("elapsed after variables declared {:?}", start_time.elapsed());

    // update replace args to match cmd args
    let cmd_args: Vec<String> = env::args().collect();
    let mut replace_color_ind = 0;
    for color in cmd_args.iter() {
        
        if color == "green" {
            replace_values[replace_color_ind][0] = green;
            replace_color_ind += 1;
        } else if color == "orange" {
            replace_values[replace_color_ind][0] = orange;
            replace_color_ind += 1;
        } else if color == "blue" {
            replace_values[replace_color_ind][0] = blue;
            replace_color_ind += 1;
        } else if color == "red" {
            replace_values[replace_color_ind][0] = red;
            replace_color_ind += 1;
        } else if color == "white" {
            replace_values[replace_color_ind][0] = white;
            replace_color_ind += 1;
        } else if color == "yellow" {
            replace_values[replace_color_ind][0] = yellow;
            replace_color_ind += 1;
        }
    }

    eprintln!("elapsed after colors evaluated {:?}", start_time.elapsed());

    let source_img_path = "img/cube_sourceimage.jpg";

    let img = image::open(source_img_path).unwrap();

    let img_width = 600;
    let img_height = 400;

    let mut new_img_buffer = image::ImageBuffer::new(img_width, img_height);

    eprintln!("elapsed before image generation {:?}", start_time.elapsed());

    for pixel in img.pixels() {
        let (x, y, rgba) = pixel;
        let rgb: [u8; 3] = [rgba[0], rgba[1], rgba[2]];
        
        let mut new_pixel_value: [u8; 3] = rgb;
        
        if x > 160 && x < 446 && y > 70 && y < 362 {
            let rgb_as_i32: [i32; 3] = [i32::from(rgba[0]), i32::from(rgba[1]), i32::from(rgba[2])];
            
            for values in replace_values.iter() {
                let looking_for = [
                    i32::from(values[1][0]),
                    i32::from(values[1][1]),
                    i32::from(values[1][2]),
                ];
                    
                if to_replace(looking_for, rgb_as_i32) {
                    let replace_value = values[0];
                    new_pixel_value = replace_value;
                    break;
                }
            }
        }
        let new_img_pixel = new_img_buffer.get_pixel_mut(x, y);
        *new_img_pixel = image::Rgb(new_pixel_value);
    }

    eprintln!("after image generation {:?}", start_time.elapsed());

    // generate image unique ID (with piece values)
    let mut img_uid: [char; 27] = ['0'; 27];
    let mut cur_digit = 0;
    
    for color in cmd_args.iter() {
        if color == "green" {
            img_uid[cur_digit] = 'g';
            cur_digit += 1;
        } else if color == "orange" {
            img_uid[cur_digit] = 'o';
            cur_digit += 1;
        } else if color == "blue" {
            img_uid[cur_digit] = 'b';
            cur_digit += 1;
        } else if color == "red" {
            img_uid[cur_digit] = 'r';
            cur_digit += 1;
        } else if color == "white" {
            img_uid[cur_digit] = 'w';
            cur_digit += 1;
        } else if color == "yellow" {
            img_uid[cur_digit] = 'y';
            cur_digit += 1;
        }
    }
    eprintln!("elapsed end {:?}", start_time.elapsed());

    // save image
    new_img_buffer.save(format!("img/out/{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}.jpg", img_uid[0], img_uid[1], img_uid[2], img_uid[3], img_uid[4], img_uid[5], img_uid[6], img_uid[7], img_uid[8], img_uid[9], img_uid[10], img_uid[11], img_uid[12], img_uid[13], img_uid[14], img_uid[15], img_uid[16], img_uid[17], img_uid[18], img_uid[19], img_uid[20], img_uid[21], img_uid[22], img_uid[23], img_uid[24], img_uid[25], img_uid[26])).unwrap();

    eprintln!("elapsed saved {:?}", start_time.elapsed());
}
