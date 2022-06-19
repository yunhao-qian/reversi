# Reversi

This project implements the Reversi game (also known as Othello). It also includes a game-playing agent using the minimax algorithm. Users are able to play against either another human or a CPU player (i.e., the game-playing agent).

## Features

For each of the Black (first) and White (second) players, users can pick whether it will be a human or a CPU player. All the four combinations are possible: human versus human, human versus CPU, CPU versus human, and CPU versus CPU (although the last combination is not very useful). The entire game history is recorded, so moves made by human players may be undone for an unlimited number of times, while moves made by CPUs cannot be undone. Moreover, users may stop in the middle of a game and start a new game.

CPU players are driven by the minimax algorithm with alpha-beta pruning. By changing the search depth and comprehensiveness of the heuristic function, the game-playing agent has three difficulty levels: easy, normal, and hard. The main code of this agent is written in Rust and then compiled into WebAssembly, resulting in faster response and smaller memory footprints compared to a JavaScript implementation.

## Acknowledgement

The heuristic functions are based on the paper *[An Analysis of Heuristics in Othello](https://courses.cs.washington.edu/courses/cse573/04au/Project/mini1/RUSSIA/Final_Paper.pdf)*.

## Screen Recording

The following GIF is a screen recording of a CPU-versus-CPU game. As you can see from it, the response time of CPU players is barely noticeable.

![CPU-versus-CPU game](images/cpu-vs-cpu.gif)
