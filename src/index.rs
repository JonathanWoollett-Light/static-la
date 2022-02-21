use crate::*;
use std::ops::{Index, IndexMut};

type Pair = (usize, usize);

// MatrixDxD
// --------------------------------------------------
impl<T> Index<Pair> for MatrixDxD<T> {
    type Output = T;
    fn index<'a>(&'a self, (row, column): (usize, usize)) -> &'a Self::Output {
        assert!(row < self.rows, "Row out of bounds");
        assert!(column < self.columns, "Columns out of bounds");
        &self.data[row * self.columns + column]
    }
}
impl<T> IndexMut<Pair> for MatrixDxD<T> {
    fn index_mut<'a>(&'a mut self, (row, column): (usize, usize)) -> &'a mut T {
        assert!(row < self.rows, "Row out of bounds");
        assert!(column < self.columns, "Columns out of bounds");
        &mut self.data[row * self.columns + column]
    }
}
// MatrixDxS
// --------------------------------------------------
impl<T, const COLUMNS: usize> Index<Pair> for MatrixDxS<T, COLUMNS> {
    type Output = T;
    fn index<'a>(&'a self, (row, column): (usize, usize)) -> &'a Self::Output {
        assert!(row < self.rows, "Row out of bounds");
        assert!(column < COLUMNS, "Columns out of bounds");
        &self.data[row * COLUMNS + column]
    }
}
impl<T, const COLUMNS: usize> IndexMut<Pair> for MatrixDxS<T, COLUMNS> {
    fn index_mut<'a>(&'a mut self, (row, column): (usize, usize)) -> &'a mut T {
        assert!(row < self.rows, "Row out of bounds");
        assert!(column < COLUMNS, "Columns out of bounds");
        &mut self.data[row * COLUMNS + column]
    }
}
// MatrixSxD
// --------------------------------------------------
impl<T, const ROWS: usize> Index<Pair> for MatrixSxD<T, ROWS> {
    type Output = T;
    fn index<'a>(&'a self, (row, column): (usize, usize)) -> &'a Self::Output {
        assert!(row < ROWS, "Row out of bounds");
        assert!(column < self.columns, "Columns out of bounds");
        &self.data[row * self.columns + column]
    }
}
impl<T, const ROWS: usize> IndexMut<Pair> for MatrixSxD<T, ROWS> {
    fn index_mut<'a>(&'a mut self, (row, column): (usize, usize)) -> &'a mut T {
        assert!(row < ROWS, "Row out of bounds");
        assert!(column < self.columns, "Columns out of bounds");
        &mut self.data[row * self.columns + column]
    }
}
// MatrixSxS
// --------------------------------------------------
impl<T, const ROWS: usize, const COLUMNS: usize> Index<Pair> for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = T;
    fn index<'a>(&'a self, (row, column): (usize, usize)) -> &'a Self::Output {
        assert!(row < ROWS, "Row out of bounds");
        assert!(column < COLUMNS, "Columns out of bounds");
        &self.data[row * COLUMNS + column]
    }
}
impl<T, const ROWS: usize, const COLUMNS: usize> IndexMut<Pair> for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn index_mut<'a>(&'a mut self, (row, column): (usize, usize)) -> &'a mut T {
        assert!(row < ROWS, "Row out of bounds");
        assert!(column < COLUMNS, "Columns out of bounds");
        &mut self.data[row * COLUMNS + column]
    }
}
// Tests
// --------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::*;
    use std::convert::TryFrom;
    // MatrixDxD
    // --------------------------------------------------
    #[test]
    fn dxd() {
        let a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        assert_eq!(a[(0, 0)], 1);
        assert_eq!(a[(0, 1)], 2);
        assert_eq!(a[(0, 2)], 3);
        assert_eq!(a[(1, 0)], 4);
        assert_eq!(a[(1, 1)], 5);
        assert_eq!(a[(1, 2)], 6);
    }
    // MatrixDxS
    // --------------------------------------------------
    #[test]
    fn dxs() {
        let a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
        assert_eq!(a[(0, 0)], 1);
        assert_eq!(a[(0, 1)], 2);
        assert_eq!(a[(0, 2)], 3);
        assert_eq!(a[(1, 0)], 4);
        assert_eq!(a[(1, 1)], 5);
        assert_eq!(a[(1, 2)], 6);
    }
    // MatrixSxD
    // --------------------------------------------------
    #[test]
    fn sxd() {
        let a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        assert_eq!(a[(0, 0)], 1);
        assert_eq!(a[(0, 1)], 2);
        assert_eq!(a[(0, 2)], 3);
        assert_eq!(a[(1, 0)], 4);
        assert_eq!(a[(1, 1)], 5);
        assert_eq!(a[(1, 2)], 6);
    }
    // MatrixSxS
    // --------------------------------------------------
    #[test]
    fn sxs() {
        let a = MatrixSxS::<i32, 2, 3>::from([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(a[(0, 0)], 1);
        assert_eq!(a[(0, 1)], 2);
        assert_eq!(a[(0, 2)], 3);
        assert_eq!(a[(1, 0)], 4);
        assert_eq!(a[(1, 1)], 5);
        assert_eq!(a[(1, 2)], 6);
    }
}
