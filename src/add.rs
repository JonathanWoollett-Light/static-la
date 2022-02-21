use crate::*;
use std::ops::{Add, AddAssign};

// MatrixDxD
// --------------------------------------------------
impl<T: AddAssign + Copy> Add<MatrixDxD<T>> for MatrixDxD<T> {
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        assert_eq!(self.columns, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
        self
    }
}
impl<T: AddAssign + Copy, const COLUMNS: usize> Add<MatrixDxS<T, COLUMNS>> for MatrixDxD<T> {
    type Output = MatrixDxS<T, COLUMNS>;
    fn add(self, mut other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");
        for (a, b) in self.data.iter().zip(other.data.iter_mut()) {
            *b += *a;
        }
        other
    }
}
impl<T: AddAssign + Copy, const ROWS: usize> Add<MatrixSxD<T, ROWS>> for MatrixDxD<T> {
    type Output = MatrixSxD<T, ROWS>;
    fn add(self, mut other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        assert_eq!(self.columns, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter().zip(other.data.iter_mut()) {
            *b += *a;
        }
        other
    }
}
impl<T: AddAssign + Copy, const ROWS: usize, const COLUMNS: usize> Add<MatrixSxS<T, ROWS, COLUMNS>>
    for MatrixDxD<T>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn add(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");
        for (a, b) in self.data.iter().zip(other.data.iter_mut()) {
            *b += *a;
        }
        other
    }
}
// MatrixDxS
// --------------------------------------------------
impl<T: AddAssign + Copy, const COLUMNS: usize> Add<MatrixDxS<T, COLUMNS>>
    for MatrixDxS<T, COLUMNS>
{
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
        self
    }
}
impl<T: Add<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    Add<MatrixSxD<T, ROWS>> for MatrixDxS<T, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn add(self, other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");
        let mut data = [Default::default(); ROWS * COLUMNS];
        for i in 0..ROWS * COLUMNS {
            data[i] = self.data[i] + other.data[i];
        }
        Self::Output { data }
    }
}
impl<T: AddAssign + Copy, const COLUMNS: usize> Add<MatrixDxD<T>> for MatrixDxS<T, COLUMNS> {
    type Output = Self;
    fn add(mut self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
        self
    }
}
impl<T: AddAssign + Copy, const ROWS: usize, const COLUMNS: usize> Add<MatrixSxS<T, ROWS, COLUMNS>>
    for MatrixDxS<T, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn add(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        for (a, b) in self.data.iter().zip(other.data.iter_mut()) {
            *b += *a;
        }
        other
    }
}
// MatrixSxD
// --------------------------------------------------
impl<T: AddAssign + Copy, const ROWS: usize> Add<MatrixSxD<T, ROWS>> for MatrixSxD<T, ROWS> {
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        assert_eq!(self.columns, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
        self
    }
}
impl<T: Add<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    Add<MatrixDxS<T, COLUMNS>> for MatrixSxD<T, ROWS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn add(self, other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");
        let mut data = [Default::default(); ROWS * COLUMNS];
        for i in 0..ROWS * COLUMNS {
            data[i] = self.data[i] + other.data[i];
        }
        Self::Output { data }
    }
}
impl<T: AddAssign + Copy, const ROWS: usize> Add<MatrixDxD<T>> for MatrixSxD<T, ROWS> {
    type Output = Self;
    fn add(mut self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(self.columns, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
        self
    }
}
impl<T: AddAssign + Copy, const ROWS: usize, const COLUMNS: usize> Add<MatrixSxS<T, ROWS, COLUMNS>>
    for MatrixSxD<T, ROWS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn add(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");
        for (a, b) in self.data.iter().zip(other.data.iter_mut()) {
            *b += *a;
        }
        other
    }
}
// MatrixSxS
// --------------------------------------------------
impl<T: AddAssign + Copy, const ROWS: usize, const COLUMNS: usize> Add<MatrixSxS<T, ROWS, COLUMNS>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
        self
    }
}
impl<T: AddAssign + Copy, const ROWS: usize, const COLUMNS: usize> Add<MatrixDxS<T, COLUMNS>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = Self;
    fn add(mut self, other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
        self
    }
}
impl<T: AddAssign + Copy, const ROWS: usize, const COLUMNS: usize> Add<MatrixSxD<T, ROWS>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = Self;
    fn add(mut self, other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
        self
    }
}
impl<T: AddAssign + Copy, const ROWS: usize, const COLUMNS: usize> Add<MatrixDxD<T>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = Self;
    fn add(mut self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
        self
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
        let a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a + b;
        let d = MatrixDxD::try_from(vec![vec![8, 10, 12], vec![14, 16, 18]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn dxd_dxs() {
        let a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        let c = a + b;
        let d = MatrixDxS::from(vec![[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxd_sxd() {
        let a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a + b;
        let d = MatrixSxD::try_from([vec![8, 10, 12], vec![14, 16, 18]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn dxd_sxs() {
        let a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
        let c = a + b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
    // MatrixDxS
    // --------------------------------------------------
    #[test]
    fn dxs_dxs() {
        let a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        let c = a + b;
        let d = MatrixDxS::from(vec![[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxs_sxd() {
        let a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a + b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxs_dxd() {
        let a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a + b;
        let d = MatrixDxS::from(vec![[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxs_sxs() {
        let a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
        let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
        let c = a + b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
    // MatrixSxD
    // --------------------------------------------------
    #[test]
    fn sxd_sxd() {
        let a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a + b;
        let d = MatrixSxD::try_from([vec![8, 10, 12], vec![14, 16, 18]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn sxd_dxs() {
        let a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        let c = a + b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxd_sxs() {
        let a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
        let c = a + b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxd_dxd() {
        let a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a + b;
        let d = MatrixSxD::try_from([vec![8, 10, 12], vec![14, 16, 18]]).unwrap();
        assert_eq!(c, d);
    }
    // MatrixSxS
    // --------------------------------------------------
    #[test]
    fn sxs_sxs() {
        let a = MatrixSxS::<i32, 2, 3>::from([[1, 2, 3], [4, 5, 6]]);
        let b = MatrixSxS::<i32, 2, 3>::from([[7, 8, 9], [10, 11, 12]]);
        let c = a + b;
        let d = MatrixSxS::<i32, 2, 3>::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxs_dxs() {
        let a = MatrixSxS::from([[1, 2, 3], [4, 5, 6]]);
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        let c = a + b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxs_sxd() {
        let a = MatrixSxS::from([[1, 2, 3], [4, 5, 6]]);
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a + b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxs_dxd() {
        let a = MatrixSxS::from([[1, 2, 3], [4, 5, 6]]);
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a + b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
}
