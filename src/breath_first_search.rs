use std::collections::vec_deque::VecDeque;
use std::collections::HashMap;

use crate::graph::Graph;
use crate::position::Position;
use crate::node::Node;

#[derive(Debug, Clone)]
pub struct BreathFirstSearch {}

impl BreathFirstSearch {
    pub fn search(graph: &Graph, start: &str, goal: &str) -> Option<HashMap<String, String>> {
        let nodes = graph.get_nodes();
        let mut frontier = VecDeque::new();
        let mut explored = HashMap::new();
        let mut parent = "".to_string();

        frontier.push_back((nodes[start].clone(), parent.clone()));

        while let Some((node, parent_id)) = frontier.pop_front() {
            explored.insert(format!("{}", node.position), parent_id.clone());
            parent = format!("{}", node.position);

            for neighbour in node.neighbours.iter() {
                if !explored.contains_key(neighbour) {
                    if neighbour != goal {
                        frontier.push_back((nodes[neighbour].clone(), parent.clone()));
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

    pub fn get_path(result: &HashMap<String, String>, graph: &Graph, goal: &str) -> Vec<u8> {
        let mut tiles = vec![0; graph.size];
        let nodes = graph.get_nodes();
        let mut next = Some(goal);

        if let Some(node) = nodes.get(goal) {
            BreathFirstSearch::update_tile(graph, &mut tiles, node);
        }

        while let Some(tile) = next {
            match result.get(tile) {
                Some(parent) => {
                    if let Some(node) = nodes.get(parent) {
                        BreathFirstSearch::update_tile(graph, &mut tiles, node);
                    }
                    next = Some(parent);
                }
                _ => {
                    next = None;
                }
            }
        }
        tiles
    }

    fn update_tile(graph: &Graph, tiles: &mut Vec<u8>, node: &Node) {
        let position: Position = node.position;
        let index = graph.get_index_at_position(position);
        tiles[index] = 1;
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
        let graph = Graph::new(tiles, 3, 3);
        let result = BreathFirstSearch::search(&graph, "0-1", "2-1");
        let unwrap_result = result.unwrap();

        assert_eq!("1-1".to_string(), unwrap_result["2-1"]);
        assert_eq!("0-1".to_string(), unwrap_result["1-1"]);
        assert_eq!("".to_string(), unwrap_result["0-1"]);
    }

    #[test]
    fn get_vec_returns_result_as_vector() {
        let tiles: Vec<u8> = vec![
            2, 2, 2, 2, 2, 2,
            2, 1, 2, 2, 1, 2,
            2, 1, 2, 2, 1, 2,
            2, 1, 1, 1, 1, 2,
            2, 1, 2, 2, 1, 2,
            2, 1, 2, 2, 2, 2,
        ];
        let graph = Graph::new(tiles, 6, 6);
        let result = BreathFirstSearch::search(&graph, "1-1", "1-4");
        let unwrap_result = result.unwrap();

        assert_eq!(vec![
            0, 0, 0, 0, 0, 0,
            0, 1, 0, 0, 1, 0,
            0, 1, 0, 0, 1, 0,
            0, 1, 1, 1, 1, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0], BreathFirstSearch::get_path(&unwrap_result, &graph, "1-4"));
    }
}
