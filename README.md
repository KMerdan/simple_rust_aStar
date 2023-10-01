# Simple A-Star with GUI

This is a Rust project that implements a-start pathfinding algorithm for a grid-based map.

## Installation

To build and run the project, you need to have Rust and Cargo installed on your system. You can install Rust and Cargo by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Once you have Rust and Cargo installed, you can build and run the project using the following commands:

```bash
cargo build 
cargo run
```

## Usage

The project provides a simple graphical user interface for visualizing the pathfinding algorithm. To use the GUI, run the project using the `cargo run` command, and then use the mouse right click to select the start and end positions on the map, left click to setup [free, occupaied, unknown]. The pathfinder will then find the shortest path between the two positions, and highlight the path on the map.

You can customize the map by editing the `grid_map.rs` file, which defines the size and layout of the map, as well as the initial state of each cell.

