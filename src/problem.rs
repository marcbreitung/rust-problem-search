use crate::node::Node;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Problem {
    pub nodes: HashMap<String, Node>,
    pub start: String,
    pub goal: String,
    pub size: usize,
}

impl Problem {
    pub fn new(nodes: HashMap<String, Node>, start: String, goal: String, size: usize) -> Self {
        Problem { nodes, start, goal, size }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_with_nodes_and_size_returns_position() {
        let nodes = HashMap::new();
        let problem = Problem::new(nodes.clone(), "1-1".to_string(), "1-2".to_string(), 10);

        assert_eq!(nodes.clone(), problem.nodes);
        assert_eq!("1-1".to_string(), problem.start);
        assert_eq!("1-2".to_string(), problem.goal);
        assert_eq!(10, problem.size);
    }
}
