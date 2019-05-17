use crate::node::Node;
use crate::position::Position;
use crate::tile::Tile;
use std::collections::HashMap;

#[derive(Debug, Clone)]
/// The graph build with nodes
pub struct Graph {
    pub tiles: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub size: usize,
}

impl Graph {
    pub fn new(tiles: Vec<u8>, width: u32, height: u32) -> Self {
        let size: usize = (width * height) as usize;
        Graph {
            tiles,
            width,
            height,
            size,
        }
    }

    pub fn get_nodes(&self) -> HashMap<String, Node> {
        self.get_neighbours_with_tile(Tile::Path)
    }

    pub fn get_end_nodes(&self) -> HashMap<String, Node> {
        self.get_neighbours_with_tile(Tile::None)
    }

    pub fn get_index_at_position(&self, position: Position) -> usize {
        (position.row * self.width + position.column) as usize
    }

    fn get_neighbours_with_tile(&self, tile: Tile) -> HashMap<String, Node> {
        let mut nodes = HashMap::new();
        for (index, value) in self.tiles.iter().enumerate() {
            let position = self.get_position_at_index(index);
            let neighbours = if *value == 1 {
                self.get_neighbours_at_position(position, tile)
            } else {
                vec![]
            };
            let neighbours = neighbours.iter().map(|p| format!("{}", p)).collect();
            nodes.insert(
                format!("{}", position),
                Node::new(position.clone(), Tile::from_u8(*value), neighbours),
            );
        }
        nodes
    }

    fn get_neighbours_at_position(&self, position: Position, tile: Tile) -> Vec<Position> {
        let mut result = vec![];
        let rows = vec![
            position.row as i32 - 1,
            position.row as i32,
            position.row as i32 + 1,
            position.row as i32,
        ];
        let columns = vec![
            position.column as i32,
            position.column as i32 + 1,
            position.column as i32,
            position.column as i32 - 1,
        ];
        let positions = rows.iter().zip(columns.iter());

        for (row, column) in positions {
            if *row >= 0 && *column >= 0 && *row < self.height as i32 && *column < self.width as i32
            {
                let neighbour_position = Position::new(*row as u32, *column as u32);
                if let Some(value) = self.get_value_at_position(neighbour_position) {
                    if value == tile {
                        result.push(neighbour_position);
                    }
                }
            }
        }
        result
    }

    fn get_position_at_index(&self, index: usize) -> Position {
        let row = index as u32 / self.width;
        let column = index as u32 - (self.width * row);
        Position::new(row, column)
    }

    fn get_value_at_index(&self, index: usize) -> Option<Tile> {
        let mut result = None;
        let value = self.tiles.get(index);
        if let Some(value) = value {
            result = Some(Tile::from_u8(*value));
        }
        result
    }

    fn get_value_at_position(&self, position: Position) -> Option<Tile> {
        let index = self.get_index_at_position(position);
        self.get_value_at_index(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_new_graph() {
        let tiles: Vec<u8> = vec![2, 1, 2, 2, 1, 1, 2, 1, 2];
        let graph = Graph::new(tiles, 18, 18);
        assert_eq!(18, graph.width);
        assert_eq!(18, graph.height);
        assert_eq!(324, graph.size);
    }

    #[test]
    fn get_nodes_with_tiles_returns_nodes() {
        let tiles: Vec<u8> = vec![2, 1, 2, 2, 1, 1, 2, 1, 2];
        let graph = Graph::new(tiles, 3, 3);
        let nodes = graph.get_nodes();

        assert_eq!(
            Node::new(Position::new(0, 0), Tile::Ground, vec![]),
            nodes["0-0"]
        );
        assert_eq!(
            Node::new(Position::new(0, 1), Tile::Path, vec!["1-1".to_string()]),
            nodes["0-1"]
        );
        assert_eq!(
            Node::new(Position::new(0, 2), Tile::Ground, vec![]),
            nodes["0-2"]
        );

        assert_eq!(
            Node::new(Position::new(1, 0), Tile::Ground, vec![]),
            nodes["1-0"]
        );
        assert_eq!(
            Node::new(
                Position::new(1, 1),
                Tile::Path,
                vec!["0-1".to_string(), "1-2".to_string(), "2-1".to_string()]
            ),
            nodes["1-1"]
        );
        assert_eq!(
            Node::new(Position::new(1, 2), Tile::Path, vec!["1-1".to_string()]),
            nodes["1-2"]
        );

        assert_eq!(
            Node::new(Position::new(2, 0), Tile::Ground, vec![]),
            nodes["2-0"]
        );
        assert_eq!(
            Node::new(Position::new(2, 1), Tile::Path, vec!["1-1".to_string()]),
            nodes["2-1"]
        );
        assert_eq!(
            Node::new(Position::new(2, 2), Tile::Ground, vec![]),
            nodes["2-2"]
        );
    }

    #[test]
    fn get_end_nodes_with_tiles_returns_nodes() {
        let tiles: Vec<u8> = vec![2, 1, 2, 0, 2, 1, 1, 0, 2, 1, 2, 0, 0, 0, 0, 0];
        let graph = Graph::new(tiles, 4, 4);
        let nodes = graph.get_end_nodes();

        assert_eq!(
            Node::new(Position::new(2, 1), Tile::Path, vec!["3-1".to_string()]),
            nodes["2-1"]
        );
        assert_eq!(
            Node::new(Position::new(1, 2), Tile::Path, vec!["1-3".to_string()]),
            nodes["1-2"]
        );
    }

    #[test]
    fn get_index_at_position_returns_index() {
        let graph = Graph::new(vec![], 3, 3);
        assert_eq!(4, graph.get_index_at_position(Position::new(1, 1)));
    }

    #[test]
    fn get_neighbours_with_tile_with_tile_returns_nodes() {
        let tiles: Vec<u8> = vec![2, 1, 2, 2, 1, 1, 2, 1, 2];
        let graph = Graph::new(tiles, 3, 3);
        let nodes = graph.get_neighbours_with_tile(Tile::Path);

        assert_eq!(
            Node::new(Position::new(0, 0), Tile::Ground, vec![]),
            nodes["0-0"]
        );
        assert_eq!(
            Node::new(Position::new(0, 1), Tile::Path, vec!["1-1".to_string()]),
            nodes["0-1"]
        );
        assert_eq!(
            Node::new(Position::new(0, 2), Tile::Ground, vec![]),
            nodes["0-2"]
        );

        assert_eq!(
            Node::new(Position::new(1, 0), Tile::Ground, vec![]),
            nodes["1-0"]
        );
        assert_eq!(
            Node::new(
                Position::new(1, 1),
                Tile::Path,
                vec!["0-1".to_string(), "1-2".to_string(), "2-1".to_string()]
            ),
            nodes["1-1"]
        );
        assert_eq!(
            Node::new(Position::new(1, 2), Tile::Path, vec!["1-1".to_string()]),
            nodes["1-2"]
        );

        assert_eq!(
            Node::new(Position::new(2, 0), Tile::Ground, vec![]),
            nodes["2-0"]
        );
        assert_eq!(
            Node::new(Position::new(2, 1), Tile::Path, vec!["1-1".to_string()]),
            nodes["2-1"]
        );
        assert_eq!(
            Node::new(Position::new(2, 2), Tile::Ground, vec![]),
            nodes["2-2"]
        );
    }

    #[test]
    fn get_neighbours_at_position_returns_vec_with_path_neighbours() {
        let tiles: Vec<u8> = vec![2, 1, 2, 2, 1, 1, 2, 1, 2];
        let graph = Graph::new(tiles, 3, 3);
        let nodes = graph.get_neighbours_at_position(Position::new(1, 1), Tile::Path);

        assert_eq!(3, nodes.len());
        assert_eq!(Some(&Position::new(0, 1)), nodes.get(0));
        assert_eq!(Some(&Position::new(1, 2)), nodes.get(1));
        assert_eq!(Some(&Position::new(2, 1)), nodes.get(2));
    }

    #[test]
    fn get_position_at_index_returns_position() {
        let graph = Graph::new(vec![], 3, 3);
        assert_eq!(Position::new(1, 1), graph.get_position_at_index(4 as usize));
    }

    #[test]
    fn get_value_at_index_returns_value() {
        let tiles: Vec<u8> = vec![2, 1, 2, 2, 1, 1, 2, 1, 2];
        let graph = Graph::new(tiles, 3, 3);

        assert_eq!(Some(Tile::Ground), graph.get_value_at_index(2 as usize));
        assert_eq!(Some(Tile::Path), graph.get_value_at_index(4 as usize));
        assert_eq!(None, graph.get_value_at_index(10 as usize));
    }

    #[test]
    fn get_value_at_position_returns_value() {
        let tiles: Vec<u8> = vec![2, 1, 2, 2, 1, 1, 2, 1, 2];
        let graph = Graph::new(tiles, 3, 3);

        assert_eq!(
            Some(Tile::Ground),
            graph.get_value_at_position(Position::new(0, 2))
        );
        assert_eq!(
            Some(Tile::Path),
            graph.get_value_at_position(Position::new(1, 1))
        );
    }
}
