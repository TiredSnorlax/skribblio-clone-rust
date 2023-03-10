<script lang="ts">
  import { page } from "$app/stores";
  import PlayingPage from "$lib/components/room/PlayingPage.svelte";
  import ResultsPage from "$lib/components/room/ResultsPage.svelte";
  import UsernamePrompt from "$lib/components/room/UsernamePrompt.svelte";
  import WaitingPage from "$lib/components/room/WaitingPage.svelte";
  import { serverDomain } from "$lib/domains";
  import { sessionID, userdata } from "$lib/stores";
  import type {
    IGameState,
    IPlayer,
    IRoom,
    IServerMsg,
    PlayerData,
  } from "$lib/types/server";
  import { onDestroy, onMount } from "svelte";

  let roomData: IRoom | null;
  $: players = roomData?.players || {};

  let socket: WebSocket | null;

  let textMsgs: string[] = [];
  let drawMsg = "";
  let gameMsgs: string[] = [];

  let gameStarted = false;
  let gameState: IGameState | null = null;
  let gameEnded = false;

  let promptUsername = true;
  let newUsername = "";

  function connect() {
    disconnect();

    const { location } = window;

    const proto = location.protocol.startsWith("https") ? "wss" : "ws";
    const wsUri = `${proto}://${serverDomain}/ws/${$page.params.id}?session=${$sessionID}&username=${newUsername}`;

    console.log("Connecting...");
    socket = new WebSocket(wsUri);

    socket.onopen = () => {
      console.log("Connected");
      if (!$sessionID) socket?.send("GET_ID");
      checkPlayerExists();
      getRoomData();
    };

    socket.onmessage = (ev) => {
      // console.log(ev.data);
      let data: IServerMsg = JSON.parse(ev.data);
      console.log(data);

      let category = Object.keys(data.msg_type)[0];
      let type = data.msg_type[category];
      console.log(category, type);
      if (category === "Relay") {
        if (type === "Draw") {
          drawMsg = data.content;
        }
      } else if (category === "Data" && type === "UserID") {
        console.log(data.content);
        sessionID.set(data.content);
        sessionStorage.setItem("session", data.content);
        console.log("get player data");
      } else if (category === "Game") {
        if (type === "PlayerJoined" || type === "PlayerLeft") {
          gameMsgs = [...gameMsgs, data.content];
        } else if (type === "GuessResult") {
          textMsgs = [...textMsgs, data.content];
        } else if (type === "NewTurn") {
          newRound(data.content);
        } else if (type === "EndGame") {
          console.log("Game is over");
          roomData = { ...JSON.parse(data.content) };
          players = roomData!.players;
          gameEnded = true;
        }
      }
    };

    socket.onclose = (event) => {
      console.log("Disconnected");
      console.log(event.reason);
      socket = null;
    };

    socket.onerror = (err) => {
      console.log(err);
    };
  }

  function disconnect() {
    if (socket) {
      console.log("Disconnecting...");
      socket.close();
      socket = null;
    }
  }

  const getRoomData = async () => {
    const res = await fetch(
      "http://" + serverDomain + "/details/" + $page.params.id,
      {
        method: "POST",
      }
    );
    roomData = await res.json();
    console.log(roomData);
    gameState = roomData!.state;
    gameStarted = roomData?.status === "STARTED";
  };

  const newRound = async (content: string) => {
    let roomData = { ...JSON.parse(content) };
    players = roomData.players;
    console.log(roomData);
    if (roomData) {
      gameState = roomData.state;
      gameStarted = true;
    }
  };

  const checkPlayerExists = async () => {
    if (!$sessionID) return;
    const res = await fetch(
      `http://${serverDomain}/room/${$page.params.id}/player/${$sessionID}`,
      {
        method: "POST",
      }
    );
    let data: PlayerData = await res.json();
    if (data) {
      userdata.set(data.player);
      console.log("Player already exists", data);
      sessionID.set(data.user_id);
      sessionStorage.setItem("session", data.user_id);
      promptUsername = false;
    }
  };

  onMount(async () => {
    sessionID.set(sessionStorage.getItem("session"));
    await checkPlayerExists();
    if (!promptUsername) {
      connect();
    }
  });

  onDestroy(() => {
    disconnect();
  });
</script>

{#if roomData && !gameEnded}
  {#if gameStarted}
    <PlayingPage
      {socket}
      {textMsgs}
      {drawMsg}
      {gameState}
      roomId={roomData.room_id}
      {players}
    />
  {:else}
    <WaitingPage
      {socket}
      {gameMsgs}
      bind:players
      roomId={roomData.room_id}
      owner={roomData.owner}
    />
  {/if}
{/if}

{#if promptUsername}
  <UsernamePrompt bind:newUsername {connect} />
{/if}

{#if gameEnded}
  <ResultsPage {players} />
{/if}

<style>
  :global(body) {
    background: lightblue;
  }
</style>
