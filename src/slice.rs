use crate::*;
use std::{convert::TryInto, ops::Range};

/// Internal function used for const slice sizing.
pub const fn range_len(a: Range<usize>) -> usize {
    a.end - a.start
}
type Slice = (Range<usize>, Range<usize>);

/// A trait for dynamic slicing along rows and dynamic slicing along columns.
pub trait SliceDxD<T> {
    /// Given a range of rows and a range of columns returns a [`MatrixDxD`] holding
    ///  references to the value within the ranges.
    fn slice_dxd(&self, slice: Slice) -> MatrixDxD<&T>;
}
/// A trait for dynamic slicing along rows and static slicing along columns.
pub trait SliceDxS<T> {
    /// Given a range of rows and a constant range of columns returns a [`MatrixDxS`] holding
    ///  references to the value within the ranges.
    fn slice_dxs<const COLUMNS: Range<usize>>(
        &self,
        rows: Range<usize>,
    ) -> MatrixDxS<&T, { range_len(COLUMNS) }>;
}
/// A trait for static slicing along rows and dynamic slicing along columns.
pub trait SliceSxD<T> {
    /// Given a constant range of rows and a range of columns returns a [`MatrixSxD`] holding
    ///  references to the value within the ranges.
    fn slice_sxd<const ROWS: Range<usize>>(
        &self,
        columns: Range<usize>,
    ) -> MatrixSxD<&T, { range_len(ROWS) }>;
}
/// A trait for static slicing along rows and static slicing along columns.
pub trait SliceSxS<T> {
    /// Given a constant range of rows and a constant range of columns returns a [`MatrixSxS`]
    ///  holding references to the value within the ranges.
    fn slice_sxs<const ROWS: Range<usize>, const COLUMNS: Range<usize>>(
        &self,
    ) -> MatrixSxS<&T, { range_len(ROWS) }, { range_len(COLUMNS) }>
    where
        [(); range_len(ROWS) * range_len(COLUMNS)]:;
}

// MatrixDxD
// --------------------------------------------------
impl<T> SliceDxD<T> for MatrixDxD<T> {
    fn slice_dxd(&self, (rows, columns): Slice) -> MatrixDxD<&T> {
        assert!(rows.end < self.rows, "Row out of bounds");
        assert!(columns.end < self.columns, "Columns out of bounds");
        let data: Vec<Vec<&T>> = self
            .data
            .chunks_exact(self.columns)
            .map(|r| {
                r.iter()
                    .skip(columns.start)
                    .take(columns.end - columns.start)
                    .collect::<Vec<_>>()
            })
            .skip(rows.start)
            .take(rows.end - rows.start)
            .collect::<Vec<_>>();

        MatrixDxD::try_from(data).unwrap()
    }
}
impl<T: std::fmt::Debug> SliceSxD<T> for MatrixDxD<T> {
    fn slice_sxd<const ROWS: Range<usize>>(
        &self,
        columns: Range<usize>,
    ) -> MatrixSxD<&T, { range_len(ROWS) }>
    where
        [(); range_len(ROWS)]:,
    {
        assert!(ROWS.end < self.rows, "Row out of bounds");
        assert!(columns.end < self.columns, "Columns out of bounds");
        let data: [Vec<&T>; range_len(ROWS)] = self
            .data
            .chunks_exact(self.columns)
            .map(|r| {
                r.iter()
                    .skip(columns.start)
                    .take(columns.end - columns.start)
                    .collect::<Vec<_>>()
            })
            .skip(ROWS.start)
            .take(range_len(ROWS))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        MatrixSxD::try_from(data).unwrap()
    }
}
impl<T: std::fmt::Debug> SliceDxS<T> for MatrixDxD<T> {
    fn slice_dxs<const COLUMNS: Range<usize>>(
        &self,
        rows: Range<usize>,
    ) -> MatrixDxS<&T, { range_len(COLUMNS) }>
    where
        [(); range_len(COLUMNS)]:,
    {
        assert!(rows.end < self.rows, "Row out of bounds");
        assert!(COLUMNS.end < self.columns, "Columns out of bounds");
        let data: Vec<[&T; range_len(COLUMNS)]> = self
            .data
            .chunks_exact(self.columns)
            .map(|r| {
                r.iter()
                    .skip(COLUMNS.start)
                    .take(range_len(COLUMNS))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .skip(rows.start)
            .take(rows.end - rows.start)
            .collect::<Vec<_>>();

        MatrixDxS::from(data)
    }
}
impl<T: std::fmt::Debug> SliceSxS<T> for MatrixDxD<T> {
    fn slice_sxs<const ROWS: Range<usize>, const COLUMNS: Range<usize>>(
        &self,
    ) -> MatrixSxS<&T, { range_len(ROWS) }, { range_len(COLUMNS) }>
    where
        [(); { range_len(ROWS) } * { range_len(COLUMNS) }]:,
    {
        assert!(ROWS.end < self.rows, "Row out of bounds");
        assert!(COLUMNS.end < self.columns, "Columns out of bounds");
        let data: [[&T; range_len(COLUMNS)]; range_len(ROWS)] = self
            .data
            .chunks_exact(self.columns)
            .map(|r| {
                r.iter()
                    .skip(COLUMNS.start)
                    .take(range_len(COLUMNS))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .skip(ROWS.start)
            .take(range_len(ROWS))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        MatrixSxS::from(data)
    }
}
// MatrixDxS
// --------------------------------------------------
impl<T, const C: usize> SliceDxD<T> for MatrixDxS<T, C> {
    fn slice_dxd(&self, (rows, columns): Slice) -> MatrixDxD<&T> {
        assert!(rows.end < self.rows, "Row out of bounds");
        assert!(columns.end < C, "Columns out of bounds");
        let data: Vec<Vec<&T>> = self
            .data
            .chunks_exact(C)
            .map(|r| {
                r.iter()
                    .skip(columns.start)
                    .take(columns.end - columns.start)
                    .collect::<Vec<_>>()
            })
            .skip(rows.start)
            .take(rows.end - rows.start)
            .collect::<Vec<_>>();

        MatrixDxD::try_from(data).unwrap()
    }
}
impl<T: std::fmt::Debug, const C: usize> SliceSxD<T> for MatrixDxS<T, C> {
    fn slice_sxd<const ROWS: Range<usize>>(
        &self,
        columns: Range<usize>,
    ) -> MatrixSxD<&T, { range_len(ROWS) }>
    where
        [(); range_len(ROWS)]:,
    {
        assert!(ROWS.end < self.rows, "Row out of bounds");
        assert!(columns.end < C, "Columns out of bounds");
        let data: [Vec<&T>; range_len(ROWS)] = self
            .data
            .array_chunks::<C>()
            .map(|r| {
                r.iter()
                    .skip(columns.start)
                    .take(columns.end - columns.start)
                    .collect::<Vec<_>>()
            })
            .skip(ROWS.start)
            .take(range_len(ROWS))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        MatrixSxD::try_from(data).unwrap()
    }
}
impl<T: std::fmt::Debug, const C: usize> SliceDxS<T> for MatrixDxS<T, C>
where
    [(); C]:,
{
    fn slice_dxs<const COLUMNS: Range<usize>>(
        &self,
        rows: Range<usize>,
    ) -> MatrixDxS<&T, { range_len(COLUMNS) }>
    where
        [(); range_len(COLUMNS)]:,
    {
        assert!(rows.end < self.rows, "Row out of bounds");
        assert!(COLUMNS.end < C, "Columns out of bounds");
        let data: Vec<[&T; range_len(COLUMNS)]> = self
            .data
            .array_chunks::<C>()
            .map(|r| {
                r.iter()
                    .skip(COLUMNS.start)
                    .take(range_len(COLUMNS))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .skip(rows.start)
            .take(rows.end - rows.start)
            .collect::<Vec<_>>();

        MatrixDxS::from(data)
    }
}
impl<T: std::fmt::Debug, const C: usize> SliceSxS<T> for MatrixDxS<T, C> {
    fn slice_sxs<const ROWS: Range<usize>, const COLUMNS: Range<usize>>(
        &self,
    ) -> MatrixSxS<&T, { range_len(ROWS) }, { range_len(COLUMNS) }>
    where
        [(); { range_len(ROWS) } * { range_len(COLUMNS) }]:,
    {
        assert!(ROWS.end < self.rows, "Row out of bounds");
        assert!(COLUMNS.end < C, "Columns out of bounds");
        let data: [[&T; range_len(COLUMNS)]; range_len(ROWS)] = self
            .data
            .array_chunks::<C>()
            .map(|r| {
                r.iter()
                    .skip(COLUMNS.start)
                    .take(range_len(COLUMNS))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .skip(ROWS.start)
            .take(range_len(ROWS))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        MatrixSxS::from(data)
    }
}
// MatrixSxD
// --------------------------------------------------
impl<T, const R: usize> SliceDxD<T> for MatrixSxD<T, R> {
    fn slice_dxd(&self, (rows, columns): Slice) -> MatrixDxD<&T> {
        assert!(rows.end < R, "Row out of bounds");
        assert!(columns.end < self.columns, "Columns out of bounds");
        let data: Vec<Vec<&T>> = self
            .data
            .chunks_exact(self.columns)
            .map(|r| {
                r.iter()
                    .skip(columns.start)
                    .take(columns.end - columns.start)
                    .collect::<Vec<_>>()
            })
            .skip(rows.start)
            .take(rows.end - rows.start)
            .collect::<Vec<_>>();

        MatrixDxD::try_from(data).unwrap()
    }
}
impl<T: std::fmt::Debug, const R: usize> SliceSxD<T> for MatrixSxD<T, R> {
    fn slice_sxd<const ROWS: Range<usize>>(
        &self,
        columns: Range<usize>,
    ) -> MatrixSxD<&T, { range_len(ROWS) }>
    where
        [(); range_len(ROWS)]:,
    {
        assert!(ROWS.end < R, "Row out of bounds");
        assert!(columns.end < self.columns, "Columns out of bounds");
        let data: [Vec<&T>; range_len(ROWS)] = self
            .data
            .chunks_exact(self.columns)
            .map(|r| {
                r.iter()
                    .skip(columns.start)
                    .take(columns.end - columns.start)
                    .collect::<Vec<_>>()
            })
            .skip(ROWS.start)
            .take(range_len(ROWS))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        MatrixSxD::try_from(data).unwrap()
    }
}
impl<T: std::fmt::Debug, const R: usize> SliceDxS<T> for MatrixSxD<T, R>
where
    [(); R]:,
{
    fn slice_dxs<const COLUMNS: Range<usize>>(
        &self,
        rows: Range<usize>,
    ) -> MatrixDxS<&T, { range_len(COLUMNS) }>
    where
        [(); range_len(COLUMNS)]:,
    {
        assert!(rows.end < R, "Row out of bounds");
        assert!(COLUMNS.end < self.columns, "Columns out of bounds");
        let data: Vec<[&T; range_len(COLUMNS)]> = self
            .data
            .chunks_exact(self.columns)
            .map(|r| {
                r.iter()
                    .skip(COLUMNS.start)
                    .take(range_len(COLUMNS))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .skip(rows.start)
            .take(rows.end - rows.start)
            .collect::<Vec<_>>();

        MatrixDxS::from(data)
    }
}
impl<T: std::fmt::Debug, const R: usize> SliceSxS<T> for MatrixSxD<T, R> {
    fn slice_sxs<const ROWS: Range<usize>, const COLUMNS: Range<usize>>(
        &self,
    ) -> MatrixSxS<&T, { range_len(ROWS) }, { range_len(COLUMNS) }>
    where
        [(); { range_len(ROWS) } * { range_len(COLUMNS) }]:,
    {
        assert!(ROWS.end < R, "Row out of bounds");
        assert!(COLUMNS.end < self.columns, "Columns out of bounds");
        let data: [[&T; range_len(COLUMNS)]; range_len(ROWS)] = self
            .data
            .chunks_exact(self.columns)
            .map(|r| {
                r.iter()
                    .skip(COLUMNS.start)
                    .take(range_len(COLUMNS))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .skip(ROWS.start)
            .take(range_len(ROWS))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        MatrixSxS::from(data)
    }
}
// MatrixSxS
// --------------------------------------------------
impl<T, const R: usize, const C: usize> SliceDxD<T> for MatrixSxS<T, R, C>
where
    [(); R * C]:,
{
    fn slice_dxd(&self, (rows, columns): Slice) -> MatrixDxD<&T> {
        assert!(rows.end < R, "Row out of bounds");
        assert!(columns.end < C, "Columns out of bounds");
        let data: Vec<Vec<&T>> = self
            .data
            .array_chunks::<C>()
            .map(|r| {
                r.iter()
                    .skip(columns.start)
                    .take(columns.end - columns.start)
                    .collect::<Vec<_>>()
            })
            .skip(rows.start)
            .take(rows.end - rows.start)
            .collect::<Vec<_>>();

        MatrixDxD::try_from(data).unwrap()
    }
}
impl<T: std::fmt::Debug, const R: usize, const C: usize> SliceSxD<T> for MatrixSxS<T, R, C>
where
    [(); R * C]:,
{
    fn slice_sxd<const ROWS: Range<usize>>(
        &self,
        columns: Range<usize>,
    ) -> MatrixSxD<&T, { range_len(ROWS) }>
    where
        [(); range_len(ROWS)]:,
    {
        assert!(ROWS.end < R, "Row out of bounds");
        assert!(columns.end < C, "Columns out of bounds");
        let data: [Vec<&T>; range_len(ROWS)] = self
            .data
            .array_chunks::<C>()
            .map(|r| {
                r.iter()
                    .skip(columns.start)
                    .take(columns.end - columns.start)
                    .collect::<Vec<_>>()
            })
            .skip(ROWS.start)
            .take(range_len(ROWS))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        MatrixSxD::try_from(data).unwrap()
    }
}
impl<T: std::fmt::Debug, const R: usize, const C: usize> SliceDxS<T> for MatrixSxS<T, R, C>
where
    [(); R * C]:,
{
    fn slice_dxs<const COLUMNS: Range<usize>>(
        &self,
        rows: Range<usize>,
    ) -> MatrixDxS<&T, { range_len(COLUMNS) }>
    where
        [(); range_len(COLUMNS)]:,
    {
        assert!(rows.end < R, "Row out of bounds");
        assert!(COLUMNS.end < C, "Columns out of bounds");
        let data: Vec<[&T; range_len(COLUMNS)]> = self
            .data
            .array_chunks::<C>()
            .map(|r| {
                r.iter()
                    .skip(COLUMNS.start)
                    .take(range_len(COLUMNS))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .skip(rows.start)
            .take(rows.end - rows.start)
            .collect::<Vec<_>>();

        MatrixDxS::from(data)
    }
}
impl<T: std::fmt::Debug, const R: usize, const C: usize> SliceSxS<T> for MatrixSxS<T, R, C>
where
    [(); R * C]:,
{
    fn slice_sxs<const ROWS: Range<usize>, const COLUMNS: Range<usize>>(
        &self,
    ) -> MatrixSxS<&T, { range_len(ROWS) }, { range_len(COLUMNS) }>
    where
        [(); { range_len(ROWS) } * { range_len(COLUMNS) }]:,
    {
        assert!(ROWS.end < R, "Row out of bounds");
        assert!(COLUMNS.end < C, "Columns out of bounds");
        let data: [[&T; range_len(COLUMNS)]; range_len(ROWS)] = self
            .data
            .array_chunks::<C>()
            .map(|r| {
                r.iter()
                    .skip(COLUMNS.start)
                    .take(range_len(COLUMNS))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .skip(ROWS.start)
            .take(range_len(ROWS))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        MatrixSxS::from(data)
    }
}

// Tests
// --------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::*;
    // MatrixDxD
    // --------------------------------------------------
    #[test]
    fn dxd_dxd() {
        let a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        assert_eq!(
            a.slice_dxd((0..1, 0..2)),
            MatrixDxD::try_from(vec![vec![&1, &2]]).unwrap()
        );
    }
    #[test]
    fn dxd_dxs() {
        let a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        assert_eq!(
            a.slice_dxs::<{ 0..2 }>(0..1),
            MatrixDxS::from(vec![[&1, &2]])
        );
    }
    #[test]
    fn dxd_sxd() {
        let a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        assert_eq!(
            a.slice_sxd::<{ 0..1 }>(0..2),
            MatrixSxD::try_from([vec![&1, &2]]).unwrap()
        );
    }
    #[test]
    fn dxd_sxs() {
        let a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        assert_eq!(
            a.slice_sxs::<{ 0..1 }, { 0..2 }>(),
            MatrixSxS::from([[&1, &2]])
        );
    }
    // MatrixDxS
    // --------------------------------------------------
    #[test]
    fn dxs_dxd() {
        let a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
        assert_eq!(
            a.slice_dxd((0..1, 0..2)),
            MatrixDxD::try_from(vec![vec![&1, &2]]).unwrap()
        );
    }
    #[test]
    fn dxs_dxs() {
        let a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
        assert_eq!(
            a.slice_dxs::<{ 0..2 }>(0..1),
            MatrixDxS::from(vec![[&1, &2]])
        );
    }
    #[test]
    fn dxs_sxd() {
        let a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
        assert_eq!(
            a.slice_sxd::<{ 0..1 }>(0..2),
            MatrixSxD::try_from([vec![&1, &2]]).unwrap()
        );
    }
    #[test]
    fn dxs_sxs() {
        let a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
        assert_eq!(
            a.slice_sxs::<{ 0..1 }, { 0..2 }>(),
            MatrixSxS::from([[&1, &2]])
        );
    }
    // MatrixSxD
    // --------------------------------------------------
    #[test]
    fn sxd_dxd() {
        let a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        assert_eq!(
            a.slice_dxd((0..1, 0..2)),
            MatrixDxD::try_from(vec![vec![&1, &2]]).unwrap()
        );
    }
    #[test]
    fn sxd_dxs() {
        let a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        assert_eq!(
            a.slice_dxs::<{ 0..2 }>(0..1),
            MatrixDxS::from(vec![[&1, &2]])
        );
    }
    #[test]
    fn sxd_sxd() {
        let a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        assert_eq!(
            a.slice_sxd::<{ 0..1 }>(0..2),
            MatrixSxD::try_from([vec![&1, &2]]).unwrap()
        );
    }
    #[test]
    fn sxd_sxs() {
        let a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        assert_eq!(
            a.slice_sxs::<{ 0..1 }, { 0..2 }>(),
            MatrixSxS::from([[&1, &2]])
        );
    }
    // MatrixSxS
    // --------------------------------------------------
    #[test]
    fn sxs_dxd() {
        let a = MatrixSxS::from([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(
            a.slice_dxd((0..1, 0..2)),
            MatrixDxD::try_from(vec![vec![&1, &2]]).unwrap()
        );
    }
    #[test]
    fn sxs_dxs() {
        let a = MatrixSxS::from([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(
            a.slice_dxs::<{ 0..2 }>(0..1),
            MatrixDxS::from(vec![[&1, &2]])
        );
    }
    #[test]
    fn sxs_sxd() {
        let a = MatrixSxS::from([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(
            a.slice_sxd::<{ 0..1 }>(0..2),
            MatrixSxD::try_from([vec![&1, &2]]).unwrap()
        );
    }
    #[test]
    fn sxs_sxs() {
        let a = MatrixSxS::from([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(
            a.slice_sxs::<{ 0..1 }, { 0..2 }>(),
            MatrixSxS::from([[&1, &2]])
        );
    }
}
