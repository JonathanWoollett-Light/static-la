use crate::*;
use std::ops::Index;

type Pair = (usize, usize);

// MatrixDxD
// --------------------------------------------------
impl<T> Index<Pair> for MatrixDxD<T> {
    type Output = T;
    fn index<'a>(&'a self, (row, column): (usize, usize)) -> &'a Self::Output {
        &self.data[row * self.columns + column]
    }
}
// MatrixDxS
// --------------------------------------------------
impl<T, const COLUMNS: usize> Index<Pair> for MatrixDxS<T, COLUMNS> {
    type Output = T;
    fn index<'a>(&'a self, (row, column): (usize, usize)) -> &'a Self::Output {
        &self.data[row * COLUMNS + column]
    }
}
// MatrixSxD
// --------------------------------------------------
impl<T, const ROWS: usize> Index<Pair> for MatrixSxD<T, ROWS> {
    type Output = T;
    fn index<'a>(&'a self, (row, column): (usize, usize)) -> &'a Self::Output {
        &self.data[row * self.columns + column]
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
        &self.data[row * COLUMNS + column]
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
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
        let a = MatrixSxS::from([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(a[(0, 0)], 1);
        assert_eq!(a[(0, 1)], 2);
        assert_eq!(a[(0, 2)], 3);
        assert_eq!(a[(1, 0)], 4);
        assert_eq!(a[(1, 1)], 5);
        assert_eq!(a[(1, 2)], 6);
    }
}
