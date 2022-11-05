
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

pub fn compare_floats(x: f64, y: f64) -> bool{
    if (x - y).abs() < 0.001{
        return true;
    }
    return false;
}

pub fn compare_quad(x: (f64, f64, f64, f64), y: (f64, f64, f64, f64)) -> bool{
    if !compare_floats(x.0, y.0){
        return false;
    }
    else if !compare_floats(x.1, y.1){
        return false;
    }
    else if !compare_floats(x.2, y.2){
        return false;
    }
    else if !compare_floats(x.3, y.3){
        return false;
    }
    else{
        return true;
    }
}

pub fn compare_trio(x: (f64, f64, f64), y: (f64, f64, f64)) -> bool{
    if !compare_floats(x.0, y.0){
        return false;
    }
    else if !compare_floats(x.1, y.1){
        return false;
    }
    else if !compare_floats(x.2, y.2){
        return false;
    }
    else{
        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::quantize::{self, compare_quad, compare_floats, compare_trio, get_coefficients, get_luma, coefficients_to_scaled_ints, scaled_ints_to_coefficients};

    #[test]
    fn test_get_coefficients() {
        let answer = (0.5, 0.057, 0.294, 0.15);
        assert!(compare_quad(get_coefficients(0.299, 0.587, 0.114, 1.0), answer));
    }

    #[test]
    fn test_get_luma() {
        let answer = (0.299, 0.587, 0.114, 1.0);
        assert!(compare_quad(get_luma(0.5, 0.057, 0.294, 0.15), answer));
    }

    #[test]
    fn test_coefficients_to_scaled_ints() {
        let answer: (i64, i64, i64) = (-15, 15, 0);
        assert_eq!(coefficients_to_scaled_ints(-0.3, 3.0, 0.0), answer);
    }

    #[test]
    fn test_scaled_ints_to_coefficients() {
        let answer = (-0.3, 0.3, 0.0);
        assert_eq!(scaled_ints_to_coefficients(-15, 15, 0), answer);
    }
}