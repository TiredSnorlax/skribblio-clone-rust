<script lang="ts">
  import Chat from "$lib/components/playing/Chat.svelte";
  import DrawingBoard from "$lib/components/playing/DrawingBoard.svelte";
  import type { IGameState, IPlayer, IServerMsg } from "$lib/types/server";
  import { sessionID } from "$lib/stores";
  import PlayersList from "../playing/PlayersList.svelte";
  import { getMsgType } from "$lib/helper";
  import RoundChangeMenu from "../playing/RoundChangeMenu.svelte";

  export let socket: WebSocket | null;

  export let textMsgs: string[] = [];
  export let drawMsg: string = "";
  export let gameState: IGameState | null;
  export let players: { [key: string]: IPlayer };
  export let roomId: string;

  let timeLeft = 60;

  let roundChange = false;

  let currently_drawing: string | null = null;

  const getCurrentlyDrawing = (state: IGameState) => {
    console.log(
      "currently drawing",
      Object.keys(players)[state.currently_drawing]
    );
    return Object.keys(players)[state.currently_drawing];
  };

  const timer = (time: number) => {
    timeLeft = time;
    if (time > 0) {
      setTimeout(() => {
        timer(time - 1);
      }, 200);
    } else {
      endRound();
    }
  };

  const endRound = () => {
    console.log("Change");
    if ($sessionID !== currently_drawing) return;
    console.log("can draw");
    let msgContent = {
      user_id: $sessionID,
      room_id: roomId,
    };
    let msg: IServerMsg = {
      msg_type: getMsgType("Game", "EndTurn"),
      content: JSON.stringify(msgContent),
    };

    socket?.send(JSON.stringify(msg));
    timeLeft = 60;
  };

  const onRoundChange = (gameState: IGameState | null) => {
    if (!gameState) return;
    currently_drawing = getCurrentlyDrawing(gameState);
    roundChange = true;
    setTimeout(() => {
      timer(timeLeft);
    }, 4000);
    timeLeft = 60;
  };

  $: onRoundChange(gameState);
</script>

<div class="page">
  <div class="top">
    <p class="timer">{timeLeft}</p>
    <h2>Round: {gameState?.current_round} / {gameState?.total_rounds}</h2>
    <div class="wordHint">
      {#if $sessionID === currently_drawing}
        <p>{gameState?.correct_word}</p>
      {:else if gameState}
        {#each gameState.correct_word.split("") as char}
          <p>
            {Math.random() > 1 / gameState.correct_word.length ? "_" : char}
          </p>
        {/each}
      {/if}
    </div>
  </div>
  <div class="content">
    <PlayersList {players} {currently_drawing} />
    <DrawingBoard {socket} {currently_drawing} bind:drawMsg />
    <Chat {socket} {textMsgs} />
  </div>
</div>

{#if roundChange}
  <RoundChangeMenu
    bind:roundChange
    {currently_drawing}
    {gameState}
    players={Object.values(players)}
  />
{/if}

<style>
  .page {
    height: 100vh;
    width: 100vw;

    display: flex;
    flex-direction: column;
    text-align: center;
  }

  .content {
    flex: 1 1 auto;
    display: grid;
    grid-template-columns: 1fr 6fr 2fr;
  }

  .top {
    display: flex;
    background: white;
    justify-content: center;
    align-items: center;
  }

  h2 {
    padding: 0.5rem 1rem;
    width: fit-content;
    margin-inline: auto;
    border-radius: 3rem;
  }

  .timer {
    padding: 0.5rem;
    margin: 0.5rem 1rem;
    width: 64px;
    border-radius: 50%;
    background: red;
    aspect-ratio: 1;
    font-size: 2rem;

    display: flex;
    justify-content: center;
    align-items: center;
    color: white;
  }

  .wordHint {
    flex: 1 1 auto;
    padding: 1rem;
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 5px;

    font-size: 2rem;
  }
</style>
