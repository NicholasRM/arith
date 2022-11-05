use csc411_image::Rgb;

/// A struct containing a 2x2 block of RGB pixels
/// and the coordinates of the top left pixel
#[derive(Clone)]
pub struct PixelBlock {
    pub row_position: usize,
    pub col_position: usize,
    pub top_left: Rgb,
    pub top_right: Rgb,
    pub bottom_left: Rgb,
    pub bottom_right: Rgb,
}

impl PixelBlock {
    /// Packs 4 pixels and the coordinates of the top left corner into the struct
    /// 
    /// # Arguments
    /// 
    /// * `row_position`: the row index of the top left pixel in the group
    /// * `col_position`: the column index of the top left pixel in the group
    /// * `top_left`: the top left pixel (position = (r,c))
    /// * `top_right`: the top right pixel (position = (r, c+1))
    /// * `bottom_left`: the bottom left pixel (position = (r+1), c))
    /// * `bottom_right`: the bottom right pixel (position = ((r+1), (c+1)))
    pub fn pack(row_position: usize, col_position: usize, top_left: Rgb, top_right: Rgb, bottom_left: Rgb, bottom_right: Rgb) -> Self {
        PixelBlock{
            row_position,
            col_position,
            top_left,
            top_right,
            bottom_left,
            bottom_right
        }
    }

    /// Consumes the struct and returns a Vec of tuples
    /// containing the row, the column, and the pixel for
    /// each pixel in the group
    /// 
    /// # Arguments
    /// 
    /// * `self`: the value of the struct itself
    pub fn unpack(self) -> Vec<(usize, usize, Rgb)> {
        vec![
            (self.row_position, self.col_position, self.top_left),
            (self.row_position, self.col_position + 1, self.top_right),
            (self.row_position + 1, self.col_position, self.bottom_left),
            (self.row_position + 1, self.col_position + 1, self.bottom_right)
        ]
    }
}