use crate::*;
use std::ops::{BitAnd, BitAndAssign};

// T
// --------------------------------------------------
impl<T: BitAndAssign + Copy> BitAnd<T> for MatrixDxD<T> {
    type Output = Self;
    fn bitand(mut self, x: T) -> Self::Output {
        for a in self.data.iter_mut() {
            *a &= x;
        }
        self
    }
}
impl<T: BitAndAssign + Copy, const COLUMNS: usize> BitAnd<T> for MatrixDxS<T, COLUMNS> {
    type Output = Self;
    fn bitand(mut self, x: T) -> Self::Output {
        for a in self.data.iter_mut() {
            *a &= x;
        }
        self
    }
}
impl<T: BitAndAssign + Copy, const ROWS: usize> BitAnd<T> for MatrixSxD<T, ROWS> {
    type Output = Self;
    fn bitand(mut self, x: T) -> Self::Output {
        for a in self.data.iter_mut() {
            *a &= x;
        }
        self
    }
}
impl<T: BitAndAssign + Copy, const ROWS: usize, const COLUMNS: usize> BitAnd<T>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = Self;
    fn bitand(mut self, x: T) -> Self::Output {
        for a in self.data.iter_mut() {
            *a &= x;
        }
        self
    }
}
// MatrixDxD
// --------------------------------------------------
impl<T: BitAndAssign + Copy> BitAnd<MatrixDxD<T>> for MatrixDxD<T> {
    type Output = Self;
    fn bitand(mut self, other: Self) -> Self {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        assert_eq!(self.columns, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a &= *b;
        }
        self
    }
}
impl<T: BitAndAssign + Copy, const COLUMNS: usize> BitAnd<MatrixDxS<T, COLUMNS>> for MatrixDxD<T> {
    type Output = MatrixDxS<T, COLUMNS>;
    fn bitand(self, mut other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");
        for (a, b) in self.data.iter().zip(other.data.iter_mut()) {
            *b &= *a;
        }
        other
    }
}
impl<T: BitAndAssign + Copy, const ROWS: usize> BitAnd<MatrixSxD<T, ROWS>> for MatrixDxD<T> {
    type Output = MatrixSxD<T, ROWS>;
    fn bitand(self, mut other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        assert_eq!(self.columns, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter().zip(other.data.iter_mut()) {
            *b &= *a;
        }
        other
    }
}
impl<T: BitAndAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    BitAnd<MatrixSxS<T, ROWS, COLUMNS>> for MatrixDxD<T>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn bitand(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");
        for (a, b) in self.data.iter().zip(other.data.iter_mut()) {
            *b &= *a;
        }
        other
    }
}
// MatrixDxS
// --------------------------------------------------
impl<T: BitAndAssign + Copy, const COLUMNS: usize> BitAnd<MatrixDxS<T, COLUMNS>>
    for MatrixDxS<T, COLUMNS>
{
    type Output = Self;
    fn bitand(mut self, other: Self) -> Self {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a &= *b;
        }
        self
    }
}
impl<T: BitAnd<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    BitAnd<MatrixSxD<T, ROWS>> for MatrixDxS<T, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn bitand(self, other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");
        let mut data = [Default::default(); ROWS * COLUMNS];
        for i in 0..ROWS * COLUMNS {
            data[i] = self.data[i] & other.data[i];
        }
        Self::Output { data }
    }
}
impl<T: BitAndAssign + Copy, const COLUMNS: usize> BitAnd<MatrixDxD<T>> for MatrixDxS<T, COLUMNS> {
    type Output = Self;
    fn bitand(mut self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(self.rows, other.rows, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a &= *b;
        }
        self
    }
}
impl<T: BitAndAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    BitAnd<MatrixSxS<T, ROWS, COLUMNS>> for MatrixDxS<T, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn bitand(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        for (a, b) in self.data.iter().zip(other.data.iter_mut()) {
            *b &= *a;
        }
        other
    }
}
// MatrixSxD
// --------------------------------------------------
impl<T: BitAndAssign + Copy, const ROWS: usize> BitAnd<MatrixSxD<T, ROWS>> for MatrixSxD<T, ROWS> {
    type Output = Self;
    fn bitand(mut self, other: Self) -> Self {
        assert_eq!(self.columns, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a &= *b;
        }
        self
    }
}
impl<T: BitAnd<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    BitAnd<MatrixDxS<T, COLUMNS>> for MatrixSxD<T, ROWS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn bitand(self, other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");
        let mut data = [Default::default(); ROWS * COLUMNS];
        for i in 0..ROWS * COLUMNS {
            data[i] = self.data[i] & other.data[i];
        }
        Self::Output { data }
    }
}
impl<T: BitAndAssign + Copy, const ROWS: usize> BitAnd<MatrixDxD<T>> for MatrixSxD<T, ROWS> {
    type Output = Self;
    fn bitand(mut self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(self.columns, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a &= *b;
        }
        self
    }
}
impl<T: BitAndAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    BitAnd<MatrixSxS<T, ROWS, COLUMNS>> for MatrixSxD<T, ROWS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn bitand(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");
        for (a, b) in self.data.iter().zip(other.data.iter_mut()) {
            *b &= *a;
        }
        other
    }
}
// MatrixSxS
// --------------------------------------------------
impl<T: BitAndAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    BitAnd<MatrixSxS<T, ROWS, COLUMNS>> for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = Self;
    fn bitand(mut self, other: Self) -> Self {
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a &= *b;
        }
        self
    }
}
impl<T: BitAndAssign + Copy, const ROWS: usize, const COLUMNS: usize> BitAnd<MatrixDxS<T, COLUMNS>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = Self;
    fn bitand(mut self, other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a &= *b;
        }
        self
    }
}
impl<T: BitAndAssign + Copy, const ROWS: usize, const COLUMNS: usize> BitAnd<MatrixSxD<T, ROWS>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = Self;
    fn bitand(mut self, other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a &= *b;
        }
        self
    }
}
impl<T: BitAndAssign + Copy, const ROWS: usize, const COLUMNS: usize> BitAnd<MatrixDxD<T>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = Self;
    fn bitand(mut self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a &= *b;
        }
        self
    }
}
// TransposeDxD
// --------------------------------------------------
impl<'a, T: BitAndAssign + Copy> BitAnd<MatrixDxD<T>> for TransposeDxD<'a, T> {
    type Output = MatrixDxD<T>;
    fn bitand(self, mut other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(self.rows(), other.rows, "Non-matching rows");
        assert_eq!(self.columns(), other.columns, "Non-matching columns");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b &= *a;
        }
        other
    }
}
impl<'a, T: BitAndAssign + Copy, const COLUMNS: usize> BitAnd<MatrixDxS<T, COLUMNS>>
    for TransposeDxD<'a, T>
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn bitand(self, mut other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows(), other.rows, "Non-matching rows");
        assert_eq!(self.columns(), COLUMNS, "Non-matching columns");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b &= *a;
        }
        other
    }
}
impl<'a, T: BitAndAssign + Copy, const ROWS: usize> BitAnd<MatrixSxD<T, ROWS>>
    for TransposeDxD<'a, T>
{
    type Output = MatrixSxD<T, ROWS>;
    fn bitand(self, mut other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(self.rows(), ROWS, "Non-matching rows");
        assert_eq!(self.columns(), other.columns, "Non-matching columns");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b &= *a;
        }
        other
    }
}
impl<'a, T: BitAndAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    BitAnd<MatrixSxS<T, ROWS, COLUMNS>> for TransposeDxD<'a, T>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn bitand(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows(), ROWS, "Non-matching rows");
        assert_eq!(self.columns(), COLUMNS, "Non-matching columns");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b &= *a;
        }
        other
    }
}
// TransposeDxS
// --------------------------------------------------
impl<'a, T: BitAnd<Output = T> + Default + Copy, const ROWS: usize, const COLUMNS: usize>
    BitAnd<MatrixDxS<T, COLUMNS>> for TransposeDxS<'a, T, ROWS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn bitand(self, other: MatrixDxS<T, COLUMNS>) -> MatrixSxS<T, ROWS, COLUMNS> {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(self.columns(), COLUMNS, "Non-matching columns");

        let mut data = [Default::default(); ROWS * COLUMNS];
        for (c, (a, b)) in data.iter_mut().zip(self.iter().zip(other.data.iter())) {
            *c = *a & *b;
        }
        Self::Output { data }
    }
}
impl<'a, T: BitAndAssign + Copy, const ROWS: usize> BitAnd<MatrixSxD<T, ROWS>>
    for TransposeDxS<'a, T, ROWS>
{
    type Output = MatrixSxD<T, ROWS>;
    fn bitand(self, mut other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(self.columns(), other.columns, "Non-matching columns");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b &= *a;
        }
        other
    }
}
impl<'a, T: BitAnd<Output = T> + Copy, const ROWS: usize> BitAnd<MatrixDxD<T>>
    for TransposeDxS<'a, T, ROWS>
{
    type Output = MatrixSxD<T, ROWS>;
    fn bitand(self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(self.columns(), other.columns, "Non-matching columns");
        Self::Output {
            data: self
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| *a & *b)
                .collect(),
            columns: self.columns(),
        }
    }
}
impl<'a, T: BitAndAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    BitAnd<MatrixSxS<T, ROWS, COLUMNS>> for TransposeDxS<'a, T, ROWS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn bitand(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.columns(), COLUMNS, "Non-matching columns");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b &= *a;
        }
        other
    }
}
// TransposeSxD
// --------------------------------------------------
impl<'a, T: BitAnd<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    BitAnd<MatrixSxD<T, ROWS>> for TransposeSxD<'a, T, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn bitand(self, other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(self.rows(), ROWS, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");

        let mut data = [Default::default(); ROWS * COLUMNS];
        for (c, (a, b)) in data.iter_mut().zip(self.iter().zip(other.data.iter())) {
            *c = *a & *b;
        }
        Self::Output { data }
    }
}
impl<'a, T: BitAndAssign + Copy, const COLUMNS: usize> BitAnd<MatrixDxS<T, COLUMNS>>
    for TransposeSxD<'a, T, COLUMNS>
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn bitand(self, mut other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows(), other.rows, "Non-matching rows");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b &= *a;
        }
        other
    }
}
impl<'a, T: BitAnd<Output = T> + Copy, const COLUMNS: usize> BitAnd<MatrixDxD<T>>
    for TransposeSxD<'a, T, COLUMNS>
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn bitand(self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(self.rows(), other.rows, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");

        Self::Output {
            data: self
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| *a & *b)
                .collect(),
            rows: self.rows(),
        }
    }
}
impl<'a, T: BitAndAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    BitAnd<MatrixSxS<T, ROWS, COLUMNS>> for TransposeSxD<'a, T, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn bitand(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows(), ROWS, "Non-matching columns");
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b &= *a;
        }
        other
    }
}
// TransposeSxS
// --------------------------------------------------
impl<'a, T: BitAndAssign + Copy, const ROWS: usize, const COLUMNS: usize>
    BitAnd<MatrixSxS<T, ROWS, COLUMNS>> for TransposeSxS<'a, T, COLUMNS, ROWS>
where
    [(); ROWS * COLUMNS]:,
    [(); COLUMNS * ROWS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn bitand(self, mut other: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        for (a, b) in self.iter().zip(other.data.iter_mut()) {
            *b &= *a;
        }
        other
    }
}
impl<'a, T: BitAnd<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    BitAnd<MatrixDxS<T, COLUMNS>> for TransposeSxS<'a, T, COLUMNS, ROWS>
where
    [(); COLUMNS * ROWS]:,
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn bitand(self, other: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        let mut data = [Default::default(); ROWS * COLUMNS];
        for (c, (a, b)) in data.iter_mut().zip(self.iter().zip(other.data.iter())) {
            *c = *a & *b;
        }
        Self::Output { data }
    }
}
impl<'a, T: BitAnd<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    BitAnd<MatrixSxD<T, ROWS>> for TransposeSxS<'a, T, COLUMNS, ROWS>
where
    [(); COLUMNS * ROWS]:,
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn bitand(self, other: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");
        let mut data = [Default::default(); ROWS * COLUMNS];
        for (c, (a, b)) in data.iter_mut().zip(self.iter().zip(other.data.iter())) {
            *c = *a & *b;
        }
        Self::Output { data }
    }
}
impl<'a, T: BitAnd<Output = T> + Copy + Default, const ROWS: usize, const COLUMNS: usize>
    BitAnd<MatrixDxD<T>> for TransposeSxS<'a, T, COLUMNS, ROWS>
where
    [(); COLUMNS * ROWS]:,
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxS<T, ROWS, COLUMNS>;
    fn bitand(self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(ROWS, other.rows, "Non-matching rows");
        assert_eq!(COLUMNS, other.columns, "Non-matching columns");
        let mut data = [Default::default(); ROWS * COLUMNS];
        for (c, (a, b)) in data.iter_mut().zip(self.iter().zip(other.data.iter())) {
            *c = *a & *b;
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
        let a =
            MatrixDxD::try_from(vec![vec![false, true, false], vec![true, false, true]]).unwrap();
        assert_eq!(
            a & true,
            MatrixDxD::try_from(vec![vec![false, true, false], vec![true, false, true]]).unwrap()
        );
    }
    #[test]
    fn t_dxs() {
        let a = MatrixDxS::from(vec![[false, true, false], [true, false, true]]);
        assert_eq!(
            a & true,
            MatrixDxS::from(vec![[false, true, false], [true, false, true]])
        );
    }
    #[test]
    fn t_sxd() {
        let a = MatrixSxD::try_from([vec![false, true, false], vec![true, false, true]]).unwrap();
        assert_eq!(
            a & true,
            MatrixSxD::try_from([vec![false, true, false], vec![true, false, true]]).unwrap()
        );
    }
    #[test]
    fn t_sxs() {
        let a = MatrixSxS::<bool, 2, 3>::from([[false, true, false], [true, false, true]]);
        assert_eq!(
            a & true,
            MatrixSxS::<bool, 2, 3>::from([[false, true, false], [true, false, true]])
        );
    }
    // MatrixDxD
    // --------------------------------------------------
    #[test]
    fn dxd_dxd() {
        let a =
            MatrixDxD::try_from(vec![vec![true, true, false], vec![true, false, false]]).unwrap();
        let b =
            MatrixDxD::try_from(vec![vec![false, true, false], vec![true, false, true]]).unwrap();
        let c = a & b;
        let d =
            MatrixDxD::try_from(vec![vec![false, true, false], vec![true, false, false]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn dxd_dxs() {
        let a =
            MatrixDxD::try_from(vec![vec![true, true, false], vec![true, false, false]]).unwrap();
        let b = MatrixDxS::from(vec![[false, true, false], [true, false, true]]);
        let c = a & b;
        let d = MatrixDxS::from(vec![[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxd_sxd() {
        let a =
            MatrixDxD::try_from(vec![vec![true, true, false], vec![true, false, false]]).unwrap();
        let b = MatrixSxD::try_from([vec![false, true, false], vec![true, false, true]]).unwrap();
        let c = a & b;
        let d = MatrixSxD::try_from([vec![false, true, false], vec![true, false, false]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn dxd_sxs() {
        let a =
            MatrixDxD::try_from(vec![vec![true, true, false], vec![true, false, false]]).unwrap();
        let b = MatrixSxS::from([[false, true, false], [true, false, true]]);
        let c = a & b;
        let d = MatrixSxS::from([[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
    // MatrixDxS
    // --------------------------------------------------
    #[test]
    fn dxs_dxs() {
        let a = MatrixDxS::from(vec![[true, true, false], [true, false, false]]);
        let b = MatrixDxS::from(vec![[false, true, false], [true, false, true]]);
        let c = a & b;
        let d = MatrixDxS::from(vec![[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxs_sxd() {
        let a = MatrixDxS::from(vec![[true, true, false], [true, false, false]]);
        let b = MatrixSxD::try_from([vec![false, true, false], vec![true, false, true]]).unwrap();
        let c = a & b;
        let d = MatrixSxS::from([[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxs_dxd() {
        let a = MatrixDxS::from(vec![[true, true, false], [true, false, false]]);
        let b =
            MatrixDxD::try_from(vec![vec![false, true, false], vec![true, false, true]]).unwrap();
        let c = a & b;
        let d = MatrixDxS::from(vec![[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxs_sxs() {
        let a = MatrixDxS::from(vec![[true, true, false], [true, false, false]]);
        let b = MatrixSxS::from([[false, true, false], [true, false, true]]);
        let c = a & b;
        let d = MatrixSxS::from([[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
    // MatrixSxD
    // --------------------------------------------------
    #[test]
    fn sxd_sxd() {
        let a = MatrixSxD::try_from([vec![true, true, false], vec![true, false, false]]).unwrap();
        let b = MatrixSxD::try_from([vec![false, true, false], vec![true, false, true]]).unwrap();
        let c = a & b;
        let d = MatrixSxD::try_from([vec![false, true, false], vec![true, false, false]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn sxd_dxs() {
        let a = MatrixSxD::try_from([vec![true, true, false], vec![true, false, false]]).unwrap();
        let b = MatrixDxS::from(vec![[false, true, false], [true, false, true]]);
        let c = a & b;
        let d = MatrixSxS::from([[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxd_sxs() {
        let a = MatrixSxD::try_from([vec![true, true, false], vec![true, false, false]]).unwrap();
        let b = MatrixSxS::from([[false, true, false], [true, false, true]]);
        let c = a & b;
        let d = MatrixSxS::from([[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxd_dxd() {
        let a = MatrixSxD::try_from([vec![true, true, false], vec![true, false, false]]).unwrap();
        let b =
            MatrixDxD::try_from(vec![vec![false, true, false], vec![true, false, true]]).unwrap();
        let c = a & b;
        let d = MatrixSxD::try_from([vec![false, true, false], vec![true, false, false]]).unwrap();
        assert_eq!(c, d);
    }
    // MatrixSxS
    // --------------------------------------------------
    #[test]
    fn sxs_sxs() {
        let a = MatrixSxS::<bool, 2, 3>::from([[true, true, false], [true, false, false]]);
        let b = MatrixSxS::<bool, 2, 3>::from([[false, true, false], [true, false, true]]);
        let c = a & b;
        let d = MatrixSxS::<bool, 2, 3>::from([[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxs_dxs() {
        let a = MatrixSxS::from([[true, true, false], [true, false, false]]);
        let b = MatrixDxS::from(vec![[false, true, false], [true, false, true]]);
        let c = a & b;
        let d = MatrixSxS::from([[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxs_sxd() {
        let a = MatrixSxS::from([[true, true, false], [true, false, false]]);
        let b = MatrixSxD::try_from([vec![false, true, false], vec![true, false, true]]).unwrap();
        let c = a & b;
        let d = MatrixSxS::from([[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxs_dxd() {
        let a = MatrixSxS::from([[true, true, false], [true, false, false]]);
        let b =
            MatrixDxD::try_from(vec![vec![false, true, false], vec![true, false, true]]).unwrap();
        let c = a & b;
        let d = MatrixSxS::from([[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
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
            MatrixDxD::try_from(vec![vec![false, true, false], vec![true, false, true]]).unwrap();
        let c = a.transpose_ref() & b;
        let d =
            MatrixDxD::try_from(vec![vec![false, true, false], vec![true, false, false]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_dxd_dxs() {
        let a = MatrixDxD::try_from(vec![
            vec![true, true],
            vec![true, false],
            vec![false, false],
        ])
        .unwrap();
        let b = MatrixDxS::from(vec![[false, true, false], [true, false, true]]);
        let c = a.transpose_ref() & b;
        let d = MatrixDxS::from(vec![[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_dxd_sxd() {
        let a = MatrixDxD::try_from(vec![
            vec![true, true],
            vec![true, false],
            vec![false, false],
        ])
        .unwrap();
        let b = MatrixSxD::try_from([vec![false, true, false], vec![true, false, true]]).unwrap();
        let c = a.transpose_ref() & b;
        let d = MatrixSxD::try_from([vec![false, true, false], vec![true, false, false]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_dxd_sxs() {
        let a = MatrixDxD::try_from(vec![
            vec![true, true],
            vec![true, false],
            vec![false, false],
        ])
        .unwrap();
        let b = MatrixSxS::from([[false, true, false], [true, false, true]]);
        let c = a.transpose_ref() & b;
        let d = MatrixSxS::from([[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
    // TransposeDxS
    // --------------------------------------------------
    #[test]
    fn transpose_dxs_dxs() {
        let a =
            MatrixSxD::try_from([vec![true, true], vec![true, false], vec![false, false]]).unwrap();
        let b = MatrixDxS::from(vec![[false, true, false], [true, false, true]]);
        let c = a.transpose_ref() & b;
        let d = MatrixDxS::from(vec![[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_dxs_sxd() {
        let a =
            MatrixSxD::try_from([vec![true, true], vec![true, false], vec![false, false]]).unwrap();
        let b = MatrixSxD::try_from([vec![false, true, false], vec![true, false, true]]).unwrap();
        let c = a.transpose_ref() & b;
        let d = MatrixSxS::from([[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_dxs_dxd() {
        let a =
            MatrixSxD::try_from([vec![true, true], vec![true, false], vec![false, false]]).unwrap();
        let b =
            MatrixDxD::try_from(vec![vec![false, true, false], vec![true, false, true]]).unwrap();
        let c = a.transpose_ref() & b;
        let d = MatrixDxS::from(vec![[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_dxs_sxs() {
        let a =
            MatrixSxD::try_from([vec![true, true], vec![true, false], vec![false, false]]).unwrap();
        let b = MatrixSxS::from([[false, true, false], [true, false, true]]);
        let c = a.transpose_ref() & b;
        let d = MatrixSxS::from([[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
    // TransposeSxD
    // --------------------------------------------------
    #[test]
    fn transpose_sxd_sxd() {
        let a = MatrixDxS::from(vec![[true, true], [true, false], [false, false]]);
        let b = MatrixSxD::try_from([vec![false, true, false], vec![true, false, true]]).unwrap();
        let c = a.transpose_ref() & b;
        let d = MatrixSxD::try_from([vec![false, true, false], vec![true, false, false]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_sxd_dxs() {
        let a = MatrixDxS::from(vec![[true, true], [true, false], [false, false]]);
        let b = MatrixDxS::from(vec![[false, true, false], [true, false, true]]);
        let c = a.transpose_ref() & b;
        let d = MatrixSxS::from([[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_sxd_sxs() {
        let a = MatrixDxS::from(vec![[true, true], [true, false], [false, false]]);
        let b = MatrixSxS::from([[false, true, false], [true, false, true]]);
        let c = a.transpose_ref() & b;
        let d = MatrixSxS::from([[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_sxd_dxd() {
        let a = MatrixDxS::from(vec![[true, true], [true, false], [false, false]]);
        let b =
            MatrixDxD::try_from(vec![vec![false, true, false], vec![true, false, true]]).unwrap();
        let c = a.transpose_ref() & b;
        let d = MatrixSxD::try_from([vec![false, true, false], vec![true, false, false]]).unwrap();
        assert_eq!(c, d);
    }
    // TransposeSxS
    // --------------------------------------------------
    #[test]
    fn transpose_sxs_sxs() {
        let a = MatrixSxS::<bool, 3, 2>::from([[true, true], [true, false], [false, false]]);
        let b = MatrixSxS::<bool, 2, 3>::from([[false, true, false], [true, false, true]]);
        let c = a.transpose_ref() & b;
        let d = MatrixSxS::<bool, 2, 3>::from([[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_sxs_dxs() {
        let a = MatrixSxS::<bool, 3, 2>::from([[true, true], [true, false], [false, false]]);
        let b = MatrixDxS::from(vec![[false, true, false], [true, false, true]]);
        let c = a.transpose_ref() & b;
        let d = MatrixSxS::from([[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_sxs_sxd() {
        let a = MatrixSxS::<bool, 3, 2>::from([[true, true], [true, false], [false, false]]);
        let b = MatrixSxD::try_from([vec![false, true, false], vec![true, false, true]]).unwrap();
        let c = a.transpose_ref() & b;
        let d = MatrixSxS::from([[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
    #[test]
    fn transpose_sxs_dxd() {
        let a = MatrixSxS::<bool, 3, 2>::from([[true, true], [true, false], [false, false]]);
        let b =
            MatrixDxD::try_from(vec![vec![false, true, false], vec![true, false, true]]).unwrap();
        let c = a.transpose_ref() & b;
        let d = MatrixSxS::from([[false, true, false], [true, false, false]]);
        assert_eq!(c, d);
    }
}
