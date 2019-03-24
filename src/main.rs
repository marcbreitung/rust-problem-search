mod state;
mod node;
mod problem;
mod breath_first_search;

use state::State;
use node::Node;
use problem::Problem;
use breath_first_search::BreathFirstSearch;

fn main() {
    let start = State::new(1, 1);
    let goal = State::new(5, 5);
    let problem = Problem::new(start, goal);

    println!("{:?}", problem.get_initial());

    let node_11 = Node::new(State::new(1, 1), None);
    let node_12 = Node::new(State::new(1, 2), Some(Box::new(node_11.clone())));
    let node_13 = Node::new(State::new(1, 3), Some(Box::new(node_12.clone())));

    println!("{:?}", problem.goal_test(&node_11));

    println!("{:?}", node_11);
    println!("{:?}", node_12);
    println!("{:?}", node_13);

    let mut breath_first_search = BreathFirstSearch::new();
    breath_first_search.search(&problem);
}
