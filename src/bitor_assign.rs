use crate::*;
use std::ops::BitOrAssign;

// MatrixDxD
// --------------------------------------------------
impl<T: BitOrAssign + Copy> BitOrAssign<MatrixDxD<T>> for MatrixDxD<T> {
    fn bitor_assign(&mut self, other: Self) {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        assert_eq!(self.columns, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a |= *b;
        }
    }
}
impl<T: BitOrAssign + Copy, const COLUMNS: usize> BitOrAssign<MatrixDxS<T, COLUMNS>>
    for MatrixDxD<T>
{
    fn bitor_assign(&mut self, other: MatrixDxS<T, COLUMNS>) {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a |= *b;
        }
    }
}
impl<T: BitOrAssign + Copy, const ROWS: usize> BitOrAssign<MatrixSxD<T, ROWS>> for MatrixDxD<T> {
    fn bitor_assign(&mut self, other: MatrixSxD<T, ROWS>) {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        assert_eq!(self.columns, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a |= *b;
        }
    }
}
impl<T: BitOrAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    BitOrAssign<MatrixSxS<T, ROWS, COLUMNS>> for MatrixDxD<T>
where
    [(); ROWS * COLUMNS]:,
{
    fn bitor_assign(&mut self, other: MatrixSxS<T, ROWS, COLUMNS>) {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a |= *b;
        }
    }
}
// MatrixDxS
// --------------------------------------------------
impl<T: BitOrAssign + Copy, const COLUMNS: usize> BitOrAssign<MatrixDxS<T, COLUMNS>>
    for MatrixDxS<T, COLUMNS>
{
    fn bitor_assign(&mut self, other: Self) {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a |= *b;
        }
    }
}
impl<T: BitOrAssign + Copy, const ROWS: usize, const COLUMNS: usize> BitOrAssign<MatrixSxD<T, ROWS>>
    for MatrixDxS<T, COLUMNS>
{
    fn bitor_assign(&mut self, other: MatrixSxD<T, ROWS>) {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a |= *b;
        }
    }
}
impl<T: BitOrAssign + Copy, const COLUMNS: usize> BitOrAssign<MatrixDxD<T>>
    for MatrixDxS<T, COLUMNS>
{
    fn bitor_assign(&mut self, other: MatrixDxD<T>) {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a |= *b;
        }
    }
}
impl<T: BitOrAssign + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    BitOrAssign<MatrixSxS<T, ROWS, COLUMNS>> for MatrixDxS<T, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn bitor_assign(&mut self, other: MatrixSxS<T, ROWS, COLUMNS>) {
        assert_eq!(self.rows, ROWS, "Non-matching rows");

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a |= *b;
        }
    }
}
// MatrixSxD
// --------------------------------------------------
impl<T: BitOrAssign + Copy, const ROWS: usize> BitOrAssign<MatrixSxD<T, ROWS>>
    for MatrixSxD<T, ROWS>
{
    fn bitor_assign(&mut self, other: Self) {
        assert_eq!(self.columns, other.columns, "Non-matching columns");

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a |= *b;
        }
    }
}
impl<T: BitOrAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    BitOrAssign<MatrixDxS<T, COLUMNS>> for MatrixSxD<T, ROWS>
{
    fn bitor_assign(&mut self, other: MatrixDxS<T, COLUMNS>) {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a |= *b;
        }
    }
}
impl<T: BitOrAssign + Copy, const ROWS: usize> BitOrAssign<MatrixDxD<T>> for MatrixSxD<T, ROWS> {
    fn bitor_assign(&mut self, other: MatrixDxD<T>) {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(self.columns, other.columns, "Non-matching columns");

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a |= *b;
        }
    }
}
impl<T: BitOrAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    BitOrAssign<MatrixSxS<T, ROWS, COLUMNS>> for MatrixSxD<T, ROWS>
where
    [(); ROWS * COLUMNS]:,
{
    fn bitor_assign(&mut self, other: MatrixSxS<T, ROWS, COLUMNS>) {
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a |= *b;
        }
    }
}
// MatrixSxS
// --------------------------------------------------
impl<T: BitOrAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    BitOrAssign<MatrixSxS<T, ROWS, COLUMNS>> for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn bitor_assign(&mut self, other: Self) {
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a |= *b;
        }
    }
}
impl<T: BitOrAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    BitOrAssign<MatrixDxS<T, COLUMNS>> for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn bitor_assign(&mut self, other: MatrixDxS<T, COLUMNS>) {
        assert_eq!(ROWS, other.rows, "Non-matching rows");

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a |= *b;
        }
    }
}
impl<T: BitOrAssign + Copy, const ROWS: usize, const COLUMNS: usize> BitOrAssign<MatrixSxD<T, ROWS>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn bitor_assign(&mut self, other: MatrixSxD<T, ROWS>) {
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a |= *b;
        }
    }
}
impl<T: BitOrAssign + Copy, const ROWS: usize, const COLUMNS: usize> BitOrAssign<MatrixDxD<T>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn bitor_assign(&mut self, other: MatrixDxD<T>) {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a |= *b;
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
    fn dxd_dxd() {
        let mut a =
            MatrixDxD::try_from(vec![vec![true, true, false], vec![true, false, false]]).unwrap();
        let b =
            MatrixDxD::try_from(vec![vec![false, true, false], vec![true, false, true]]).unwrap();
        a |= b;
        let d =
            MatrixDxD::try_from(vec![vec![true, true, false], vec![true, false, true]]).unwrap();
        assert_eq!(a, d);
    }
    #[test]
    fn dxd_dxs() {
        let mut a =
            MatrixDxD::try_from(vec![vec![true, true, false], vec![true, false, false]]).unwrap();
        let b = MatrixDxS::from(vec![[false, true, false], [true, false, true]]);
        a |= b;
        let d = MatrixDxS::from(vec![[true, true, false], [true, false, true]]);
        assert_eq!(a, d);
    }
    #[test]
    fn dxd_sxd() {
        let mut a =
            MatrixDxD::try_from(vec![vec![true, true, false], vec![true, false, false]]).unwrap();
        let b = MatrixSxD::try_from([vec![false, true, false], vec![true, false, true]]).unwrap();
        a |= b;
        let d = MatrixSxD::try_from([vec![true, true, false], vec![true, false, true]]).unwrap();
        assert_eq!(a, d);
    }
    #[test]
    fn dxd_sxs() {
        let mut a =
            MatrixDxD::try_from(vec![vec![true, true, false], vec![true, false, false]]).unwrap();
        let b = MatrixSxS::from([[false, true, false], [true, false, true]]);
        a |= b;
        let d = MatrixSxS::from([[true, true, false], [true, false, true]]);
        assert_eq!(a, d);
    }
    // MatrixDxS
    // --------------------------------------------------
    #[test]
    fn dxs_dxs() {
        let mut a = MatrixDxS::from(vec![[true, true, false], [true, false, false]]);
        let b = MatrixDxS::from(vec![[false, true, false], [true, false, true]]);
        a |= b;
        let d = MatrixDxS::from(vec![[true, true, false], [true, false, true]]);
        assert_eq!(a, d);
    }
    #[test]
    fn dxs_sxd() {
        let mut a = MatrixDxS::from(vec![[true, true, false], [true, false, false]]);
        let b = MatrixSxD::try_from([vec![false, true, false], vec![true, false, true]]).unwrap();
        a |= b;
        let d = MatrixSxS::from([[true, true, false], [true, false, true]]);
        assert_eq!(a, d);
    }
    #[test]
    fn dxs_dxd() {
        let mut a = MatrixDxS::from(vec![[true, true, false], [true, false, false]]);
        let b =
            MatrixDxD::try_from(vec![vec![false, true, false], vec![true, false, true]]).unwrap();
        a |= b;
        let d = MatrixDxS::from(vec![[true, true, false], [true, false, true]]);
        assert_eq!(a, d);
    }
    #[test]
    fn dxs_sxs() {
        let mut a = MatrixDxS::from(vec![[true, true, false], [true, false, false]]);
        let b = MatrixSxS::from([[false, true, false], [true, false, true]]);
        a |= b;
        let d = MatrixSxS::from([[true, true, false], [true, false, true]]);
        assert_eq!(a, d);
    }
    // MatrixSxD
    // --------------------------------------------------
    #[test]
    fn sxd_sxd() {
        let mut a =
            MatrixSxD::try_from([vec![true, true, false], vec![true, false, false]]).unwrap();
        let b = MatrixSxD::try_from([vec![false, true, false], vec![true, false, true]]).unwrap();
        a |= b;
        let d = MatrixSxD::try_from([vec![true, true, false], vec![true, false, true]]).unwrap();
        assert_eq!(a, d);
    }
    #[test]
    fn sxd_dxs() {
        let mut a =
            MatrixSxD::try_from([vec![true, true, false], vec![true, false, false]]).unwrap();
        let b = MatrixDxS::from(vec![[false, true, false], [true, false, true]]);
        a |= b;
        let d = MatrixSxS::from([[true, true, false], [true, false, true]]);
        assert_eq!(a, d);
    }
    #[test]
    fn sxd_sxs() {
        let mut a =
            MatrixSxD::try_from([vec![true, true, false], vec![true, false, false]]).unwrap();
        let b = MatrixSxS::from([[false, true, false], [true, false, true]]);
        a |= b;
        let d = MatrixSxS::from([[true, true, false], [true, false, true]]);
        assert_eq!(a, d);
    }
    #[test]
    fn sxd_dxd() {
        let mut a =
            MatrixSxD::try_from([vec![true, true, false], vec![true, false, false]]).unwrap();
        let b =
            MatrixDxD::try_from(vec![vec![false, true, false], vec![true, false, true]]).unwrap();
        a |= b;
        let d = MatrixSxD::try_from([vec![true, true, false], vec![true, false, true]]).unwrap();
        assert_eq!(a, d);
    }
    // MatrixSxS
    // --------------------------------------------------
    #[test]
    fn sxs_sxs() {
        let mut a = MatrixSxS::<bool, 2, 3>::from([[true, true, false], [true, false, false]]);
        let b = MatrixSxS::<bool, 2, 3>::from([[false, true, false], [true, false, true]]);
        a |= b;
        let d = MatrixSxS::<bool, 2, 3>::from([[true, true, false], [true, false, true]]);
        assert_eq!(a, d);
    }
    #[test]
    fn sxs_dxs() {
        let mut a = MatrixSxS::from([[true, true, false], [true, false, false]]);
        let b = MatrixDxS::from(vec![[false, true, false], [true, false, true]]);
        a |= b;
        let d = MatrixSxS::from([[true, true, false], [true, false, true]]);
        assert_eq!(a, d);
    }
    #[test]
    fn sxs_sxd() {
        let mut a = MatrixSxS::from([[true, true, false], [true, false, false]]);
        let b = MatrixSxD::try_from([vec![false, true, false], vec![true, false, true]]).unwrap();
        a |= b;
        let d = MatrixSxS::from([[true, true, false], [true, false, true]]);
        assert_eq!(a, d);
    }
    #[test]
    fn sxs_dxd() {
        let mut a = MatrixSxS::from([[true, true, false], [true, false, false]]);
        let b =
            MatrixDxD::try_from(vec![vec![false, true, false], vec![true, false, true]]).unwrap();
        a |= b;
        let d = MatrixSxS::from([[true, true, false], [true, false, true]]);
        assert_eq!(a, d);
    }
}
