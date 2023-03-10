<script lang="ts">
  import { getMsgType } from "$lib/helper";
  import { sessionID, userdata } from "$lib/stores";
  import type { IServerMsg, IGuessMsg } from "$lib/types/server";
  import { onMount } from "svelte";

  export let socket: WebSocket | null;
  export let textMsgs: string[];

  let mounted = false;

  let msgListEle: HTMLDivElement;

  let messages: IGuessMsg[] = [];
  let newMsg = "";

  const sendMsg = () => {
    if (newMsg.length === 0 || !socket || !$sessionID || !$userdata) return;
    let msgContent: IGuessMsg = {
      user_id: $sessionID,
      username: $userdata.username,
      content: newMsg,
    };

    let msgToSend: IServerMsg = {
      msg_type: getMsgType("Game", "Guess"),
      content: JSON.stringify(msgContent),
    };

    console.log(msgToSend);

    socket.send(JSON.stringify(msgToSend));

    // messages = [...messages, msgContent];
    newMsg = "";
    msgListEle.scrollTop = msgListEle.scrollHeight;
  };

  const receiveMsg = (msgs: string[], mounted: boolean) => {
    if (!mounted || msgs.length === 0) return;
    let lastMsgData = textMsgs[textMsgs.length - 1];
    let lastMsg: IGuessMsg = JSON.parse(lastMsgData);
    messages = [...messages, lastMsg];
    msgListEle.scrollTop = msgListEle.scrollHeight;
  };

  $: {
    receiveMsg(textMsgs, mounted);
  }

  onMount(() => {
    mounted = true;
  });
</script>

<div class="container">
  <h2>Chat</h2>
  <div class="msgList" bind:this={msgListEle}>
    {#each messages as message}
      <div
        class="msgItem"
        class:self={message.user_id === $sessionID}
        class:correct={message.correct}
      >
        <p>{message.username}:</p>
        <p>{message.correct ? "Guessed Correctly!" : message.content}</p>
      </div>
    {/each}
  </div>
  <div class="inputContainer">
    <input type="text" bind:value={newMsg} />
    <button on:click={sendMsg}>Send</button>
  </div>
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: stretch;

    background: grey;
  }

  h2 {
    margin-top: 1rem;
    text-align: center;
    font-size: 2rem;
  }

  .msgList {
    flex: 1 1 auto;
    padding: 1rem;

    height: 300px;

    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: flex-start;
    gap: 1rem;

    overflow-y: auto;
  }

  .msgItem {
    display: flex;
    justify-content: center;
    align-items: center;

    padding: 0.5rem;

    background: white;
    border-radius: 0.5rem;
  }

  .msgItem.self {
    align-self: flex-end;
    background: lightgreen;
  }

  .msgItem.correct {
    background-color: green !important;
  }

  .msgItem > p:first-child {
    font-weight: bold;
  }

  .msgItem > p:nth-child(2) {
    flex: 1 1 auto;
  }

  .inputContainer {
    padding: 1rem;
    background: lightblue;

    display: flex;
    justify-content: center;
    align-items: center;
    gap: 1rem;
  }

  .inputContainer input {
    flex: 1 1 auto;
    padding: 0.5rem;
    font-size: 1.2rem;
    border-radius: 2rem;
    border: 2px solid black;
    outline: none;
  }
</style>
