use crate::state::State;
use crate::node::Node;
use crate::graph::Graph;

/// Defines the structure the execute the search
pub struct Problem {
    pub start: State,
    pub goal: State,
    pub graph: Graph,
}

impl Problem {
    /// Returns a new problem with the given start state, goal state and the graph
    ///
    /// # Arguments
    ///
    ///  * `start` - A State which defines the search starting point
    ///  * `goal` - A State which defines the search goal
    ///  * `graph` - A Graph which defines the searchable structure
    ///
    /// # Example
    ///
    /// ```
    /// use crate::rust_problem_search::state::State;
    /// use crate::rust_problem_search::node::Node;
    /// use crate::rust_problem_search::graph::Graph;
    /// use crate::rust_problem_search::problem::Problem;
    ///
    /// let start = State::new(1, 1);
    /// let goal = State::new(5, 5);
    /// let graph = Graph::new(vec![1, 1, 1, 1], 2, 2);
    /// let problem = Problem::new(start, goal, graph);
    /// ```
    pub fn new(start: State, goal: State, graph: Graph) -> Self {
        Problem {
            start,
            goal,
            graph,
        }
    }

    /// Checks if the given state is the goal
    pub fn goal_test(&self, node: &Node) -> bool {
        self.goal == node.state
    }

    /// Returns the initial state
    pub fn get_initial(&self) -> Node {
        Node::new(self.start.clone(), None)
    }

    /// Returns all possible actions by the given state
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

    /// Returns the state with the closest distance to the goal
    pub fn get_closest(&self) -> State {
        let mut closest = self.start.clone();
        for (state) in self.graph.get_end_states().iter() {
            if self.goal.distance(state) < self.goal.distance(&closest) {
                closest = state.clone();
            }
        }

        closest
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

    #[test]
    fn get_closest_returns_closest_state_to_goal() {
        let start = State::new(0, 1);
        let goal = State::new(3, 3);
        let graph = Graph::new(vec![
            2, 1, 2, 2, 0,
            2, 1, 1, 1, 0,
            2, 1, 2, 2, 0,
            2, 2, 2, 2, 0,
            0, 0, 0, 0, 0,
        ], 5, 5);
        let problem = Problem::new(start, goal, graph);

        let closest = State::new(1, 3);

        assert_eq!(problem.get_closest(), closest);
    }
}
