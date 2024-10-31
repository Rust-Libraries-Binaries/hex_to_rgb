## hex_to_rgb
hex_to_rgb is a simple Rust library that converts a hexadecimal color code (in #RRGGBB format) to an RGB tuple ((u8, u8, u8)). This crate is designed for quick, lightweight color conversions, making it useful for applications that work with color manipulation and representation.

## Installation
To use hex_to_rgb, add it to your Cargo.toml:

[dependencies]
hex_to_rgb = "0.1.0"

Then, in your Rust code:
```rust
use hex_to_rgb::hex_to_rgb;

fn main() {
let rgb = hex_to_rgb("#FF5733").unwrap();
println!("The color #FF5733 is represented as RGB {:?}", rgb);
// Outputs: The color #FF5733 is represented as RGB (255, 87, 51)
}
```

## Features
Converts hex color codes (like #RRGGBB) to RGB tuples ((u8, u8, u8)).
Lightweight and easy to use.
Includes error handling for invalid hex inputs.

## License
This project is licensed under the MIT License.

## Author
Ben Santora (<bensatlantik@gmail.com>)