use crate::*;
use std::convert::TryInto;

/// A trait allowing the combination of matrices along their row axis.
pub trait AddRows<T> {
    type Output;
    /// This is somewhat inefficient, it will often not be as efficient as [`std::vec::Vec::push`].
    /// ```text
    ///                               ┌───────┐
    /// ┌───────┐          ┌───────┐  │ 1 2 3 │
    /// │ 1 3 5 │          │ 7 8 9 │  │ 4 5 6 │
    /// │ 2 4 6 │.add_rows(│ 1 2 3 │)=│ 7 8 9 │
    /// └───────┘          └───────┘  │ 1 2 3 │
    ///                               └───────┘
    /// ```
    fn add_rows(self, rows: T) -> Self::Output;
}
// MatrixSxS
// --------------------------------------------------
// MatrixSxS
impl<T: Clone + Default + Copy, const ROWS: usize, const COLUMNS: usize, const R: usize>
    AddRows<MatrixSxS<T, R, COLUMNS>> for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS + R]:,
{
    type Output = MatrixSxS<T, { ROWS + R }, COLUMNS>;
    fn add_rows(self, rows: MatrixSxS<T, R, COLUMNS>) -> Self::Output {
        let mut data = [[Default::default(); COLUMNS]; { ROWS + R }];
        // Copies old data
        for i in 0..ROWS {
            data[i].clone_from_slice(&self.data[i]);
        }
        // Copies new data
        for i in 0..R {
            data[i + ROWS].clone_from_slice(&rows.data[i]);
        }
        Self::Output { data }
    }
}
// MatrixDxS
impl<T: Clone, const ROWS: usize, const COLUMNS: usize> AddRows<MatrixDxS<T, COLUMNS>>
    for MatrixSxS<T, ROWS, COLUMNS>
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn add_rows(self, rows: MatrixDxS<T, COLUMNS>) -> Self::Output {
        let data = self
            .data
            .iter()
            .cloned()
            .chain(rows.data.into_iter())
            .collect::<Vec<_>>();
        Self::Output { data }
    }
}
// MatrixSxD
impl<T: Clone + Default + Copy, const ROWS: usize, const COLUMNS: usize, const R: usize>
    AddRows<MatrixSxD<T, R>> for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS + R]:,
{
    type Output = MatrixSxS<T, { ROWS + R }, COLUMNS>;
    fn add_rows(self, rows: MatrixSxD<T, R>) -> Self::Output {
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(COLUMNS, rows.data[0].len(), "Non-matching columns");

        let mut data = [[Default::default(); COLUMNS]; { ROWS + R }];
        // Copies old data
        for i in 0..ROWS {
            data[i] = self.data[i];
        }
        // Copies new data
        for i in 0..R {
            data[i + ROWS].clone_from_slice(&rows.data[i]);
        }
        Self::Output { data }
    }
}
// MatrixDxD
impl<T: Clone + std::fmt::Debug, const ROWS: usize, const COLUMNS: usize> AddRows<MatrixDxD<T>>
    for MatrixSxS<T, ROWS, COLUMNS>
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn add_rows(self, columns: MatrixDxD<T>) -> Self::Output {
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(COLUMNS, columns.data[0].len(), "Non-matching columns");

        let data = self
            .data
            .iter()
            .cloned()
            .chain(columns.data.into_iter().map(|v| v.try_into().unwrap()))
            .collect::<Vec<_>>();

        Self::Output { data }
    }
}
// MatrixDxS
// --------------------------------------------------
// MatrixSxS
impl<T: Clone + Default + Copy, const ROWS: usize, const COLUMNS: usize>
    AddRows<MatrixSxS<T, ROWS, COLUMNS>> for MatrixDxS<T, COLUMNS>
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn add_rows(mut self, rows: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        self.data.reserve(ROWS);
        for i in 0..ROWS {
            self.data.push(rows.data[i]);
        }
        self
    }
}
// MatrixDxS
impl<T: Clone, const COLUMNS: usize> AddRows<MatrixDxS<T, COLUMNS>> for MatrixDxS<T, COLUMNS> {
    type Output = MatrixDxS<T, COLUMNS>;
    fn add_rows(mut self, rows: MatrixDxS<T, COLUMNS>) -> Self::Output {
        self.data.reserve(rows.data.len());
        for row in rows.data.into_iter() {
            self.data.push(row);
        }
        self
    }
}
// MatrixSxD
impl<T: Clone + std::fmt::Debug, const ROWS: usize, const COLUMNS: usize>
    AddRows<MatrixSxD<T, ROWS>> for MatrixDxS<T, COLUMNS>
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn add_rows(mut self, rows: MatrixSxD<T, ROWS>) -> Self::Output {
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(COLUMNS, rows.data[0].len(), "Non-matching columns");

        self.data.reserve(rows.data.len());
        for row in rows.data.iter() {
            self.data.push(row.clone().try_into().unwrap());
        }
        self
    }
}
// MatrixDxD
impl<T: Clone + std::fmt::Debug, const COLUMNS: usize> AddRows<MatrixDxD<T>>
    for MatrixDxS<T, COLUMNS>
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn add_rows(mut self, rows: MatrixDxD<T>) -> Self::Output {
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(COLUMNS, rows.data[0].len(), "Non-matching columns");

        self.data.reserve(rows.data.len());
        for row in rows.data.into_iter() {
            self.data.push(row.try_into().unwrap());
        }
        self
    }
}
// MatrixSxD
// --------------------------------------------------
// MatrixSxS
impl<T: Clone + Default + Copy, const ROWS: usize, const COLUMNS: usize, const R: usize>
    AddRows<MatrixSxS<T, R, COLUMNS>> for MatrixSxD<T, ROWS>
where
    [(); ROWS + R]:,
{
    type Output = MatrixSxS<T, { ROWS + R }, COLUMNS>;
    fn add_rows(self, rows: MatrixSxS<T, R, COLUMNS>) -> Self::Output {
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(self.data[0].len(), COLUMNS, "Non-matching columns");
        let mut data = [[Default::default(); COLUMNS]; { ROWS + R }];

        // Copies old data
        for i in 0..ROWS {
            data[i].clone_from_slice(&self.data[i]);
        }
        // Copies new data
        for i in 0..R {
            data[i + ROWS] = rows.data[i];
        }
        Self::Output { data }
    }
}
// MatrixDxS
impl<T: Clone + std::fmt::Debug, const ROWS: usize, const COLUMNS: usize>
    AddRows<MatrixDxS<T, COLUMNS>> for MatrixSxD<T, ROWS>
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn add_rows(self, mut rows: MatrixDxS<T, COLUMNS>) -> Self::Output {
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(self.data[0].len(), COLUMNS, "Non-matching columns");

        rows.data.reserve(ROWS);
        for i in 0..ROWS {
            rows.data
                .insert(0, self.data[i].clone().try_into().unwrap());
        }
        rows
    }
}
// MatrixSxD
impl<T: Clone + Default + std::fmt::Debug, const ROWS: usize, const R: usize>
    AddRows<MatrixSxD<T, R>> for MatrixSxD<T, ROWS>
where
    [(); ROWS + R]:,
{
    type Output = MatrixSxD<T, { ROWS + R }>;
    fn add_rows(self, rows: MatrixSxD<T, R>) -> Self::Output {
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(
            self.data[0].len(),
            rows.data[0].len(),
            "Non-matching columns"
        );
        let mut data: [Vec<T>; ROWS + R] = (0..ROWS + R)
            .map(|_| vec![Default::default(); self.data[0].len()])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        // Copies old data
        for i in 0..ROWS {
            data[i].clone_from_slice(&self.data[i]);
        }
        for i in 0..R {
            data[i + ROWS].clone_from_slice(&rows.data[i]);
        }
        Self::Output { data }
    }
}
// MatrixDxD
impl<T: Clone + std::fmt::Debug, const ROWS: usize> AddRows<MatrixDxD<T>> for MatrixSxD<T, ROWS> {
    type Output = MatrixDxD<T>;
    fn add_rows(self, mut rows: MatrixDxD<T>) -> Self::Output {
        assert_eq!(ROWS, rows.data.len(), "Non-matching rows");
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(
            self.data[0].len(),
            rows.data[0].len(),
            "Non-matching columns"
        );

        rows.data.reserve(ROWS);
        for i in 0..ROWS {
            rows.data.insert(0, self.data[i].clone());
        }
        rows
    }
}
// MatrixDxD
// --------------------------------------------------
// MatrixSxS
impl<T: Clone + std::fmt::Debug, const ROWS: usize, const COLUMNS: usize>
    AddRows<MatrixSxS<T, ROWS, COLUMNS>> for MatrixDxD<T>
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn add_rows(self, rows: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(self.data[0].len(), COLUMNS, "Non-matching columns");

        let data = self
            .data
            .into_iter()
            .map(|v| v.try_into().unwrap())
            .chain(rows.data.iter().cloned())
            .collect::<Vec<_>>();

        Self::Output { data }
    }
}
// MatrixDxS
impl<T: Clone + std::fmt::Debug, const COLUMNS: usize> AddRows<MatrixDxS<T, COLUMNS>>
    for MatrixDxD<T>
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn add_rows(self, mut rows: MatrixDxS<T, COLUMNS>) -> Self::Output {
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(self.data[0].len(), COLUMNS, "Non-matching columns");

        rows.data.reserve(self.data.len());
        for old_row in self.data.into_iter() {
            rows.data.insert(0, old_row.clone().try_into().unwrap());
        }
        rows
    }
}
// MatrixSxD
impl<T: Clone + std::fmt::Debug, const ROWS: usize> AddRows<MatrixSxD<T, ROWS>> for MatrixDxD<T> {
    type Output = MatrixDxD<T>;
    fn add_rows(mut self, rows: MatrixSxD<T, ROWS>) -> Self::Output {
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(
            self.data[0].len(),
            rows.data[0].len(),
            "Non-matching columns"
        );

        self.data.reserve(ROWS);
        for i in 0..ROWS {
            self.data.push(rows.data[i].clone());
        }
        self
    }
}
// MatrixDxD
impl<T: Clone + std::fmt::Debug> AddRows<MatrixDxD<T>> for MatrixDxD<T> {
    type Output = MatrixDxD<T>;
    fn add_rows(mut self, rows: MatrixDxD<T>) -> Self::Output {
        assert_eq!(self.data.len(), rows.data.len(), "Non-matching rows");
        // We guarantee `other.data[0].len() == other.data[i].len()`.
        assert_eq!(
            self.data[0].len(),
            rows.data[0].len(),
            "Non-matching columns"
        );

        self.data.extend(rows.data.into_iter());
        self
    }
}
