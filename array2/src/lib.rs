pub struct Array2<T: Clone> {
    matrix: Vec<T>,
    height: usize,
    width: usize,
}

impl<T: Clone> Array2<T> {
    /// Method that returns an Array2 with a Vec of elements,
    /// as well as defined dimensions, in row major order
    ///
    /// #Arguments
    /// input: a Vec containing inputted items of T
    /// row_amt: the number of rows in the Vec
    /// col_amt: the number of columns in the Vec
    ///
    /// Self: an instance of Array2
    pub fn from(input: Vec<T>, row_amt: usize, col_amt: usize) -> Self {
        assert!(input.len() == row_amt * col_amt);
        Array2 {
            matrix: input,
            height: row_amt,
            width: col_amt,
        }
    }

    /// Method that indexes matrix and returns the element at the coordinates
    ///
    /// #Arguments
    /// &self: an immutable reference to an instance of Array2
    /// row: the row index of the element
    /// col: the col index of the element
    ///
    /// &T: a reference to an element in the matrix
    pub fn get(&self, row: usize, col: usize) -> &T {
        assert!(row < self.height && col < self.width);
        &(self.matrix[row * self.height + col])
    }

    /// Checks to see if matrix is in row major order and
    /// returns an iterator over the elements in row major order
    ///
    /// # Arguments
    /// * &self: an immutable reference to an instance of Array2
    ///
    /// impl Iterator<Item=(usize,usize,&T)>: an iterator over references to items in matrix with the appropriate coordinates in
    /// the order (row, col)
    pub fn iter_row_maj(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.matrix
            .iter()
            .enumerate()
            .map(|(idx, val)| ((idx / self.width), (idx % self.width), val))
    }

    /// Checks to see if the matrix is not in row major order and
    /// returns an iterator over the elements in column major order
    ///
    /// #Arguments
    /// &self: an immutable reference to an instance of Array2
    ///
    /// impl Iterator<Item=(usize, usize, &T)>: an iterator over references to items in matrix with the appropriate coordinates in
    /// the order (row, col)
    pub fn iter_col_maj(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        let mut cols: Vec<_> = Vec::new();
        for i in 0..(self.width) {
            let col_iter = self.matrix.iter().skip(i).step_by(self.width);
            cols.push(col_iter);
        }
        cols.into_iter()
            .flatten()
            .enumerate()
            .map(|(idx, val)| ((idx % self.height), (idx / self.height), val))
    }
}

#[cfg(test)]
mod tests {
    use crate::Array2;
    #[test]
    fn col_iter_test() {
        let arr = Array2::from((1..=6).collect(), 2, 3);
        let iter: Vec<_> = arr.iter_col_maj().collect();
        let ans: Vec<(usize, usize, &i32)> = vec![
            (0, 0, &1),
            (1, 0, &4),
            (0, 1, &2),
            (1, 1, &5),
            (0, 2, &3),
            (1, 2, &6),
        ];
        assert_eq!(iter, ans);
    }

    #[test]
    fn row_iter_test() {
        let arr = Array2::from((1..=6).collect(), 2, 3);
        let iter: Vec<_> = arr.iter_row_maj().collect();
        let ans: Vec<(usize, usize, &i32)> = vec![
            (0, 0, &1),
            (0, 1, &2),
            (0, 2, &3),
            (1, 0, &4),
            (1, 1, &5),
            (1, 2, &6),
        ];
        assert_eq!(iter, ans);
    }

    #[test]
    fn get_test() {
        let arr = Array2::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3);
        let four = arr.get(1, 0);
        assert_eq!(four, &4);
    }
}
