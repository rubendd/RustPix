mod mylib;
use clap::{App, Arg};
use mylib::*;

fn main() {
    // Define the application's command-line interface
    let matches = App::new("Rustpix")
        .about("Just a simple image processing tool")
        .arg(
            Arg::with_name("infile")
                .required(true)
                .index(1)
                .help("Input image path"),
        )
        .arg(
            Arg::with_name("outfile")
                .required(true)
                .index(2)
                .help("Output image path"),
        )
        .arg(
            Arg::with_name("blur")
                .short("b")
                .long("blur")
                .value_name("radius")
                .help("Apply blur to the image"),
        )
        .arg(
            Arg::with_name("invert")
                .short("i")
                .long("invert")
                .help("Invert image colors"),
        )
        .arg(
            Arg::with_name("huerotate")
                .allow_hyphen_values(true)
                .short("r")
                .long("huerotate")
                .value_name("degrees")
                .help("Rotate the image clockwise"),
        )
        .arg(
            Arg::with_name("brighten")
                .allow_hyphen_values(true)
                .short("h")
                .long("brighten")
                .value_name("value")
                .help("Increase image brightness"),
        )
        .arg(
            Arg::with_name("cut")
                .short("c")
                .long("cut")
                .value_name("values")
                .multiple(true)
                .number_of_values(4)
                .help("Crop the image using a vector of values (x, y, width, height)"),
        )
        .arg(
            Arg::with_name("greyscale")
                .short("g")
                .long("greyscale")
                .help("Apply a grayscale filter to the image"),
        )
        .get_matches();

    // Get argument values
    let infile = matches.value_of("infile").unwrap().to_string();
    let outfile = matches.value_of("outfile").unwrap().to_string();
    let blur_radius = matches
        .value_of("blur")
        .map(|v| v.parse().unwrap())
        .unwrap_or(0);
    let invert_colors = matches.is_present("invert");
    let rotate_degrees = matches
        .value_of("huerotate")
        .map(|v| v.parse().unwrap())
        .unwrap_or(0);
    let brighten_value: i32 = matches
        .value_of("brighten")
        .map(|v| v.parse().unwrap())
        .unwrap_or(0);
    let grayscale = matches.is_present("greyscale");
    let cut_values: Option<Vec<u32>> = matches
        .values_of("cut")
        .map(|v| v.map(|val| val.parse().unwrap()).collect());

    // Check if the number of cut values is not equal to 4
    if let Some(values) = &cut_values {
        if values.len() != 4 {
            eprintln!("Error: Invalid number of cut values. The number of values must be 4.");
            std::process::exit(1);
        }
    }

    // Call the function to process the image
    generate(
        infile.as_str(),
        outfile.as_str(),
        blur_radius,
        brighten_value,
        &cut_values.unwrap_or_default(),
        invert_colors,
        rotate_degrees,
        grayscale,
    );
}
