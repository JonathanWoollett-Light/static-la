use crate::*;
use std::{convert::TryInto, fmt::Debug, ops::Neg};

// MatrixDxD
// --------------------------------------------------
impl<T: Neg<Output = T> + Copy> Neg for MatrixDxD<T> {
    type Output = Self;
    fn neg(mut self) -> Self {
        for a in self.data.iter_mut() {
            *a = -*a;
        }
        self
    }
}
// MatrixDxS
// --------------------------------------------------
impl<T: Neg<Output = T> + Copy, const COLUMNS: usize> Neg for MatrixDxS<T, COLUMNS> {
    type Output = Self;
    fn neg(mut self) -> Self {
        for a in self.data.iter_mut() {
            *a = -*a;
        }
        self
    }
}
// MatrixSxD
// --------------------------------------------------
impl<T: Neg<Output = T> + Copy, const ROWS: usize> Neg for MatrixSxD<T, ROWS> {
    type Output = Self;
    fn neg(mut self) -> Self {
        for a in self.data.iter_mut() {
            *a = -*a;
        }
        self
    }
}
// MatrixSxS
// --------------------------------------------------
impl<T: Neg<Output = T> + Copy, const ROWS: usize, const COLUMNS: usize> Neg
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = Self;
    fn neg(mut self) -> Self {
        for a in self.data.iter_mut() {
            *a = -*a;
        }
        self
    }
}
// TransposeDxD
// --------------------------------------------------
impl<'a, T: Neg<Output = T> + Copy> Neg for TransposeDxD<'a, T> {
    type Output = MatrixDxD<T>;
    fn neg(self) -> Self::Output {
        Self::Output {
            data: self.iter().map(|v| -*v).collect(),
            rows: self.rows(),
            columns: self.columns(),
        }
    }
}
// TransposeDxS
// --------------------------------------------------
impl<'a, T: Neg<Output = T> + Copy, const ROWS: usize> Neg for TransposeDxS<'a, T, ROWS> {
    type Output = MatrixSxD<T, ROWS>;
    fn neg(self) -> Self::Output {
        Self::Output {
            data: self.iter().map(|v| -*v).collect(),
            columns: self.columns(),
        }
    }
}
// TransposeSxD
// --------------------------------------------------
impl<'a, T: Neg<Output = T> + Copy, const COLUMNS: usize> Neg for TransposeSxD<'a, T, COLUMNS> {
    type Output = MatrixDxS<T, COLUMNS>;
    fn neg(self) -> Self::Output {
        Self::Output {
            data: self.iter().map(|v| -*v).collect(),
            rows: self.rows(),
        }
    }
}
// TransposeSxS
// --------------------------------------------------
impl<'a, T: Neg<Output = T> + Copy + Debug, const ROWS: usize, const COLUMNS: usize> Neg
    for TransposeSxS<'a, T, COLUMNS, ROWS>
where
    [(); ROWS * COLUMNS]:,
    [(); COLUMNS * ROWS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn neg(self) -> Self::Output {
        Self::Output {
            data: self
                .iter()
                .map(|v| -*v)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
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
        let d = MatrixDxD::try_from(vec![vec![-1, -2, -3], vec![-4, -5, -6]]).unwrap();
        assert_eq!(-a, d);
    }
    // MatrixDxS
    // --------------------------------------------------
    #[test]
    fn dxs() {
        let a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
        let b = MatrixDxS::from(vec![[-1, -2, -3], [-4, -5, -6]]);
        assert_eq!(-a, b);
    }
    // MatrixSxD
    // --------------------------------------------------
    #[test]
    fn sxd() {
        let a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixSxD::try_from([vec![-1, -2, -3], vec![-4, -5, -6]]).unwrap();
        assert_eq!(-a, b);
    }
    // MatrixSxS
    // --------------------------------------------------
    #[test]
    fn sxs() {
        let a = MatrixSxS::<i32, 2, 3>::from([[1, 2, 3], [4, 5, 6]]);
        let b = MatrixSxS::<i32, 2, 3>::from([[-1, -2, -3], [-4, -5, -6]]);
        assert_eq!(-a, b);
    }
    // TransposeDxD
    // --------------------------------------------------
    #[test]
    fn transpose_dxd_dxd() {
        let a = MatrixDxD::try_from(vec![vec![1, 4], vec![2, 5], vec![3, 6]]).unwrap();
        let b = MatrixDxD::try_from(vec![vec![-1, -2, -3], vec![-4, -5, -6]]).unwrap();
        assert_eq!(-a.transpose_ref(), b);
    }
    // TransposeDxS
    // --------------------------------------------------
    #[test]
    fn transpose_dxs_dxs() {
        let a = MatrixSxD::try_from([vec![1, 4], vec![2, 5], vec![3, 6]]).unwrap();
        let b = MatrixDxS::from(vec![[-1, -2, -3], [-4, -5, -6]]);
        assert_eq!(-a.transpose_ref(), b);
    }
    // TransposeSxD
    // --------------------------------------------------
    #[test]
    fn transpose_sxd_sxd() {
        let a = MatrixDxS::from(vec![[1, 4], [2, 5], [3, 6]]);
        let b = MatrixSxD::try_from([vec![-1, -2, -3], vec![-4, -5, -6]]).unwrap();
        assert_eq!(-a.transpose_ref(), b);
    }
    // TransposeSxS
    // --------------------------------------------------
    #[test]
    fn transpose_sxs_sxs() {
        let a = MatrixSxS::<i32, 3, 2>::from([[1, 4], [2, 5], [3, 6]]);
        let b = MatrixSxS::<i32, 2, 3>::from([[-1, -2, -3], [-4, -5, -6]]);
        assert_eq!(-a.transpose_ref(), b);
    }
}
