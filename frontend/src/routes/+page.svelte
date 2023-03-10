<script lang="ts">
  import { clientDomain, serverDomain } from "$lib/domains";

  let roomId = "";

  const checkRoomId = async () => {
    const res = await fetch("http://" + serverDomain + "/details/" + roomId, {
      method: "POST",
    });
    const data = await res.json();
    console.log(data);
  };

  const getNewRoomId = async () => {
    const res = await fetch("http://" + serverDomain + "/room/new", {
      method: "POST",
    });
    const data = await res.text();
    console.log(data);

    window.location.href = `http://${clientDomain}/room/${data}`;
  };
</script>

<div class="page">
  <div>
    <h2>Join a room</h2>
    <input type="text" bind:value={roomId} />
    <button on:click={checkRoomId}>Enter</button>
  </div>

  <div>
    <button on:click={getNewRoomId}>New</button>
  </div>
</div>

<style>
  .page {
    width: 100vw;
    height: 100vh;

    display: flex;
    justify-content: center;
    align-items: center;
    gap: 1rem;
  }

  .page > div {
    width: 300px;
    aspect-ratio: 1;
    border-radius: 0.5rem;
    background: lightcoral;

    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }
</style>
