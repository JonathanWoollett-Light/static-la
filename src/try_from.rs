use crate::*;
use std::convert::TryFrom;

// MatrixDxD
// --------------------------------------------------
impl<T> TryFrom<Vec<Vec<T>>> for MatrixDxD<T> {
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
        let (rows, columns) = (data.len(), data[0].len());
        Ok(Self {
            data: data.into_iter().flatten().collect::<Vec<_>>(),
            rows,
            columns,
        })
    }
}
impl<T> TryFrom<(usize, usize, Vec<T>)> for MatrixDxD<T> {
    type Error = &'static str;
    fn try_from((rows, columns, data): (usize, usize, Vec<T>)) -> Result<Self, Self::Error> {
        if rows * columns == data.len() {
            Ok(Self {
                data: data,
                rows,
                columns,
            })
        } else {
            Err("Inside `Vec`s differ in length.")
        }
    }
}
impl<T: Clone> TryFrom<(usize, usize, &[T])> for MatrixDxD<T> {
    type Error = &'static str;
    fn try_from((rows, columns, data): (usize, usize, &[T])) -> Result<Self, Self::Error> {
        if rows * columns == data.len() {
            Ok(Self {
                data: Vec::from(data),
                rows,
                columns,
            })
        } else {
            Err("Inside `Vec`s differ in length.")
        }
    }
}
// MatrixSxD
// --------------------------------------------------
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
impl<T, const ROWS: usize> TryFrom<(usize, Vec<T>)> for MatrixSxD<T, ROWS> {
    type Error = &'static str;
    fn try_from((columns, data): (usize, Vec<T>)) -> Result<Self, Self::Error> {
        if ROWS * columns == data.len() {
            Ok(Self {
                data: data,
                columns,
            })
        } else {
            Err("Inside `Vec`s differ in length.")
        }
    }
}
impl<T: Clone, const ROWS: usize> TryFrom<(usize, &[T])> for MatrixSxD<T, ROWS> {
    type Error = &'static str;
    fn try_from((columns, data): (usize, &[T])) -> Result<Self, Self::Error> {
        if ROWS * columns == data.len() {
            Ok(Self {
                data: Vec::from(data),
                columns,
            })
        } else {
            Err("Inside `Vec`s differ in length.")
        }
    }
}
// MatrixDxS
// --------------------------------------------------
impl<T, const COLUMNS: usize> TryFrom<(usize, Vec<T>)> for MatrixDxS<T, COLUMNS> {
    type Error = &'static str;
    fn try_from((rows, data): (usize, Vec<T>)) -> Result<Self, Self::Error> {
        if COLUMNS * rows == data.len() {
            Ok(Self { data: data, rows })
        } else {
            Err("Inside `Vec`s differ in length.")
        }
    }
}
impl<T: Clone, const COLUMNS: usize> TryFrom<(usize, &[T])> for MatrixDxS<T, COLUMNS> {
    type Error = &'static str;
    fn try_from((rows, data): (usize, &[T])) -> Result<Self, Self::Error> {
        if COLUMNS * rows == data.len() {
            Ok(Self {
                data: Vec::from(data),
                rows,
            })
        } else {
            Err("Inside `Vec`s differ in length.")
        }
    }
}
