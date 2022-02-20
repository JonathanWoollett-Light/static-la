use crate::*;
use std::convert::TryFrom;

impl<T> TryFrom<Vec<Vec<T>>> for MatrixDxD<T> {
    type Error = &'static str;
    fn try_from(data: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        // If it contains some rows.
        if !data.is_empty() {
            // Check every row has the same number of columns.
            let columns = data[0].len();
            for row in data.iter() {
                if row.len() != columns {
                    return Err("Inside `Vec`s differ in length.");
                }
            }
        }
        Ok(Self { data })
    }
}
impl<T, const ROWS: usize> TryFrom<[Vec<T>; ROWS]> for MatrixSxD<T, ROWS> {
    type Error = &'static str;
    fn try_from(data: [Vec<T>; ROWS]) -> Result<Self, Self::Error> {
        // If it contains some rows.
        if ROWS != 0 {
            // Check every row has the same number of columns.
            let columns = data[0].len();
            for row in data.iter() {
                if row.len() != columns {
                    return Err("Inside `Vec`s differ in length.");
                }
            }
        }
        Ok(Self { data })
    }
}
