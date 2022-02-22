use crate::*;
use std::ops::{Add, AddAssign};

// T
// --------------------------------------------------
impl<T: AddAssign + Copy> Add<T> for MatrixDxD<T> {
    type Output = Self;
    fn add(mut self, x: T) -> Self::Output {
        for a in self.data.iter_mut() {
            *a += x;
        }
        self
    }
}
impl<T: AddAssign + Copy, const COLUMNS: usize> Add<T> for MatrixDxS<T, COLUMNS> {
    type Output = Self;
    fn add(mut self, x: T) -> Self::Output {
        for a in self.data.iter_mut() {
            *a += x;
        }
        self
    }
}
impl<T: AddAssign + Copy, const ROWS: usize> Add<T> for MatrixSxD<T, ROWS> {
    type Output = Self;
    fn add(mut self, x: T) -> Self::Output {
        for a in self.data.iter_mut() {
            *a += x;
        }
        self
    }
}
impl<T: AddAssign + Copy, const ROWS: usize, const COLUMNS: usize> Add<T>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = Self;
    fn add(mut self, x: T) -> Self::Output {
        for a in self.data.iter_mut() {
            *a += x;
        }
        self
    }
}
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
// TransposeDxD
// --------------------------------------------------
impl<'a, T: AddAssign + Copy> Add<MatrixDxD<T>> for TransposeDxD<'a, T> {
    type Output = MatrixDxD<T>;
    fn add(self, mut other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(self.rows(), other.rows, "Non-matching rows");
        assert_eq!(self.columns(), other.columns, "Non-matching columns");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b += *a;
        }
        other
    }
}
impl<'a, T: AddAssign + Copy, const COLUMNS: usize> Add<MatrixDxS<T, COLUMNS>>
    for TransposeDxD<'a, T>
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn add(self, mut other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows(), other.rows, "Non-matching rows");
        assert_eq!(self.columns(), COLUMNS, "Non-matching columns");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b += *a;
        }
        other
    }
}
impl<'a, T: AddAssign + Copy, const ROWS: usize> Add<MatrixSxD<T, ROWS>> for TransposeDxD<'a, T> {
    type Output = MatrixSxD<T, ROWS>;
    fn add(self, mut other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(self.rows(), ROWS, "Non-matching rows");
        assert_eq!(self.columns(), other.columns, "Non-matching columns");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b += *a;
        }
        other
    }
}
impl<'a, T: AddAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    Add<MatrixSxS<T, ROWS, COLUMNS>> for TransposeDxD<'a, T>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn add(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows(), ROWS, "Non-matching rows");
        assert_eq!(self.columns(), COLUMNS, "Non-matching columns");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b += *a;
        }
        other
    }
}
// TransposeDxS
// --------------------------------------------------
impl<'a, T: Add<Output = T> + Default + Copy, const ROWS: usize, const COLUMNS: usize>
    Add<MatrixDxS<T, COLUMNS>> for TransposeDxS<'a, T, ROWS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn add(self, other: MatrixDxS<T, COLUMNS>) -> MatrixSxS<T, ROWS, COLUMNS> {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(self.columns(), COLUMNS, "Non-matching columns");

        let mut data = [Default::default(); ROWS * COLUMNS];
        for (c, (a, b)) in data.iter_mut().zip(self.iter().zip(other.data.iter())) {
            *c = *a + *b;
        }
        Self::Output { data }
    }
}
impl<'a, T: AddAssign + Copy, const ROWS: usize> Add<MatrixSxD<T, ROWS>>
    for TransposeDxS<'a, T, ROWS>
{
    type Output = MatrixSxD<T, ROWS>;
    fn add(self, mut other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(self.columns(), other.columns, "Non-matching columns");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b += *a;
        }
        other
    }
}
impl<'a, T: Add<Output = T> + Copy, const ROWS: usize> Add<MatrixDxD<T>>
    for TransposeDxS<'a, T, ROWS>
{
    type Output = MatrixSxD<T, ROWS>;
    fn add(self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(self.columns(), other.columns, "Non-matching columns");
        Self::Output {
            data: self
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| *a + *b)
                .collect(),
            columns: self.columns(),
        }
    }
}
impl<'a, T: AddAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    Add<MatrixSxS<T, ROWS, COLUMNS>> for TransposeDxS<'a, T, ROWS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn add(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.columns(), COLUMNS, "Non-matching columns");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b += *a;
        }
        other
    }
}
// TransposeSxD
// --------------------------------------------------
impl<'a, T: Add<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    Add<MatrixSxD<T, ROWS>> for TransposeSxD<'a, T, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn add(self, other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(self.rows(), ROWS, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");

        let mut data = [Default::default(); ROWS * COLUMNS];
        for (c, (a, b)) in data.iter_mut().zip(self.iter().zip(other.data.iter())) {
            *c = *a + *b;
        }
        Self::Output { data }
    }
}
impl<'a, T: AddAssign + Copy, const COLUMNS: usize> Add<MatrixDxS<T, COLUMNS>>
    for TransposeSxD<'a, T, COLUMNS>
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn add(self, mut other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows(), other.rows, "Non-matching rows");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b += *a;
        }
        other
    }
}
impl<'a, T: Add<Output = T> + Copy, const COLUMNS: usize> Add<MatrixDxD<T>>
    for TransposeSxD<'a, T, COLUMNS>
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn add(self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(self.rows(), other.rows, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");

        Self::Output {
            data: self
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| *a + *b)
                .collect(),
            rows: self.rows(),
        }
    }
}
impl<'a, T: AddAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    Add<MatrixSxS<T, ROWS, COLUMNS>> for TransposeSxD<'a, T, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn add(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows(), ROWS, "Non-matching columns");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b += *a;
        }
        other
    }
}
// TransposeSxS
// --------------------------------------------------
impl<'a, T: AddAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    Add<MatrixSxS<T, ROWS, COLUMNS>> for TransposeSxS<'a, T, COLUMNS, ROWS>
where
    [(); ROWS * COLUMNS]:,
    [(); COLUMNS * ROWS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn add(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b += *a;
        }
        other
    }
}
impl<'a, T: Add<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    Add<MatrixDxS<T, COLUMNS>> for TransposeSxS<'a, T, COLUMNS, ROWS>
where
    [(); COLUMNS * ROWS]:,
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn add(self, other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        let mut data = [Default::default(); ROWS * COLUMNS];
        for (c, (a, b)) in data.iter_mut().zip(self.iter().zip(other.data.iter())) {
            *c = *a + *b;
        }
        Self::Output { data }
    }
}
impl<'a, T: Add<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    Add<MatrixSxD<T, ROWS>> for TransposeSxS<'a, T, COLUMNS, ROWS>
where
    [(); COLUMNS * ROWS]:,
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn add(self, other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");
        let mut data = [Default::default(); ROWS * COLUMNS];
        for (c, (a, b)) in data.iter_mut().zip(self.iter().zip(other.data.iter())) {
            *c = *a + *b;
        }
        Self::Output { data }
    }
}
impl<'a, T: Add<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    Add<MatrixDxD<T>> for TransposeSxS<'a, T, COLUMNS, ROWS>
where
    [(); COLUMNS * ROWS]:,
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn add(self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");
        let mut data = [Default::default(); ROWS * COLUMNS];
        for (c, (a, b)) in data.iter_mut().zip(self.iter().zip(other.data.iter())) {
            *c = *a + *b;
        }
        Self::Output { data }
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
        let a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        assert_eq!(
            a + 3,
            MatrixDxD::try_from(vec![vec![4, 5, 6], vec![7, 8, 9]]).unwrap()
        );
    }
    #[test]
    fn t_dxs() {
        let a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
        assert_eq!(a + 3, MatrixDxS::from(vec![[4, 5, 6], [7, 8, 9]]));
    }
    #[test]
    fn t_sxd() {
        let a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        assert_eq!(
            a + 3,
            MatrixSxD::try_from([vec![4, 5, 6], vec![7, 8, 9]]).unwrap()
        );
    }
    #[test]
    fn t_sxs() {
        let a = MatrixSxS::<i32, 2, 3>::from([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(a + 3, MatrixSxS::<i32, 2, 3>::from([[4, 5, 6], [7, 8, 9]]));
    }
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
    // TransposeDxD
    // --------------------------------------------------
    #[test]
    fn transpose_dxd_dxd() {
        let a = MatrixDxD::try_from(vec![vec![1, 4], vec![2, 5], vec![3, 6]]).unwrap();
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a.transpose_ref() + b;
        let d = MatrixDxD::try_from(vec![vec![8, 10, 12], vec![14, 16, 18]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_dxd_dxs() {
        let a = MatrixDxD::try_from(vec![vec![1, 4], vec![2, 5], vec![3, 6]]).unwrap();
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        let c = a.transpose_ref() + b;
        let d = MatrixDxS::from(vec![[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_dxd_sxd() {
        let a = MatrixDxD::try_from(vec![vec![1, 4], vec![2, 5], vec![3, 6]]).unwrap();
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a.transpose_ref() + b;
        let d = MatrixSxD::try_from([vec![8, 10, 12], vec![14, 16, 18]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_dxd_sxs() {
        let a = MatrixDxD::try_from(vec![vec![1, 4], vec![2, 5], vec![3, 6]]).unwrap();
        let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
        let c = a.transpose_ref() + b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
    // TransposeDxS
    // --------------------------------------------------
    #[test]
    fn transpose_dxs_dxs() {
        let a = MatrixSxD::try_from([vec![1, 4], vec![2, 5], vec![3, 6]]).unwrap();
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        let c = a.transpose_ref() + b;
        let d = MatrixDxS::from(vec![[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_dxs_sxd() {
        let a = MatrixSxD::try_from([vec![1, 4], vec![2, 5], vec![3, 6]]).unwrap();
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a.transpose_ref() + b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_dxs_dxd() {
        let a = MatrixSxD::try_from([vec![1, 4], vec![2, 5], vec![3, 6]]).unwrap();
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a.transpose_ref() + b;
        let d = MatrixDxS::from(vec![[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_dxs_sxs() {
        let a = MatrixSxD::try_from([vec![1, 4], vec![2, 5], vec![3, 6]]).unwrap();
        let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
        let c = a.transpose_ref() + b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
    // TransposeSxD
    // --------------------------------------------------
    #[test]
    fn transpose_sxd_sxd() {
        let a = MatrixDxS::from(vec![[1, 4], [2, 5], [3, 6]]);
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a.transpose_ref() + b;
        let d = MatrixSxD::try_from([vec![8, 10, 12], vec![14, 16, 18]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_sxd_dxs() {
        let a = MatrixDxS::from(vec![[1, 4], [2, 5], [3, 6]]);
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        let c = a.transpose_ref() + b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_sxd_sxs() {
        let a = MatrixDxS::from(vec![[1, 4], [2, 5], [3, 6]]);
        let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
        let c = a.transpose_ref() + b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_sxd_dxd() {
        let a = MatrixDxS::from(vec![[1, 4], [2, 5], [3, 6]]);
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a.transpose_ref() + b;
        let d = MatrixSxD::try_from([vec![8, 10, 12], vec![14, 16, 18]]).unwrap();
        assert_eq!(c, d);
    }
    // TransposeSxS
    // --------------------------------------------------
    #[test]
    fn transpose_sxs_sxs() {
        let a = MatrixSxS::<i32, 3, 2>::from([[1, 4], [2, 5], [3, 6]]);
        let b = MatrixSxS::<i32, 2, 3>::from([[7, 8, 9], [10, 11, 12]]);
        let c = a.transpose_ref() + b;
        let d = MatrixSxS::<i32, 2, 3>::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_sxs_dxs() {
        let a = MatrixSxS::<i32, 3, 2>::from([[1, 4], [2, 5], [3, 6]]);
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        let c = a.transpose_ref() + b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_sxs_sxd() {
        let a = MatrixSxS::<i32, 3, 2>::from([[1, 4], [2, 5], [3, 6]]);
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a.transpose_ref() + b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_sxs_dxd() {
        let a = MatrixSxS::<i32, 3, 2>::from([[1, 4], [2, 5], [3, 6]]);
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a.transpose_ref() + b;
        let d = MatrixSxS::from([[8, 10, 12], [14, 16, 18]]);
        assert_eq!(c, d);
    }
}
