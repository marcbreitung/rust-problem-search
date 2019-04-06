use std::collections::VecDeque;

use crate::node::Node;
use crate::problem::Problem;

pub struct BreathFirstSearch {
    pub frontier: VecDeque<Node>,
    pub explored: VecDeque<Node>,
}

impl BreathFirstSearch {
    pub fn new() -> Self {
        BreathFirstSearch {
            frontier: VecDeque::new(),
            explored: VecDeque::new(),
        }
    }
    pub fn search(&mut self, problem: &Problem) -> Option<Node> {
        self.frontier = VecDeque::new();
        self.explored = VecDeque::new();

        let initial = problem.get_initial();

        if problem.goal_test(&initial) {
            return Some(initial);
        }

        self.frontier.push_back(initial);

        while let Some(node) = self.frontier.pop_front() {
            self.explored.push_back(node.clone());

            let actions = problem.get_actions(&node.state);

            for action in actions {
                let child = Node::new(action, Some(Box::new(node.clone())));

                if self.should_add_node(&child) {
                    if problem.goal_test(&child) {
                        return Some(child);
                    } else {
                        self.frontier.push_back(child);
                    }
                }
            }
        }

        None
    }
    pub fn search_vec(&mut self, problem: &Problem) -> Option<Vec<u8>> {
        let mut search = self.search(problem);

        if search == None {
            None
        } else {
            let mut result = vec![0; (problem.graph.width * problem.graph.height) as usize];

            while let Some(node) = search {
                let index = problem.graph.get_index(node.state.row, node.state.column);
                result[index] = 1;

                match node.parent {
                    Some(parent) => {
                        search = Some(*parent.clone());
                    }
                    None => {
                        search = None;
                    }
                }
            }

            Some(result)
        }
    }
    pub fn should_add_node(&self, node: &Node) -> bool {
        self.explored.iter().any(|x| x == node) == false && self.frontier.iter().any(|x| x == node) == false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::State;
    use crate::graph::Graph;

    #[test]
    fn new_returns_new_breath_first_search() {
        let mut breath_first_search = BreathFirstSearch::new();

        assert_eq!(breath_first_search.frontier.pop_front(), None);
        assert_eq!(breath_first_search.explored.pop_front(), None);
    }

    #[test]
    fn search_start_is_goal_returns_start_as_result() {
        let mut breath_first_search = BreathFirstSearch::new();

        let start = State::new(1, 1);
        let goal = State::new(1, 1);
        let graph = Graph::new(vec![1, 1, 1, 1], 2, 2);
        let problem = Problem::new(start, goal, graph);

        let result = Node::new(State::new(1, 1), None);

        assert_eq!(breath_first_search.search(&problem), Some(result));
    }

    #[test]
    fn search_no_solution_exits_returns_none() {
        let mut breath_first_search = BreathFirstSearch::new();

        let start = State::new(1, 1);
        let goal = State::new(5, 5);
        let graph = Graph::new(vec![1, 1, 1, 1], 2, 2);
        let problem = Problem::new(start, goal, graph);

        assert_eq!(breath_first_search.search(&problem), None);
    }

    #[test]
    fn should_add_node_with_non_existing_node_returns_true() {
        let breath_first_search = BreathFirstSearch::new();
        let node = Node::new(State::new(5, 5), None);
        assert!(breath_first_search.should_add_node(&node));
    }

    #[test]
    fn should_add_node_with_existing_node_in_frontier_returns_false() {
        let mut breath_first_search = BreathFirstSearch::new();
        breath_first_search.frontier.push_back(Node::new(State::new(5, 5), None));
        let node = Node::new(State::new(5, 5), None);
        assert!(!breath_first_search.should_add_node(&node));
    }

    #[test]
    fn should_add_node_with_existing_node_in_explored_returns_false() {
        let mut breath_first_search = BreathFirstSearch::new();
        breath_first_search.explored.push_back(Node::new(State::new(5, 5), None));
        let node = Node::new(State::new(5, 5), None);
        assert!(!breath_first_search.should_add_node(&node));
    }

    #[test]
    fn search_with_valid_solution_returns_option_with_solution() {
        let mut breath_first_search = BreathFirstSearch::new();

        let start = State::new(0, 1);
        let goal = State::new(1, 2);
        let graph = Graph::new(vec![
            2, 1, 2, 2,
            2, 1, 1, 2,
            2, 2, 2, 2,
            2, 2, 2, 2,
        ], 4, 4);
        let problem = Problem::new(start, goal, graph);

        let node_a = Node::new(State::new(0, 1), None);
        let node_b = Node::new(State::new(1, 1), Some(Box::new(node_a.clone())));
        let node_c = Node::new(State::new(1, 2), Some(Box::new(node_b.clone())));

        assert_eq!(breath_first_search.search(&problem), Some(node_c));
    }

    #[test]
    fn search_vec_with_valid_solution_returns_vec_with_solution() {
        let mut breath_first_search = BreathFirstSearch::new();

        let start = State::new(0, 1);
        let goal = State::new(1, 2);
        let graph = Graph::new(vec![
            2, 1, 2, 2,
            2, 1, 1, 2,
            2, 2, 2, 2,
            2, 2, 2, 2,
        ], 4, 4);
        let problem = Problem::new(start, goal, graph);

        let result = vec![
            0, 1, 0, 0,
            0, 1, 1, 0,
            0, 0, 0, 0,
            0, 0, 0, 0,
        ];

        assert_eq!(breath_first_search.search_vec(&problem), Some(result));
    }

    #[test]
    fn search_vec_large_with_valid_solution_returns_vec_with_solution() {
        let mut breath_first_search = BreathFirstSearch::new();

        let start = State::new(1, 1);
        let goal = State::new(16, 16);
        let graph = Graph::new(vec![
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 1, 2, 2, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 1, 2, 2, 1, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 1, 2, 2, 1, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 1, 2, 2, 1, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 1, 2, 2, 1, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 1, 2, 2, 1, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 1, 2, 2, 1, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 1, 1, 1, 1, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        ], 18, 18);
        let problem = Problem::new(start, goal, graph);

        let result = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        assert_eq!(breath_first_search.search_vec(&problem), Some(result));
    }
}
