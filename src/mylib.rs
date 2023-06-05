use image::{DynamicImage};

/// Process the image with the specified parameters and save the result.
///
/// # Arguments
///
/// * `infile` - The path to the input image file.
/// * `outfile` - The path to save the output image file.
/// * `blur_v` - The blur radius to apply to the image.
/// * `bright_value` - The brightness adjustment value for the image.
/// * `crop_values` - The crop values for the image (x, y, width, height).
/// * `invert` - A flag indicating whether to invert the colors of the image.
/// * `huerotate` - The number of degrees to rotate the image clockwise.
/// * `gray_scale` - A flag indicating whether to apply a grayscale filter to the image.
///
/// # Panics
///
/// This function panics if there is an error opening the input image file or saving the output image file.
pub fn generate(infile: &str, outfile: &str, blur_v: u32, bright_value: i32, crop_values: &[u32], invert: bool, huerotate: i32, gray_scale: bool) {
    // Open the input image file
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    
    println!("Processing...");

    // Crop the image if crop values are provided
    if !crop_values.is_empty() {
        let (x, y, width, height) = extract_crop_values(crop_values);
        img = crop_image(img, x, y, width, height);
    }

    // Invert the colors of the image if the invert flag is set
    if invert {
        invert_colors(&mut img);
    }

    // Apply grayscale filter to the image if the gray_scale flag is set
    if gray_scale {
        img = convert_to_grayscale(&img);
    }

    // Apply image effects (blur, brightness, rotation)
    let processed_img = apply_image_effects(&img, blur_v, bright_value, huerotate);
    
    // Save the processed image to the output file
    save_image(processed_img, outfile).expect("Failed to save image.");
}

/// Extract the crop values (x, y, width, height) from the given slice.
///
/// # Arguments
///
/// * `values` - The slice containing the crop values.
///
/// # Panics
///
/// This function panics if the number of crop values is not equal to 4.
fn extract_crop_values(values: &[u32]) -> (u32, u32, u32, u32) {
    if values.len() != 4 {
        panic!("Error: Invalid number of crop values. The number of values must be 4.");
    }
    (values[0], values[1], values[2], values[3])
}

/// Crop the given image based on the specified coordinates and dimensions.
///
/// # Arguments
///
/// * `img` - The input image to crop.
/// * `x` - The x-coordinate of the top-left corner of the crop area.
/// * `y` - The y-coordinate of the top-left corner of the crop area.
/// * `width` - The width of the crop area.
/// * `height` - The height of the crop area.
///
/// # Returns
///
/// The cropped image.
fn crop_image(mut img: DynamicImage, x: u32, y: u32, width: u32, height: u32) -> DynamicImage {
    img.crop(x, y, width, height)
}

/// Invert the colors of the given image in-place.
///
/// # Arguments
///
/// * `img` - The image to invert the colors of.
fn invert_colors(img: &mut DynamicImage) {
    img.invert();
}

/// Convert the given image to grayscale.
///
/// # Arguments
///
/// * `img` - The image to convert to grayscale.
///
/// # Returns
///
/// The grayscale version of the image.
fn convert_to_grayscale(img: &DynamicImage) -> DynamicImage {
    img.grayscale()
}

/// Apply image effects (blur, brightness, rotation) to the given image.
///
/// # Arguments
///
/// * `img` - The input image to apply effects to.
/// * `blur_v` - The blur radius.
/// * `bright_value` - The brightness adjustment value.
/// * `huerotate` - The number of degrees to rotate each pixel.
///
/// # Returns
///
/// The processed image.
fn apply_image_effects(img: &DynamicImage, blur_v: u32, bright_value: i32, huerotate: i32) -> DynamicImage {
    let processed_img = img
        .blur(blur_v as f32)
        .brighten(bright_value as i32)
        .huerotate(huerotate as i32);
    processed_img
}

/// Save the given image to the specified output file.
///
/// # Arguments
///
/// * `img` - The image to save.
/// * `outfile` - The path to save the image file.
///
/// # Returns
///
/// An `image::ImageResult` indicating the result of the save operation.
fn save_image(img: DynamicImage, outfile: &str) -> image::ImageResult<()> {
    img.save(outfile)
}
