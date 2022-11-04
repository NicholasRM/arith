use csc411_image::Rgb;

#[derive(Clone)]
pub struct PixelBlock {
    pub w_position: usize,
    pub h_position: usize,
    pub top_left: Rgb,
    pub top_right: Rgb,
    pub bottom_left: Rgb,
    pub bottom_right: Rgb,
}

impl PixelBlock {
    pub fn from(w_position: usize, h_position: usize, top_left: Rgb, top_right: Rgb, bottom_left: Rgb, bottom_right: Rgb) -> Self {
        PixelBlock { w_position, h_position, top_left, top_right, bottom_left, bottom_right }
    }
}