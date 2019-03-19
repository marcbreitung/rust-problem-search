use std::collections::VecDeque;

use crate::node::Node;

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

    pub fn search(&mut self, problem: Problem) -> Result<Node, io::Error> {
        self.frontier = VecDeque::new();
        self.explored = VecDeque::new();

        let initial = problem.get_initial();

        if problem.goal_test(&initial) {
            return Result(initial);
        }

        self.frontier.push_back(initial);

        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn breath_first_search_new() {
        let state = BreathFirstSearch::new();
        assert_eq!(state.frontier.pop_front(), None);
        assert_eq!(state.explored.pop_front(), None);
    }
}
