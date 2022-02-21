use crate::*;
use std::ops::{Div, DivAssign};

// MatrixDxD
// --------------------------------------------------
impl<T: DivAssign + Copy> Div<MatrixDxD<T>> for MatrixDxD<T> {
    type Output = Self;
    fn div(mut self, other: Self) -> Self {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        assert_eq!(self.columns, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a /= *b;
        }
        self
    }
}
impl<T: Div<Output = T> + Copy, const COLUMNS: usize> Div<MatrixDxS<T, COLUMNS>> for MatrixDxD<T> {
    type Output = MatrixDxS<T, COLUMNS>;
    fn div(self, mut other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");
        for (a, b) in self.data.iter().zip(other.data.iter_mut()) {
            *b = *a / *b;
        }
        other
    }
}
impl<T: Div<Output = T> + Copy, const ROWS: usize> Div<MatrixSxD<T, ROWS>> for MatrixDxD<T> {
    type Output = MatrixSxD<T, ROWS>;
    fn div(self, mut other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        assert_eq!(self.columns, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter().zip(other.data.iter_mut()) {
            *b = *a / *b;
        }
        other
    }
}
impl<T: Div<Output = T> + Copy, const ROWS: usize, const COLUMNS: usize>
    Div<MatrixSxS<T, ROWS, COLUMNS>> for MatrixDxD<T>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn div(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");
        for (a, b) in self.data.iter().zip(other.data.iter_mut()) {
            *b = *a / *b;
        }
        other
    }
}
// MatrixDxS
// --------------------------------------------------
impl<T: DivAssign + Copy, const COLUMNS: usize> Div<MatrixDxS<T, COLUMNS>>
    for MatrixDxS<T, COLUMNS>
{
    type Output = Self;
    fn div(mut self, other: Self) -> Self {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a /= *b;
        }
        self
    }
}
impl<T: Div<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    Div<MatrixSxD<T, ROWS>> for MatrixDxS<T, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn div(self, other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");
        let mut data = [Default::default(); ROWS * COLUMNS];
        for i in 0..ROWS * COLUMNS {
            data[i] = self.data[i] / other.data[i];
        }
        Self::Output { data }
    }
}
impl<T: DivAssign + Copy, const COLUMNS: usize> Div<MatrixDxD<T>> for MatrixDxS<T, COLUMNS> {
    type Output = Self;
    fn div(mut self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a /= *b;
        }
        self
    }
}
impl<T: Div<Output = T> + Copy, const ROWS: usize, const COLUMNS: usize>
    Div<MatrixSxS<T, ROWS, COLUMNS>> for MatrixDxS<T, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn div(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        for (a, b) in self.data.iter().zip(other.data.iter_mut()) {
            *b = *a / *b;
        }
        other
    }
}
// MatrixSxD
// --------------------------------------------------
impl<T: DivAssign + Copy, const ROWS: usize> Div<MatrixSxD<T, ROWS>> for MatrixSxD<T, ROWS> {
    type Output = Self;
    fn div(mut self, other: Self) -> Self {
        assert_eq!(self.columns, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a /= *b;
        }
        self
    }
}
impl<T: Div<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    Div<MatrixDxS<T, COLUMNS>> for MatrixSxD<T, ROWS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn div(self, other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");
        let mut data = [Default::default(); ROWS * COLUMNS];
        for i in 0..ROWS * COLUMNS {
            data[i] = self.data[i] / other.data[i];
        }
        Self::Output { data }
    }
}
impl<T: DivAssign + Copy, const ROWS: usize> Div<MatrixDxD<T>> for MatrixSxD<T, ROWS> {
    type Output = Self;
    fn div(mut self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(self.columns, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a /= *b;
        }
        self
    }
}
impl<T: Div<Output = T> + Copy, const ROWS: usize, const COLUMNS: usize>
    Div<MatrixSxS<T, ROWS, COLUMNS>> for MatrixSxD<T, ROWS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn div(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");
        for (a, b) in self.data.iter().zip(other.data.iter_mut()) {
            *b = *a / *b;
        }
        other
    }
}
// MatrixSxS
// --------------------------------------------------
impl<T: DivAssign + Copy, const ROWS: usize, const COLUMNS: usize> Div<MatrixSxS<T, ROWS, COLUMNS>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = Self;
    fn div(mut self, other: Self) -> Self {
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a /= *b;
        }
        self
    }
}
impl<T: DivAssign + Copy, const ROWS: usize, const COLUMNS: usize> Div<MatrixDxS<T, COLUMNS>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = Self;
    fn div(mut self, other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a /= *b;
        }
        self
    }
}
impl<T: DivAssign + Copy, const ROWS: usize, const COLUMNS: usize> Div<MatrixSxD<T, ROWS>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = Self;
    fn div(mut self, other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a /= *b;
        }
        self
    }
}
impl<T: DivAssign + Copy, const ROWS: usize, const COLUMNS: usize> Div<MatrixDxD<T>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = Self;
    fn div(mut self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a /= *b;
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
        let a = MatrixDxD::try_from(vec![vec![1., 2., 3.], vec![4., 5., 6.]]).unwrap();
        let b = MatrixDxD::try_from(vec![vec![7., 8., 9.], vec![10., 11., 12.]]).unwrap();
        let c = a / b;
        let d = MatrixDxD::try_from(vec![
            vec![1. / 7., 2. / 8., 3. / 9.],
            vec![4. / 10., 5. / 11., 6. / 12.],
        ])
        .unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn dxd_dxs() {
        let a = MatrixDxD::try_from(vec![vec![1., 2., 3.], vec![4., 5., 6.]]).unwrap();
        let b = MatrixDxS::from(vec![[7., 8., 9.], [10., 11., 12.]]);
        let c = a / b;
        let d = MatrixDxS::from(vec![
            [1. / 7., 2. / 8., 3. / 9.],
            [4. / 10., 5. / 11., 6. / 12.],
        ]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxd_sxd() {
        let a = MatrixDxD::try_from(vec![vec![1., 2., 3.], vec![4., 5., 6.]]).unwrap();
        let b = MatrixSxD::try_from([vec![7., 8., 9.], vec![10., 11., 12.]]).unwrap();
        let c = a / b;
        let d = MatrixSxD::try_from([
            vec![1. / 7., 2. / 8., 3. / 9.],
            vec![4. / 10., 5. / 11., 6. / 12.],
        ])
        .unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn dxd_sxs() {
        let a = MatrixDxD::try_from(vec![vec![1., 2., 3.], vec![4., 5., 6.]]).unwrap();
        let b = MatrixSxS::from([[7., 8., 9.], [10., 11., 12.]]);
        let c = a / b;
        let d = MatrixSxS::from([[1. / 7., 2. / 8., 3. / 9.], [4. / 10., 5. / 11., 6. / 12.]]);
        assert_eq!(c, d);
    }
    // MatrixDxS
    // --------------------------------------------------
    #[test]
    fn dxs_dxs() {
        let a = MatrixDxS::from(vec![[1., 2., 3.], [4., 5., 6.]]);
        let b = MatrixDxS::from(vec![[7., 8., 9.], [10., 11., 12.]]);
        let c = a / b;
        let d = MatrixDxS::from(vec![
            [1. / 7., 2. / 8., 3. / 9.],
            [4. / 10., 5. / 11., 6. / 12.],
        ]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxs_sxd() {
        let a = MatrixDxS::from(vec![[1., 2., 3.], [4., 5., 6.]]);
        let b = MatrixSxD::try_from([vec![7., 8., 9.], vec![10., 11., 12.]]).unwrap();
        let c = a / b;
        let d = MatrixSxS::from([[1. / 7., 2. / 8., 3. / 9.], [4. / 10., 5. / 11., 6. / 12.]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxs_dxd() {
        let a = MatrixDxS::from(vec![[1., 2., 3.], [4., 5., 6.]]);
        let b = MatrixDxD::try_from(vec![vec![7., 8., 9.], vec![10., 11., 12.]]).unwrap();
        let c = a / b;
        let d = MatrixDxS::from(vec![
            [1. / 7., 2. / 8., 3. / 9.],
            [4. / 10., 5. / 11., 6. / 12.],
        ]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxs_sxs() {
        let a = MatrixDxS::from(vec![[1., 2., 3.], [4., 5., 6.]]);
        let b = MatrixSxS::from([[7., 8., 9.], [10., 11., 12.]]);
        let c = a / b;
        let d = MatrixSxS::from([[1. / 7., 2. / 8., 3. / 9.], [4. / 10., 5. / 11., 6. / 12.]]);
        assert_eq!(c, d);
    }
    // MatrixSxD
    // --------------------------------------------------
    #[test]
    fn sxd_sxd() {
        let a = MatrixSxD::try_from([vec![1., 2., 3.], vec![4., 5., 6.]]).unwrap();
        let b = MatrixSxD::try_from([vec![7., 8., 9.], vec![10., 11., 12.]]).unwrap();
        let c = a / b;
        let d = MatrixSxD::try_from([
            vec![1. / 7., 2. / 8., 3. / 9.],
            vec![4. / 10., 5. / 11., 6. / 12.],
        ])
        .unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn sxd_dxs() {
        let a = MatrixSxD::try_from([vec![1., 2., 3.], vec![4., 5., 6.]]).unwrap();
        let b = MatrixDxS::from(vec![[7., 8., 9.], [10., 11., 12.]]);
        let c = a / b;
        let d = MatrixSxS::from([[1. / 7., 2. / 8., 3. / 9.], [4. / 10., 5. / 11., 6. / 12.]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxd_sxs() {
        let a = MatrixSxD::try_from([vec![1., 2., 3.], vec![4., 5., 6.]]).unwrap();
        let b = MatrixSxS::from([[7., 8., 9.], [10., 11., 12.]]);
        let c = a / b;
        let d = MatrixSxS::from([[1. / 7., 2. / 8., 3. / 9.], [4. / 10., 5. / 11., 6. / 12.]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxd_dxd() {
        let a = MatrixSxD::try_from([vec![1., 2., 3.], vec![4., 5., 6.]]).unwrap();
        let b = MatrixDxD::try_from(vec![vec![7., 8., 9.], vec![10., 11., 12.]]).unwrap();
        let c = a / b;
        let d = MatrixSxD::try_from([
            vec![1. / 7., 2. / 8., 3. / 9.],
            vec![4. / 10., 5. / 11., 6. / 12.],
        ])
        .unwrap();
        assert_eq!(c, d);
    }
    // MatrixSxS
    // --------------------------------------------------
    #[test]
    fn sxs_sxs() {
        let a = MatrixSxS::from([[1., 2., 3.], [4., 5., 6.]]);
        let b = MatrixSxS::from([[7., 8., 9.], [10., 11., 12.]]);
        let c = a / b;
        let d = MatrixSxS::from([[1. / 7., 2. / 8., 3. / 9.], [4. / 10., 5. / 11., 6. / 12.]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxs_dxs() {
        let a = MatrixSxS::from([[1., 2., 3.], [4., 5., 6.]]);
        let b = MatrixDxS::from(vec![[7., 8., 9.], [10., 11., 12.]]);
        let c = a / b;
        let d = MatrixSxS::from([[1. / 7., 2. / 8., 3. / 9.], [4. / 10., 5. / 11., 6. / 12.]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxs_sxd() {
        let a = MatrixSxS::from([[1., 2., 3.], [4., 5., 6.]]);
        let b = MatrixSxD::try_from([vec![7., 8., 9.], vec![10., 11., 12.]]).unwrap();
        let c = a / b;
        let d = MatrixSxS::from([[1. / 7., 2. / 8., 3. / 9.], [4. / 10., 5. / 11., 6. / 12.]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxs_dxd() {
        let a = MatrixSxS::from([[1., 2., 3.], [4., 5., 6.]]);
        let b = MatrixDxD::try_from(vec![vec![7., 8., 9.], vec![10., 11., 12.]]).unwrap();
        let c = a / b;
        let d = MatrixSxS::from([[1. / 7., 2. / 8., 3. / 9.], [4. / 10., 5. / 11., 6. / 12.]]);
        assert_eq!(c, d);
    }
}
