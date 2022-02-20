use crate::*;
use std::ops::{Mul, MulAssign};

// MatrixDxD
// --------------------------------------------------
impl<T: MulAssign + Copy> Mul<MatrixDxD<T>> for MatrixDxD<T> {
    type Output = Self;
    fn mul(mut self, other: Self) -> Self {
        assert_eq!(self.data.len(), other.data.len(), "Non-matching rows");
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(
            self.data[0].len(),
            other.data[0].len(),
            "Non-matching columns"
        );
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            for (x, y) in a.iter_mut().zip(b.iter()) {
                *x *= *y;
            }
        }
        self
    }
}
impl<T: Mul<Output = T> + Copy, const COLUMNS: usize> Mul<MatrixDxS<T, COLUMNS>> for MatrixDxD<T> {
    type Output = MatrixDxS<T, COLUMNS>;
    fn mul(self, mut other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(self.data.len(), other.data.len(), "Non-matching rows");
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(COLUMNS, other.data[0].len(), "Non-matching columns");
        for (a, b) in self.data.iter().zip(other.data.iter_mut()) {
            for i in 0..COLUMNS {
                b[i] = a[i] * b[i];
            }
        }
        other
    }
}
impl<T: Mul<Output = T> + Copy, const ROWS: usize> Mul<MatrixSxD<T, ROWS>> for MatrixDxD<T> {
    type Output = MatrixSxD<T, ROWS>;
    fn mul(self, mut other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(ROWS, other.data.len(), "Non-matching rows");
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(
            self.data[0].len(),
            other.data[0].len(),
            "Non-matching columns"
        );
        for i in 0..ROWS {
            for (a, b) in self.data[i].iter().zip(other.data[i].iter_mut()) {
                *b = *a * *b;
            }
        }
        other
    }
}
impl<T: Mul<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    Mul<MatrixSxS<T, ROWS, COLUMNS>> for MatrixDxD<T>
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn mul(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.data.len(), ROWS, "Non-matching rows");
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(self.data[0].len(), COLUMNS, "Non-matching columns");
        for i in 0..ROWS {
            for j in 0..COLUMNS {
                other.data[i][j] = self.data[i][j] * other.data[i][j];
            }
        }
        other
    }
}
// MatrixDxS
// --------------------------------------------------
impl<T: MulAssign + Copy, const COLUMNS: usize> Mul<MatrixDxS<T, COLUMNS>>
    for MatrixDxS<T, COLUMNS>
{
    type Output = Self;
    fn mul(mut self, other: Self) -> Self {
        assert_eq!(self.data.len(), other.data.len(), "Non-matching rows");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            for i in 0..COLUMNS {
                a[i] *= b[i];
            }
        }
        self
    }
}
impl<T: Mul<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    Mul<MatrixSxD<T, ROWS>> for MatrixDxS<T, COLUMNS>
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn mul(self, other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(self.data.len(), ROWS, "Non-matching rows");
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(COLUMNS, other.data[0].len(), "Non-matching columns");
        let mut data = [[Default::default(); COLUMNS]; ROWS];
        for i in 0..ROWS {
            for j in 0..COLUMNS {
                data[i][j] = self.data[i][j] * other.data[i][j];
            }
        }
        Self::Output { data }
    }
}
impl<T: MulAssign + Copy, const COLUMNS: usize> Mul<MatrixDxD<T>> for MatrixDxS<T, COLUMNS> {
    type Output = Self;
    fn mul(mut self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(self.data.len(), other.data.len(), "Non-matching rows");
        assert_eq!(COLUMNS, other.data[0].len(), "Non-matching columns");
        for (x, y) in self.data.iter_mut().zip(other.data.iter()) {
            for i in 0..COLUMNS {
                x[i] *= y[i];
            }
        }
        self
    }
}
impl<T: Mul<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    Mul<MatrixSxS<T, ROWS, COLUMNS>> for MatrixDxS<T, COLUMNS>
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn mul(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.data.len(), ROWS, "Non-matching rows");
        for i in 0..ROWS {
            for j in 0..COLUMNS {
                other.data[i][j] = self.data[i][j] * other.data[i][j];
            }
        }
        other
    }
}
// MatrixSxD
// --------------------------------------------------
impl<T: MulAssign + Copy, const ROWS: usize> Mul<MatrixSxD<T, ROWS>> for MatrixSxD<T, ROWS> {
    type Output = Self;
    fn mul(mut self, other: Self) -> Self {
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(
            self.data[0].len(),
            other.data[0].len(),
            "Non-matching columns"
        );
        for i in 0..ROWS {
            for (x, y) in self.data[i].iter_mut().zip(other.data[i].iter()) {
                *x *= *y;
            }
        }
        self
    }
}
impl<T: Mul<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    Mul<MatrixDxS<T, COLUMNS>> for MatrixSxD<T, ROWS>
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn mul(self, other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(ROWS, other.data.len(), "Non-matching rows");
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(self.data[0].len(), COLUMNS, "Non-matching columns");
        let mut data = [[Default::default(); COLUMNS]; ROWS];
        for i in 0..ROWS {
            for j in 0..COLUMNS {
                data[i][j] = self.data[i][j] * other.data[i][j];
            }
        }
        Self::Output { data }
    }
}
impl<T: MulAssign + Copy, const ROWS: usize> Mul<MatrixDxD<T>> for MatrixSxD<T, ROWS> {
    type Output = Self;
    fn mul(mut self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(ROWS, other.data.len(), "Non-matching rows");
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(
            self.data[0].len(),
            other.data[0].len(),
            "Non-matching columns"
        );
        for i in 0..ROWS {
            for (x, y) in self.data[i].iter_mut().zip(other.data[i].iter()) {
                *x *= *y;
            }
        }
        self
    }
}
impl<T: Mul<Output = T> + Copy, const ROWS: usize, const COLUMNS: usize>
    Mul<MatrixSxS<T, ROWS, COLUMNS>> for MatrixSxD<T, ROWS>
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn mul(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(self.data[0].len(), COLUMNS, "Non-matching columns");
        for i in 0..ROWS {
            for j in 0..COLUMNS {
                other.data[i][j] = self.data[i][j] * other.data[i][j];
            }
        }
        other
    }
}
// MatrixSxS
// --------------------------------------------------
impl<T: MulAssign + Copy, const ROWS: usize, const COLUMNS: usize> Mul<MatrixSxS<T, ROWS, COLUMNS>>
    for MatrixSxS<T, ROWS, COLUMNS>
{
    type Output = Self;
    fn mul(mut self, other: Self) -> Self {
        for i in 0..ROWS {
            for j in 0..COLUMNS {
                self.data[i][j] *= other.data[i][j];
            }
        }
        self
    }
}
impl<T: MulAssign + Copy, const ROWS: usize, const COLUMNS: usize> Mul<MatrixDxS<T, COLUMNS>>
    for MatrixSxS<T, ROWS, COLUMNS>
{
    type Output = Self;
    fn mul(mut self, other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(ROWS, other.data.len(), "Non-matching rows");
        for i in 0..ROWS {
            for j in 0..COLUMNS {
                self.data[i][j] *= other.data[i][j];
            }
        }
        self
    }
}

impl<T: MulAssign + Copy, const ROWS: usize, const COLUMNS: usize> Mul<MatrixSxD<T, ROWS>>
    for MatrixSxS<T, ROWS, COLUMNS>
{
    type Output = Self;
    fn mul(mut self, other: MatrixSxD<T, ROWS>) -> Self::Output {
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(COLUMNS, other.data[0].len(), "Non-matching columns");
        for i in 0..ROWS {
            for j in 0..COLUMNS {
                self.data[i][j] *= other.data[i][j];
            }
        }
        self
    }
}
impl<T: MulAssign + Copy, const ROWS: usize, const COLUMNS: usize> Mul<MatrixDxD<T>>
    for MatrixSxS<T, ROWS, COLUMNS>
{
    type Output = Self;
    fn mul(mut self, other: MatrixDxD<T>) -> Self::Output {
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(ROWS, other.data.len(), "Non-matching rows");
        assert_eq!(COLUMNS, other.data[0].len(), "Non-matching columns");
        for i in 0..ROWS {
            for j in 0..COLUMNS {
                self.data[i][j] *= other.data[i][j];
            }
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
        let c = a * b;
        let d = MatrixDxD::try_from(vec![vec![7., 16., 27.], vec![40., 55., 72.]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn dxd_dxs() {
        let a = MatrixDxD::try_from(vec![vec![1., 2., 3.], vec![4., 5., 6.]]).unwrap();
        let b = MatrixDxS::from(vec![[7., 8., 9.], [10., 11., 12.]]);
        let c = a * b;
        let d = MatrixDxS::from(vec![
            [1. * 7., 2. * 8., 3. * 9.],
            [4. * 10., 5. * 11., 6. * 12.],
        ]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxd_sxd() {
        let a = MatrixDxD::try_from(vec![vec![1., 2., 3.], vec![4., 5., 6.]]).unwrap();
        let b = MatrixSxD::try_from([vec![7., 8., 9.], vec![10., 11., 12.]]).unwrap();
        let c = a * b;
        let d = MatrixSxD::try_from([
            vec![1. * 7., 2. * 8., 3. * 9.],
            vec![4. * 10., 5. * 11., 6. * 12.],
        ])
        .unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn dxd_sxs() {
        let a = MatrixDxD::try_from(vec![vec![1., 2., 3.], vec![4., 5., 6.]]).unwrap();
        let b = MatrixSxS::from([[7., 8., 9.], [10., 11., 12.]]);
        let c = a * b;
        let d = MatrixSxS::from([[1. * 7., 2. * 8., 3. * 9.], [4. * 10., 5. * 11., 6. * 12.]]);
        assert_eq!(c, d);
    }
    // MatrixDxS
    // --------------------------------------------------
    #[test]
    fn dxs_dxs() {
        let a = MatrixDxS::from(vec![[1., 2., 3.], [4., 5., 6.]]);
        let b = MatrixDxS::from(vec![[7., 8., 9.], [10., 11., 12.]]);
        let c = a * b;
        let d = MatrixDxS::from(vec![[7., 16., 27.], [40., 55., 72.]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxs_sxd() {
        let a = MatrixDxS::from(vec![[1., 2., 3.], [4., 5., 6.]]);
        let b = MatrixSxD::try_from([vec![7., 8., 9.], vec![10., 11., 12.]]).unwrap();
        let c = a * b;
        let d = MatrixSxS::from([[1. * 7., 2. * 8., 3. * 9.], [4. * 10., 5. * 11., 6. * 12.]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxs_dxd() {
        let a = MatrixDxS::from(vec![[1., 2., 3.], [4., 5., 6.]]);
        let b = MatrixDxD::try_from(vec![vec![7., 8., 9.], vec![10., 11., 12.]]).unwrap();
        let c = a * b;
        let d = MatrixDxS::from(vec![
            [1. * 7., 2. * 8., 3. * 9.],
            [4. * 10., 5. * 11., 6. * 12.],
        ]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxs_sxs() {
        let a = MatrixDxS::from(vec![[1., 2., 3.], [4., 5., 6.]]);
        let b = MatrixSxS::from([[7., 8., 9.], [10., 11., 12.]]);
        let c = a * b;
        let d = MatrixSxS::from([[1. * 7., 2. * 8., 3. * 9.], [4. * 10., 5. * 11., 6. * 12.]]);
        assert_eq!(c, d);
    }
    // MatrixSxD
    // --------------------------------------------------
    #[test]
    fn sxd_sxd() {
        let a = MatrixSxD::try_from([vec![1., 2., 3.], vec![4., 5., 6.]]).unwrap();
        let b = MatrixSxD::try_from([vec![7., 8., 9.], vec![10., 11., 12.]]).unwrap();
        let c = a * b;
        let d = MatrixSxD::try_from([vec![7., 16., 27.], vec![40., 55., 72.]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn sxd_dxs() {
        let a = MatrixSxD::try_from([vec![1., 2., 3.], vec![4., 5., 6.]]).unwrap();
        let b = MatrixDxS::from(vec![[7., 8., 9.], [10., 11., 12.]]);
        let c = a * b;
        let d = MatrixSxS::from([[1. * 7., 2. * 8., 3. * 9.], [4. * 10., 5. * 11., 6. * 12.]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxd_sxs() {
        let a = MatrixSxD::try_from([vec![1., 2., 3.], vec![4., 5., 6.]]).unwrap();
        let b = MatrixSxS::from([[7., 8., 9.], [10., 11., 12.]]);
        let c = a * b;
        let d = MatrixSxS::from([[1. * 7., 2. * 8., 3. * 9.], [4. * 10., 5. * 11., 6. * 12.]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxd_dxd() {
        let a = MatrixSxD::try_from([vec![1., 2., 3.], vec![4., 5., 6.]]).unwrap();
        let b = MatrixDxD::try_from(vec![vec![7., 8., 9.], vec![10., 11., 12.]]).unwrap();
        let c = a * b;
        let d = MatrixSxD::try_from([
            vec![1. * 7., 2. * 8., 3. * 9.],
            vec![4. * 10., 5. * 11., 6. * 12.],
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
        let c = a * b;
        let d = MatrixSxS::from([[7., 16., 27.], [40., 55., 72.]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxs_dxs() {
        let a = MatrixSxS::from([[1., 2., 3.], [4., 5., 6.]]);
        let b = MatrixDxS::from(vec![[7., 8., 9.], [10., 11., 12.]]);
        let c = a * b;
        let d = MatrixSxS::from([[1. * 7., 2. * 8., 3. * 9.], [4. * 10., 5. * 11., 6. * 12.]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxs_sxd() {
        let a = MatrixSxS::from([[1., 2., 3.], [4., 5., 6.]]);
        let b = MatrixSxD::try_from([vec![7., 8., 9.], vec![10., 11., 12.]]).unwrap();
        let c = a * b;
        let d = MatrixSxS::from([[1. * 7., 2. * 8., 3. * 9.], [4. * 10., 5. * 11., 6. * 12.]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxs_dxd() {
        let a = MatrixSxS::from([[1., 2., 3.], [4., 5., 6.]]);
        let b = MatrixDxD::try_from(vec![vec![7., 8., 9.], vec![10., 11., 12.]]).unwrap();
        let c = a * b;
        let d = MatrixSxS::from([[1. * 7., 2. * 8., 3. * 9.], [4. * 10., 5. * 11., 6. * 12.]]);
        assert_eq!(c, d);
    }
}
