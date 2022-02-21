use crate::*;

impl<T> MatrixDxD<T> {
    /// Number of elements in matrix.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Number of columns.
    pub fn columns(&self) -> usize {
        self.columns
    }
    /// Number of rows.
    pub fn rows(&self) -> usize {
        self.rows
    }
}
impl<T, const COLUMNS: usize> MatrixDxS<T, COLUMNS> {
    /// Number of elements in matrix.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Number of columns.
    pub const fn columns(&self) -> usize {
        COLUMNS
    }
    /// Number of rows.
    pub fn rows(&self) -> usize {
        self.rows
    }
}
impl<T, const ROWS: usize> MatrixSxD<T, ROWS> {
    /// Number of elements in matrix.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Number of rows.
    pub const fn rows(&self) -> usize {
        ROWS
    }
    /// Number of columns.
    pub fn columns(&self) -> usize {
        self.columns
    }
}
impl<T, const ROWS: usize, const COLUMNS: usize> MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    /// Number of elements in matrix.
    pub const fn len(&self) -> usize {
        ROWS * COLUMNS
    }
    /// Number of rows.
    pub const fn rows(&self) -> usize {
        ROWS
    }
    /// Number of columns.
    pub const fn columns(&self) -> usize {
        COLUMNS
    }
}
