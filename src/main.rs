extern crate image;
extern crate dict;

use std::time::{Instant};
use std::env;
use std::fs;

use dict::{ Dict, DictIface };
use image::GenericImageView;

pub fn path_exists(path: &str) -> bool {
    return fs::metadata(path).is_ok();
}

fn colors_are_similar(looking_for: [i32; 3], actual: [i32; 3]) -> bool {
    let max_val_difference = 3;
    
    for val_index in 0..3 {
        let looking_for_val = looking_for[val_index];
        let actual_val = actual[val_index];
        
        let val_difference = (looking_for_val - actual_val).abs();
        
        if val_difference > max_val_difference {
            return false;
        }
    }

    return true;
}

fn display_time_elapsed(title: &str, start_time: Instant) {
    println!(
        "{}{}  {:?}",
        std::iter::repeat(" ").take(83 - title.len()).collect::<String>().to_string(),
        title,
        start_time.elapsed()
    );
}

fn main() {
    let start_time = Instant::now();

    let mut is_large_img: bool = false;
    let mut display_run_times: bool = false;

    // colors
    let mut colors = Dict::<[u8; 3]>::new();
    colors.add( "green".to_string(), [99, 250, 119] );
    colors.add( "orange".to_string(), [249, 128, 42] );
    colors.add( "blue".to_string(), [58, 194, 254] );
    colors.add( "red".to_string(), [236, 97, 100] );
    colors.add( "white".to_string(), [227, 227, 219] );
    colors.add( "yellow".to_string(), [203, 247, 76] );

    // replace values array, each element has:
    //  - first index --> replacement color
    //  - second index --> color to replace (with replacement color)
    let placeholder_color: [u8; 3] = [0, 0, 0];

    let mut replace_values: [[[u8; 3]; 2]; 27] = [
        [placeholder_color, [0, 0, 115]],
        [placeholder_color, [0, 0, 108]],
        [placeholder_color, [0, 0, 73]],
        [placeholder_color, [0, 0, 122]],
        [placeholder_color, [0, 0, 101]],
        [placeholder_color, [0, 0, 80]],
        [placeholder_color, [0, 0, 129]],
        [placeholder_color, [0, 0, 94]],
        [placeholder_color, [0, 0, 87]],

        [placeholder_color, [0, 0, 136]],
        [placeholder_color, [0, 0, 157]],
        [placeholder_color, [0, 0, 178]],
        [placeholder_color, [0, 0, 143]],
        [placeholder_color, [0, 0, 164]],
        [placeholder_color, [0, 0, 185]],
        [placeholder_color, [0, 0, 150]],
        [placeholder_color, [0, 0, 171]],
        [placeholder_color, [0, 0, 192]],
        
        [placeholder_color, [0, 0, 199]],
        [placeholder_color, [0, 0, 206]],
        [placeholder_color, [0, 0, 213]],
        [placeholder_color, [0, 0, 220]],
        [placeholder_color, [0, 0, 227]],
        [placeholder_color, [0, 0, 234]],
        [placeholder_color, [0, 0, 241]],
        [placeholder_color, [0, 0, 248]],
        [placeholder_color, [0, 0, 255]],
    ];

    // add colors as customized in command line arguments
    // make is_large_image true if '--large' or '-L' flag is present
    // make display_run_times true if '--display-times' or '-T' flag is present
    // generate image UID from colors
    let command_line_args: Vec<String> = env::args().collect();
    let mut current_color_index = 0;

    // image UID is list of chars where each char is the first letter of the color (default 'p' --> placeholder)
    let mut img_uid: [char; 27] = ['p'; 27];

    for arg in command_line_args.iter() {
        let arg_lowercase_string = arg.to_lowercase();
        let arg_lowercase = arg_lowercase_string.as_str();

        if colors.contains_key( arg_lowercase ) {
            replace_values[current_color_index][0] = *colors.get(arg_lowercase).unwrap();
            img_uid[current_color_index] = arg_lowercase.chars().next().unwrap();
            current_color_index += 1;
        }
        
        if !is_large_img && (arg == "-TL" || arg == "-LT" || arg == "-L" || arg == "--large") {
            is_large_img = true;
        }
        
        if !display_run_times && (arg == "-TL" || arg == "-LT" || arg == "-T" || arg == "--display-times" ) {
            display_run_times = true;
        }
    }

    // create final image output path
    let out_img_filename = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}.jpg", img_uid[0], img_uid[1], img_uid[2], img_uid[3], img_uid[4], img_uid[5], img_uid[6], img_uid[7], img_uid[8], img_uid[9], img_uid[10], img_uid[11], img_uid[12], img_uid[13], img_uid[14], img_uid[15], img_uid[16], img_uid[17], img_uid[18], img_uid[19], img_uid[20], img_uid[21], img_uid[22], img_uid[23], img_uid[24], img_uid[25], img_uid[26]);
    let mut out_img_directory = "img/out/small/";

    if is_large_img {
        out_img_directory = "img/out/large/";
    }

    let out_img_path = format!("{}{}", out_img_directory, out_img_filename);

    if path_exists(&out_img_path.to_string()) {
        println!("Output Image Already Exists");
        return;
    }

    if display_run_times {
        display_time_elapsed("Variables Declared, Command Line Arguments Evaluated, and Output Image Path Created", start_time);
    }

    // source path and dimensions variables, taking into account whether image is large
    let mut is_large_img_multiplier = 1;
    let mut source_img_path = "img/cube_sourceimage_small.jpg";

    if is_large_img {
        is_large_img_multiplier = 2;
        source_img_path = "img/cube_sourceimage.jpg";
    }
    
    // dimensions are prearranged values from source image
    let img_width = 300 * is_large_img_multiplier;
    let img_height = 200 * is_large_img_multiplier;

    // get source image and create new image buffer
    let img = image::open(source_img_path).unwrap();
    let mut new_img_buffer = image::ImageBuffer::new(img_width, img_height);

    if display_run_times {
        display_time_elapsed("Image Variables Created, Got Source Image, and Created New Image Buffer", start_time);
    }

    // loop through pixels and replace colors if applicable
    for pixel in img.pixels() {
        let (x, y, rgba) = pixel;
        let rgb: [u8; 3] = [rgba[0], rgba[1], rgba[2]];
        
        let mut new_pixel_value: [u8; 3] = rgb;
        
        // only check if one the possible pixels in the cube (these are prearranged values from source image)
        if x > (80 * is_large_img_multiplier) && x < (223 * is_large_img_multiplier) && y > (35 * is_large_img_multiplier) && y < (181 * is_large_img_multiplier) {
            let rgb_as_i32: [i32; 3] = [i32::from(rgba[0]), i32::from(rgba[1]), i32::from(rgba[2])];
            
            // check every possible replace value
            for values in replace_values.iter() {
                let looking_for = [
                    i32::from(values[1][0]),
                    i32::from(values[1][1]),
                    i32::from(values[1][2]),
                ];
                    
                if colors_are_similar(looking_for, rgb_as_i32) {
                    let replace_value = values[0];
                    new_pixel_value = replace_value;
                    break;
                }
            }
        }

        // get current pixel mutator and put new pixel value
        let new_img_pixel = new_img_buffer.get_pixel_mut(x, y);
        *new_img_pixel = image::Rgb(new_pixel_value);
    }

    if display_run_times {
        display_time_elapsed("New Image Buffer Pixels Populated and Generated", start_time);
    }

    // save image
    new_img_buffer.save(out_img_path).unwrap();

    if display_run_times {
        display_time_elapsed("New Image Saved", start_time);
    }
}
