use crate::*;
use std::convert::TryFrom;

impl<T: std::iter::Sum<T> + Clone> MatrixDxD<T> {
    /// Gets sum of all elements.
    pub fn sum(&self) -> T {
        self.data.iter().cloned().sum()
    }
    /// Gets the sum of each row.
    pub fn row_sum(&self) -> ColumnVectorD<T> {
        ColumnVectorD::from(
            self.data
                .chunks_exact(self.columns)
                .map(|r| [r.iter().cloned().sum()])
                .collect::<Vec<_>>(),
        )
    }
    /// Gets the sum of each column.
    pub fn column_sum(&self) -> RowVectorD<T> {
        RowVectorD::try_from([(0..self.columns)
            .map(|i| {
                self.data
                    .iter()
                    .skip(i)
                    .step_by(self.columns)
                    .cloned()
                    .sum()
            })
            .collect::<Vec<_>>()])
        .unwrap()
    }
}
impl<T: std::iter::Sum<T> + Copy + Default + std::fmt::Debug, const COLUMNS: usize>
    MatrixDxS<T, COLUMNS>
where
    [(); 1 * COLUMNS]:,
{
    /// Gets sum of all elements.
    pub fn sum(&self) -> T {
        self.data.iter().cloned().sum()
    }
    /// Gets the sum of each row.
    pub fn row_sum(&self) -> ColumnVectorD<T> {
        ColumnVectorD::from(
            self.data
                .chunks_exact(COLUMNS)
                .map(|r| [r.iter().cloned().sum()])
                .collect::<Vec<_>>(),
        )
    }
    /// Gets the sum of each column.
    pub fn column_sum(&self) -> RowVectorS<T, COLUMNS> {
        // Will dropping the `Default` and `Copy` traits to use `try_into` affect performance?
        let mut sums: [T; COLUMNS] = [Default::default(); COLUMNS];
        for i in 0..COLUMNS {
            sums[i] = self.data.iter().skip(i).step_by(COLUMNS).cloned().sum();
        }
        RowVectorS::from([sums])
    }
}
impl<T: std::iter::Sum<T> + Copy + Default + std::fmt::Debug, const ROWS: usize> MatrixSxD<T, ROWS>
where
    [(); ROWS * 1]:,
{
    /// Gets sum of all elements.
    pub fn sum(&self) -> T {
        self.data.iter().cloned().sum()
    }
    /// Gets the sum of each row.
    pub fn row_sum(&self) -> ColumnVectorS<T, ROWS> {
        let mut sums: [[T; 1]; ROWS] = [[Default::default(); 1]; ROWS];
        for (i, row) in (0..ROWS).zip(self.data.chunks_exact(self.columns)) {
            sums[i][0] = row.iter().cloned().sum();
        }
        ColumnVectorS::from(sums)
    }
    /// Gets the sum of each column.
    pub fn column_sum(&self) -> RowVectorD<T> {
        RowVectorD::try_from([(0..self.columns)
            .map(|i| {
                self.data
                    .iter()
                    .skip(i)
                    .step_by(self.columns)
                    .cloned()
                    .sum()
            })
            .collect::<Vec<_>>()])
        .unwrap()
    }
}
impl<
        T: std::iter::Sum<T> + Copy + Default + std::fmt::Debug,
        const ROWS: usize,
        const COLUMNS: usize,
    > MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
    [(); ROWS * 1]:,
    [(); 1 * COLUMNS]:,
{
    /// Gets sum of all elements.
    pub fn sum(&self) -> T {
        self.data.iter().cloned().sum()
    }
    /// Gets the sum of each row.
    pub fn row_sum(&self) -> ColumnVectorS<T, ROWS> {
        let mut sums: [[T; 1]; ROWS] = [[Default::default(); 1]; ROWS];
        for (i, row) in (0..ROWS).zip(self.data.chunks_exact(COLUMNS)) {
            sums[i][0] = row.iter().cloned().sum();
        }
        ColumnVectorS::from(sums)
    }
    /// Gets the sum of each column.
    pub fn column_sum(&self) -> RowVectorS<T, COLUMNS> {
        let mut sums: [T; COLUMNS] = [Default::default(); COLUMNS];
        for i in 0..COLUMNS {
            sums[i] = self.data.iter().skip(i).step_by(COLUMNS).cloned().sum();
        }
        RowVectorS::from([sums])
    }
}
