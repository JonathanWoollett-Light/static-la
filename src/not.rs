use crate::*;
use std::{convert::TryInto, fmt::Debug, ops::Not};

// MatrixDxD
// --------------------------------------------------
impl<T: Not<Output = T> + Copy> Not for MatrixDxD<T> {
    type Output = Self;
    fn not(mut self) -> Self {
        for a in self.data.iter_mut() {
            *a = !*a;
        }
        self
    }
}
// MatrixDxS
// --------------------------------------------------
impl<T: Not<Output = T> + Copy, const COLUMNS: usize> Not for MatrixDxS<T, COLUMNS> {
    type Output = Self;
    fn not(mut self) -> Self {
        for a in self.data.iter_mut() {
            *a = !*a;
        }
        self
    }
}
// MatrixSxD
// --------------------------------------------------
impl<T: Not<Output = T> + Copy, const ROWS: usize> Not for MatrixSxD<T, ROWS> {
    type Output = Self;
    fn not(mut self) -> Self {
        for a in self.data.iter_mut() {
            *a = !*a;
        }
        self
    }
}
// MatrixSxS
// --------------------------------------------------
impl<T: Not<Output = T> + Copy, const ROWS: usize, const COLUMNS: usize> Not
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = Self;
    fn not(mut self) -> Self {
        for a in self.data.iter_mut() {
            *a = !*a;
        }
        self
    }
}
// TransposeDxD
// --------------------------------------------------
impl<'a, T: Not<Output = T> + Copy> Not for TransposeDxD<'a, T> {
    type Output = MatrixDxD<T>;
    fn not(self) -> Self::Output {
        Self::Output {
            data: self.iter().map(|v| !*v).collect(),
            rows: self.rows(),
            columns: self.columns(),
        }
    }
}
// TransposeDxS
// --------------------------------------------------
impl<'a, T: Not<Output = T> + Copy, const ROWS: usize> Not for TransposeDxS<'a, T, ROWS> {
    type Output = MatrixSxD<T, ROWS>;
    fn not(self) -> Self::Output {
        Self::Output {
            data: self.iter().map(|v| !*v).collect(),
            columns: self.columns(),
        }
    }
}
// TransposeSxD
// --------------------------------------------------
impl<'a, T: Not<Output = T> + Copy, const COLUMNS: usize> Not for TransposeSxD<'a, T, COLUMNS> {
    type Output = MatrixDxS<T, COLUMNS>;
    fn not(self) -> Self::Output {
        Self::Output {
            data: self.iter().map(|v| !*v).collect(),
            rows: self.rows(),
        }
    }
}
// TransposeSxS
// --------------------------------------------------
impl<'a, T: Not<Output = T> + Copy + Debug, const ROWS: usize, const COLUMNS: usize> Not
    for TransposeSxS<'a, T, COLUMNS, ROWS>
where
    [(); ROWS * COLUMNS]:,
    [(); COLUMNS * ROWS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn not(self) -> Self::Output {
        Self::Output {
            data: self
                .iter()
                .map(|v| !*v)
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
        let a =
            MatrixDxD::try_from(vec![vec![true, true, false], vec![true, false, false]]).unwrap();
        let d =
            MatrixDxD::try_from(vec![vec![false, false, true], vec![false, true, true]]).unwrap();
        assert_eq!(!a, d);
    }
    // MatrixDxS
    // --------------------------------------------------
    #[test]
    fn dxs() {
        let a = MatrixDxS::from(vec![[true, true, false], [true, false, false]]);
        let b = MatrixDxS::from(vec![[false, false, true], [false, true, true]]);
        assert_eq!(!a, b);
    }
    // MatrixSxD
    // --------------------------------------------------
    #[test]
    fn sxd() {
        let a = MatrixSxD::try_from([vec![true, true, false], vec![true, false, false]]).unwrap();
        let b = MatrixSxD::try_from([vec![false, false, true], vec![false, true, true]]).unwrap();
        assert_eq!(!a, b);
    }
    // MatrixSxS
    // --------------------------------------------------
    #[test]
    fn sxs() {
        let a = MatrixSxS::<bool, 2, 3>::from([[true, true, false], [true, false, false]]);
        let b = MatrixSxS::<bool, 2, 3>::from([[false, false, true], [false, true, true]]);
        assert_eq!(!a, b);
    }
    // TransposeDxD
    // --------------------------------------------------
    #[test]
    fn transpose_dxd_dxd() {
        let a = MatrixDxD::try_from(vec![
            vec![true, true],
            vec![true, false],
            vec![false, false],
        ])
        .unwrap();
        let b =
            MatrixDxD::try_from(vec![vec![false, false, true], vec![false, true, true]]).unwrap();
        assert_eq!(!a.transpose_ref(), b);
    }
    // TransposeDxS
    // --------------------------------------------------
    #[test]
    fn transpose_dxs_dxs() {
        let a =
            MatrixSxD::try_from([vec![true, true], vec![true, false], vec![false, false]]).unwrap();
        let b = MatrixDxS::from(vec![[false, false, true], [false, true, true]]);
        assert_eq!(!a.transpose_ref(), b);
    }
    // TransposeSxD
    // --------------------------------------------------
    #[test]
    fn transpose_sxd_sxd() {
        let a = MatrixDxS::from(vec![[true, true], [true, false], [false, false]]);
        let b = MatrixSxD::try_from([vec![false, false, true], vec![false, true, true]]).unwrap();
        assert_eq!(!a.transpose_ref(), b);
    }
    // TransposeSxS
    // --------------------------------------------------
    #[test]
    fn transpose_sxs_sxs() {
        let a = MatrixSxS::<bool, 3, 2>::from([[true, true], [true, false], [false, false]]);
        let b = MatrixSxS::<bool, 2, 3>::from([[false, false, true], [false, true, true]]);
        assert_eq!(!a.transpose_ref(), b);
    }
}
