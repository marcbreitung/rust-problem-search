use std::collections::VecDeque;

use crate::node::Node;
use crate::state::State;
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

        let node = self.frontier.pop_front();

        if let Some(node) = node {
            println!("{:?}", node);
        }

        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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
        let problem = Problem::new(start, goal);
        let result = Node::new(State::new(1, 1), None);

        assert_eq!(breath_first_search.search(&problem), Some(result));
    }

    #[test]
    fn search_no_solution_exits_returns_none() {
        let mut breath_first_search = BreathFirstSearch::new();
        let start = State::new(1, 1);
        let goal = State::new(5, 5);
        let problem = Problem::new(start, goal);

        assert_eq!(breath_first_search.search(&problem), None);
    }
}
