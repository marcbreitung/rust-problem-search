# Problem Search (Rust Implementation)

[![Build Status](https://travis-ci.org/marcbreitung/rust-problem-search.svg?branch=master)](https://travis-ci.org/marcbreitung/rust-problem-search)

A simple breath first search.


Build a graph with a flat vec. 

* 0 is an empty tile
* 1 is a path tile
* 2 is a ground tile


```rust
let tiles: Vec<u8> = vec![
    2, 2, 2, 2, 2, 2,
    2, 1, 2, 2, 1, 2,
    2, 1, 2, 2, 1, 2,
    2, 1, 1, 1, 1, 2,
    2, 1, 2, 2, 1, 2,
    2, 1, 2, 2, 2, 2,
];

let graph = Graph::new(tiles, 6, 6);
```

Define the problem. 

The get_path_nodes function takes into account the path tiles as possible path.

```rust
let problem = Problem::new(
    graph.get_path_nodes(),
    "1-1".to_string(),
    "1-4".to_string(),
    graph.size,
);
```

The get_possible_nodes function takes into account the path and empty tiles as possible path to calculate a possible path.

```rust
let problem = Problem::new(
    graph.get_possible_nodes(),
    "1-1".to_string(),
    "1-4".to_string(),
    graph.size,
);
```

Execute the search.

```rust
let result = BreathFirstSearch::search(&problem);
let unwrap_result = result.unwrap();

assert_eq!(
    vec![
        0, 0, 0, 0, 0, 0,
        0, 1, 0, 0, 1, 0,
        0, 1, 0, 0, 1, 0,
        0, 1, 1, 1, 1, 0,
        0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0
    ],
    BreathFirstSearch::get_path(&unwrap_result, &graph, &problem)
);
```