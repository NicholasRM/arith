use csc411_image::Rgb;

pub fn rgb_to_float(pixel :&Rgb) -> (f64, f64, f64){
    (pixel.red as f64 / 255.0, pixel.green as f64 / 255.0, pixel.blue as f64)
}

pub fn float_to_rgb(rgb_floats: (f64, f64, f64)) -> Rgb{
    Rgb{red: (rgb_floats.0 * 255.0) as u16, green: (rgb_floats.1 * 255.0) as u16, blue: (rgb_floats.2 * 255.0) as u16}
}

#[cfg(test)]
mod tests {
    use crate::pixel_conversion::{rgb_to_float, float_to_rgb};
    use csc411_image::Rgb;

    #[test]
    fn test_rgb_to_float() {
        let red_example = &Rgb{red: 255, green: 0, blue: 0};
        assert_eq!(rgb_to_float(red_example), (1.0, 0.0, 0.0));
    }

    #[test]
    fn test_float_to_rgb(){
        let red_example = (1.0, 0.0, 0.0);
        let output = float_to_rgb(red_example);
        assert_eq!(output.red, 255);
        assert_eq!(output.green, 0);
        assert_eq!(output.blue, 0);
    }
}