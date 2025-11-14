<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { type Minesweeper, Game } from "$lib/minesweeper/game.svelte";
  let game = new Game();

  let difficulty: "easy" | "medium" | "hard" = $state("easy");

  $effect(() => {
    invoke<Minesweeper.DisplayBoard>("get_game_state").then((result) => {
      game.setGameState(result);
    });
    const canvas = document.getElementById("game-board") as HTMLCanvasElement;
    game.setBoard(canvas);

    return () => {
      game.destroy();
    };
  });

  $inspect(game.gameState);

  $effect(() => {
    if (game.gameState) {
      game.render();
    }
  });
</script>

<main class="container">
  <h1>{game.gameState?.difficulty}</h1>
  <div class="game-info">
    <button
      onclick={() => {
        invoke<Minesweeper.DisplayBoard>("new_game", { difficulty }).then(
          (result) => {
            game.setGameState(result);
            game.render();
          },
        );
      }}>New Game</button
    >
    <select bind:value={difficulty}>
      <option value="easy">Easy</option>
      <option value="medium">Medium</option>
      <option value="hard">Hard</option>
    </select>
  </div>
  {#if game.gameState?.game_state === "Won"}
    <h2>You won!</h2>
    <p>Time taken: {game.getDuration() / 1000} seconds</p>
  {:else if game.gameState?.game_state === "Lost"}
    <h2>You lost!</h2>
  {/if}
  <div id="game-container">
    <canvas id="game-board" width="600" height="600"></canvas>
  </div>
</main>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #fff;
    background-color: #000;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .game-info {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1rem;
  }

  h1 {
    text-align: center;
  }
</style>
