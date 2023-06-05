# RustPix


RustPix is a simple image processing tool built with Rust. It provides a command-line interface for applying various image effects and transformations to input images.

## Features

- Blur: Apply blur effect to the image.
- Invert Colors: Invert the colors of the image.
- Hue Rotation: Rotate the color of the pixels by a specified number of degrees.
- Brightness Adjustment: Increase or decrease the brightness of the image.
- Grayscale: Apply a grayscale filter to the image.
- Crop: Crop the image using specified coordinates and dimensions.

## Usage

To use RustPix, simply run the following command:

rustpix <input_image> <output_image> [options]


Replace `<input_image>` with the path to the input image file and `<output_image>` with the desired path for the processed image.

### Options

- `-b, --blur <radius>`: Apply blur effect with the specified radius.
- `-i, --invert`: Invert the colors of the image.
- `-r, --rotate <degrees>`: Rotate the color of the pixels by the specified number of degrees.
- `-h, --brightness <value>`: Adjust the brightness of the image by the specified value.
- `-g, --grayscale`: Apply a grayscale filter to the image.
- `-c, --crop <x> <y> <width> <height>`: Crop the image using the specified coordinates and dimensions.

## Getting Started

To get started with RustPix, follow these steps:

1. Clone the repository: `git clone https://github.com/your-username/rustpix.git`
2. Navigate to the project directory: `cd rustpix`
3. Build the project: `cargo build`
4. Run RustPix: `cargo run -- <input_image> <output_image> [options]`

## Installation

For detailed installation instructions, please refer to the [INSTALL.md](INSTALL.md) file.

## Contributing

Contributions are welcome! If you find any bugs or have suggestions for new features, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file 