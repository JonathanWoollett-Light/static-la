use crate::*;
impl<T, const ROWS: usize, const COLUMNS: usize> From<[[T; COLUMNS]; ROWS]>
    for MatrixSxS<T, ROWS, COLUMNS>
{
    fn from(data: [[T; COLUMNS]; ROWS]) -> Self {
        Self { data }
    }
}
impl<T, const COLUMNS: usize> From<Vec<[T; COLUMNS]>> for MatrixDxS<T, COLUMNS> {
    fn from(data: Vec<[T; COLUMNS]>) -> Self {
        Self { data }
    }
}
