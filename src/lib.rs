/// Converts a hex color code (e.g., "#FF5733") to an RGB tuple (e.g., (255, 87, 51)).
/// 
/// # Arguments
/// * `hex` - A string slice representing the hex color code in the form "#RRGGBB".
/// 
/// # Returns
/// * `Ok((u8, u8, u8))` - The RGB representation as a tuple of three `u8` values if the input is valid.
/// * `Err(&'static str)` - An error message if the input is invalid.
///
/// # Example
/// ```
/// use hex_to_rgb::hex_to_rgb;
///
/// let rgb = hex_to_rgb("#FF5733").unwrap();
/// assert_eq!(rgb, (255, 87, 51));
/// ```
pub fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), &'static str> {
    // Ensure the hex code starts with '#' and is exactly 7 characters
    if !hex.starts_with('#') || hex.len() != 7 {
        return Err("Invalid hex color format. Use '#RRGGBB'.");
    }
    
    // Parse the RGB values from the hex string
    let r = u8::from_str_radix(&hex[1..3], 16).map_err(|_| "Invalid hex digits")?;
    let g = u8::from_str_radix(&hex[3..5], 16).map_err(|_| "Invalid hex digits")?;
    let b = u8::from_str_radix(&hex[5..7], 16).map_err(|_| "Invalid hex digits")?;
    
    Ok((r, g, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(hex_to_rgb("#FF5733").unwrap(), (255, 87, 51));
        assert_eq!(hex_to_rgb("#000000").unwrap(), (0, 0, 0));
        assert_eq!(hex_to_rgb("#FFFFFF").unwrap(), (255, 255, 255));
    }

    #[test]
    fn test_invalid_hex_to_rgb() {
        assert!(hex_to_rgb("FF5733").is_err()); // Missing '#'
        assert!(hex_to_rgb("#FFF").is_err());   // Incorrect length
        assert!(hex_to_rgb("#GG5733").is_err()); // Invalid hex digits
    }
}
