use crate::*;
use std::convert::TryInto;

/// A trait allowing the combination of matrices along their column axis.
pub trait AddColumns<T> {
    type Output;
    /// This is somewhat inefficient, it will often not be as efficient as [`std::vec::Vec::push`].
    /// ```text
    /// ┌───────┐             ┌─────┐  ┌───────────┐
    /// │ 1 3 5 │             │ 7 9 │  │ 1 3 5 7 9 │
    /// │ 2 4 6 │.add_columns(│ 8 1 │)=│ 2 4 6 8 1 │
    /// └───────┘             └─────┘  └───────────┘
    /// ```
    fn add_columns(self, columns: T) -> Self::Output;
}

// MatrixSxS
// --------------------------------------------------
// MatrixDxS
impl<T: Clone + Default + Copy, const ROWS: usize, const COLUMNS: usize, const C: usize>
    AddColumns<MatrixDxS<T, C>> for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); COLUMNS + C]:,
{
    type Output = MatrixSxS<T, ROWS, { COLUMNS + C }>;
    fn add_columns(self, columns: MatrixDxS<T, C>) -> Self::Output {
        assert_eq!(ROWS, columns.data.len(), "Non-matching rows");
        let mut data = [[Default::default(); { COLUMNS + C }]; ROWS];

        for i in 0..ROWS {
            // TODOl: Use assignment here.
            // Copies old data
            data[i][..COLUMNS].clone_from_slice(&self.data[i]);
            // Copies new data
            data[i][COLUMNS..].clone_from_slice(&columns.data[i]);
        }
        Self::Output { data }
    }
}
// MatrixSxD
impl<T: Clone + Default + Copy, const ROWS: usize, const COLUMNS: usize>
    AddColumns<MatrixSxD<T, ROWS>> for MatrixSxS<T, ROWS, COLUMNS>
{
    type Output = MatrixSxD<T, ROWS>;
    fn add_columns(self, mut columns: MatrixSxD<T, ROWS>) -> Self::Output {
        for i in 0..ROWS {
            // Copies old data
            let mut a = Vec::from(self.data[i].as_slice());
            // Copies new data
            a.append(&mut columns.data[i]);
            columns.data[i] = a;
        }
        columns
    }
}
// MatrixSxS
impl<T: Clone + Default + Copy, const ROWS: usize, const COLUMNS: usize, const C: usize>
    AddColumns<MatrixSxS<T, ROWS, C>> for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); COLUMNS + C]:,
{
    type Output = MatrixSxS<T, ROWS, { COLUMNS + C }>;
    fn add_columns(self, columns: MatrixSxS<T, ROWS, C>) -> Self::Output {
        let mut data = [[Default::default(); { COLUMNS + C }]; ROWS];
        for i in 0..ROWS {
            // Copies old data
            data[i][..COLUMNS].clone_from_slice(&self.data[i]);
            // Copies new data
            data[i][COLUMNS..].clone_from_slice(&columns.data[i]);
        }
        Self::Output { data }
    }
}
// MatrixDxD
impl<T: Clone + Default + std::fmt::Debug, const ROWS: usize, const COLUMNS: usize>
    AddColumns<MatrixDxD<T>> for MatrixSxS<T, ROWS, COLUMNS>
{
    type Output = MatrixSxD<T, ROWS>;
    fn add_columns(self, columns: MatrixDxD<T>) -> Self::Output {
        assert_eq!(ROWS, columns.data.len(), "Non-matching rows");
        let mut data: [Vec<T>; ROWS] = (0..ROWS)
            .map(|_| vec![Default::default(); COLUMNS + columns.data[0].len()])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        for i in 0..ROWS {
            // Copies old data
            data[i][..COLUMNS].clone_from_slice(&self.data[i]);
            // Copies new data
            data[i][COLUMNS..].clone_from_slice(&columns.data[i]);
        }
        Self::Output { data }
    }
}
// MatrixSxD
// --------------------------------------------------
// MatrixSxD
impl<T: Clone + Default + Copy + std::fmt::Debug, const ROWS: usize> AddColumns<MatrixSxD<T, ROWS>>
    for MatrixSxD<T, ROWS>
{
    type Output = MatrixSxD<T, ROWS>;
    fn add_columns(self, columns: MatrixSxD<T, ROWS>) -> Self::Output {
        let mut data: [Vec<T>; ROWS] = (0..ROWS)
            .map(|_| vec![Default::default(); self.data[0].len() + columns.data[0].len()])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        for i in 0..ROWS {
            // Copies old data
            data[i][..self.data[0].len()].clone_from_slice(&self.data[i]);
            // Copies new data
            data[i][self.data[0].len()..].clone_from_slice(&columns.data[i]);
        }
        Self::Output { data }
    }
}
// MatrixDxS
impl<T: Clone + Default + Copy + std::fmt::Debug, const ROWS: usize, const COLUMNS: usize>
    AddColumns<MatrixDxS<T, COLUMNS>> for MatrixSxD<T, ROWS>
{
    type Output = MatrixSxD<T, ROWS>;
    fn add_columns(self, columns: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(ROWS, columns.data.len(), "Non-matching rows");
        let mut data: [Vec<T>; ROWS] = (0..ROWS)
            .map(|_| vec![Default::default(); self.data[0].len() + columns.data[0].len()])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        for i in 0..ROWS {
            // Copies old data
            data[i][..self.data[0].len()].clone_from_slice(&self.data[i]);
            // Copies new data
            data[i][self.data[0].len()..].clone_from_slice(&columns.data[i]);
        }
        Self::Output { data }
    }
}
// MatrixSxS
impl<T: Clone + Default + Copy + std::fmt::Debug, const ROWS: usize, const COLUMNS: usize>
    AddColumns<MatrixSxS<T, ROWS, COLUMNS>> for MatrixSxD<T, ROWS>
{
    type Output = MatrixSxD<T, ROWS>;
    fn add_columns(self, columns: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        let mut data: [Vec<T>; ROWS] = (0..ROWS)
            .map(|_| vec![Default::default(); self.data[0].len() + COLUMNS])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        for i in 0..ROWS {
            // Copies old data
            data[i][..self.data[0].len()].clone_from_slice(&self.data[i]);
            // Copies new data
            data[i][self.data[0].len()..].clone_from_slice(&columns.data[i]);
        }
        Self::Output { data }
    }
}
// MatrixDxD
impl<T: Clone + Default + std::fmt::Debug, const ROWS: usize> AddColumns<MatrixDxD<T>>
    for MatrixSxD<T, ROWS>
{
    type Output = MatrixSxD<T, ROWS>;
    fn add_columns(self, columns: MatrixDxD<T>) -> Self::Output {
        assert_eq!(ROWS, columns.data.len(), "Non-matching rows");
        let mut data: [Vec<T>; ROWS] = (0..ROWS)
            .map(|_| vec![Default::default(); self.data[0].len() + columns.data[0].len()])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        for i in 0..ROWS {
            // Copies old data
            data[i][..self.data[0].len()].clone_from_slice(&self.data[i]);
            // Copies new data
            data[i][self.data[0].len()..].clone_from_slice(&columns.data[i]);
        }
        Self::Output { data }
    }
}
// MatrixDxS
// --------------------------------------------------
// MatrixSxD
impl<T: Clone + Default + Copy + std::fmt::Debug, const ROWS: usize, const COLUMNS: usize>
    AddColumns<MatrixSxD<T, ROWS>> for MatrixDxS<T, COLUMNS>
{
    type Output = MatrixSxD<T, ROWS>;
    fn add_columns(self, columns: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(self.data.len(), ROWS, "Non-matching rows");
        let mut data: [Vec<T>; ROWS] = (0..ROWS)
            .map(|_| vec![Default::default(); COLUMNS + columns.data[0].len()])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        for i in 0..ROWS {
            // Copies old data
            data[i][..COLUMNS].clone_from_slice(&self.data[i]);
            // Copies new data
            data[i][COLUMNS..].clone_from_slice(&columns.data[i]);
        }
        Self::Output { data }
    }
}
// MatrixDxS
impl<T: Clone + Default + Copy + std::fmt::Debug, const COLUMNS: usize, const C: usize>
    AddColumns<MatrixDxS<T, C>> for MatrixDxS<T, COLUMNS>
where
    [(); COLUMNS + C]:,
{
    type Output = MatrixDxS<T, { COLUMNS + C }>;
    fn add_columns(self, columns: MatrixDxS<T, C>) -> Self::Output {
        assert_eq!(self.data.len(), columns.data.len(), "Non-matching rows");
        let mut data = vec![[Default::default(); COLUMNS + C]; self.data.len()];

        for i in 0..self.data.len() {
            // Copies old data
            data[i][..COLUMNS].clone_from_slice(&self.data[i]);
            // Copies new data
            data[i][COLUMNS..].clone_from_slice(&columns.data[i]);
        }
        Self::Output { data }
    }
}
// MatrixSxS
impl<
        T: Clone + Default + Copy + std::fmt::Debug,
        const ROWS: usize,
        const COLUMNS: usize,
        const C: usize,
    > AddColumns<MatrixSxS<T, ROWS, C>> for MatrixDxS<T, COLUMNS>
where
    [(); COLUMNS + C]:,
{
    type Output = MatrixSxS<T, ROWS, { COLUMNS + C }>;
    fn add_columns(self, columns: MatrixSxS<T, ROWS, C>) -> Self::Output {
        assert_eq!(self.data.len(), ROWS, "Non-matching rows");
        let mut data = [[Default::default(); COLUMNS + C]; ROWS];

        for i in 0..ROWS {
            // Copies old data
            data[i][..COLUMNS].clone_from_slice(&self.data[i]);
            // Copies new data
            data[i][COLUMNS..].clone_from_slice(&columns.data[i]);
        }
        Self::Output { data }
    }
}
// MatrixDxD
impl<T: Clone + Default + std::fmt::Debug, const COLUMNS: usize> AddColumns<MatrixDxD<T>>
    for MatrixDxS<T, COLUMNS>
{
    type Output = MatrixDxD<T>;
    fn add_columns(self, columns: MatrixDxD<T>) -> Self::Output {
        assert_eq!(self.data.len(), columns.data.len(), "Non-matching rows");
        let mut data =
            vec![vec![Default::default(); COLUMNS + columns.data[0].len()]; self.data.len()];

        for i in 0..self.data.len() {
            // Copies old data
            data[i][..COLUMNS].clone_from_slice(&self.data[i]);
            // Copies new data
            data[i][COLUMNS..].clone_from_slice(&columns.data[i]);
        }
        Self::Output { data }
    }
}
// MatrixDxD
// --------------------------------------------------
// MatrixSxD
impl<T: Clone + Default + Copy + std::fmt::Debug, const ROWS: usize> AddColumns<MatrixSxD<T, ROWS>>
    for MatrixDxD<T>
{
    type Output = MatrixSxD<T, ROWS>;
    fn add_columns(self, columns: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(self.data.len(), ROWS, "Non-matching rows");

        let ncols = self.data[0].len() + columns.data[0].len();
        let mut data: [Vec<T>; ROWS] = (0..ROWS)
            .map(|_| vec![Default::default(); ncols])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let cols = self.data[0].len();
        for i in 0..ROWS {
            // Copies old data
            data[i][..cols].clone_from_slice(&self.data[i]);
            // Copies new data
            data[i][cols..].clone_from_slice(&columns.data[i]);
        }
        Self::Output { data }
    }
}
// MatrixDxS
impl<T: Clone + Default + Copy + std::fmt::Debug, const COLUMNS: usize>
    AddColumns<MatrixDxS<T, COLUMNS>> for MatrixDxD<T>
{
    type Output = MatrixDxD<T>;
    fn add_columns(self, columns: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(self.data.len(), columns.data.len(), "Non-matching rows");

        let ncols = self.data.len() + COLUMNS;
        let mut data = vec![vec![Default::default(); ncols]; self.data.len()];

        let cols = self.data.len();
        for i in 0..self.data.len() {
            // Copies old data
            data[i][..cols].clone_from_slice(&self.data[i]);
            // Copies new data
            data[i][cols..].clone_from_slice(&columns.data[i]);
        }
        Self::Output { data }
    }
}
// MatrixSxS
impl<T: Clone + Default + Copy + std::fmt::Debug, const ROWS: usize, const COLUMNS: usize>
    AddColumns<MatrixSxS<T, ROWS, COLUMNS>> for MatrixDxD<T>
{
    type Output = MatrixSxD<T, ROWS>;
    fn add_columns(self, columns: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.data.len(), ROWS, "Non-matching rows");

        let ncols = self.data[0].len() + columns.data[0].len();
        let mut data: [Vec<T>; ROWS] = (0..ROWS)
            .map(|_| vec![Default::default(); ncols])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let cols = self.data[0].len();
        for i in 0..ROWS {
            // Copies old data
            data[i][..cols].clone_from_slice(&self.data[i]);
            // Copies new data
            data[i][cols..].clone_from_slice(&columns.data[i]);
        }
        Self::Output { data }
    }
}
// MatrixDxD
impl<T: Clone + Default + std::fmt::Debug> AddColumns<MatrixDxD<T>> for MatrixDxD<T> {
    type Output = MatrixDxD<T>;
    fn add_columns(self, columns: MatrixDxD<T>) -> Self::Output {
        assert_eq!(self.data.len(), columns.data.len(), "Non-matching rows");
        let ncols = self.data[0].len() + columns.data[0].len();
        let mut data = vec![vec![Default::default(); ncols]; self.data.len()];

        let cols = self.data[0].len();
        for i in 0..self.data.len() {
            // Copies old data
            data[i][..cols].clone_from_slice(&self.data[i]);
            // Copies new data
            data[i][cols..].clone_from_slice(&columns.data[i]);
        }
        Self::Output { data }
    }
}
