# Rust Cellular Automata

### An interactive SDL2 implementation of Conway's Game of Life

## Installation

Clone the repo and navigate into it.
```bash
git clone https://github.com/ethanavatar/Rust-CA.git
cd Rust-CA/
```

Build the app using:
```bash
cargo build
```

## Usage

Run the app using:
```bash
cargo run
```

A window will open with a random initial state and start simulating immediately.

You can also use:
 - `SPACE` to pause the simulation
 - `ESCAPE` to clear the current board
 - `R` to create a new random board
 - `LEFT MOUSE` to add or remove cells
 - `PERIOD` to pause and step one generation at a time

By default, the window is 1200x1200 pixels and the game board is 200x200 cells. It runs as fast as it can without an FPS cap.
