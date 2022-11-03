
fn rgb_to_float(pixel :&Rgb) -> (f64, f64, f64){
    (pixel.red as f64 / 255.0, pixel.green as f64 / 255.0, pixel.blue as f64)
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_test() {
        let arr = Array2::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3);
        let four = arr.get(1, 0);
        assert_eq!(four, &4);
    }
}