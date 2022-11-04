
pub fn get_coefficients(y1: f64, y2: f64, y3: f64, y4: f64) -> (f64, f64, f64, f64){
    let a = (y4 + y3 + y2 + y1) / 4.0;
    let b = (y4 + y3 - y2 - y1) / 4.0;
    let c = (y4 - y3 + y2 - y1) / 4.0;
    let d = (y4 - y3 - y2 + y1) / 4.0;
    (a, b, c, d)
}

pub fn get_luma(a: f64, b: f64, c: f64, d: f64) -> (f64, f64, f64, f64){
    let y1 = a - b - c + d;
    let y2 = a - b + c - d;
    let y3 = a + b - c - d;
    let y4 = a + b + c + d;
    (y1, y2, y3, y4)
}

pub fn coefficients_to_scaled_ints(b: f64, c: f64, d: f64) -> (i64, i64, i64){
    let b_sign = (b.clamp(-0.3, 0.3) * 50.0) as i64;
    let c_sign = (c.clamp(-0.3, 0.3) * 50.0) as i64;
    let d_sign = (d.clamp(-0.3, 0.3) * 50.0) as i64;
    (b_sign, c_sign, d_sign)
}

pub fn scaled_ints_to_coefficients(b_sign: i64, c_sign: i64, d_sign: i64) -> (f64, f64, f64){
    let b = b_sign as f64 / 50.0;
    let c = c_sign as f64 / 50.0;
    let d = d_sign as f64 / 50.0;
    (b, c, d)
}

#[cfg(test)]
mod tests {
    use crate::pixel_conversion;
    use csc411_image::Rgb;

    #[test]
    fn test_quantize() {
        assert!(true);
    }
}