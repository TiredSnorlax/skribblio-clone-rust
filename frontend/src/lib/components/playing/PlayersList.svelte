<script lang="ts">
  import { sessionID } from "$lib/stores";
  import type { IPlayer } from "$lib/types/server";

  export let players: { [key: string]: IPlayer };
  export let currently_drawing: string | null;
</script>

<div class="container">
  {#each Object.entries(players) as [id, player], i}
    <div
      class="player"
      class:you={$sessionID === id}
      class:drawing={currently_drawing && id === currently_drawing}
    >
      <h3>#{i + 1}</h3>
      <div>
        <p>{player.username} {$sessionID === id ? "(You)" : ""}</p>
        <p>Score: {player.score}</p>
      </div>
    </div>
  {/each}
</div>

<style>
  .container {
    background: white;
    padding: 1rem 0;
  }

  .player {
    width: 100%;
    text-align: center;
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
  }

  .player > div {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .player:nth-child(even) {
    background: lightcyan;
  }

  .player > div p:first-child {
    font-weight: bold;
  }

  .player.you {
    background: lightgrey;
  }

  .player.drawing {
    outline: 2px solid blue;
  }
</style>
