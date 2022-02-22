use crate::*;
use std::ops::{Rem, RemAssign};

// T
// --------------------------------------------------
impl<T: RemAssign + Copy> Rem<T> for MatrixDxD<T> {
    type Output = Self;
    fn rem(mut self, x: T) -> Self::Output {
        for a in self.data.iter_mut() {
            *a %= x;
        }
        self
    }
}
impl<T: RemAssign + Copy, const COLUMNS: usize> Rem<T> for MatrixDxS<T, COLUMNS> {
    type Output = Self;
    fn rem(mut self, x: T) -> Self::Output {
        for a in self.data.iter_mut() {
            *a %= x;
        }
        self
    }
}
impl<T: RemAssign + Copy, const ROWS: usize> Rem<T> for MatrixSxD<T, ROWS> {
    type Output = Self;
    fn rem(mut self, x: T) -> Self::Output {
        for a in self.data.iter_mut() {
            *a %= x;
        }
        self
    }
}
impl<T: RemAssign + Copy, const ROWS: usize, const COLUMNS: usize> Rem<T>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = Self;
    fn rem(mut self, x: T) -> Self::Output {
        for a in self.data.iter_mut() {
            *a %= x;
        }
        self
    }
}
// MatrixDxD
// --------------------------------------------------
impl<T: RemAssign + Copy> Rem<MatrixDxD<T>> for MatrixDxD<T> {
    type Output = Self;
    fn rem(mut self, other: Self) -> Self {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        assert_eq!(self.columns, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a %= *b;
        }
        self
    }
}
impl<T: Rem<Output = T> + Copy, const COLUMNS: usize> Rem<MatrixDxS<T, COLUMNS>> for MatrixDxD<T> {
    type Output = MatrixDxS<T, COLUMNS>;
    fn rem(self, mut other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");
        for (a, b) in self.data.iter().zip(other.data.iter_mut()) {
            *b = *a % *b;
        }
        other
    }
}
impl<T: Rem<Output = T> + Copy, const ROWS: usize> Rem<MatrixSxD<T, ROWS>> for MatrixDxD<T> {
    type Output = MatrixSxD<T, ROWS>;
    fn rem(self, mut other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        assert_eq!(self.columns, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter().zip(other.data.iter_mut()) {
            *b = *a % *b;
        }
        other
    }
}
impl<T: Rem<Output = T> + Copy, const ROWS: usize, const COLUMNS: usize>
    Rem<MatrixSxS<T, ROWS, COLUMNS>> for MatrixDxD<T>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn rem(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");
        for (a, b) in self.data.iter().zip(other.data.iter_mut()) {
            *b = *a % *b;
        }
        other
    }
}
// MatrixDxS
// --------------------------------------------------
impl<T: RemAssign + Copy, const COLUMNS: usize> Rem<MatrixDxS<T, COLUMNS>>
    for MatrixDxS<T, COLUMNS>
{
    type Output = Self;
    fn rem(mut self, other: Self) -> Self {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a %= *b;
        }
        self
    }
}
impl<T: Rem<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    Rem<MatrixSxD<T, ROWS>> for MatrixDxS<T, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn rem(self, other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");
        let mut data = [Default::default(); ROWS * COLUMNS];
        for i in 0..ROWS * COLUMNS {
            data[i] = self.data[i] % other.data[i];
        }
        Self::Output { data }
    }
}
impl<T: RemAssign + Copy, const COLUMNS: usize> Rem<MatrixDxD<T>> for MatrixDxS<T, COLUMNS> {
    type Output = Self;
    fn rem(mut self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a %= *b;
        }
        self
    }
}
impl<T: Rem<Output = T> + Copy, const ROWS: usize, const COLUMNS: usize>
    Rem<MatrixSxS<T, ROWS, COLUMNS>> for MatrixDxS<T, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn rem(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        for (a, b) in self.data.iter().zip(other.data.iter_mut()) {
            *b = *a % *b;
        }
        other
    }
}
// MatrixSxD
// --------------------------------------------------
impl<T: RemAssign + Copy, const ROWS: usize> Rem<MatrixSxD<T, ROWS>> for MatrixSxD<T, ROWS> {
    type Output = Self;
    fn rem(mut self, other: Self) -> Self {
        assert_eq!(self.columns, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a %= *b;
        }
        self
    }
}
impl<T: Rem<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    Rem<MatrixDxS<T, COLUMNS>> for MatrixSxD<T, ROWS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn rem(self, other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");
        let mut data = [Default::default(); ROWS * COLUMNS];
        for i in 0..ROWS * COLUMNS {
            data[i] = self.data[i] % other.data[i];
        }
        Self::Output { data }
    }
}
impl<T: RemAssign + Copy, const ROWS: usize> Rem<MatrixDxD<T>> for MatrixSxD<T, ROWS> {
    type Output = Self;
    fn rem(mut self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(self.columns, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a %= *b;
        }
        self
    }
}
impl<T: Rem<Output = T> + Copy, const ROWS: usize, const COLUMNS: usize>
    Rem<MatrixSxS<T, ROWS, COLUMNS>> for MatrixSxD<T, ROWS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn rem(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");
        for (a, b) in self.data.iter().zip(other.data.iter_mut()) {
            *b = *a % *b;
        }
        other
    }
}
// MatrixSxS
// --------------------------------------------------
impl<T: RemAssign + Copy, const ROWS: usize, const COLUMNS: usize> Rem<MatrixSxS<T, ROWS, COLUMNS>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = Self;
    fn rem(mut self, other: Self) -> Self {
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a %= *b;
        }
        self
    }
}
impl<T: RemAssign + Copy, const ROWS: usize, const COLUMNS: usize> Rem<MatrixDxS<T, COLUMNS>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = Self;
    fn rem(mut self, other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a %= *b;
        }
        self
    }
}
impl<T: RemAssign + Copy, const ROWS: usize, const COLUMNS: usize> Rem<MatrixSxD<T, ROWS>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = Self;
    fn rem(mut self, other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a %= *b;
        }
        self
    }
}
impl<T: RemAssign + Copy, const ROWS: usize, const COLUMNS: usize> Rem<MatrixDxD<T>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = Self;
    fn rem(mut self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a %= *b;
        }
        self
    }
}
// TransposeDxD
// --------------------------------------------------
impl<'a, T: Rem<Output = T> + Copy> Rem<MatrixDxD<T>> for TransposeDxD<'a, T> {
    type Output = MatrixDxD<T>;
    fn rem(self, mut other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(self.rows(), other.rows, "Non-matching rows");
        assert_eq!(self.columns(), other.columns, "Non-matching columns");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b = *a % *b;
        }
        other
    }
}
impl<'a, T: Rem<Output = T> + Copy, const COLUMNS: usize> Rem<MatrixDxS<T, COLUMNS>>
    for TransposeDxD<'a, T>
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn rem(self, mut other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows(), other.rows, "Non-matching rows");
        assert_eq!(self.columns(), COLUMNS, "Non-matching columns");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b = *a % *b;
        }
        other
    }
}
impl<'a, T: Rem<Output = T> + Copy, const ROWS: usize> Rem<MatrixSxD<T, ROWS>>
    for TransposeDxD<'a, T>
{
    type Output = MatrixSxD<T, ROWS>;
    fn rem(self, mut other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(self.rows(), ROWS, "Non-matching rows");
        assert_eq!(self.columns(), other.columns, "Non-matching columns");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b = *a % *b;
        }
        other
    }
}
impl<'a, T: Rem<Output = T> + Copy, const ROWS: usize, const COLUMNS: usize>
    Rem<MatrixSxS<T, ROWS, COLUMNS>> for TransposeDxD<'a, T>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn rem(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows(), ROWS, "Non-matching rows");
        assert_eq!(self.columns(), COLUMNS, "Non-matching columns");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b = *a % *b;
        }
        other
    }
}
// TransposeDxS
// --------------------------------------------------
impl<'a, T: Rem<Output = T> + Default + Copy, const ROWS: usize, const COLUMNS: usize>
    Rem<MatrixDxS<T, COLUMNS>> for TransposeDxS<'a, T, ROWS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn rem(self, other: MatrixDxS<T, COLUMNS>) -> MatrixSxS<T, ROWS, COLUMNS> {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(self.columns(), COLUMNS, "Non-matching columns");

        let mut data = [Default::default(); ROWS * COLUMNS];
        for (c, (a, b)) in data.iter_mut().zip(self.iter().zip(other.data.iter())) {
            *c = *a % *b;
        }
        Self::Output { data }
    }
}
impl<'a, T: Rem<Output = T> + Copy, const ROWS: usize> Rem<MatrixSxD<T, ROWS>>
    for TransposeDxS<'a, T, ROWS>
{
    type Output = MatrixSxD<T, ROWS>;
    fn rem(self, mut other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(self.columns(), other.columns, "Non-matching columns");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b = *a % *b;
        }
        other
    }
}
impl<'a, T: Rem<Output = T> + Copy, const ROWS: usize> Rem<MatrixDxD<T>>
    for TransposeDxS<'a, T, ROWS>
{
    type Output = MatrixSxD<T, ROWS>;
    fn rem(self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(self.columns(), other.columns, "Non-matching columns");
        Self::Output {
            data: self
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| *a % *b)
                .collect(),
            columns: self.columns(),
        }
    }
}
impl<'a, T: Rem<Output = T> + Copy, const ROWS: usize, const COLUMNS: usize>
    Rem<MatrixSxS<T, ROWS, COLUMNS>> for TransposeDxS<'a, T, ROWS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn rem(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.columns(), COLUMNS, "Non-matching columns");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b = *a % *b;
        }
        other
    }
}
// TransposeSxD
// --------------------------------------------------
impl<'a, T: Rem<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    Rem<MatrixSxD<T, ROWS>> for TransposeSxD<'a, T, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn rem(self, other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(self.rows(), ROWS, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");

        let mut data = [Default::default(); ROWS * COLUMNS];
        for (c, (a, b)) in data.iter_mut().zip(self.iter().zip(other.data.iter())) {
            *c = *a % *b;
        }
        Self::Output { data }
    }
}
impl<'a, T: Rem<Output = T> + Copy, const COLUMNS: usize> Rem<MatrixDxS<T, COLUMNS>>
    for TransposeSxD<'a, T, COLUMNS>
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn rem(self, mut other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows(), other.rows, "Non-matching rows");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b = *a % *b;
        }
        other
    }
}
impl<'a, T: Rem<Output = T> + Copy, const COLUMNS: usize> Rem<MatrixDxD<T>>
    for TransposeSxD<'a, T, COLUMNS>
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn rem(self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(self.rows(), other.rows, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");

        Self::Output {
            data: self
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| *a % *b)
                .collect(),
            rows: self.rows(),
        }
    }
}
impl<'a, T: Rem<Output = T> + Copy, const ROWS: usize, const COLUMNS: usize>
    Rem<MatrixSxS<T, ROWS, COLUMNS>> for TransposeSxD<'a, T, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn rem(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows(), ROWS, "Non-matching columns");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b = *a % *b;
        }
        other
    }
}
// TransposeSxS
// --------------------------------------------------
impl<'a, T: Rem<Output = T> + Copy, const ROWS: usize, const COLUMNS: usize>
    Rem<MatrixSxS<T, ROWS, COLUMNS>> for TransposeSxS<'a, T, COLUMNS, ROWS>
where
    [(); ROWS * COLUMNS]:,
    [(); COLUMNS * ROWS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn rem(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b = *a % *b;
        }
        other
    }
}
impl<'a, T: Rem<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    Rem<MatrixDxS<T, COLUMNS>> for TransposeSxS<'a, T, COLUMNS, ROWS>
where
    [(); COLUMNS * ROWS]:,
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn rem(self, other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        let mut data = [Default::default(); ROWS * COLUMNS];
        for (c, (a, b)) in data.iter_mut().zip(self.iter().zip(other.data.iter())) {
            *c = *a % *b;
        }
        Self::Output { data }
    }
}
impl<'a, T: Rem<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    Rem<MatrixSxD<T, ROWS>> for TransposeSxS<'a, T, COLUMNS, ROWS>
where
    [(); COLUMNS * ROWS]:,
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn rem(self, other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");
        let mut data = [Default::default(); ROWS * COLUMNS];
        for (c, (a, b)) in data.iter_mut().zip(self.iter().zip(other.data.iter())) {
            *c = *a % *b;
        }
        Self::Output { data }
    }
}
impl<'a, T: Rem<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    Rem<MatrixDxD<T>> for TransposeSxS<'a, T, COLUMNS, ROWS>
where
    [(); COLUMNS * ROWS]:,
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn rem(self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");
        let mut data = [Default::default(); ROWS * COLUMNS];
        for (c, (a, b)) in data.iter_mut().zip(self.iter().zip(other.data.iter())) {
            *c = *a % *b;
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
            a % 3,
            MatrixDxD::try_from(vec![vec![1, 2, 0], vec![1, 2, 0]]).unwrap()
        );
    }
    #[test]
    fn t_dxs() {
        let a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
        assert_eq!(a % 3, MatrixDxS::from(vec![[1, 2, 0], [1, 2, 0]]));
    }
    #[test]
    fn t_sxd() {
        let a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        assert_eq!(
            a % 3,
            MatrixSxD::try_from([vec![1, 2, 0], vec![1, 2, 0]]).unwrap()
        );
    }
    #[test]
    fn t_sxs() {
        let a = MatrixSxS::<i32, 2, 3>::from([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(a % 3, MatrixSxS::<i32, 2, 3>::from([[1, 2, 0], [1, 2, 0]]));
    }
    // MatrixDxD
    // --------------------------------------------------
    #[test]
    fn dxd_dxd() {
        let a = MatrixDxD::try_from(vec![vec![11, 45, 39], vec![4, 5, 6]]).unwrap();
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a % b;
        let d = MatrixDxD::try_from(vec![vec![4, 5, 3], vec![4, 5, 6]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn dxd_dxs() {
        let a = MatrixDxD::try_from(vec![vec![11, 45, 39], vec![4, 5, 6]]).unwrap();
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        let c = a % b;
        let d = MatrixDxS::from(vec![[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxd_sxd() {
        let a = MatrixDxD::try_from(vec![vec![11, 45, 39], vec![4, 5, 6]]).unwrap();
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a % b;
        let d = MatrixSxD::try_from([vec![4, 5, 3], vec![4, 5, 6]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn dxd_sxs() {
        let a = MatrixDxD::try_from(vec![vec![11, 45, 39], vec![4, 5, 6]]).unwrap();
        let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
        let c = a % b;
        let d = MatrixSxS::from([[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    // MatrixDxS
    // --------------------------------------------------
    #[test]
    fn dxs_dxs() {
        let a = MatrixDxS::from(vec![[11, 45, 39], [4, 5, 6]]);
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        let c = a % b;
        let d = MatrixDxS::from(vec![[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxs_sxd() {
        let a = MatrixDxS::from(vec![[11, 45, 39], [4, 5, 6]]);
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a % b;
        let d = MatrixSxS::from([[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxs_dxd() {
        let a = MatrixDxS::from(vec![[11, 45, 39], [4, 5, 6]]);
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a % b;
        let d = MatrixDxS::from(vec![[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxs_sxs() {
        let a = MatrixDxS::from(vec![[11, 45, 39], [4, 5, 6]]);
        let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
        let c = a % b;
        let d = MatrixSxS::from([[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    // MatrixSxD
    // --------------------------------------------------
    #[test]
    fn sxd_sxd() {
        let a = MatrixSxD::try_from([vec![11, 45, 39], vec![4, 5, 6]]).unwrap();
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a % b;
        let d = MatrixSxD::try_from([vec![4, 5, 3], vec![4, 5, 6]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn sxd_dxs() {
        let a = MatrixSxD::try_from([vec![11, 45, 39], vec![4, 5, 6]]).unwrap();
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        let c = a % b;
        let d = MatrixSxS::from([[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxd_sxs() {
        let a = MatrixSxD::try_from([vec![11, 45, 39], vec![4, 5, 6]]).unwrap();
        let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
        let c = a % b;
        let d = MatrixSxS::from([[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxd_dxd() {
        let a = MatrixSxD::try_from([vec![11, 45, 39], vec![4, 5, 6]]).unwrap();
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a % b;
        let d = MatrixSxD::try_from([vec![4, 5, 3], vec![4, 5, 6]]).unwrap();
        assert_eq!(c, d);
    }
    // MatrixSxS
    // --------------------------------------------------
    #[test]
    fn sxs_sxs() {
        let a = MatrixSxS::<i32, 2, 3>::from([[11, 45, 39], [4, 5, 6]]);
        let b = MatrixSxS::<i32, 2, 3>::from([[7, 8, 9], [10, 11, 12]]);
        let c = a % b;
        let d = MatrixSxS::<i32, 2, 3>::from([[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxs_dxs() {
        let a = MatrixSxS::from([[11, 45, 39], [4, 5, 6]]);
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        let c = a % b;
        let d = MatrixSxS::from([[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxs_sxd() {
        let a = MatrixSxS::from([[11, 45, 39], [4, 5, 6]]);
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a % b;
        let d = MatrixSxS::from([[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxs_dxd() {
        let a = MatrixSxS::from([[11, 45, 39], [4, 5, 6]]);
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a % b;
        let d = MatrixSxS::from([[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    // TransposeDxD
    // --------------------------------------------------
    #[test]
    fn transpose_dxd_dxd() {
        let a = MatrixDxD::try_from(vec![vec![11, 4], vec![45, 5], vec![39, 6]]).unwrap();
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a.transpose_ref() % b;
        let d = MatrixDxD::try_from(vec![vec![4, 5, 3], vec![4, 5, 6]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_dxd_dxs() {
        let a = MatrixDxD::try_from(vec![vec![11, 4], vec![45, 5], vec![39, 6]]).unwrap();
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        let c = a.transpose_ref() % b;
        let d = MatrixDxS::from(vec![[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_dxd_sxd() {
        let a = MatrixDxD::try_from(vec![vec![11, 4], vec![45, 5], vec![39, 6]]).unwrap();
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a.transpose_ref() % b;
        let d = MatrixSxD::try_from([vec![4, 5, 3], vec![4, 5, 6]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_dxd_sxs() {
        let a = MatrixDxD::try_from(vec![vec![11, 4], vec![45, 5], vec![39, 6]]).unwrap();
        let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
        let c = a.transpose_ref() % b;
        let d = MatrixSxS::from([[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    // TransposeDxS
    // --------------------------------------------------
    #[test]
    fn transpose_dxs_dxs() {
        let a = MatrixSxD::try_from([vec![11, 4], vec![45, 5], vec![39, 6]]).unwrap();
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        let c = a.transpose_ref() % b;
        let d = MatrixDxS::from(vec![[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_dxs_sxd() {
        let a = MatrixSxD::try_from([vec![11, 4], vec![45, 5], vec![39, 6]]).unwrap();
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a.transpose_ref() % b;
        let d = MatrixSxS::from([[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_dxs_dxd() {
        let a = MatrixSxD::try_from([vec![11, 4], vec![45, 5], vec![39, 6]]).unwrap();
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a.transpose_ref() % b;
        let d = MatrixDxS::from(vec![[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_dxs_sxs() {
        let a = MatrixSxD::try_from([vec![11, 4], vec![45, 5], vec![39, 6]]).unwrap();
        let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
        let c = a.transpose_ref() % b;
        let d = MatrixSxS::from([[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    // TransposeSxD
    // --------------------------------------------------
    #[test]
    fn transpose_sxd_sxd() {
        let a = MatrixDxS::from(vec![[11, 4], [45, 5], [39, 6]]);
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a.transpose_ref() % b;
        let d = MatrixSxD::try_from([vec![4, 5, 3], vec![4, 5, 6]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_sxd_dxs() {
        let a = MatrixDxS::from(vec![[11, 4], [45, 5], [39, 6]]);
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        let c = a.transpose_ref() % b;
        let d = MatrixSxS::from([[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_sxd_sxs() {
        let a = MatrixDxS::from(vec![[11, 4], [45, 5], [39, 6]]);
        let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
        let c = a.transpose_ref() % b;
        let d = MatrixSxS::from([[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_sxd_dxd() {
        let a = MatrixDxS::from(vec![[11, 4], [45, 5], [39, 6]]);
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a.transpose_ref() % b;
        let d = MatrixSxD::try_from([vec![4, 5, 3], vec![4, 5, 6]]).unwrap();
        assert_eq!(c, d);
    }
    // TransposeSxS
    // --------------------------------------------------
    #[test]
    fn transpose_sxs_sxs() {
        let a = MatrixSxS::<i32, 3, 2>::from([[11, 4], [45, 5], [39, 6]]);
        let b = MatrixSxS::<i32, 2, 3>::from([[7, 8, 9], [10, 11, 12]]);
        let c = a.transpose_ref() % b;
        let d = MatrixSxS::<i32, 2, 3>::from([[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_sxs_dxs() {
        let a = MatrixSxS::<i32, 3, 2>::from([[11, 4], [45, 5], [39, 6]]);
        let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        let c = a.transpose_ref() % b;
        let d = MatrixSxS::from([[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_sxs_sxd() {
        let a = MatrixSxS::<i32, 3, 2>::from([[11, 4], [45, 5], [39, 6]]);
        let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a.transpose_ref() % b;
        let d = MatrixSxS::from([[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_sxs_dxd() {
        let a = MatrixSxS::<i32, 3, 2>::from([[11, 4], [45, 5], [39, 6]]);
        let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let c = a.transpose_ref() % b;
        let d = MatrixSxS::from([[4, 5, 3], [4, 5, 6]]);
        assert_eq!(c, d);
    }
}
