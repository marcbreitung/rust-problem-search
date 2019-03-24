use crate::state::State;
use crate::node::Node;
use crate::graph::Graph;

pub struct Problem {
    pub start: State,
    pub goal: State,
    pub graph: Graph,
}

impl Problem {
    pub fn new(start: State, goal: State, graph: Graph) -> Self {
        Problem {
            start,
            goal,
            graph,
        }
    }
    pub fn goal_test(&self, node: &Node) -> bool {
        self.goal == node.state
    }
    pub fn get_initial(&self) -> Node {
        Node::new(self.start.clone(), None)
    }
    pub fn get_actions(&self, state: &State) -> Vec<State> {
        let mut result = vec![];
        let neighbours = self.graph.get_neighbours(state.row, state.column);
        for neighbour in &neighbours {
            let index = self.graph.get_index(neighbour.row, neighbour.column);
            let value = self.graph.get_value(index);
            if let Some(value) = value {
                if value == 1 {
                    result.push(State::new(neighbour.row, neighbour.column));
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_new_problem() {
        let start = State::new(1, 1);
        let goal = State::new(5, 5);
        let graph = Graph::new(vec![1, 1, 1, 1], 2, 2);
        let problem = Problem::new(start, goal, graph);

        assert_eq!(State::new(1, 1), problem.start);
        assert_eq!(State::new(5, 5), problem.goal);
    }

    #[test]
    fn goal_test_node_is_goal_returns_true() {
        let start = State::new(1, 1);
        let goal = State::new(5, 5);
        let graph = Graph::new(vec![1, 1, 1, 1], 2, 2);
        let problem = Problem::new(start, goal, graph);
        let node = Node::new(State::new(5, 5), None);

        assert!(problem.goal_test(&node));
    }

    #[test]
    fn goal_test_node_is_not_goal_returns_false() {
        let start = State::new(1, 1);
        let goal = State::new(5, 5);
        let graph = Graph::new(vec![1, 1, 1, 1], 2, 2);
        let problem = Problem::new(start, goal, graph);
        let node = Node::new(State::new(10, 10), None);

        assert!(!problem.goal_test(&node));
    }

    #[test]
    fn get_initial_returns_initial_state() {
        let start = State::new(1, 1);
        let goal = State::new(5, 5);
        let graph = Graph::new(vec![1, 1, 1, 1], 2, 2);
        let problem = Problem::new(start, goal, graph);
        let initial = problem.get_initial();

        assert_eq!(State::new(1, 1), initial.state);
        assert_eq!(None, initial.parent);
    }

    #[test]
    fn get_actions_with_state_returns_actions() {
        let start = State::new(1, 1);
        let goal = State::new(5, 5);
        let graph = Graph::new(vec![
            2, 1, 2,
            1, 1, 1,
            2, 1, 2,
        ], 3, 3);
        let problem = Problem::new(start, goal, graph);

        let state = State::new(1, 1);
        let result = vec![
            State::new(0, 1),
            State::new(1, 2),
            State::new(2, 1),
            State::new(1, 0),
        ];

        assert_eq!(problem.get_actions(&state), result);

        let state = State::new(0, 0);
        let result = vec![
            State::new(0, 1),
            State::new(1, 0),
        ];

        assert_eq!(problem.get_actions(&state), result);
    }
}
