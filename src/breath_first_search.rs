use std::collections::vec_deque::VecDeque;
use std::collections::HashMap;
use crate::node::Node;

#[derive(Debug, Clone)]
pub struct BreathFirstSearch {}

impl BreathFirstSearch {
    pub fn search(graph: &HashMap<String, Node>, start: &str, goal: &str) -> Option<HashMap<String, String>> {
        let mut frontier = VecDeque::new();
        let mut explored = HashMap::new();
        let mut parent = start.to_string();

        frontier.push_back(graph[start].clone());

        while let Some(node) = frontier.pop_front() {
            explored.insert(format!("{}", node.position), parent.clone());
            parent = format!("{}", node.position);

            for neighbour in node.neighbours.iter() {
                if !explored.contains_key(neighbour) {
                    if neighbour != goal {
                        frontier.push_back(graph[neighbour].clone());
                    } else {
                        explored.insert(neighbour.clone(), parent.clone());
                        frontier.clear();
                        return Some(explored);
                    }
                }
            }
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;

    #[test]
    fn search_with_valid_path_returns_result() {
        let tiles: Vec<u8> = vec![
            2, 1, 2,
            2, 1, 2,
            2, 1, 2,
        ];
        let mut graph_struct = Graph::new(3, 3);
        let result = BreathFirstSearch::search(&graph_struct.get_nodes(tiles), "0-1", "2-1");
        let unwrap_result = result.unwrap();

        assert_eq!("0-1".to_string(), unwrap_result["1-1"]);
        assert_eq!("1-1".to_string(), unwrap_result["2-1"]);
        assert_eq!("0-1".to_string(), unwrap_result["0-1"]);
    }
}