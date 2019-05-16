use rust_problem_search::graph::Graph;

fn main() {
    let tiles: Vec<u8> = vec![
        2, 1, 2, 0,
        2, 1, 1, 0,
        2, 1, 2, 0,
        0, 0, 0, 0,
    ];
    let graph = Graph::new(tiles, 4, 4);
    let nodes = graph.get_end_nodes();

    println!("{:#?}", nodes);
}