use std::fmt;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
/// Defines a position, based on row and column
pub struct Position {
    pub row: u32,
    pub column: u32,
}

impl Position {
    /// Returns a new Position
    ///
    /// # Arguments
    ///
    /// * `row` - A u32 defines the row
    /// * `column` - A u32 defines the column
    ///
    /// # Example
    ///
    /// ```
    /// use crate::rust_problem_search::position::Position;
    ///
    /// let position = Position::new(10, 25);
    /// ```
    pub fn new(row: u32, column: u32) -> Self {
        Position {
            row,
            column,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}-{:?}", self.row, self.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_with_row_and_column_returns_position() {
        let position = Position::new(1, 0);
        assert_eq!(1, position.row);
        assert_eq!(0, position.column);
    }

    #[test]
    fn display_prints_formatted_position() {
        let position = Position::new(1, 0);
        let display = "1-0".to_string();
        assert_eq!(display, format!("{}", position));
    }
}