mod state;
mod node;
mod problem;
mod breath_first_search;

use state::State;
use problem::Problem;
use breath_first_search::BreathFirstSearch;

fn main() {
    let start = State::new(1, 1);
    let goal = State::new(5, 5);
    let problem = Problem::new(start, goal);

    let mut breath_first_search = BreathFirstSearch::new();
    breath_first_search.search(&problem);
}
