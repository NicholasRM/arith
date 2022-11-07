use array2::Array2;
use csc411_image::{self, RgbImage, Read, Rgb, Write};
use crate::{pixel_block::PixelBlock, pixel_conversion, quantize};
use csc411_rpegio;

/// Function to handle compressing a `.ppm` file received from 
/// the user
/// 
/// # Arguments:
/// * `filename`: the file name received from the user. Input is None iff the user does not enter
/// a filename as a command line argument
pub fn compress(filename: Option<&str>){
    //extract the 
    let (arr, height, width, denom) = read_uncompressed_image(filename);

    //cast denominator as f64 for use later
    let denominator = denom as f64;

    let mut compressed_img: Vec<u32> = Vec::new();
    for (_, _, group) in arr.iter_row_maj() {
        // unpack each block of pixels into the underlying Rgb
        let pixel_vec = group.clone().unpack();

        // set up accumulator values to store the pb, pr, and luma values
        let (mut pb_avg, mut pr_avg) = (0.0,0.0);
        let mut luma = Vec::new();

        for (_, _, pixel) in pixel_vec.iter() {
            // convert each pixel in the group into the floating point representation and then into component video
            let float_point_pixel = pixel_conversion::rgb_to_float(pixel, denominator);
            let (y, pb, pr) = pixel_conversion::get_compression_tuple(float_point_pixel);

            // increment the pb and pr, then push the luma into the vector
            pb_avg += pb;
            pr_avg += pr;
            luma.push(y);
        }

        //extract the luma out and compute the coefficients
        let (a, b, c, d) = quantize::get_coefficients(luma[0], luma[1], luma[2], luma[3]);
        //average pb and pr
        pb_avg /= 4.0;
        pr_avg /= 4.0;

        //turn each floating point value into a scaled integer or index
        let a_int = quantize::singular_coefficient_to_scaled_int(a);
        let (b_int, c_int, d_int) = quantize::coefficients_to_scaled_ints(b, c, d);
        let (pb_idx, pr_idx) = quantize::chroma_to_indices(pb_avg, pr_avg);

        //push each of these values into a word
        let word = quantize::construct_word(a_int, b_int, c_int, d_int, pb_idx, pr_idx);

        //push that word into the compressed image
        compressed_img.push(word);
    }

    // turn each 32 bit word into 4 bytes in Big-Endian order, then write the image to standard output
    let compressed_arr: Vec<[u8; 4]> = compressed_img.into_iter().map(|x| x.to_be_bytes()).collect();
    write_compressed_image(&compressed_arr, width as u32, height as u32);
}


/// Function to handle decompressing a `.rpeg` file received from 
/// the user
/// 
/// # Arguments:
/// * `filename`: the file name received from the user. Input is None iff the user does not enter
/// a filename as a command line argument
pub fn decompress(filename: Option<&str>){
    // extract the image data and dimensions from the image
    let (rpeg_img, width, height) = read_compressed_image(filename);

    let mut img = Vec::new();
    for (row, col, word) in rpeg_img.into_iter() {
        // extract all values from each of the words
        let (a, b, c, d, pb, pr) = quantize::parse_word(word);

        // convert each of the integer values into a floating point representation
        let a_float = quantize::singular_scaled_int_to_coefficient(a);
        let (b_float, c_float, d_float) = quantize::scaled_ints_to_coefficients(b, c, d);
        let (pb_chroma, pr_chroma) = quantize::indices_to_chroma(pb, pr);

        // convert the coefficients back into luma
        let (y1, y2, y3, y4) = quantize::get_luma(a_float, b_float, c_float, d_float);
        let (top_left, top_right, bottom_left, bottom_right) = (
            /* 
                Collect each of the luma with the pb and pr chroma values and decompress each of them with a denominator of
                255.0, and then convert each of the floating point Rgb values back into Rgb pixels.
                These Rgb pixels are then stored into the associated parentheses.
            */
            pixel_conversion::float_to_rgb(pixel_conversion::get_decompression_tuple((y1, pb_chroma, pr_chroma)), 255.0),
            pixel_conversion::float_to_rgb(pixel_conversion::get_decompression_tuple((y2, pb_chroma, pr_chroma)), 255.0),
            pixel_conversion::float_to_rgb(pixel_conversion::get_decompression_tuple((y3, pb_chroma, pr_chroma)), 255.0),
            pixel_conversion::float_to_rgb(pixel_conversion::get_decompression_tuple((y4, pb_chroma, pr_chroma)), 255.0),
        );
        // each of the pixels are packed into a PixelBlock, then unpacked out and the vector is appended to an accumulator
        let group = PixelBlock::pack(row, col, top_left, top_right, bottom_left, bottom_right);
        img.append(&mut group.unpack());
    }

    // each of the coordinates of the pixels are turned into a row major index, which are then sorted
    let mut indexed_img: Vec<(usize, Rgb)> = img.into_iter().map(|(r, c, pixel)| ((r * width as usize + c), pixel)).collect();
    indexed_img.sort_by_key(|(idx, _)| *idx);

    // the indicies are stripped from the image and a new RgbImage is created with the pixels
    let stripped_img: Vec<Rgb> = indexed_img.into_iter().map(|(_, pixel)| pixel).collect();
    let new_img = RgbImage {
        pixels: stripped_img,
        width,
        height,
        denominator: 255
    };

    // write the image to standard out
    write_uncompressed_image(new_img);
}

/// Helper function to read an image that is not compressed. Returns an Array2 of PixelBlocks,
/// coordinates for height and width, and the denominator. The order of the tuple is as follows:
/// (`Array of pixel groups`, `height`, `width`, `denominator`)
/// 
/// # Arguments
/// * `filename`: the file name received from the user. Input is None iff the user does not enter
/// a filename as a command line argument
fn read_uncompressed_image(filename: Option<&str>) -> (Array2<PixelBlock>, usize, usize, u16) {
    // read the image and store it as an RgbImage
    let img = RgbImage::read(filename).unwrap();

    let mut arr_contents = Vec::new();
    //trim the width and height prior so that they are rounded down to the nearest even number
    let (trimmed_h, trimmed_w) = (img.height & !1_u32, img.width & !1_u32);

    for r in (0..trimmed_h).step_by(2) {
        for c in (0..trimmed_w).step_by(2) {
            /*
                construct a new Vector containing groups of pixels stored in
                the image. Each group of pixels is indexed starting at the 
                top left corner of the block.
            */
            arr_contents.push(
                PixelBlock::pack(
                    r as usize, c as usize, 
                    img.pixels.get((r * img.width + c) as usize).unwrap().to_owned(),
                    img.pixels.get((r * img.width + (c + 1)) as usize).unwrap().to_owned(),
                    img.pixels.get(((r + 1) * img.width + c) as usize).unwrap().to_owned(),
                    img.pixels.get(((r + 1) * img.width + (c + 1)) as usize).unwrap().to_owned()
                )
            )
        }
    }

    // construct an Array2<PixelBlock> given half the width and height, each of the individual dimensions, and the denominator
    (
        Array2::from(arr_contents, trimmed_h as usize / 2, trimmed_w as usize / 2),
        trimmed_h as usize,
        trimmed_w as usize,
        img.denominator
    )

}

/// Helper function to read an image that is compressed. Returns a Vec<(usize, usize, u32)>,
/// and the dimensions of the image. The tuple returned is in the following format:
/// (`Vec of words`, `width`, `height`)
/// 
/// # Arguments:
/// * filename: the file name received from the user. Input is None iff the user does not enter
/// a filename as a command line argument
fn read_compressed_image(filename: Option<&str>) -> (Vec<(usize, usize, u32)>, u32, u32) {
    //read image in using rpegio crate
    let (img, width, height) = csc411_rpegio::read_in_rpeg_data(filename).unwrap();

    // map row major coordinates over each word and convert each collection of Big-Endian bytes into a 32 bit word
    let rpeg_img: Vec<(usize, usize, u32)> = img.into_iter()
        .enumerate()
        .map(|(idx, bytes)| (
            (idx / (width / 2) as usize) * 2,
            (idx % (width / 2) as usize) * 2,
            u32::from_be_bytes(bytes)
        ))
        .collect();
    
    // return the image and dimensions
    (
        rpeg_img,
        width,
        height
    )

}

/// Helper function to write an uncompressed image to standard out.
/// 
/// # Arguments:
/// * `img`: an RgbImage containing the decompressed output of the image
fn write_uncompressed_image(img: RgbImage) {
    let _ = img.write(None);
}

/// Helper function to write an compressed image to standard out.
/// 
/// # Arguments:
/// * `compressed_img`: a Vec of 32 bit words represented in 4 Big-Endian bytes
/// * `width`: the width of the image
/// * `height`: the height of the image
fn write_compressed_image(compressed_arr: &Vec<[u8; 4]>, width: u32, height: u32) {
    csc411_rpegio::output_rpeg_data(compressed_arr.as_slice(), width, height);
}

#[cfg(test)]
mod tests {
    use csc411_image::*;

    use super::*;

    #[test]
    fn test_read_uncompressed() {
        let (arr, height, width, denominator) = read_uncompressed_image(Some("Connor.ppm"));
        let new_arr: Vec<(usize, usize, Rgb)> = arr.iter_row_maj().map(|(_, _, group)| group.clone().unpack()).flatten().collect();
        let mut new_img: Vec<(usize, Rgb)> = new_arr.into_iter().map(|(r, c, pixel)| (r*width + c, pixel)).collect();
        new_img.sort_by_key(|(idx, _)| *idx);
        let cleaned_img: Vec<Rgb> = new_img.into_iter().map(|(_, pixel)| pixel).collect();
        let dest_img = RgbImage {
            pixels: cleaned_img,
            width: width as u32,
            height: height as u32,
            denominator
        };
        let _ = dest_img.write(Some("SameConnor.ppm"));
        assert!(true);
    }
}