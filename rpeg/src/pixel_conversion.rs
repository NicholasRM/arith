
fn rgb_to_float(pixel :&Rgb) -> (f64, f64, f64){
    (pixel.red as f64 / 255.0, pixel.green as f64 / 255.0, pixel.blue as f64)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rgb_to_float() {
        let red_example: &Rgb = Rgb{red: 255, green: 0, blue: 0};
        assert_eq!(rgb_to_float(red_example), (1.0, 0.0, 0.0));
    }
}