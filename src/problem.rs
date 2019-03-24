use crate::state::State;
use crate::node::Node;

pub struct Problem {
    pub start: State,
    pub goal: State,
}

impl Problem {
    pub fn new(start: State, goal: State) -> Self {
        Problem {
            start,
            goal,
        }
    }

    pub fn goal_test(&self, node: &Node) -> bool {
        self.goal == node.state
    }

    pub fn get_initial(&self) -> Node {
        Node::new(self.start.clone(), None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_new_problem() {
        let start = State::new(1, 1);
        let goal = State::new(5, 5);
        let problem = Problem::new(start, goal);

        assert_eq!(State::new(1, 1), problem.start);
        assert_eq!(State::new(5, 5), problem.goal);
    }

    #[test]
    fn get_initial_returns_initial_state() {
        let start = State::new(1, 1);
        let goal = State::new(5, 5);
        let problem = Problem::new(start, goal);
        let initial = problem.get_initial();

        assert_eq!(State::new(1, 1), initial.state);
        assert_eq!(None, initial.parent);
    }

    #[test]
    fn goal_test_node_is_goal_returns_true() {
        let start = State::new(1, 1);
        let goal = State::new(5, 5);
        let problem = Problem::new(start, goal);
        let node = Node::new(State::new(5, 5), None);

        assert!(problem.goal_test(&node));
    }

    #[test]
    fn goal_test_node_is_not_goal_returns_false() {
        let start = State::new(1, 1);
        let goal = State::new(5, 5);
        let problem = Problem::new(start, goal);
        let node = Node::new(State::new(10, 10), None);

        assert!(!problem.goal_test(&node));
    }
}