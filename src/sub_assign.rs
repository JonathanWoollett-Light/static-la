use crate::*;
use std::ops::SubAssign;

// MatrixDxD
// --------------------------------------------------
impl<T: SubAssign + Copy> SubAssign<MatrixDxD<T>> for MatrixDxD<T> {
    fn sub_assign(&mut self, other: Self) {
        assert_eq!(self.data.len(), other.data.len(), "Non-matching rows");
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(
            self.data[0].len(),
            other.data[0].len(),
            "Non-matching columns"
        );
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            for (x, y) in a.iter_mut().zip(b.iter()) {
                *x -= *y;
            }
        }
    }
}
impl<T: SubAssign + Copy, const COLUMNS: usize> SubAssign<MatrixDxS<T, COLUMNS>> for MatrixDxD<T> {
    fn sub_assign(&mut self, other: MatrixDxS<T, COLUMNS>) {
        assert_eq!(self.data.len(), other.data.len(), "Non-matching rows");
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(COLUMNS, other.data[0].len(), "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            for i in 0..COLUMNS {
                a[i] -= b[i];
            }
        }
    }
}
impl<T: SubAssign + Copy, const ROWS: usize> SubAssign<MatrixSxD<T, ROWS>> for MatrixDxD<T> {
    fn sub_assign(&mut self, other: MatrixSxD<T, ROWS>) {
        assert_eq!(ROWS, other.data.len(), "Non-matching rows");
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(
            self.data[0].len(),
            other.data[0].len(),
            "Non-matching columns"
        );
        for i in 0..ROWS {
            for (a, b) in self.data[i].iter_mut().zip(other.data[i].iter()) {
                *a -= *b;
            }
        }
    }
}
impl<T: SubAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    SubAssign<MatrixSxS<T, ROWS, COLUMNS>> for MatrixDxD<T>
{
    fn sub_assign(&mut self, other: MatrixSxS<T, ROWS, COLUMNS>) {
        assert_eq!(self.data.len(), ROWS, "Non-matching rows");
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(self.data[0].len(), COLUMNS, "Non-matching columns");

        for i in 0..ROWS {
            for j in 0..COLUMNS {
                self.data[i][j] -= other.data[i][j];
            }
        }
    }
}
// MatrixDxS
// --------------------------------------------------
impl<T: SubAssign + Copy, const COLUMNS: usize> SubAssign<MatrixDxS<T, COLUMNS>>
    for MatrixDxS<T, COLUMNS>
{
    fn sub_assign(&mut self, other: Self) {
        assert_eq!(self.data.len(), other.data.len(), "Non-matching rows");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            for i in 0..COLUMNS {
                a[i] -= b[i];
            }
        }
    }
}
impl<T: SubAssign + Copy, const ROWS: usize, const COLUMNS: usize> SubAssign<MatrixSxD<T, ROWS>>
    for MatrixDxS<T, COLUMNS>
{
    fn sub_assign(&mut self, other: MatrixSxD<T, ROWS>) {
        assert_eq!(self.data.len(), ROWS, "Non-matching rows");
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(COLUMNS, other.data[0].len(), "Non-matching columns");

        for i in 0..ROWS {
            for j in 0..COLUMNS {
                self.data[i][j] -= other.data[i][j];
            }
        }
    }
}
impl<T: SubAssign + Copy, const COLUMNS: usize> SubAssign<MatrixDxD<T>> for MatrixDxS<T, COLUMNS> {
    fn sub_assign(&mut self, other: MatrixDxD<T>) {
        assert_eq!(self.data.len(), other.data.len(), "Non-matching rows");
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(self.data[0].len(), COLUMNS, "Non-matching columns");

        for (x, y) in self.data.iter_mut().zip(other.data.iter()) {
            for i in 0..COLUMNS {
                x[i] -= y[i];
            }
        }
    }
}
impl<T: SubAssign + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    SubAssign<MatrixSxS<T, ROWS, COLUMNS>> for MatrixDxS<T, COLUMNS>
{
    fn sub_assign(&mut self, other: MatrixSxS<T, ROWS, COLUMNS>) {
        assert_eq!(self.data.len(), ROWS, "Non-matching rows");

        for i in 0..ROWS {
            for j in 0..COLUMNS {
                self.data[i][j] -= other.data[i][j];
            }
        }
    }
}
// MatrixSxD
// --------------------------------------------------
impl<T: SubAssign + Copy, const ROWS: usize> SubAssign<MatrixSxD<T, ROWS>> for MatrixSxD<T, ROWS> {
    fn sub_assign(&mut self, other: Self) {
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(
            self.data[0].len(),
            other.data[0].len(),
            "Non-matching columns"
        );

        for i in 0..ROWS {
            for (x, y) in self.data[i].iter_mut().zip(other.data[i].iter()) {
                *x -= *y;
            }
        }
    }
}
impl<T: SubAssign + Copy, const ROWS: usize, const COLUMNS: usize> SubAssign<MatrixDxS<T, COLUMNS>>
    for MatrixSxD<T, ROWS>
{
    fn sub_assign(&mut self, other: MatrixDxS<T, COLUMNS>) {
        assert_eq!(ROWS, other.data.len(), "Non-matching rows");
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(self.data[0].len(), COLUMNS, "Non-matching columns");

        for i in 0..ROWS {
            for j in 0..COLUMNS {
                self.data[i][j] -= other.data[i][j];
            }
        }
    }
}
impl<T: SubAssign + Copy, const ROWS: usize> SubAssign<MatrixDxD<T>> for MatrixSxD<T, ROWS> {
    fn sub_assign(&mut self, other: MatrixDxD<T>) {
        assert_eq!(ROWS, other.data.len(), "Non-matching rows");
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(
            self.data[0].len(),
            other.data[0].len(),
            "Non-matching columns"
        );

        for i in 0..ROWS {
            for (x, y) in self.data[i].iter_mut().zip(other.data[i].iter()) {
                *x -= *y;
            }
        }
    }
}
impl<T: SubAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    SubAssign<MatrixSxS<T, ROWS, COLUMNS>> for MatrixSxD<T, ROWS>
{
    fn sub_assign(&mut self, other: MatrixSxS<T, ROWS, COLUMNS>) {
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(self.data[0].len(), COLUMNS, "Non-matching columns");

        for i in 0..ROWS {
            for j in 0..COLUMNS {
                self.data[i][j] -= other.data[i][j];
            }
        }
    }
}
// MatrixSxS
// --------------------------------------------------
impl<T: SubAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    SubAssign<MatrixSxS<T, ROWS, COLUMNS>> for MatrixSxS<T, ROWS, COLUMNS>
{
    fn sub_assign(&mut self, other: Self) {
        for i in 0..ROWS {
            for j in 0..COLUMNS {
                self.data[i][j] -= other.data[i][j];
            }
        }
    }
}
impl<T: SubAssign + Copy, const ROWS: usize, const COLUMNS: usize> SubAssign<MatrixDxS<T, COLUMNS>>
    for MatrixSxS<T, ROWS, COLUMNS>
{
    fn sub_assign(&mut self, other: MatrixDxS<T, COLUMNS>) {
        assert_eq!(ROWS, other.data.len(), "Non-matching rows");

        for i in 0..ROWS {
            for j in 0..COLUMNS {
                self.data[i][j] -= other.data[i][j];
            }
        }
    }
}
impl<T: SubAssign + Copy, const ROWS: usize, const COLUMNS: usize> SubAssign<MatrixSxD<T, ROWS>>
    for MatrixSxS<T, ROWS, COLUMNS>
{
    fn sub_assign(&mut self, other: MatrixSxD<T, ROWS>) {
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(COLUMNS, other.data[0].len(), "Non-matching columns");

        for i in 0..ROWS {
            for j in 0..COLUMNS {
                self.data[i][j] -= other.data[i][j];
            }
        }
    }
}
impl<T: SubAssign + Copy, const ROWS: usize, const COLUMNS: usize> SubAssign<MatrixDxD<T>>
    for MatrixSxS<T, ROWS, COLUMNS>
{
    fn sub_assign(&mut self, other: MatrixDxD<T>) {
        assert_eq!(ROWS, other.data.len(), "Non-matching rows");
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(COLUMNS, other.data[0].len(), "Non-matching columns");

        for i in 0..ROWS {
            for j in 0..COLUMNS {
                self.data[i][j] -= other.data[i][j];
            }
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
        let mut a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        a -= b;
        let d = MatrixDxD::try_from(vec![vec![-6, -6, -6], vec![-6, -6, -6]]).unwrap();
        assert_eq!(a, d);
    }
    #[test]
    fn dxd_dxs() {
        let mut a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        a -= b;
        let d = MatrixDxS::from(vec![[-6, -6, -6], [-6, -6, -6]]);
        assert_eq!(a, d);
    }
    #[test]
    fn dxd_sxd() {
        let mut a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        a -= b;
        let d = MatrixSxD::try_from([vec![-6, -6, -6], vec![-6, -6, -6]]).unwrap();
        assert_eq!(a, d);
    }
    #[test]
    fn dxd_sxs() {
        let mut a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
        a -= b;
        let d = MatrixSxS::from([[-6, -6, -6], [-6, -6, -6]]);
        assert_eq!(a, d);
    }
    // MatrixDxS
    // --------------------------------------------------
    #[test]
    fn dxs_dxs() {
        let mut a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        a -= b;
        let d = MatrixDxS::from(vec![[-6, -6, -6], [-6, -6, -6]]);
        assert_eq!(a, d);
    }
    #[test]
    fn dxs_sxd() {
        let mut a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        a -= b;
        let d = MatrixSxS::from([[-6, -6, -6], [-6, -6, -6]]);
        assert_eq!(a, d);
    }
    #[test]
    fn dxs_dxd() {
        let mut a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        a -= b;
        let d = MatrixDxS::from(vec![[-6, -6, -6], [-6, -6, -6]]);
        assert_eq!(a, d);
    }
    #[test]
    fn dxs_sxs() {
        let mut a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
        let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
        a -= b;
        let d = MatrixSxS::from([[-6, -6, -6], [-6, -6, -6]]);
        assert_eq!(a, d);
    }
    // MatrixSxD
    // --------------------------------------------------
    #[test]
    fn sxd_sxd() {
        let mut a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        a -= b;
        let d = MatrixSxD::try_from([vec![-6, -6, -6], vec![-6, -6, -6]]).unwrap();
        assert_eq!(a, d);
    }
    #[test]
    fn sxd_dxs() {
        let mut a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        a -= b;
        let d = MatrixSxS::from([[-6, -6, -6], [-6, -6, -6]]);
        assert_eq!(a, d);
    }
    #[test]
    fn sxd_sxs() {
        let mut a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
        a -= b;
        let d = MatrixSxS::from([[-6, -6, -6], [-6, -6, -6]]);
        assert_eq!(a, d);
    }
    #[test]
    fn sxd_dxd() {
        let mut a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        a -= b;
        let d = MatrixSxD::try_from([vec![-6, -6, -6], vec![-6, -6, -6]]).unwrap();
        assert_eq!(a, d);
    }
    // MatrixSxS
    // --------------------------------------------------
    #[test]
    fn sxs_sxs() {
        let mut a = MatrixSxS::from([[1, 2, 3], [4, 5, 6]]);
        let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
        a -= b;
        let d = MatrixSxS::from([[-6, -6, -6], [-6, -6, -6]]);
        assert_eq!(a, d);
    }
    #[test]
    fn sxs_dxs() {
        let mut a = MatrixSxS::from([[1, 2, 3], [4, 5, 6]]);
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        a -= b;
        let d = MatrixSxS::from([[-6, -6, -6], [-6, -6, -6]]);
        assert_eq!(a, d);
    }
    #[test]
    fn sxs_sxd() {
        let mut a = MatrixSxS::from([[1, 2, 3], [4, 5, 6]]);
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        a -= b;
        let d = MatrixSxS::from([[-6, -6, -6], [-6, -6, -6]]);
        assert_eq!(a, d);
    }
    #[test]
    fn sxs_dxd() {
        let mut a = MatrixSxS::from([[1, 2, 3], [4, 5, 6]]);
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        a -= b;
        let d = MatrixSxS::from([[-6, -6, -6], [-6, -6, -6]]);
        assert_eq!(a, d);
    }
}
