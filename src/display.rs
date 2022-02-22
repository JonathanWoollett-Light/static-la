use crate::*;
use std::fmt;

impl<T: fmt::Display, const ROWS: usize, const COLUMNS: usize> fmt::Display
    for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut width = Default::default();
        let middle = self.data
            .array_chunks::<COLUMNS>()
            .map(|r|{
                let t = format!("│ {}│\n",
                    r.iter()
                    .map(|x|format!("{} ",x))
                    .collect::<String>()
                );
                width = std::cmp::max(t.chars().count(),width);
                t
            }).collect::<String>();
        let bar = "─".repeat(width-3);
        let top = format!("┌{}┐\n",bar);
        let bottom = format!("└{}┘",bar);
        write!(f, "\n{}{}{}", top,middle,bottom)
    }
}
impl<T: fmt::Display> fmt::Display for MatrixDxD<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut width = Default::default();
        let middle = self.data
            .chunks_exact(self.columns)
            .map(|r|{
                let t = format!("│ {}│\n",
                    r.iter()
                    .map(|x|format!("{} ",x))
                    .collect::<String>()
                );
                width = std::cmp::max(t.chars().count(),width);
                t
            }).collect::<String>();
        let bar = "─".repeat(width-3);
        let top = format!("┌{}┐\n",bar);
        let bottom = format!("└{}┘",bar);
        write!(f, "\n{}{}{}", top,middle,bottom)
    }
}
impl<T: fmt::Display, const ROWS: usize> fmt::Display for MatrixSxD<T, ROWS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut width = Default::default();
        let middle = self.data
            .chunks_exact(self.columns)
            .map(|r|{
                let t = format!("│ {}│\n",
                    r.iter()
                    .map(|x|format!("{} ",x))
                    .collect::<String>()
                );
                width = std::cmp::max(t.chars().count(),width);
                t
            }).collect::<String>();
        let bar = "─".repeat(width-3);
        let top = format!("┌{}┐\n",bar);
        let bottom = format!("└{}┘",bar);
        write!(f, "\n{}{}{}", top,middle,bottom)
    }
}
impl<T: fmt::Display, const COLUMNS: usize> fmt::Display for MatrixDxS<T, COLUMNS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut width = Default::default();
        let middle = self.data
            .array_chunks::<COLUMNS>()
            .map(|r|{
                let t = format!("│ {}│\n",
                    r.iter()
                    .map(|x|format!("{} ",x))
                    .collect::<String>()
                );
                width = std::cmp::max(t.chars().count(),width);
                t
            }).collect::<String>();
        let bar = "─".repeat(width-3);
        let top = format!("┌{}┐\n",bar);
        let bottom = format!("└{}┘",bar);
        write!(f, "\n{}{}{}", top,middle,bottom)
    }
}