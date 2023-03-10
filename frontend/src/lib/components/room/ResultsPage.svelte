<script lang="ts">
  import type { IPlayer } from "$lib/types/server";
  export let players: { [key: string]: IPlayer };
</script>

<div class="container">
  <h1>Results</h1>

  {#each Object.values(players) as player, i}
    <p>#{i} - {player.username} ({player.score})</p>
  {/each}
</div>

<style>
  .container {
    position: fixed;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;

    inset: 0;

    background: lightcoral;
  }
</style>
