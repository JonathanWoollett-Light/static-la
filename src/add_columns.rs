use crate::*;

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
    [(); ROWS * COLUMNS]:,
    [(); ROWS * { COLUMNS + C }]:,
{
    type Output = MatrixSxS<T, ROWS, { COLUMNS + C }>;
    fn add_columns(self, columns: MatrixDxS<T, C>) -> Self::Output {
        assert_eq!(ROWS, columns.rows, "Non-matching rows");
        let mut data = [Default::default(); ROWS * { COLUMNS + C }];

        for (new_column, (old_column, add_column)) in
            data.array_chunks_mut::<{ COLUMNS + C }>().zip(
                self.data
                    .array_chunks::<COLUMNS>()
                    .zip(columns.data.array_chunks::<C>()),
            )
        {
            // Copies old data
            new_column[..COLUMNS].clone_from_slice(old_column);
            // Copies new data
            new_column[COLUMNS..].clone_from_slice(add_column);
        }
        Self::Output { data }
    }
}
// MatrixSxD
impl<T: Clone + Default + Copy, const ROWS: usize, const COLUMNS: usize>
    AddColumns<MatrixSxD<T, ROWS>> for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxD<T, ROWS>;
    fn add_columns(self, columns: MatrixSxD<T, ROWS>) -> Self::Output {
        let data = self
            .data
            .array_chunks::<COLUMNS>()
            .zip(columns.data.chunks_exact(columns.columns))
            .map(|(a, b)| [a, b].concat())
            .flatten()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            columns: COLUMNS + columns.columns,
        }
    }
}
// MatrixSxS
impl<T: Clone + Default + Copy, const ROWS: usize, const COLUMNS: usize, const C: usize>
    AddColumns<MatrixSxS<T, ROWS, C>> for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
    [(); ROWS * C]:,
    [(); ROWS * { COLUMNS + C }]:,
{
    type Output = MatrixSxS<T, ROWS, { COLUMNS + C }>;
    fn add_columns(self, columns: MatrixSxS<T, ROWS, C>) -> Self::Output {
        let mut data = [Default::default(); ROWS * { COLUMNS + C }];

        for (new_column, (old_column, add_column)) in
            data.array_chunks_mut::<{ COLUMNS + C }>().zip(
                self.data
                    .array_chunks::<COLUMNS>()
                    .zip(columns.data.array_chunks::<C>()),
            )
        {
            // Copies old data
            new_column[..COLUMNS].clone_from_slice(old_column);
            // Copies new data
            new_column[COLUMNS..].clone_from_slice(add_column);
        }
        Self::Output { data }
    }
}
// MatrixDxD
impl<T: Clone + Default + std::fmt::Debug, const ROWS: usize, const COLUMNS: usize>
    AddColumns<MatrixDxD<T>> for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxD<T, ROWS>;
    fn add_columns(self, columns: MatrixDxD<T>) -> Self::Output {
        let data = self
            .data
            .array_chunks::<COLUMNS>()
            .zip(columns.data.chunks_exact(columns.columns))
            .map(|(a, b)| [a, b].concat())
            .flatten()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            columns: COLUMNS + columns.columns,
        }
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
        let data = self
            .data
            .chunks_exact(self.columns)
            .zip(columns.data.chunks_exact(columns.columns))
            .map(|(a, b)| [a, b].concat())
            .flatten()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            columns: self.columns + columns.columns,
        }
    }
}
// MatrixDxS
impl<T: Clone + Default + Copy + std::fmt::Debug, const ROWS: usize, const COLUMNS: usize>
    AddColumns<MatrixDxS<T, COLUMNS>> for MatrixSxD<T, ROWS>
{
    type Output = MatrixSxD<T, ROWS>;
    fn add_columns(self, columns: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(ROWS, columns.rows, "Non-matching rows");
        let data = self
            .data
            .chunks_exact(self.columns)
            .zip(columns.data.array_chunks::<COLUMNS>())
            .map(|(a, b)| [a, b].concat())
            .flatten()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            columns: self.columns + COLUMNS,
        }
    }
}
// MatrixSxS
impl<T: Clone + Default + Copy + std::fmt::Debug, const ROWS: usize, const COLUMNS: usize>
    AddColumns<MatrixSxS<T, ROWS, COLUMNS>> for MatrixSxD<T, ROWS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxD<T, ROWS>;
    fn add_columns(self, columns: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        let data = self
            .data
            .chunks_exact(self.columns)
            .zip(columns.data.array_chunks::<COLUMNS>())
            .map(|(a, b)| [a, b].concat())
            .flatten()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            columns: self.columns + COLUMNS,
        }
    }
}
// MatrixDxD
impl<T: Clone + Default + std::fmt::Debug, const ROWS: usize> AddColumns<MatrixDxD<T>>
    for MatrixSxD<T, ROWS>
{
    type Output = MatrixSxD<T, ROWS>;
    fn add_columns(self, columns: MatrixDxD<T>) -> Self::Output {
        assert_eq!(ROWS, columns.rows, "Non-matching rows");
        let data = self
            .data
            .chunks_exact(self.columns)
            .zip(columns.data.chunks_exact(columns.columns))
            .map(|(a, b)| [a, b].concat())
            .flatten()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            columns: self.columns + columns.columns,
        }
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
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        let data = self
            .data
            .array_chunks::<COLUMNS>()
            .zip(columns.data.chunks_exact(columns.columns))
            .map(|(a, b)| [a, b].concat())
            .flatten()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            columns: COLUMNS + columns.columns,
        }
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
        assert_eq!(self.rows, columns.rows, "Non-matching rows");
        let data = self
            .data
            .array_chunks::<COLUMNS>()
            .zip(columns.data.array_chunks::<C>())
            .map(|(a, b)| [a.as_slice(), b.as_slice()].concat())
            .flatten()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            rows: self.rows,
        }
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
    [(); ROWS * { COLUMNS + C }]:,
    [(); ROWS * C]:,
{
    type Output = MatrixSxS<T, ROWS, { COLUMNS + C }>;
    fn add_columns(self, columns: MatrixSxS<T, ROWS, C>) -> Self::Output {
        assert_eq!(self.rows, ROWS, "Non-matching rows");
        let mut data = [Default::default(); ROWS * { COLUMNS + C }];

        for (new_column, (old_column, add_column)) in
            data.array_chunks_mut::<{ COLUMNS + C }>().zip(
                self.data
                    .array_chunks::<COLUMNS>()
                    .zip(columns.data.array_chunks::<C>()),
            )
        {
            // Copies old data
            new_column[..COLUMNS].clone_from_slice(old_column);
            // Copies new data
            new_column[COLUMNS..].clone_from_slice(add_column);
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
        assert_eq!(self.rows, columns.rows, "Non-matching rows");
        let data = self
            .data
            .array_chunks::<COLUMNS>()
            .zip(columns.data.chunks_exact(columns.columns))
            .map(|(a, b)| [a, b].concat())
            .flatten()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            rows: self.rows,
            columns: COLUMNS + columns.columns,
        }
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
        assert_eq!(self.rows, ROWS, "Non-matching rows");

        let data = self
            .data
            .chunks_exact(self.columns)
            .zip(columns.data.chunks_exact(columns.columns))
            .map(|(a, b)| [a, b].concat())
            .flatten()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            columns: self.columns + columns.columns,
        }
    }
}
// MatrixDxS
impl<T: Clone + Default + Copy + std::fmt::Debug, const COLUMNS: usize>
    AddColumns<MatrixDxS<T, COLUMNS>> for MatrixDxD<T>
{
    type Output = MatrixDxD<T>;
    fn add_columns(self, columns: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows, columns.rows, "Non-matching rows");
        let data = self
            .data
            .chunks_exact(self.columns)
            .zip(columns.data.array_chunks::<COLUMNS>())
            .map(|(a, b)| [a, b].concat())
            .flatten()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            rows: self.rows,
            columns: self.columns + COLUMNS,
        }
    }
}
// MatrixSxS
impl<T: Clone + Default + Copy + std::fmt::Debug, const ROWS: usize, const COLUMNS: usize>
    AddColumns<MatrixSxS<T, ROWS, COLUMNS>> for MatrixDxD<T>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixSxD<T, ROWS>;
    fn add_columns(self, columns: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.rows, ROWS, "Non-matching rows");

        let data = self
            .data
            .chunks_exact(self.columns)
            .zip(columns.data.array_chunks::<COLUMNS>())
            .map(|(a, b)| [a, b].concat())
            .flatten()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            columns: self.columns + COLUMNS,
        }
    }
}
// MatrixDxD
impl<T: Clone + Default + std::fmt::Debug> AddColumns<MatrixDxD<T>> for MatrixDxD<T> {
    type Output = MatrixDxD<T>;
    fn add_columns(self, columns: MatrixDxD<T>) -> Self::Output {
        assert_eq!(self.rows, columns.rows, "Non-matching rows");
        let data = self
            .data
            .chunks_exact(self.columns)
            .zip(columns.data.chunks_exact(columns.columns))
            .map(|(a, b)| [a, b].concat())
            .flatten()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            rows: self.rows,
            columns: self.columns + columns.columns,
        }
    }
}
