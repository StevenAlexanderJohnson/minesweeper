# Minesweeper Solver Competition

This repository hosts a friendly competition: write the fastest script that solves a Minesweeper board by interacting with the local HTTP API provided by this project.

**Quick summary:** write a script that talks to the HTTP endpoints to reveal and flag cells. The fastest correct solver wins.

## Overview

- **Objective:** Implement an automated solver script that interacts with the repository's HTTP API to solve Minesweeper boards as quickly as possible.
- **Difficulty levels:** There are three levels: `Easy`, `Medium`, and `Hard`.
- **Completion condition:** A game is considered complete when the number of unrevealed squares plus the number of flagged squares equals the bomb count:

  U + F = B

  where `U` = unrevealed squares, `F` = flagged squares, and `B` = bomb count.

## Rules

- Your solver must only interact with the board via the provided HTTP endpoints (see below). Directly reading or modifying the game's internal memory/state outside of these endpoints is not allowed.
- You may start a new game, reveal cells, and flag cells through the API.
- The solver must correctly reach the completion condition above; incorrect flagging that prevents satisfying the condition does not count.
- Time is measured as elapsed real time from the new board API call until the completion condition is met. The lower the time, the better.

## HTTP Endpoints (usage)

The app exposes a small HTTP API for interacting with the Minesweeper game. Typical endpoints include:

- `GET /api/get_game_state` — fetch the current board state and metadata (dimensions, bomb count, revealed cells, flagged cells).
- `POST /api/new_game` — start a new game (optionally with difficulty).
- `POST /api/reveal_cell` — reveal a cell at a given coordinate.
- `POST /api/flag_cell` — toggle a flag at a given coordinate.

The API will return the state of the board for each endpoint.

## How to participate

Running the application will start an API on `127.0.0.1:9091`.

1. Start the app so the HTTP server is available (see next section).
2. Implement a script in any language that:
   - Calls `POST /api/new_game` to start the game.
   - Calls `GET /api/get_game_state` to read the board if you need to for whatever reason.
   - Calls `POST /api/reveal_cell` and `POST /api/flag_cell` to interact with cells.
   - Repeats until the completion condition `U + F = B` is satisfied.
3. The final solve time will be displayed on the GUI which will be updated with each API call. The final solve time is your score.

## Running the app (dev)

From the repository root you can run the project in development mode. Example commands used in this project:

```powershell
pnpm tauri dev
```

When the app is running, the local HTTP server used by the frontend will be reachable on `http://127.0.0.1:9091` (or the port the app reports). Confirm endpoints by visiting `http://127.0.0.1:9091/api/get_game_state` in your browser while the app is running.

## Files of interest

- `src-tauri/src/api.rs` — HTTP route handlers for the game API.
- `src-tauri/src/minesweeper.rs` — game model, `DisplayBoard`, and difficulty definitions.
- `src/routes/+page.svelte` — frontend example that demonstrates usage of the API.
- `src/lib/minesweeper/game.svelte.ts` — TypeScript types and client helpers used by the UI.

## Tips for solvers

- The frontend responds to each API request. You can use this to visually debug.
- Start with a correct solver (even if slow) then optimize for fewer HTTP calls and parallelism.
- Ensure your script robustly handles transient HTTP errors and unexpected board states.

Good luck — may the fastest, correct solver win!
