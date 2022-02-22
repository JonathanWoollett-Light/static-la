use crate::*;
use std::ops::AddAssign;

// T
// --------------------------------------------------
impl<T: AddAssign + Copy> AddAssign<T> for MatrixDxD<T> {
    fn add_assign(&mut self, x: T) {
        for a in self.data.iter_mut() {
            *a += x;
        }
    }
}
impl<T: AddAssign + Copy, const COLUMNS: usize> AddAssign<T> for MatrixDxS<T, COLUMNS> {
    fn add_assign(&mut self, x: T) {
        for a in self.data.iter_mut() {
            *a += x;
        }
    }
}
impl<T: AddAssign + Copy, const ROWS: usize> AddAssign<T> for MatrixSxD<T, ROWS> {
    fn add_assign(&mut self, x: T) {
        for a in self.data.iter_mut() {
            *a += x;
        }
    }
}
impl<T: AddAssign + Copy, const ROWS: usize, const COLUMNS: usize> AddAssign<T>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn add_assign(&mut self, x: T) {
        for a in self.data.iter_mut() {
            *a += x;
        }
    }
}
// MatrixDxD
// --------------------------------------------------
impl<T: AddAssign + Copy> AddAssign<MatrixDxD<T>> for MatrixDxD<T> {
    fn add_assign(&mut self, other: Self) {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        assert_eq!(self.columns, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
    }
}
impl<T: AddAssign + Copy, const COLUMNS: usize> AddAssign<MatrixDxS<T, COLUMNS>> for MatrixDxD<T> {
    fn add_assign(&mut self, other: MatrixDxS<T, COLUMNS>) {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
    }
}
impl<T: AddAssign + Copy, const ROWS: usize> AddAssign<MatrixSxD<T, ROWS>> for MatrixDxD<T> {
    fn add_assign(&mut self, other: MatrixSxD<T, ROWS>) {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        assert_eq!(self.columns, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
    }
}
impl<T: AddAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    AddAssign<MatrixSxS<T, ROWS, COLUMNS>> for MatrixDxD<T>
where
    [(); ROWS * COLUMNS]:,
{
    fn add_assign(&mut self, other: MatrixSxS<T, ROWS, COLUMNS>) {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
    }
}
// MatrixDxS
// --------------------------------------------------
impl<T: AddAssign + Copy, const COLUMNS: usize> AddAssign<MatrixDxS<T, COLUMNS>>
    for MatrixDxS<T, COLUMNS>
{
    fn add_assign(&mut self, other: Self) {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
    }
}
impl<T: AddAssign + Copy, const ROWS: usize, const COLUMNS: usize> AddAssign<MatrixSxD<T, ROWS>>
    for MatrixDxS<T, COLUMNS>
{
    fn add_assign(&mut self, other: MatrixSxD<T, ROWS>) {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
    }
}
impl<T: AddAssign + Copy, const COLUMNS: usize> AddAssign<MatrixDxD<T>> for MatrixDxS<T, COLUMNS> {
    fn add_assign(&mut self, other: MatrixDxD<T>) {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
    }
}
impl<T: AddAssign + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    AddAssign<MatrixSxS<T, ROWS, COLUMNS>> for MatrixDxS<T, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn add_assign(&mut self, other: MatrixSxS<T, ROWS, COLUMNS>) {
        assert_eq!(self.rows, ROWS, "Non-matching rows");

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
    }
}
// MatrixSxD
// --------------------------------------------------
impl<T: AddAssign + Copy, const ROWS: usize> AddAssign<MatrixSxD<T, ROWS>> for MatrixSxD<T, ROWS> {
    fn add_assign(&mut self, other: Self) {
        assert_eq!(self.columns, other.columns, "Non-matching columns");

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
    }
}
impl<T: AddAssign + Copy, const ROWS: usize, const COLUMNS: usize> AddAssign<MatrixDxS<T, COLUMNS>>
    for MatrixSxD<T, ROWS>
{
    fn add_assign(&mut self, other: MatrixDxS<T, COLUMNS>) {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
    }
}
impl<T: AddAssign + Copy, const ROWS: usize> AddAssign<MatrixDxD<T>> for MatrixSxD<T, ROWS> {
    fn add_assign(&mut self, other: MatrixDxD<T>) {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(self.columns, other.columns, "Non-matching columns");

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
    }
}
impl<T: AddAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    AddAssign<MatrixSxS<T, ROWS, COLUMNS>> for MatrixSxD<T, ROWS>
where
    [(); ROWS * COLUMNS]:,
{
    fn add_assign(&mut self, other: MatrixSxS<T, ROWS, COLUMNS>) {
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
    }
}
// MatrixSxS
// --------------------------------------------------
impl<T: AddAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    AddAssign<MatrixSxS<T, ROWS, COLUMNS>> for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn add_assign(&mut self, other: Self) {
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
    }
}
impl<T: AddAssign + Copy, const ROWS: usize, const COLUMNS: usize> AddAssign<MatrixDxS<T, COLUMNS>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn add_assign(&mut self, other: MatrixDxS<T, COLUMNS>) {
        assert_eq!(ROWS, other.rows, "Non-matching rows");

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
    }
}
impl<T: AddAssign + Copy, const ROWS: usize, const COLUMNS: usize> AddAssign<MatrixSxD<T, ROWS>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn add_assign(&mut self, other: MatrixSxD<T, ROWS>) {
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
    }
}
impl<T: AddAssign + Copy, const ROWS: usize, const COLUMNS: usize> AddAssign<MatrixDxD<T>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn add_assign(&mut self, other: MatrixDxD<T>) {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
    }
}

// Tests
// --------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::*;
    use std::convert::TryFrom;
    // T
    // --------------------------------------------------
    #[test]
    fn t_dxd() {
        let mut a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        a += 3;
        assert_eq!(
            a,
            MatrixDxD::try_from(vec![vec![4, 5, 6], vec![7, 8, 9]]).unwrap()
        );
    }
    #[test]
    fn t_dxs() {
        let mut a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
        a += 3;
        assert_eq!(a, MatrixDxS::from(vec![[4, 5, 6], [7, 8, 9]]));
    }
    #[test]
    fn t_sxd() {
        let mut a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        a += 3;
        assert_eq!(
            a,
            MatrixSxD::try_from([vec![4, 5, 6], vec![7, 8, 9]]).unwrap()
        );
    }
    #[test]
    fn t_sxs() {
        let mut a = MatrixSxS::<i32, 2, 3>::from([[1, 2, 3], [4, 5, 6]]);
        a += 3;
        assert_eq!(a, MatrixSxS::<i32, 2, 3>::from([[4, 5, 6], [7, 8, 9]]));
    }
    // MatrixDxD
    // --------------------------------------------------
    #[test]
    fn dxd_dxd() {
        let mut a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        a += b;
        let d = MatrixDxD::try_from(vec![vec![8, 10, 12], vec![14, 16, 18]]).unwrap();
        assert_eq!(a, d);
    }
    #[test]
    fn dxd_dxs() {
        let mut a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        a += b;
        let d = MatrixDxS::from(vec![[8, 10, 12], [14, 16, 18]]);
        assert_eq!(a, d);
    }
    #[test]
    fn dxd_sxd() {
        let mut a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        a += b;
        let d = MatrixSxD::try_from([vec![8, 10, 12], vec![14, 16, 18]]).unwrap();
        assert_eq!(a, d);
    }
    #[test]
    fn dxd_sxs() {
        let mut a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
        a += b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(a, d);
    }
    // MatrixDxS
    // --------------------------------------------------
    #[test]
    fn dxs_dxs() {
        let mut a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        a += b;
        let d = MatrixDxS::from(vec![[8, 10, 12], [14, 16, 18]]);
        assert_eq!(a, d);
    }
    #[test]
    fn dxs_sxd() {
        let mut a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        a += b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(a, d);
    }
    #[test]
    fn dxs_dxd() {
        let mut a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        a += b;
        let d = MatrixDxS::from(vec![[8, 10, 12], [14, 16, 18]]);
        assert_eq!(a, d);
    }
    #[test]
    fn dxs_sxs() {
        let mut a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
        let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
        a += b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(a, d);
    }
    // MatrixSxD
    // --------------------------------------------------
    #[test]
    fn sxd_sxd() {
        let mut a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        a += b;
        let d = MatrixSxD::try_from([vec![8, 10, 12], vec![14, 16, 18]]).unwrap();
        assert_eq!(a, d);
    }
    #[test]
    fn sxd_dxs() {
        let mut a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        a += b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(a, d);
    }
    #[test]
    fn sxd_sxs() {
        let mut a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
        a += b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(a, d);
    }
    #[test]
    fn sxd_dxd() {
        let mut a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        a += b;
        let d = MatrixSxD::try_from([vec![8, 10, 12], vec![14, 16, 18]]).unwrap();
        assert_eq!(a, d);
    }
    // MatrixSxS
    // --------------------------------------------------
    #[test]
    fn sxs_sxs() {
        let mut a = MatrixSxS::<i32, 2, 3>::from([[1, 2, 3], [4, 5, 6]]);
        let b = MatrixSxS::<i32, 2, 3>::from([[7, 8, 9], [10, 11, 12]]);
        a += b;
        let d = MatrixSxS::<i32, 2, 3>::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(a, d);
    }
    #[test]
    fn sxs_dxs() {
        let mut a = MatrixSxS::from([[1, 2, 3], [4, 5, 6]]);
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        a += b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(a, d);
    }
    #[test]
    fn sxs_sxd() {
        let mut a = MatrixSxS::from([[1, 2, 3], [4, 5, 6]]);
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        a += b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(a, d);
    }
    #[test]
    fn sxs_dxd() {
        let mut a = MatrixSxS::from([[1, 2, 3], [4, 5, 6]]);
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        a += b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(a, d);
    }
}
