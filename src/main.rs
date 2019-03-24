mod graph;
mod state;
mod node;
mod problem;
mod breath_first_search;

use graph::Graph;
use state::State;
use problem::Problem;
use breath_first_search::BreathFirstSearch;

fn main() {
    let start = State::new(1, 1);
    let goal = State::new(4, 4);
    let graph = Graph::new(vec![
        1, 1, 1, 1, 1,
        1, 1, 1, 1, 2,
        1, 1, 1, 1, 1,
        1, 1, 1, 1, 1,
        1, 1, 1, 1, 1,
    ], 5, 5);
    let problem = Problem::new(start, goal, graph);

    let mut breath_first_search = BreathFirstSearch::new();
    let result = breath_first_search.search(&problem);

    println!("{:?}", result);
}
