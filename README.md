# Reversi

This project implements the Reversi game (also known as Othello). It also includes a game-playing agent using the minimax algorithm. Users are able to play against either another human or a CPU player (i.e., the game-playing agent).

This game is hosted on my [personal website](https://yunhao-qian.github.io/showcase/reversi/). Click it open and play! 😊

## Features

For each of the Black (first) and White (second) players, users can pick whether it will be a human or a CPU player. All the four combinations are possible: human versus human, human versus CPU, CPU versus human, and CPU versus CPU (although the last combination is not very useful). The entire game history is recorded, so moves made by human players may be undone for an unlimited number of times, while moves made by CPUs cannot be undone. Moreover, users may stop in the middle of a game and start a new game.

CPU players are driven by the minimax algorithm with alpha-beta pruning. By changing the search depth and comprehensiveness of the heuristic function, the game-playing agent has three difficulty levels: easy, normal, and hard. The main code of this agent is written in Rust and then compiled into WebAssembly, resulting in faster response and smaller memory footprints compared to a JavaScript implementation.

## Project Structure

This project is made up of two parts:

1. A Cargo project containing a Rust implementation of the game-playing agent. It lies in the `reversi-agent` folder.

   * `reversi-agent/src/lib.rs`: This file exports the `play()` function for which a JavaScript binding is generated.
   * `reversi-agent/src/utils.rs`: Utility functions like counting scores and computing disc flips as a result of a move.

   To compile this Cargo project, execute `wasm-pack build` under the `reversi-agent` directory. Output files will be under two directories, `pkg` and `target`, where the `pkg` directory contains the JavaScript/TypeScript binding that will be used by the Node project.

2. A Node project which implements the user interface and the human player capability. The majority of its code lies in `src`.

   * `src/favicon.ico`: Webpage favicon, which is a screenshot of the user interface.
   * `src/index.html`: The main user interface.
   * `src/main.ts`: Logic of the user interface and the human player.
   * `src/style.css`: Stylesheet of the chessboard, discs, and controls.

   To compile this Node project, execute `npm run build` under the project's root directory. Output files will be under the `dist` directory, which constitute a webpage that is ready to be hosted by a server. Note that simply clicking-open the `dist/index.html` file will not work because of the browser's CORS restriction on WebAssembly files. An easy way to override this restriction is to host the webpage files on a local server by running `npm run serve` under the project's root directory. Users will then be able to visit the webpage at a localhost address.

## Acknowledgement

Heuristic functions used by the minimax algorithm are based on the paper *[An Analysis of Heuristics in Othello](https://courses.cs.washington.edu/courses/cse573/04au/Project/mini1/RUSSIA/Final_Paper.pdf)*.

## Screen Recording

The following GIF is a screen recording of a CPU-versus-CPU game. As you can see, the response time of CPU players is barely noticeable.

![CPU-versus-CPU game](images/cpu-vs-cpu.gif)
