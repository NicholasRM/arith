pub mod codec;
pub mod pixel_conversion;
pub mod pixel_block;

// #[cfg(test)]
// mod tests {
//     use crate::pixel_conversion;
//     use csc411_image::Rgb;
//     #[test]
//     fn test_rgb_to_float() {
//         let red_example: &Rgb = Rgb{red: 255, green: 0, blue: 0};
//         assert_eq!(rgb_to_float(red_example), (1.0, 0.0, 0.0));
//     }
// }