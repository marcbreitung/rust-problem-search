use std::cmp;

#[derive(Debug, Clone, PartialEq)]
/// A state inside the problem, represented by row and column
pub struct State {
    pub row: u32,
    pub column: u32,
}

impl State {
    /// Returns a new state with the given row and column
    ///
    /// # Arguments
    ///
    /// * `row` - A u32 defines the row
    /// * `column` - A u32 defines the column
    ///
    /// # Example
    ///
    /// ```
    /// use crate::rust_problem_search::state::State;
    ///
    /// let state = State::new(10, 25);
    /// ```
    pub fn new(row: u32, column: u32) -> Self {
        State {
            row,
            column,
        }
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
    /// use crate::rust_problem_search::state::State;
    ///
    /// let state_a = State::new(5, 3);
    /// let state_b = State::new(2, 7);
    ///
    /// state_b.distance(&state_a);
    /// ```
    pub fn distance(&self, state: &State) -> u32 {
        let row = cmp::max(self.row, state.row) - cmp::min(self.row, state.row);
        let column = cmp::max(self.column, state.column) - cmp::min(self.column, state.column);
        cmp::max(row, column) + cmp::min(row, column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_new_state() {
        let state = State::new(10, 25);

        assert_eq!(10, state.row);
        assert_eq!(25, state.column);
    }

    #[test]
    fn equals_two_states_are_equal_returns_true() {
        let state_a = State::new(10, 25);
        let state_b = State::new(10, 25);

        assert_eq!(state_a, state_b);
    }

    #[test]
    fn equals_two_states_are_state_unequal_returns_true() {
        let state_a = State::new(10, 25);
        let state_b = State::new(25, 10);

        assert_ne!(state_a, state_b);
    }

    #[test]
    fn distance_returns_manhattan_distance() {
        let state_a = State::new(5, 3);
        let state_b = State::new(2, 7);

        assert_eq!(7, state_b.distance(&state_a));
    }
}
