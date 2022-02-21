use crate::*;
use std::convert::TryFrom;

impl<T: Clone> TryFrom<Vec<Vec<T>>> for MatrixDxD<T> {
    type Error = &'static str;
    fn try_from(data: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        // If it contains some rows.
        if !data.is_empty() {
            // Check every row has the same number of columns.
            let cols = data[0].len();
            if data.iter().any(|c| c.len() != cols) {
                return Err("Inside `Vec`s differ in length.");
            }
        }
        Ok(Self {
            data: data.iter().cloned().flatten().collect::<Vec<_>>(),
            rows: data.len(),
            columns: data[0].len(),
        })
    }
}
impl<T: Clone, const ROWS: usize> TryFrom<[Vec<T>; ROWS]> for MatrixSxD<T, ROWS> {
    type Error = &'static str;
    fn try_from(data: [Vec<T>; ROWS]) -> Result<Self, Self::Error> {
        if ROWS > 0 {
            // Check every row has the same number of columns.
            let cols = data[0].len();
            if data.iter().any(|c| c.len() != cols) {
                return Err("Inside `Vec`s differ in length.");
            }
        }

        Ok(Self {
            data: data.concat(),
            columns: data[0].len(),
        })
    }
}
