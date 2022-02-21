use crate::*;
use std::convert::TryInto;
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
impl<T, const COLUMNS: usize> From<Vec<[T; COLUMNS]>> for MatrixDxS<T, COLUMNS> {
    fn from(data: Vec<[T; COLUMNS]>) -> Self {
        let len = data.len();
        Self {
            data: data.into_iter().flatten().collect::<Vec<_>>(),
            rows: len,
        }
    }
}
