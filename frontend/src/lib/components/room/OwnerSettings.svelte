<script lang="ts">
  import { getMsgType } from "$lib/helper";
  import { sessionID } from "$lib/stores";
  import type { IGameState, IServerMsg } from "$lib/types/server";

  export let socket: WebSocket | null;
  export let roomId: string;

  let rounds = "2";
  let roomName = "New Room";

  let isValidRound = true;

  let checkRounds = (_rounds: string) => {
    isValidRound = /^-?\d+$/.test(rounds);
  };

  const increment = () => {
    rounds = parseInt(rounds) + 1 + "";
  };
  const decrement = () => {
    let _rounds = parseInt(rounds) - 1;
    if (_rounds < 1) {
      rounds = "1";
    } else {
      rounds = _rounds + "";
    }
  };

  $: checkRounds(rounds);

  const startGame = () => {
    if (!socket) return;
    let newGameState: IGameState = {
      total_rounds: parseInt(rounds),
      current_round: 1,
      currently_drawing: 0,
      title: roomName,
      correct_word: "",
      round_start_time: 0,
    };
    let msgContent = {
      user_id: $sessionID,
      room_id: roomId,
      state: newGameState,
    };
    let msg: IServerMsg = {
      msg_type: getMsgType("Game", "StartGame"),
      content: JSON.stringify(msgContent),
    };
    console.log(msg);
    socket.send(JSON.stringify(msg));
  };
</script>

<div class="container">
  <h2>Settings</h2>
  <div class="nameContainer">
    <input type="text" bind:value={roomName} />
  </div>
  <div class="roundContainer">
    <p>Rounds</p>
    <div>
      <button on:click={decrement}>-1</button>
      <input
        type="text"
        inputmode="numeric"
        bind:value={rounds}
        class:invalid={!isValidRound}
      />
      <button on:click={increment}>+1</button>
    </div>
  </div>

  <button class="startBtn" on:click={startGame}>Start!</button>
</div>

<style>
  .container {
    background: white;
    border-radius: 0.5rem;
    padding: 1rem;

    display: flex;
    flex-direction: column;
    justify-content: center;

    gap: 2rem;
  }

  h2 {
    font-size: 2rem;
  }

  input {
    background: none;
    outline: none;
    border: 2px solid rgb(200, 200, 200);
    border-radius: 5px;
    padding: 0.5rem 1rem;

    font-size: 1.5rem;
  }

  input:focus {
    border-color: blue;
  }

  .roundContainer {
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .roundContainer > p:first-child {
    font-size: 1.5rem;
  }

  .roundContainer > div {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 1rem;
  }

  .roundContainer input {
    font-size: 1rem;
    padding: 0.5rem 0;
    width: 50px;
    text-align: center;
  }

  .roundContainer input.invalid {
    border-color: red;
  }

  .startBtn {
    color: white;
    background: green;
    border-radius: 0.5rem;
    padding: 1rem 0.5rem;
    font-size: 1.5rem;
  }
</style>
