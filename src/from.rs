use crate::*;
use std::convert::TryInto;
// MatrixDxD
// --------------------------------------------------
impl<T: Copy> From<(usize, usize, T)> for MatrixDxD<T> {
    fn from((rows, columns, x): (usize, usize, T)) -> Self {
        Self {
            data: vec![x; rows * columns],
            rows,
            columns,
        }
    }
}
// MatrixSxS
// --------------------------------------------------
impl<T: Clone + std::fmt::Debug, const ROWS: usize, const COLUMNS: usize> From<[[T; COLUMNS]; ROWS]>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn from(data: [[T; COLUMNS]; ROWS]) -> Self {
        Self {
            data: data
                .iter()
                .cloned()
                .flatten()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}
impl<T: Copy, const ROWS: usize, const COLUMNS: usize> From<T> for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn from(x: T) -> Self {
        Self {
            data: [x; ROWS * COLUMNS],
        }
    }
}
impl<T, const ROWS: usize, const COLUMNS: usize> From<[T; ROWS * COLUMNS]>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn from(data: [T; ROWS * COLUMNS]) -> Self {
        Self { data }
    }
}
impl<T: Clone, const ROWS: usize, const COLUMNS: usize> From<&[T; ROWS * COLUMNS]>
    for MatrixSxS<T, ROWS, COLUMNS>
{
    fn from(data: &[T; ROWS * COLUMNS]) -> Self {
        Self { data: data.clone() }
    }
}
// MatrixDxS
// --------------------------------------------------
impl<T, const COLUMNS: usize> From<Vec<[T; COLUMNS]>> for MatrixDxS<T, COLUMNS> {
    fn from(data: Vec<[T; COLUMNS]>) -> Self {
        let len = data.len();
        Self {
            data: data.into_iter().flatten().collect::<Vec<_>>(),
            rows: len,
        }
    }
}
impl<T: Copy, const COLUMNS: usize> From<(usize, T)> for MatrixDxS<T, COLUMNS> {
    fn from((rows, x): (usize, T)) -> Self {
        Self {
            data: vec![x; rows * COLUMNS],
            rows,
        }
    }
}
// MatrixSxD
// --------------------------------------------------
impl<T: Copy, const ROWS: usize> From<(usize, T)> for MatrixSxD<T, ROWS> {
    fn from((columns, x): (usize, T)) -> Self {
        Self {
            data: vec![x; ROWS * columns],
            columns,
        }
    }
}
