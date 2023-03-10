<script lang="ts">
  export let newUsername: string;
  export let connect: () => void;
</script>

<div class="promptPage">
  <div class="menu">
    <h1>Enter a username</h1>
    <input type="text" bind:value={newUsername} placeholder="Eg: Alex" />
    <button on:click={connect} disabled={newUsername.length === 0}
      >Continue</button
    >
  </div>
</div>

<style>
  .promptPage {
    height: 100svh;

    display: flex;
    justify-content: center;
    align-items: center;
  }

  .menu {
    padding: 1rem;
    border-radius: 0.5rem;
    background: white;

    display: flex;
    flex-direction: column;
    gap: 1rem;
    justify-content: center;
  }

  input {
    outline: none;
    border: 2px solid lightblue;
    border-radius: 2rem;
    padding: 0.5rem 1rem;
    font-size: 1.2rem;
  }

  input:focus {
    border-color: blue;
  }

  button:disabled {
    background: lightslategray;
    cursor: not-allowed;
  }

  button {
    width: 100%;
    padding: 0.5rem;
    color: white;
    background: blue;
    border-radius: 0.5rem;

    font-size: 1rem;
    font-weight: bold;

    cursor: pointer;

    transition: 0.2s;
  }
</style>
