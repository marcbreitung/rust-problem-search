use crate::state::State;

#[derive(Debug, Clone, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_new() {
        let state = State::new(10, 25);
        let node = Node::new(state, None);
        assert_eq!(State::new(10, 25), node.state);
        assert_eq!(None, node.parent);
    }
}