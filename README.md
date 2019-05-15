# Problem Search (Rust Implementation)

[![Build Status](https://travis-ci.org/marcbreitung/rust-problem-search.svg?branch=master)](https://travis-ci.org/marcbreitung/rust-problem-search)

A simple breath first search.

```rust
let tiles: Vec<u8> = vec![
    2, 2, 2, 2, 2, 2,
    2, 1, 2, 2, 1, 2,
    2, 1, 2, 2, 1, 2,
    2, 1, 1, 1, 1, 2,
    2, 1, 2, 2, 1, 2,
    2, 1, 2, 2, 2, 2,
];
let mut graph = Graph::new(tiles, 6, 6);
let result = BreathFirstSearch::search(&mut graph, "1-1", "1-4");
let unwrap_result = result.unwrap();

assert_eq!(vec![
    0, 0, 0, 0, 0, 0,
    0, 1, 0, 0, 1, 0,
    0, 1, 0, 0, 1, 0,
    0, 1, 1, 1, 1, 0,
    0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0], BreathFirstSearch::get_path(&unwrap_result, &mut graph, "1-4"));
```