use std::collections::vec_deque::VecDeque;
use std::collections::{HashMap, HashSet};

use crate::graph::Graph;
use crate::node::Node;
use crate::position::Position;
use crate::problem::Problem;

#[derive(Debug, Clone)]
pub struct BreathFirstSearch {}

impl BreathFirstSearch {
    /// Returns an Option a HashMap containing the result
    ///
    /// # Arguments
    ///
    /// * `problem` The problem contains the start, the end and the nodes where to find the path
    ///
    /// # Example
    /// ```
    /// use rust_problem_search::breath_first_search::BreathFirstSearch;
    /// use rust_problem_search::problem::Problem;
    /// use rust_problem_search::graph::Graph;
    ///
    /// let tiles: Vec<u8> = vec![
    ///     2, 1, 2,
    ///     2, 1, 2,
    ///     2, 1, 2];
    ///
    /// let graph = Graph::new(tiles, 3, 3);
    ///
    /// let problem = Problem::new(
    ///     graph.get_path_nodes(),
    ///     "0-1".to_string(),
    ///     "2-1".to_string(),
    ///     graph.size,
    /// );
    ///
    /// let result = BreathFirstSearch::search(&problem);
    /// ```
    pub fn search(problem: &Problem) -> Option<HashMap<String, String>> {
        let nodes = problem.nodes.clone();
        let start = &problem.start.clone();
        let goal = &problem.goal.clone();

        let mut frontier = VecDeque::new();
        let mut frontier_hash = HashSet::new();

        let mut explored = HashMap::new();
        let mut parent = "".to_string();

        if start == goal {
            explored.insert(start.clone(), "".to_string());
            return Some(explored);
        }

        frontier.push_back((nodes[start].clone(), parent.clone()));

        if let Some((n, _)) = frontier.get_mut(1) {
            frontier_hash.insert(format!("{}", n.position.clone()));
        }

        while let Some((node, parent_id)) = frontier.pop_front() {
            explored.insert(format!("{}", node.position), parent_id.clone());
            parent = format!("{}", node.position);

            for neighbour in node.neighbours.iter() {
                if !explored.contains_key(neighbour) && !frontier_hash.contains(neighbour) {
                    if neighbour != goal {
                        frontier_hash.insert(format!("{}", neighbour));
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

    /// Converts a flat vector from the solution found with the search function
    ///
    ///
    /// # Example
    /// ```
    /// use rust_problem_search::breath_first_search::BreathFirstSearch;
    /// use rust_problem_search::problem::Problem;
    /// use rust_problem_search::graph::Graph;
    ///
    /// let tiles: Vec<u8> = vec![
    ///     2, 2, 2, 2, 2, 2,
    ///     2, 1, 2, 2, 1, 2,
    ///     2, 1, 2, 2, 1, 2,
    ///     2, 1, 1, 1, 1, 2,
    ///     2, 1, 2, 2, 1, 2,
    ///     2, 1, 2, 2, 2, 2,
    /// ];
    /// let graph = Graph::new(tiles, 6, 6);
    ///
    /// let problem = Problem::new(
    ///     graph.get_path_nodes(),
    ///     "1-1".to_string(),
    ///     "1-4".to_string(),
    ///     graph.size,
    /// );
    /// let result = BreathFirstSearch::search(&problem);
    ///
    /// let unwrap_result = result.unwrap();
    ///
    /// assert_eq!(
    ///     vec![
    ///         0, 0, 0, 0, 0, 0,
    ///         0, 1, 0, 0, 1, 0,
    ///         0, 1, 0, 0, 1, 0,
    ///         0, 1, 1, 1, 1, 0,
    ///         0, 0, 0, 0, 0, 0,
    ///         0, 0, 0, 0, 0, 0
    ///     ],
    ///     BreathFirstSearch::get_path(&unwrap_result, &graph, &problem)
    /// );
    /// ```
    pub fn get_path(result: &HashMap<String, String>, graph: &Graph, problem: &Problem) -> Vec<u8> {
        let mut tiles = vec![0; graph.size];
        let nodes = problem.nodes.clone();
        let goal = &problem.goal.clone();
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
    use crate::graph::Graph;

    use super::*;

    #[test]
    fn search_with_valid_path_returns_result() {
        let tiles: Vec<u8> = vec![2, 1, 2, 2, 1, 2, 2, 1, 2];
        let graph = Graph::new(tiles, 3, 3);
        let problem = Problem::new(
            graph.get_path_nodes(),
            "0-1".to_string(),
            "2-1".to_string(),
            graph.size,
        );
        let result = BreathFirstSearch::search(&problem);
        let unwrap_result = result.unwrap();

        assert_eq!("1-1".to_string(), unwrap_result["2-1"]);
        assert_eq!("0-1".to_string(), unwrap_result["1-1"]);
        assert_eq!("".to_string(), unwrap_result["0-1"]);
    }

    #[test]
    fn search_with_start_equals_goal_returns_result() {
        let tiles: Vec<u8> = vec![
            2, 1, 2,
            2, 1, 2,
            2, 1, 2];
        let graph = Graph::new(tiles, 3, 3);
        let problem = Problem::new(
            graph.get_path_nodes(),
            "0-1".to_string(),
            "0-1".to_string(),
            graph.size,
        );
        let result = BreathFirstSearch::search(&problem);
        let unwrap_result = result.unwrap();

        assert_eq!("".to_string(), unwrap_result["0-1"]);
    }

    #[test]
    fn get_vec_returns_result_as_vector() {
        let tiles: Vec<u8> = vec![
            2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 1, 2, 2, 1, 2, 2, 1, 2, 2, 1, 1, 1, 1, 2, 2, 1, 2, 2, 1,
            2, 2, 1, 2, 2, 2, 2,
        ];
        let graph = Graph::new(tiles, 6, 6);
        let problem = Problem::new(
            graph.get_path_nodes(),
            "1-1".to_string(),
            "1-4".to_string(),
            graph.size,
        );
        let result = BreathFirstSearch::search(&problem);
        let unwrap_result = result.unwrap();

        assert_eq!(
            vec![
                0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0
            ],
            BreathFirstSearch::get_path(&unwrap_result, &graph, &problem)
        );
    }

    #[test]
    fn search_with_possible_path() {
        let tiles: Vec<u8> = vec![
            2, 2, 2, 0, 0, 0,
            2, 1, 1, 0, 0, 0,
            2, 1, 2, 0, 0, 0,
            0, 0, 0, 2, 2, 2,
            0, 0, 0, 1, 1, 2,
            0, 0, 0, 2, 2, 2,
        ];
        let graph = Graph::new(tiles, 6, 6);
        let problem = Problem::new(
            graph.get_possible_nodes(),
            "1-1".to_string(),
            "4-4".to_string(),
            graph.size,
        );
        let result = BreathFirstSearch::search(&problem);
        let unwrap_result = result.unwrap();

        assert_eq!(
            vec![
                0, 0, 0, 0, 0, 0,
                0, 1, 0, 0, 0, 0,
                0, 1, 0, 0, 0, 0,
                0, 1, 1, 0, 0, 0,
                0, 0, 1, 1, 1, 0,
                0, 0, 0, 0, 0, 0,
            ],
            BreathFirstSearch::get_path(&unwrap_result, &graph, &problem)
        );
    }
}
