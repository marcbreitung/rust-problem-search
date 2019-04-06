use crate::state::State;

pub struct Graph {
    pub width: u32,
    pub height: u32,
    pub nodes: Vec<u8>,
}

impl Graph {
    pub fn new(nodes: Vec<u8>, width: u32, height: u32) -> Self {
        Graph {
            nodes,
            width,
            height,
        }
    }
    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    pub fn get_value(&self, index: usize) -> Option<u8> {
        let mut result = None;
        let value = self.nodes.get(index);
        if let Some(value) = value {
            result = Some(*value);
        }
        result
    }
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
    pub fn get_state_at_index(&self, index: u32) -> State {
        let row = index / self.width;
        let col = index - (self.width * row);
        State::new(row, col)
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
}
