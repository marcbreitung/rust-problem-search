use std::cmp;
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
        Position { row, column }
    }

    /// Returns the manhattan distance between two states
    ///
    /// # Arguments
    ///
    /// * `state` - a state
    ///
    /// # Example
    ///
    /// ```
    /// use crate::rust_problem_search::position::Position;
    ///
    /// let position_a = Position::new(5, 3);
    /// let position_b = Position::new(2, 7);
    ///
    /// position_b.distance(&position_a);
    /// ```
    pub fn distance(&self, position: &Position) -> u32 {
        let row = cmp::max(self.row, position.row) - cmp::min(self.row, position.row);
        let column =
            cmp::max(self.column, position.column) - cmp::min(self.column, position.column);
        cmp::max(row, column) + cmp::min(row, column)
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

    #[test]
    fn distance_returns_manhattan_distance() {
        let position_a = Position::new(5, 3);
        let position_b = Position::new(2, 7);

        assert_eq!(7, position_b.distance(&position_a));
    }
}
