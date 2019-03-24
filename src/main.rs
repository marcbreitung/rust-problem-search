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
    let goal = State::new(5, 5);
    let graph = Graph::new(vec![1, 1, 1, 1], 2, 2);
    let problem = Problem::new(start, goal, graph);

    let mut breath_first_search = BreathFirstSearch::new();
    breath_first_search.search(&problem);

    let nodes: Vec<u8> = vec![
        1, 1, 2,
        2, 1, 2,
        2, 1, 2,
    ];

    let graph = Graph::new(nodes, 3, 3);
    let n = graph.get_neighbours(1, 1);

    println!("{:?}", n);
}
