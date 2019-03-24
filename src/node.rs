use crate::state::State;

#[derive(Debug, Clone)]
pub struct Node {
    pub state: State,
    pub parent: Option<Box<Node>>,
}

impl Node {
    pub fn new(state: State, parent: Option<Box<Node>>) -> Self {
        Node {
            state,
            parent,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.state.row == other.state.row && self.state.column == other.state.column
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_new_node() {
        let state = State::new(10, 25);
        let node = Node::new(state, None);

        assert_eq!(State::new(10, 25), node.state);
        assert_eq!(None, node.parent);
    }

    #[test]
    fn equals_equal_nodes_return_true() {
        let state_a = State::new(10, 25);
        let node_a = Node::new(state_a, None);

        let state_b = State::new(10, 25);
        let node_b = Node::new(state_b, Some(Box::new(node_a.clone())));

        assert!(node_a == node_b);
    }

    #[test]
    fn equals_unequal_nodes_return_false() {
        let state_a = State::new(10, 10);
        let node_a = Node::new(state_a, None);

        let state_b = State::new(10, 25);
        let node_b = Node::new(state_b, None);

        assert!(!(node_a == node_b));
    }
}
