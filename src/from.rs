use crate::*;
use std::convert::TryInto;
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
// Because this requires a user to specify the type for `from` this isn't implemented since its annoying
// impl<T: Clone + std::fmt::Debug, const ROWS: usize, const COLUMNS: usize> From<[T; ROWS*COLUMNS]>
//     for MatrixSxS<T, ROWS, COLUMNS>
// where
//     [(); ROWS * COLUMNS]:,
// {
//     fn from(data: [T; ROWS*COLUMNS]) -> Self {
//         Self {
//             data
//         }
//     }
// }
// This isn't as efficient as above, but it doesn't affect existing functionality and isn't annoying.
impl<T: Clone + std::fmt::Debug, const ROWS: usize, const COLUMNS: usize> From<Vec<T>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn from(data: Vec<T>) -> Self {
        assert_eq!(data.len(), ROWS * COLUMNS);
        Self {
            data: data.try_into().unwrap(),
        }
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
