use crate::state::State;

/// The graph represents the problem structure
pub struct Graph {
    pub width: u32,
    pub height: u32,
    pub nodes: Vec<u8>,
}

impl Graph {
    /// Returns a new graph with the given width and height
    ///
    /// # Arguments
    ///
    /// * `nodes` - a Vec<u8> with the type for each tile
    /// * `width` - the number of horizontal tiles
    /// * `height` - the number of vertical tiles
    ///
    /// # Example
    ///
    /// ```
    /// use crate::rust_problem_search::graph::Graph;
    ///
    /// let graph = Graph::new(vec![
    ///     1, 1, 1, 1,
    ///     1, 1, 1, 1,
    ///     1, 1, 1, 1,
    ///     1, 1, 1, 1
    /// ] , 4, 4);
    /// ```
    pub fn new(nodes: Vec<u8>, width: u32, height: u32) -> Self {
        Graph {
            nodes,
            width,
            height,
        }
    }

    /// Returns the index of the elements at the given row and column
    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    /// Returns the value at the given index as Option
    pub fn get_value(&self, index: usize) -> Option<u8> {
        let mut result = None;
        let value = self.nodes.get(index);
        if let Some(value) = value {
            result = Some(*value);
        }
        result
    }

    /// Returns a vec with all neighbours at the given row and column
    pub fn get_neighbours(&self, row: u32, column: u32) -> Vec<State> {
        let mut result = vec![];
        let rows = vec![row as i32 - 1, row as i32, row as i32 + 1, row as i32];
        let columns = vec![column as i32, column as i32 + 1, column as i32, column as i32 - 1];

        let positions = rows.iter().zip(columns.iter());

        for (row, column) in positions {
            if *row >= 0 && *column >= 0 && *row < self.height as i32 && *column < self.width as i32 {
                result.push(State::new(*row as u32, *column as u32));
            }
        }

        result
    }

    /// Returns the state ath the given index
    pub fn get_state_at_index(&self, index: usize) -> State {
        let row = index as u32 / self.width;
        let col = index as u32 - (self.width * row);
        State::new(row, col)
    }

    /// Returns all states with the given value
    pub fn get_states_with_value(&self, value: u8) -> Vec<State> {
        let mut states = vec![];

        for (index, _) in self.nodes.iter().enumerate() {
            if let Some(v) = self.get_value(index) {
                if v == value {
                    let state = self.get_state_at_index(index);
                    states.push(state)
                }
            }
        }

        states
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_new_graph() {
        let nodes: Vec<u8> = vec![
            1, 1, 2,
            2, 1, 2,
            2, 1, 2,
        ];
        let graph = Graph::new(nodes, 3, 3);

        assert_eq!(graph.width, 3);
        assert_eq!(graph.height, 3);
    }

    #[test]
    fn get_index_with_row_and_column_returns_index() {
        let nodes: Vec<u8> = vec![
            1, 1, 2,
            2, 1, 2,
            2, 1, 2,
        ];
        let graph = Graph::new(nodes, 3, 3);

        assert_eq!(graph.get_index(1, 1), 4);
    }

    #[test]
    fn get_value_with_index_returns_value() {
        let nodes: Vec<u8> = vec![
            1, 1, 2,
            2, 3, 2,
            2, 1, 2,
        ];
        let graph = Graph::new(nodes, 3, 3);

        assert_eq!(graph.get_value(4), Some(3));
    }

    #[test]
    fn get_neighbours_with_row_and_column_returns_neighbours() {
        let nodes: Vec<u8> = vec![
            1, 1, 2,
            2, 1, 2,
            2, 1, 2,
        ];
        let graph = Graph::new(nodes, 3, 3);

        let result = vec![
            State::new(0, 1),
            State::new(1, 2),
            State::new(2, 1),
            State::new(1, 0),
        ];

        assert_eq!(graph.get_neighbours(1, 1), result);
    }

    #[test]
    fn get_neighbours_with_first_row_and_first_column_returns_neighbours() {
        let nodes: Vec<u8> = vec![
            1, 1, 2,
            2, 1, 2,
            2, 1, 2,
        ];
        let graph = Graph::new(nodes, 3, 3);

        let result = vec![
            State::new(0, 1),
            State::new(1, 0)
        ];

        assert_eq!(graph.get_neighbours(0, 0), result);
    }

    #[test]
    fn get_neighbours_with_last_row_and_last_column_returns_neighbours() {
        let nodes: Vec<u8> = vec![
            1, 1, 2,
            2, 1, 2,
            2, 1, 2,
        ];
        let graph = Graph::new(nodes, 3, 3);

        let result = vec![
            State::new(1, 2),
            State::new(2, 1)
        ];

        assert_eq!(graph.get_neighbours(2, 2), result);
    }

    #[test]
    fn get_state_at_index_with_index_returns_state() {
        let nodes: Vec<u8> = vec![
            1, 1, 2,
            2, 1, 2,
            2, 1, 2,
        ];
        let graph = Graph::new(nodes, 3, 3);

        let state = State::new(1, 2);

        assert_eq!(graph.get_state_at_index(5), state);
    }

    #[test]
    fn get_states_with_value_with_value_one_returns_states_with_the_given_value() {
        let nodes: Vec<u8> = vec![
            2, 2, 2,
            2, 1, 2,
            2, 1, 2,
        ];
        let graph = Graph::new(nodes, 3, 3);

        let state_a = State::new(1, 1);
        let state_b = State::new(2, 1);

        let states = vec![state_a, state_b];

        assert_eq!(graph.get_states_with_value(1), states);
    }
}
