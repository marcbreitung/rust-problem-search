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
    fn state_new() {
        let start = State::new(1, 1);
        let goal = State::new(5, 5);
        let problem = Problem::new(start, goal);
        assert_eq!(State::new(1, 1), problem.start);
        assert_eq!(State::new(5, 5), problem.goal);
    }

    #[test]
    fn state_get_initial() {
        let start = State::new(1, 1);
        let goal = State::new(5, 5);
        let problem = Problem::new(start, goal);
        let initial = problem.get_initial();
        assert_eq!(State::new(1, 1), initial.state);
        assert_eq!(None, initial.parent);
    }

    #[test]
    fn state_goal_test_is_goal() {
        let start = State::new(1, 1);
        let goal = State::new(5, 5);
        let problem = Problem::new(start, goal);
        let node = Node::new(State::new(5, 5), None);
        assert!(problem.goal_test(&node));
    }

    #[test]
    fn state_goal_test_is_not_goal() {
        let start = State::new(1, 1);
        let goal = State::new(5, 5);
        let problem = Problem::new(start, goal);
        let node = Node::new(State::new(10, 10), None);
        assert!(!problem.goal_test(&node));
    }
}