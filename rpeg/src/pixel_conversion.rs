use csc411_image::Rgb;


/// Returns a tuple of three f64 values corresponding to the colors Red Green and Blue from `pixel`,
/// with the values divided by `denom` to acheive a range of 0.0 - 1.0 for each color
/// 
/// # Arguments:
/// * `pixel`: a reference to an Rgb pixel
/// * `denom`: the denominator of the RgbImage casted as a floating point value
pub fn rgb_to_float(pixel :&Rgb, denom: f64) -> (f64, f64, f64){
    (
        pixel.red as f64 / denom,
        pixel.green as f64 / denom,
        pixel.blue as f64 / denom
    )
}

/// Returns an Rgb pixel containg the values of `rgb_floats` mulitplied by `denom` to get them back to the correct scale
/// 
/// # Arguments:
/// * `rgb_floats`: a tuple of three floating point numbers representing Red, Green, and Blue on a scale of 0.0 - 1.0,
/// stored as (red, green, blue)
/// * `denom`: The denominator of the RgbImage
pub fn float_to_rgb(rgb_floats: (f64, f64, f64), denom: f64) -> Rgb{
    let (r, g, b) = rgb_floats;
    Rgb
    {
        red: (r.clamp(0.0,1.0) * denom) as u16,
        green: (g.clamp(0.0,1.0) * denom) as u16,
        blue: (b.clamp(0.0, 1.0) * denom) as u16
    }
}

/// Returns a tuple of floating point values representing the Luma and Chroma of `rgb_floats`
/// 
/// # Arguments:
/// * `rgb_floats`: a tuple of three floating point numbers representing Red, Green, and Blue on a scale of 0.0 - 1.0, 
/// stored as (red, green, blue)
pub fn get_compression_tuple(rgb_floats: (f64, f64, f64)) -> (f64, f64, f64) {
    let (r, g, b) = rgb_floats;
    (
        0.299*r + 0.587*g + 0.114*b,
        -0.168736*r - 0.331264*g + 0.5*b,
        0.5*r - 0.418688*g - 0.081312*b
    )
}

/// Returns a tuple of floating point values representing Red, Green, and Blue on a scale of 0.0 - 1.0 
/// derived by its luma and chroma values
/// 
/// # Arguments:
/// * `comp_vid_floats`: returns a tuple of values representing a pixels luma and chroma values, stored as (y, pb, pr)
pub fn get_decompression_tuple(comp_vid_floats: (f64, f64, f64)) -> (f64, f64, f64) {
    let (y, pb, pr) = comp_vid_floats;
    (
        1.0*y + 0.0*pb + 1.402*pr,
        1.0*y - 0.344136*pb - 0.714136*pr,
        1.0*y + 1.772*pb + 0.0*pr
    )
}

#[cfg(test)]
mod tests {
    use crate::pixel_conversion;
    use csc411_image::Rgb;

    #[test]
    fn test_rgb_to_float() {
        let red_example = &Rgb{red: 255, green: 0, blue: 0};
        assert_eq!(pixel_conversion::rgb_to_float(red_example, 255.0), (1.0, 0.0, 0.0));
    }

    #[test]
    fn test_float_to_rgb(){
        let red_example = (1.0, 0.0, 0.0);
        let output = pixel_conversion::float_to_rgb(red_example, 255.0);
        assert_eq!(output.red, 255);
        assert_eq!(output.green, 0);
        assert_eq!(output.blue, 0);
    }

    #[test]
    fn test_compression_tuple() {
        let red_example = (1.0, 0.0, 0.0);
        assert_eq!(pixel_conversion::get_compression_tuple(red_example), (0.299, -0.168736, 0.5));
    }

    #[test]
    fn test_decompression_tuple() {
        let red_example = (0.299, -0.168736, 0.5);
        let (output_r, output_g, output_b) = pixel_conversion::get_decompression_tuple(red_example);
        let diff_r = (output_r - 1.0).abs();
        let diff_g = (output_g - 1.321e-7).abs();
        let diff_b = (output_b - -1.92e-7).abs();
        assert!(diff_r < 0.00001 && diff_g < 0.00001 && diff_b < 0.00001);
    }
    #[test]
    fn round_trip(){
        let red_example = &Rgb{red: 255, green: 0, blue: 0};
        let red_floats = pixel_conversion::rgb_to_float(red_example, 255.0);
        let red_comp_vid = pixel_conversion::get_compression_tuple(red_floats);
        let new_red_floats = pixel_conversion::get_decompression_tuple(red_comp_vid);
        
        let fixed = (new_red_floats.0.clamp(0.0, 1.0), new_red_floats.1.clamp(0.0, 1.0), new_red_floats.2.clamp(0.0, 1.0));
        let new_red = pixel_conversion::float_to_rgb(fixed, 255.0);
        assert_eq!(new_red.red, 255);
        assert_eq!(new_red.green, 0);
        assert_eq!(new_red.blue, 0);
    }
}