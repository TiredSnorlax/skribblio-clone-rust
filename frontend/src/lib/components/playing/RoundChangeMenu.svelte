<script lang="ts">
  import { sessionID } from "$lib/stores";
  import type { IGameState, IPlayer } from "$lib/types/server";
  import { onMount } from "svelte";

  export let currently_drawing: string | null;
  export let gameState: IGameState | null;
  export let roundChange: boolean;
  export let players: IPlayer[];

  $: sortedPlayer = players.sort((a, b) => {
    return a.score - b.score;
  });

  onMount(() => {
    setTimeout(() => {
      roundChange = false;
    }, 4000);
  });
</script>

{#if gameState}
  <div class="bg">
    <h2>Round {gameState.current_round} of {gameState.total_rounds}</h2>
    {#if currently_drawing === $sessionID && currently_drawing}
      <p>Your word is {gameState.correct_word}</p>
    {:else}
      <div class="scoreboard">
        {#each sortedPlayer as player}
          <p><span>{player.username}</span> - {player.score}</p>
        {/each}
      </div>
    {/if}
  </div>
{/if}

<style>
  .bg {
    background: rgba(0, 0, 0, 0.4);
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    color: white;

    position: fixed;
    inset: 0;
  }

  .scoreboard p span {
    font-weight: bold;
  }
</style>
