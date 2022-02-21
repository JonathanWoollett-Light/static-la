use crate::*;
use std::cmp::PartialEq;

// MatrixDxD
// --------------------------------------------------
impl<T: PartialEq, const ROWS: usize, const COLUMNS: usize> PartialEq<MatrixSxS<T, ROWS, COLUMNS>>
    for MatrixDxD<T>
where
    [(); ROWS * COLUMNS]:,
{
    fn eq(&self, other: &MatrixSxS<T, ROWS, COLUMNS>) -> bool {
        self.data == other.data
    }
}
impl<T: PartialEq, const COLUMNS: usize> PartialEq<MatrixDxS<T, COLUMNS>> for MatrixDxD<T> {
    fn eq(&self, other: &MatrixDxS<T, COLUMNS>) -> bool {
        self.data == other.data
    }
}
impl<T: PartialEq, const ROWS: usize> PartialEq<MatrixSxD<T, ROWS>> for MatrixDxD<T> {
    fn eq(&self, other: &MatrixSxD<T, ROWS>) -> bool {
        self.data == other.data
    }
}
// MatrixDxS
// --------------------------------------------------
impl<T: PartialEq, const ROWS: usize, const COLUMNS: usize> PartialEq<MatrixSxS<T, ROWS, COLUMNS>>
    for MatrixDxS<T, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn eq(&self, other: &MatrixSxS<T, ROWS, COLUMNS>) -> bool {
        self.data == other.data
    }
}
impl<T: PartialEq, const COLUMNS: usize> PartialEq<MatrixDxD<T>> for MatrixDxS<T, COLUMNS> {
    fn eq(&self, other: &MatrixDxD<T>) -> bool {
        self.data == other.data
    }
}
impl<T: PartialEq, const ROWS: usize, const COLUMNS: usize> PartialEq<MatrixSxD<T, ROWS>>
    for MatrixDxS<T, COLUMNS>
{
    fn eq(&self, other: &MatrixSxD<T, ROWS>) -> bool {
        self.data == other.data
    }
}
// MatrixSxD
// --------------------------------------------------
impl<T: PartialEq, const ROWS: usize, const COLUMNS: usize> PartialEq<MatrixSxS<T, ROWS, COLUMNS>>
    for MatrixSxD<T, ROWS>
where
    [(); ROWS * COLUMNS]:,
{
    fn eq(&self, other: &MatrixSxS<T, ROWS, COLUMNS>) -> bool {
        self.data == other.data
    }
}
impl<T: PartialEq, const ROWS: usize> PartialEq<MatrixDxD<T>> for MatrixSxD<T, ROWS> {
    fn eq(&self, other: &MatrixDxD<T>) -> bool {
        self.data == other.data
    }
}
impl<T: PartialEq, const ROWS: usize, const COLUMNS: usize> PartialEq<MatrixDxS<T, COLUMNS>>
    for MatrixSxD<T, ROWS>
{
    fn eq(&self, other: &MatrixDxS<T, COLUMNS>) -> bool {
        self.data == other.data
    }
}
// MatrixSxS
// --------------------------------------------------
impl<T: PartialEq, const ROWS: usize, const COLUMNS: usize> PartialEq<MatrixSxD<T, ROWS>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn eq(&self, other: &MatrixSxD<T, ROWS>) -> bool {
        other.data == self.data
    }
}
impl<T: PartialEq, const ROWS: usize, const COLUMNS: usize> PartialEq<MatrixDxD<T>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn eq(&self, other: &MatrixDxD<T>) -> bool {
        other.data == self.data
    }
}
impl<T: PartialEq, const ROWS: usize, const COLUMNS: usize> PartialEq<MatrixDxS<T, COLUMNS>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn eq(&self, other: &MatrixDxS<T, COLUMNS>) -> bool {
        other.data == self.data
    }
}
