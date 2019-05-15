use crate::position::Position;
use crate::tile::Tile;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
/// Defines a node inside the graph
pub struct Node {
    pub position: Position,
    pub value: Tile,
    pub neighbours: Vec<String>,
}

impl Node {
    /// Returns a new Position
    ///
    /// # Arguments
    ///
    /// * `position` - A Position defines the position
    /// * `value` - A Tile defines type of this node
    /// * `neighbours` - A Vec<String> with the connected node keys
    ///
    /// # Example
    ///
    /// ```
    /// use crate::rust_problem_search::position::Position;
    /// use crate::rust_problem_search::node::Node;
    /// use crate::rust_problem_search::tile::Tile;
    ///
    /// let position = Position::new(1, 0);
    /// let neighbours: Vec<String> = vec![];
    /// let node = Node::new(position, Tile::Path, Vec::new());
    /// ```
    pub fn new(position: Position, value: Tile, neighbours: Vec<String>) -> Self {
        Node {
            position,
            value,
            neighbours,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_new_node() {
        let position = Position::new(1, 0);
        let neighbours: Vec<String> = vec![];
        let node = Node::new(position, Tile::Path, Vec::new());

        assert_eq!(position, node.position);
        assert_eq!(Tile::Path, node.value);
        assert_eq!(neighbours, node.neighbours);
    }
}