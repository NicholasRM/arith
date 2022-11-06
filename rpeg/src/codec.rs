use array2::Array2;
use csc411_image::{self, RgbImage, Read, Rgb, Write};
use crate::{pixel_block::{self, PixelBlock}, pixel_conversion, quantize::{self, scaled_ints_to_coefficients, indices_to_chroma}};
use csc411_rpegio;

pub fn compress(filename: Option<&str>){
    let (arr, height, width, denom) = read_uncompressed_image(filename);

    let denominator = denom as f64;

    let mut compressed_img: Vec<u32> = Vec::new();
    for (_, _, group) in arr.iter_row_maj() {
        let pixel_vec = group.clone().unpack();
        let (mut pb_avg, mut pr_avg) = (0.0,0.0);
        let mut luma = Vec::new();

        for (_, _, pixel) in pixel_vec.iter() {
            let float_point_pixel = pixel_conversion::rgb_to_float(pixel, denominator);
            let (y, pb, pr) = pixel_conversion::get_compression_tuple(float_point_pixel);
            pb_avg += pb;
            pr_avg += pr;
            luma.push(y);
        }

        let (a, b, c, d) = quantize::get_coefficients(luma[0], luma[1], luma[2], luma[3]);
        pb_avg /= 4.0;
        pr_avg /= 4.0;

        let a_int = quantize::singular_coefficient_to_scaled_int(a);
        let (b_int, c_int, d_int) = quantize::coefficients_to_scaled_ints(b, c, d);
        let (pb_idx, pr_idx) = quantize::chroma_to_indices(pb_avg, pr_avg);

        let word = quantize::construct_word(a_int, b_int, c_int, d_int, pb_idx, pr_idx);

        compressed_img.push(word);
    }

    let compressed_arr: Vec<[u8; 4]> = compressed_img.into_iter().map(|x| x.to_be_bytes()).collect();

    write_compressed_image(&compressed_arr, width as u32, height as u32);
}

pub fn decompress(filename: Option<&str>){
    let (rpeg_img, width, height) = read_compressed_image(filename);

    let mut img = Vec::new();
    for (row, col, word) in rpeg_img.into_iter() {
        let (a, b, c, d, pb, pr) = quantize::parse_word(word);
        let a_float = quantize::singular_scaled_int_to_coefficient(a);
        let (b_float, c_float, d_float) = quantize::scaled_ints_to_coefficients(b, c, d);
        let (pb_chroma, pr_chroma) = quantize::indices_to_chroma(pb, pr);
        let (y1, y2, y3, y4) = quantize::get_luma(a_float, b_float, c_float, d_float);
        let (top_left, top_right, bottom_left, bottom_right) = (
            pixel_conversion::float_to_rgb(pixel_conversion::get_decompression_tuple((y1, pb_chroma, pr_chroma)), 255.0),
            pixel_conversion::float_to_rgb(pixel_conversion::get_decompression_tuple((y2, pb_chroma, pr_chroma)), 255.0),
            pixel_conversion::float_to_rgb(pixel_conversion::get_decompression_tuple((y3, pb_chroma, pr_chroma)), 255.0),
            pixel_conversion::float_to_rgb(pixel_conversion::get_decompression_tuple((y4, pb_chroma, pr_chroma)), 255.0),
        );
        let group = PixelBlock::pack(row, col, top_left, top_right, bottom_left, bottom_right);
        img.append(&mut group.unpack());
    }

    let mut indexed_img: Vec<(usize, Rgb)> = img.into_iter().map(|(r, c, pixel)| (r * width as usize + c, pixel)).collect();
    indexed_img.sort_by_key(|(idx, _)| *idx);
    let stripped_img: Vec<Rgb> = indexed_img.into_iter().map(|(_, pixel)| pixel).collect();
    let new_img = RgbImage {
        pixels: stripped_img,
        width,
        height,
        denominator: 255
    };

    write_uncompressed_image(new_img);
}

fn read_uncompressed_image(filename: Option<&str>) -> (Array2<PixelBlock>, usize, usize, u16) {
    let img = RgbImage::read(filename).unwrap();

    let mut arr_contents = Vec::new();
    let (trimmed_h, trimmed_w) = (img.height & !1_u32, img.width & !1_u32);

    for r in (0..trimmed_h).step_by(2) {
        for c in (0..trimmed_w).step_by(2) {
            arr_contents.push(
                PixelBlock::pack(
                    r as usize, c as usize, 
                    img.pixels.get((r*img.width + c) as usize).unwrap().to_owned(),
                    img.pixels.get((r*img.width + (c + 1)) as usize).unwrap().to_owned(),
                    img.pixels.get(((r + 1)*img.width + c) as usize).unwrap().to_owned(),
                    img.pixels.get(((r + 1)*img.width + (c + 1)) as usize).unwrap().to_owned()
                )
            )
        }
    }

    (
        Array2::from(arr_contents, (img.height & !1_u32) as usize / 2, (img.width & !1_u32) as usize / 2),
        trimmed_h as usize,
        trimmed_w as usize,
        img.denominator
    )

}

fn read_compressed_image(filename: Option<&str>) -> (Vec<(usize, usize, u32)>, u32, u32) {
    let (img, width, height) = csc411_rpegio::read_in_rpeg_data(filename).unwrap();
    let rpeg_img: Vec<(usize, usize, u32)> = img.into_iter()
    .enumerate()
    .map(|(idx, bytes)| (
        (idx / width as usize) * 2,
        (idx % width as usize) * 2,
        u32::from_be_bytes(bytes)
    ))
    .collect();

    (
        rpeg_img,
        width,
        height
    )
}

fn write_uncompressed_image(img: RgbImage) {
    let _ = img.write(None);
}

fn write_compressed_image(compressed_arr: &Vec<[u8; 4]>, width: u32, height: u32) {
    csc411_rpegio::output_rpeg_data(compressed_arr.as_slice(), width, height);
}

#[cfg(test)]
mod tests {

}