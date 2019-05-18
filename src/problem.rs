use crate::node::Node;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Problem {
    pub nodes: HashMap<String, Node>,
    pub size: usize,
}

impl Problem {
    pub fn new(nodes: HashMap<String, Node>, size: usize) -> Self {
        Problem { nodes, size }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_with_nodes_and_size_returns_position() {
        let nodes = HashMap::new();
        let problem = Problem::new(nodes.clone(), 10);

        assert_eq!(nodes.clone(), problem.nodes);
        assert_eq!(10, problem.size);
    }
}
