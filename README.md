# Reversi

This project implements the Reversi game. It also includes a game-playing agent using the minmax algorithm. Users are able to play against either another human or a game-playing agent (a.k.a., CPU).

## Features

For each of the Black (first) and White (second) players, users can pick whether it will be a human or a game-playing agent. Thus, all the four combinations are possible: human versus human, human versus CPU, CPU versus human, and CPU versus CPU (although the last combination is not very useful). The entire game history is recorded, so moves made by human players may be undone by an unlimited number of times, while moves made by CPUs cannot be undone. Users can also stop in the middle of a game and start a new game.

The game-playing agent is empowered by the minmax algorithm with alpha-beta pruning. The heuristic function is based on the paper [An Analysis of Heuristics in Othello](https://courses.cs.washington.edu/courses/cse573/04au/Project/mini1/RUSSIA/Final_Paper.pdf). The main code of this agent is written in Rust and compiled into WebAssembly, resulting in faster response of CPU players and smaller memory footprints compared to a JavaScript implementation.
