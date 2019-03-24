#[derive(Debug, Clone, PartialEq)]
pub struct State {
    pub row: u32,
    pub column: u32,
}

impl State {
    pub fn new(row: u32, column: u32) -> Self {
        State {
            row,
            column,
        }
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

        assert!(state_a == state_b);
    }

    #[test]
    fn equals_two_states_are_state_unequal_returns_true() {
        let state_a = State::new(10, 25);
        let state_b = State::new(25, 10);

        assert!(state_a != state_b);
    }
}