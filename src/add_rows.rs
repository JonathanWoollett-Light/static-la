use crate::*;

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
    [(); ROWS * COLUMNS]:,
    [(); R * COLUMNS]:,
    [(); { ROWS + R } * COLUMNS]:,
{
    type Output = MatrixSxS<T, { ROWS + R }, COLUMNS>;
    fn add_rows(self, rows: MatrixSxS<T, R, COLUMNS>) -> Self::Output {
        let mut data = [Default::default(); { ROWS + R } * COLUMNS];
        // Copies old data then chains into copying new data
        for (to, from) in data.array_chunks_mut::<COLUMNS>().zip(
            self.data
                .array_chunks::<COLUMNS>()
                .chain(rows.data.array_chunks::<COLUMNS>()),
        ) {
            to.clone_from_slice(from);
        }
        Self::Output { data }
    }
}
// MatrixDxS
impl<T: Clone, const ROWS: usize, const COLUMNS: usize> AddRows<MatrixDxS<T, COLUMNS>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn add_rows(self, rows: MatrixDxS<T, COLUMNS>) -> Self::Output {
        let data = self
            .data
            .array_chunks::<COLUMNS>()
            .chain(rows.data.array_chunks::<COLUMNS>())
            .cloned()
            .flatten()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            rows: ROWS + rows.rows,
        }
    }
}
// MatrixSxD
impl<T: Clone + Default + Copy, const ROWS: usize, const COLUMNS: usize, const R: usize>
    AddRows<MatrixSxD<T, R>> for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
    [(); { ROWS + R } * COLUMNS]:,
{
    type Output = MatrixSxS<T, { ROWS + R }, COLUMNS>;
    fn add_rows(self, rows: MatrixSxD<T, R>) -> Self::Output {
        assert_eq!(COLUMNS, rows.columns, "Non-matching columns");

        let mut data = [Default::default(); { ROWS + R } * COLUMNS];
        // Copies old data then chains into copying new data
        for (to, from) in data.array_chunks_mut::<COLUMNS>().zip(
            self.data
                .array_chunks::<COLUMNS>()
                .chain(rows.data.array_chunks::<COLUMNS>()),
        ) {
            to.clone_from_slice(from);
        }
        Self::Output { data }
    }
}
// MatrixDxD
impl<T: Clone + std::fmt::Debug, const ROWS: usize, const COLUMNS: usize> AddRows<MatrixDxD<T>>
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn add_rows(self, rows: MatrixDxD<T>) -> Self::Output {
        assert_eq!(COLUMNS, rows.columns, "Non-matching columns");

        let data = self
            .data
            .array_chunks::<COLUMNS>()
            .chain(rows.data.array_chunks::<COLUMNS>())
            .cloned()
            .flatten()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            rows: ROWS + rows.rows,
        }
    }
}
// MatrixDxS
// --------------------------------------------------
// MatrixSxS
impl<T: Clone + Default + Copy, const ROWS: usize, const COLUMNS: usize>
    AddRows<MatrixSxS<T, ROWS, COLUMNS>> for MatrixDxS<T, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn add_rows(self, rows: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        let data = self
            .data
            .array_chunks::<COLUMNS>()
            .chain(rows.data.array_chunks::<COLUMNS>())
            .cloned()
            .flatten()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            rows: self.rows + ROWS,
        }
    }
}
// MatrixDxS
impl<T: Clone, const COLUMNS: usize> AddRows<MatrixDxS<T, COLUMNS>> for MatrixDxS<T, COLUMNS> {
    type Output = MatrixDxS<T, COLUMNS>;
    fn add_rows(self, rows: MatrixDxS<T, COLUMNS>) -> Self::Output {
        let data = self
            .data
            .array_chunks::<COLUMNS>()
            .chain(rows.data.array_chunks::<COLUMNS>())
            .cloned()
            .flatten()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            rows: self.rows + rows.rows,
        }
    }
}
// MatrixSxD
impl<T: Clone + std::fmt::Debug, const ROWS: usize, const COLUMNS: usize>
    AddRows<MatrixSxD<T, ROWS>> for MatrixDxS<T, COLUMNS>
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn add_rows(self, rows: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(COLUMNS, rows.columns, "Non-matching columns");

        let data = self
            .data
            .array_chunks::<COLUMNS>()
            .chain(rows.data.array_chunks::<COLUMNS>())
            .cloned()
            .flatten()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            rows: self.rows + ROWS,
        }
    }
}
// MatrixDxD
impl<T: Clone + std::fmt::Debug, const COLUMNS: usize> AddRows<MatrixDxD<T>>
    for MatrixDxS<T, COLUMNS>
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn add_rows(self, rows: MatrixDxD<T>) -> Self::Output {
        assert_eq!(COLUMNS, rows.columns, "Non-matching columns");

        let data = self
            .data
            .array_chunks::<COLUMNS>()
            .chain(rows.data.array_chunks::<COLUMNS>())
            .cloned()
            .flatten()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            rows: self.rows + rows.rows,
        }
    }
}
// MatrixSxD
// --------------------------------------------------
// MatrixSxS
impl<T: Clone + Default + Copy, const ROWS: usize, const COLUMNS: usize, const R: usize>
    AddRows<MatrixSxS<T, R, COLUMNS>> for MatrixSxD<T, ROWS>
where
    [(); R * COLUMNS]:,
    [(); { ROWS + R } * COLUMNS]:,
{
    type Output = MatrixSxS<T, { ROWS + R }, COLUMNS>;
    fn add_rows(self, rows: MatrixSxS<T, R, COLUMNS>) -> Self::Output {
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");

        let mut data = [Default::default(); { ROWS + R } * COLUMNS];
        // Copies old data then chains into copying new data
        for (to, from) in data.array_chunks_mut::<COLUMNS>().zip(
            self.data
                .array_chunks::<COLUMNS>()
                .chain(rows.data.array_chunks::<COLUMNS>()),
        ) {
            to.clone_from_slice(from);
        }
        Self::Output { data }
    }
}
// MatrixDxS
impl<T: Clone + std::fmt::Debug, const ROWS: usize, const COLUMNS: usize>
    AddRows<MatrixDxS<T, COLUMNS>> for MatrixSxD<T, ROWS>
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn add_rows(self, rows: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");

        let data = self
            .data
            .array_chunks::<COLUMNS>()
            .chain(rows.data.array_chunks::<COLUMNS>())
            .cloned()
            .flatten()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            rows: ROWS + rows.rows,
        }
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
        assert_eq!(self.columns, rows.columns, "Non-matching columns");
        let data = self
            .data
            .chunks_exact(self.columns)
            .chain(rows.data.chunks_exact(rows.columns))
            .flatten()
            .cloned()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            columns: self.columns,
        }
    }
}
// MatrixDxD
impl<T: Clone + std::fmt::Debug, const ROWS: usize> AddRows<MatrixDxD<T>> for MatrixSxD<T, ROWS> {
    type Output = MatrixDxD<T>;
    fn add_rows(self, rows: MatrixDxD<T>) -> Self::Output {
        assert_eq!(ROWS, rows.rows, "Non-matching rows");
        assert_eq!(self.columns, rows.columns, "Non-matching columns");

        let data = self
            .data
            .chunks_exact(self.columns)
            .chain(rows.data.chunks_exact(rows.columns))
            .flatten()
            .cloned()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            rows: ROWS + rows.rows,
            columns: self.columns,
        }
    }
}
// MatrixDxD
// --------------------------------------------------
// MatrixSxS
impl<T: Clone + std::fmt::Debug, const ROWS: usize, const COLUMNS: usize>
    AddRows<MatrixSxS<T, ROWS, COLUMNS>> for MatrixDxD<T>
where
    [(); ROWS * COLUMNS]:,
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn add_rows(self, rows: MatrixSxS<T, ROWS, COLUMNS>) -> Self::Output {
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");

        let data = self
            .data
            .array_chunks::<COLUMNS>()
            .chain(rows.data.array_chunks::<COLUMNS>())
            .cloned()
            .flatten()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            rows: self.rows + ROWS,
        }
    }
}
// MatrixDxS
impl<T: Clone + std::fmt::Debug, const COLUMNS: usize> AddRows<MatrixDxS<T, COLUMNS>>
    for MatrixDxD<T>
{
    type Output = MatrixDxS<T, COLUMNS>;
    fn add_rows(self, rows: MatrixDxS<T, COLUMNS>) -> Self::Output {
        assert_eq!(self.columns, COLUMNS, "Non-matching columns");

        let data = self
            .data
            .array_chunks::<COLUMNS>()
            .chain(rows.data.array_chunks::<COLUMNS>())
            .cloned()
            .flatten()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            rows: self.rows + rows.rows,
        }
    }
}
// MatrixSxD
impl<T: Clone + std::fmt::Debug, const ROWS: usize> AddRows<MatrixSxD<T, ROWS>> for MatrixDxD<T> {
    type Output = MatrixDxD<T>;
    fn add_rows(self, rows: MatrixSxD<T, ROWS>) -> Self::Output {
        assert_eq!(self.columns, ROWS, "Non-matching columns");

        let data = self
            .data
            .chunks_exact(self.columns)
            .chain(rows.data.chunks_exact(rows.columns))
            .flatten()
            .cloned()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            rows: self.rows + ROWS,
            columns: self.columns,
        }
    }
}
// MatrixDxD
impl<T: Clone + std::fmt::Debug> AddRows<MatrixDxD<T>> for MatrixDxD<T> {
    type Output = MatrixDxD<T>;
    fn add_rows(self, rows: MatrixDxD<T>) -> Self::Output {
        assert_eq!(self.rows, rows.rows, "Non-matching rows");
        assert_eq!(self.columns, rows.columns, "Non-matching columns");

        let data = self
            .data
            .chunks_exact(self.columns)
            .chain(rows.data.chunks_exact(rows.columns))
            .flatten()
            .cloned()
            .collect::<Vec<_>>();
        Self::Output {
            data,
            rows: self.rows + rows.rows,
            columns: self.columns,
        }
    }
}
