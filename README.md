# Rust Sudoku solver

A Rust program to solve lots of Sudoku puzzles very quickly.

It's based on the
[constraint propagation](https://colab.research.google.com/github/norvig/pytudes/blob/main/ipynb/Sudoku.ipynb)
approach from Peter Norvig. This implementation aims to be nearly as high level
and expressive as his Python version, while matching or exceeding his optimized
Java version in terms of speed.

On my machine (11 core M3 Pro Macbook Pro), it is able to solve 250,000 puzzles
in under 750ms when run in parallel with rayon. When I run the
[Java program](https://github.com/norvig/pytudes/blob/main/ipynb/Sudoku.java) on
my machine, it is roughly 1/3 as fast.
