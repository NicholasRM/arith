use bitpack::bitpack;
use csc411_arith;

/// Returns a tuple of 4 coefficients based on the luma from a 2 X 2 matrix of pixels, `y1`, `y2`, `y3`, and `y4`,
/// the coefficients represent the average brightness of the matrix, 
/// the degree in which brightness increases vertically from the bottom to the top of the matrix, 
/// the degree in which brightness changes horizontally from the left to the right of the matrix,
/// and the degree in which the brightness of pixels on one diagonal compare to that of another diagonal,
/// stored as (a, b, c, d)
/// 
/// # Arguments:
/// * `y1`: The luma of the top left pixel in the matrix
/// * `y2`: The luma of the top right pixel in the matrix
/// * `y3`: The luma of the bottom left pixel in the matrix
/// * `y4`: The luma of the bottom right pixel in the matrix
pub fn get_coefficients(y1: f64, y2: f64, y3: f64, y4: f64) -> (f64, f64, f64, f64){
    let a = (y4 + y3 + y2 + y1) / 4.0;
    let b = (y4 + y3 - y2 - y1) / 4.0;
    let c = (y4 - y3 + y2 - y1) / 4.0;
    let d = (y4 - y3 - y2 + y1) / 4.0;
    (a, b, c, d)
}

/// Returns a tuple of floating point values which represent the brightness of each pixel in the 2 X 2 matrix of pixels,
/// using the average brightness of the matrix and the degree of change in brightness in each direction of the matrix, 
/// stored as (y1, y2, y3, y4)
/// 
/// # Arguments:
/// * `a`: the average brightness of the matrix of pixels
/// * `b`: the degree in which brightness increases vertically from the bottom to the top of the matrix
/// * `c`: the degree in which brightness changes horizontally from the left to the right of the matrix
/// * `d`: the degree in which the brightness of pixels on one diagonal compare to that of another diagonal
pub fn get_luma(a: f64, b: f64, c: f64, d: f64) -> (f64, f64, f64, f64){
    let y1 = a - b - c + d;
    let y2 = a - b + c - d;
    let y3 = a + b - c - d;
    let y4 = a + b + c + d;
    (y1, y2, y3, y4)
}

/// Returns a tuple of integers representing the coefficinets `b`, `c`, and `d` 
/// describing degree of change within the 2 X 2 matrix, stored as (b_sign, c_sign, d_sign)
/// 
/// # Arguments:
/// * `b`: the degree in which brightness increases vertically from the bottom to the top of the matrix
/// * `c`: the degree in which brightness changes horizontally from the left to the right of the matrix
/// * `d`: the degree in which the brightness of pixels on one diagonal compare to that of another diagonal
pub fn coefficients_to_scaled_ints(b: f64, c: f64, d: f64) -> (i64, i64, i64){
    let b_sign = (b.clamp(-0.3, 0.3) * 50.0) as i64;
    let c_sign = (c.clamp(-0.3, 0.3) * 50.0) as i64;
    let d_sign = (d.clamp(-0.3, 0.3) * 50.0) as i64;
    (b_sign, c_sign, d_sign)
}

/// Returns the floating point representation of the coefficients describing the degree of change within the matrix,
/// given the integer representation, `b_sign`, `c_sign`, and `d_sign`, stored as (b, c, d)
/// 
/// # Arguments:
/// * `b_sign`: the integer represenation of the floating point coefficient b
/// * `c_sign`: the integer represenation of the floating point coefficient c
/// * `d_sign`: the integer represenation of the floating point coefficient d
pub fn scaled_ints_to_coefficients(b_sign: i64, c_sign: i64, d_sign: i64) -> (f64, f64, f64){
    let b = b_sign as f64 / 50.0;
    let c = c_sign as f64 / 50.0;
    let d = d_sign as f64 / 50.0;
    (b, c, d)
}

/// Returns the unsigned integer repsentation of the coefficient `a`
/// 
/// # Arguments:
/// * `a`: The average brightness of pixels in the matrix
pub fn singular_coefficient_to_scaled_int(a: f64) -> u64{
    let a_scale = (a * bitpack::maxu(9) as f64).round() as u64;
    a_scale
}

/// Returns the floating point repsentation of the average brightness of a matrix given `a_scale`
/// 
/// # Arguments:
/// * `a_scale`: the unsigned integer repsentation of the average brightness of the matrix
pub fn singular_scaled_int_to_coefficient(a_scale: u64) -> f64{
    let a = a_scale as f64 / bitpack::maxu(9) as f64;
    a
}

/// Returns a tuple of unsigned integers representing the two chroma values `pb`, `pr`, stored as (pbidx, pridx)
/// 
/// # Arguments:
/// * `pb`: a floating point value representing the difference between the brightness and the amount of blue
/// * `pr`: a floating point value representing the difference between the brightness and the amount of blue
pub fn chroma_to_indices(pb: f64, pr: f64) -> (u64, u64){
    (csc411_arith::index_of_chroma(pb as f32).try_into().unwrap(), csc411_arith::index_of_chroma(pr as f32).try_into().unwrap())
}

/// Returns a tuple of floating point values representing the chromacity given their unsigned integer representation
/// 
/// # Arguments:
/// * `pbidx`: the unsigned integer representation of the difference between brightness and blue
/// * `pridx`: the unsigned integer representation of the difference between brightness and red
pub fn indices_to_chroma(pbidx: u64, pridx: u64) -> (f64, f64){
    (csc411_arith::chroma_of_index(pbidx as usize) as f64, csc411_arith::chroma_of_index(pridx as usize) as f64)
}

/// Returns a u32 word which holds the signed/unsigned integer repsentation of the brightness coefficients, `a`, `b`, `c`, `d`
/// and the chromacity `pb`, and `pr`
/// 
/// # Arguments:
/// * `a`: the unsigned integer repsentation of the average brightness of the matrix
/// * `b`: the integer represenation of the degree in which brightness increases vertically 
/// from the bottom to the top of the matrix
/// * `c`: the integer represenation of the degree in which brightness changes horizontally 
/// from the left to the right of the matrix
/// * `d`: the integer represenation of the the degree in which the brightness of pixels on one diagonal 
/// compare to that of another diagonal
/// * `pb`: the unsigned integer representation of the difference between brightness and blue
/// * `pr`: the unsigned integer representation of the difference between brightness and red
pub fn construct_word(a: u64, b: i64, c: i64, d: i64, pb: u64, pr: u64) -> u32{
    let mut word: u64 = 0;
    word = bitpack::newu(word, 9, 23, a).unwrap();
    word = bitpack::news(word, 5, 18, b).unwrap();
    word = bitpack::news(word, 5, 13, c).unwrap();
    word = bitpack::news(word, 5, 8, d).unwrap();
    word = bitpack::newu(word, 4, 4, pb).unwrap();
    word = bitpack::newu(word, 4, 0, pr).unwrap();
    word as u32
}

/// Returns a tuple of six signed/unsigned integers representing the brightness and chromacity of a 2 X 2 matrix of pixels, from a u32 `word`
/// 
/// # Arguments:
/// * `word`: a u32 value that holds the values returned in the tuple
pub fn parse_word(word: u32) -> (u64, i64, i64, i64, u64, u64){
    let a = bitpack::getu(word as u64, 9, 23);
    let b = bitpack::gets(word as u64, 5, 18);
    let c = bitpack::gets(word as u64, 5, 13);
    let d = bitpack::gets(word as u64, 5, 8);
    let pb = bitpack::getu(word as u64, 4, 4);
    let pr = bitpack::getu(word as u64, 4, 0);
    (a, b, c, d, pb, pr)
}

#[cfg(test)]
mod tests {
    use crate::quantize::{get_coefficients, get_luma, coefficients_to_scaled_ints, scaled_ints_to_coefficients, 
        singular_coefficient_to_scaled_int, singular_scaled_int_to_coefficient, chroma_to_indices, indices_to_chroma,
        construct_word, parse_word};

    fn compare_floats(x: f64, y: f64) -> bool{
        if (x - y).abs() < 0.001{
            return true;
        }
        return false;
    }

    fn compare_quad(x: (f64, f64, f64, f64), y: (f64, f64, f64, f64)) -> bool{
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

    fn compare_pair(x: (f64, f64), y: (f64, f64)) -> bool{
        if !compare_floats(x.0, y.0){
            return false;
        }
        else if !compare_floats(x.1, y.1){
            return false;
        }
        else{
            return true;
        }
    }

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

    #[test]
    fn test_singular_coefficient_to_scaled_int(){
        let input = 0.5;
        let output = 256;
        assert_eq!(singular_coefficient_to_scaled_int(input), output);
    }

    #[test]
    fn test_singular_scaled_int_to_coefficient() {
        let input = 256;
        let output = 0.5;
        assert!(compare_floats(singular_scaled_int_to_coefficient(input), output));
    }

    #[test]
    fn test_chroma_to_indices(){
        let answer: (u64, u64) = (15, 0);
        assert_eq!(chroma_to_indices(0.35, -0.35), answer);
    }

    #[test]
    fn test_indices_to_chroma(){
        let answer = (0.35, -0.35);
        assert!(compare_pair(indices_to_chroma(15, 0), answer));
    }

    #[test]
    fn test_construct_word(){
        let answer: u32 = 0b10000000010001011110000011110000;
        assert_eq!(construct_word(256, -15, 15, 0, 15, 0), answer);
    }

    #[test]
    fn test_parse_word(){
        let input: u32 = 0b10000000010001011110000011110000;
        let answer: (u64, i64, i64, i64, u64, u64) = (256, -15, 15, 0, 15, 0);
        assert_eq!(parse_word(input), answer);
    }
    
}