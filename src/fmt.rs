use crate::*;
use std::fmt;

impl<T: fmt::Debug, const ROWS: usize, const COLUMNS: usize> fmt::Display
    for MatrixSxS<T, ROWS, COLUMNS>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}
impl<T: fmt::Debug> fmt::Display for MatrixDxD<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}
impl<T: fmt::Debug, const ROWS: usize> fmt::Display for MatrixSxD<T, ROWS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}
impl<T: fmt::Debug, const COLUMNS: usize> fmt::Display for MatrixDxS<T, COLUMNS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}
