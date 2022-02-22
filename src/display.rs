use crate::*;
use std::fmt;

impl<T: fmt::Display, const ROWS: usize, const COLUMNS: usize> fmt::Display
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Maximum width of element string
        let max_char_width = self
            .data
            .iter()
            .map(|v| v.to_string().chars().count())
            .max()
            .unwrap_or(0);

        let mut width = Default::default();
        let mut middle = self
            .data
            .array_chunks::<COLUMNS>()
            .map(|r| {
                let t = format!(
                    "│ {}",
                    r.iter()
                        .map(|x| {
                            let s = x.to_string();
                            format!("{}{} ", " ".repeat(max_char_width - s.chars().count()), s)
                        })
                        .collect::<String>()
                );
                width = std::cmp::max(t.chars().count(), width);
                t
            })
            .collect::<Vec<_>>();
        for row in middle.iter_mut() {
            row.push_str(&format!("{}│\n", " ".repeat(width - row.chars().count())));
        }

        let bar = " ".repeat(width - 1);
        let top = format!("┌{}┐\n", bar);
        let bottom = format!("└{}┘", bar);
        write!(
            f,
            "\n{}{}{}",
            top,
            middle.into_iter().collect::<String>(),
            bottom
        )
    }
}
impl<T: fmt::Display> fmt::Display for MatrixDxD<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Maximum width of element string
        let max_char_width = self
            .data
            .iter()
            .map(|v| v.to_string().chars().count())
            .max()
            .unwrap_or(0);

        let mut width = Default::default();
        let mut middle = self
            .data
            .chunks_exact(self.columns)
            .map(|r| {
                let t = format!(
                    "│ {}",
                    r.iter()
                        .map(|x| {
                            let s = x.to_string();
                            format!("{}{} ", " ".repeat(max_char_width - s.chars().count()), s)
                        })
                        .collect::<String>()
                );
                width = std::cmp::max(t.chars().count(), width);
                t
            })
            .collect::<Vec<_>>();
        for row in middle.iter_mut() {
            row.push_str(&format!("{}│\n", " ".repeat(width - row.chars().count())));
        }

        let bar = " ".repeat(width - 1);
        let top = format!("┌{}┐\n", bar);
        let bottom = format!("└{}┘", bar);
        write!(
            f,
            "\n{}{}{}",
            top,
            middle.into_iter().collect::<String>(),
            bottom
        )
    }
}
impl<T: fmt::Display, const ROWS: usize> fmt::Display for MatrixSxD<T, ROWS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Maximum width of element string
        let max_char_width = self
            .data
            .iter()
            .map(|v| v.to_string().chars().count())
            .max()
            .unwrap_or(0);

        let mut width = Default::default();
        let mut middle = self
            .data
            .chunks_exact(self.columns)
            .map(|r| {
                let t = format!(
                    "│ {}",
                    r.iter()
                        .map(|x| {
                            let s = x.to_string();
                            format!("{}{} ", " ".repeat(max_char_width - s.chars().count()), s)
                        })
                        .collect::<String>()
                );
                width = std::cmp::max(t.chars().count(), width);
                t
            })
            .collect::<Vec<_>>();
        for row in middle.iter_mut() {
            row.push_str(&format!("{}│\n", " ".repeat(width - row.chars().count())));
        }

        let bar = " ".repeat(width - 1);
        let top = format!("┌{}┐\n", bar);
        let bottom = format!("└{}┘", bar);
        write!(
            f,
            "\n{}{}{}",
            top,
            middle.into_iter().collect::<String>(),
            bottom
        )
    }
}
impl<T: fmt::Display, const COLUMNS: usize> fmt::Display for MatrixDxS<T, COLUMNS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Maximum width of element string
        let max_char_width = self
            .data
            .iter()
            .map(|v| v.to_string().chars().count())
            .max()
            .unwrap_or(0);

        let mut width = Default::default();
        let mut middle = self
            .data
            .array_chunks::<COLUMNS>()
            .map(|r| {
                let t = format!(
                    "│ {}",
                    r.iter()
                        .map(|x| {
                            let s = x.to_string();
                            format!("{}{} ", " ".repeat(max_char_width - s.chars().count()), s)
                        })
                        .collect::<String>()
                );
                width = std::cmp::max(t.chars().count(), width);
                t
            })
            .collect::<Vec<_>>();
        for row in middle.iter_mut() {
            row.push_str(&format!("{}│\n", " ".repeat(width - row.chars().count())));
        }

        let bar = " ".repeat(width - 1);
        let top = format!("┌{}┐\n", bar);
        let bottom = format!("└{}┘", bar);
        write!(
            f,
            "\n{}{}{}",
            top,
            middle.into_iter().collect::<String>(),
            bottom
        )
    }
}
