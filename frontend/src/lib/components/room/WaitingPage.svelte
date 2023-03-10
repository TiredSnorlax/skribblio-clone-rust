<script lang="ts">
  import { sessionID } from "$lib/stores";
  import type { IPlayer, IPlayerMovement } from "$lib/types/server";
  import OwnerSettings from "./OwnerSettings.svelte";

  export let socket: WebSocket | null;
  export let gameMsgs: string[];
  export let players: { [key: string]: IPlayer };

  export let roomId: string;
  export let owner: string;

  const handlePlayerMovement = (_gameMsgs: string[]) => {
    if (_gameMsgs.length === 0) return;
    let _players = players;
    let newMsg: IPlayerMovement = JSON.parse(_gameMsgs[gameMsgs.length - 1]);
    console.log(newMsg);
    if (newMsg.enter) {
      _players[newMsg.user_id] = newMsg.player;
      console.log(`player ${newMsg.player.username} entered`);
    } else {
      delete _players[newMsg.user_id];
      console.log(players);
    }
    players = { ..._players };
  };

  $: handlePlayerMovement(gameMsgs);
</script>

<div class="page">
  <div class="top">
    <p>Room ID:</p>
    <p>{roomId}</p>
  </div>
  <div class="center">
    {#if owner === $sessionID}
      <OwnerSettings {socket} {roomId} />
    {/if}
    <div class="public">
      <h2>
        <span
          >{Object.values(players).filter((player) => player.active).length}
        </span>Players
      </h2>
      <div class="players">
        {#each Object.entries(players) as [id, player], i}
          {#if player.active}
            <div>
              <p>{player.username} {id === owner ? "(Owner)" : ""}</p>
            </div>
          {/if}
        {/each}
      </div>
    </div>
  </div>
</div>

<style>
  .page {
    width: 100vw;
    height: 100vh;

    padding: 4rem;

    display: flex;
    justify-content: flex-start;
    align-items: center;
    flex-direction: column;

    gap: 4rem;
  }

  .top {
    font-size: 2rem;

    display: flex;
    justify-content: center;
    align-items: center;

    gap: 1rem;
  }

  .top p:nth-child(2) {
    border-radius: 0.5rem;
    padding: 1rem;
    background: white;
  }

  .center {
    display: flex;
    justify-content: center;
    align-items: flex-start;
    gap: 1rem;

    margin-top: 3rem;
  }

  .center .public {
    padding: 2rem;

    background: white;
    border-radius: 0.5rem;
  }

  .players {
    margin-top: 1rem;
    display: flex;
    flex-direction: column;
    max-height: 500px;
    overflow-y: auto;

    gap: 0.5rem;
  }

  .players > div {
    display: flex;
    align-items: center;

    width: 300px;

    padding: 0.5rem;
    border-radius: 5px;
  }

  .players > div:nth-child(odd) {
    background: lightblue;
  }

  .players > .you {
    background: lightpink !important;
    margin: 0;
  }
</style>
