<script lang="ts">
  import { getMsgType } from "$lib/helper";
  import { sessionID } from "$lib/stores";
  import type { IDrawMsg, IServerMsg } from "$lib/types/server";
  import { onMount } from "svelte";

  export let socket: WebSocket | null;
  export let drawMsg: string;
  export let currently_drawing: string | null;

  $: allowedToDraw = changeDrawer(currently_drawing);

  let canvasEle: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D | null;

  const SIZES = [5, 10, 15];
  const COLORS = ["black", "white", "red", "green", "blue"];

  let currentSize = 0;
  let currentColor = 0;

  let isDrawing = false;
  let isOtherDrawing = false;

  const changeDrawer = (currently_drawing: string | null) => {
    if (!currently_drawing) return;
    console.log("drawer changed");
    console.log(currently_drawing === $sessionID && $sessionID !== null);
    clearCanvas();
    return currently_drawing === $sessionID && $sessionID !== null;
  };

  const getMousePos = (event: MouseEvent) => {
    const rect = canvasEle.getBoundingClientRect();

    let scaleX = canvasEle.width / rect.width;
    let scaleY = canvasEle.height / rect.height;

    let x = (event.clientX - rect.left) * scaleX;
    let y = (event.clientY - rect.top) * scaleY;

    return { x, y };
  };

  const startDraw = (event: MouseEvent) => {
    if (!ctx || !allowedToDraw) return;
    isDrawing = true;

    ctx.beginPath();

    draw(event);
    let { x, y } = getMousePos(event);
    sendDrawData(x, y, true, false);
  };

  const draw = (event: MouseEvent) => {
    if (!isDrawing || !ctx || !allowedToDraw) return;

    let { x, y } = getMousePos(event);

    ctx.lineTo(x, y);
    ctx.stroke();

    // ctx.beginPath();
    // ctx.moveTo(x, y);

    sendDrawData(x, y, false, false);
  };

  const drawFromData = (data: IDrawMsg) => {
    if (!isOtherDrawing || !ctx) return;
    let { x, y, color, size } = data;

    ctx.lineWidth = SIZES[size];

    ctx.fillStyle = COLORS[color];
    ctx.strokeStyle = COLORS[color];

    ctx.lineTo(x, y);
    ctx.stroke();

    // ctx.beginPath();
    // ctx.moveTo(x, y);
  };

  const sendDrawData = (
    x: number,
    y: number,
    beginning: boolean,
    end: boolean
  ) => {
    if (!$sessionID || !socket) return;
    let data: IDrawMsg = {
      color: currentColor,
      size: currentSize,
      beginning,
      end,
      x,
      y,
    };

    let msg: IServerMsg = {
      msg_type: getMsgType("Relay", "Draw"),
      content: "*" + JSON.stringify(data),
    };

    socket.send(JSON.stringify(msg));
  };

  const sendClearBoard = () => {
    let msg: IServerMsg = {
      msg_type: getMsgType("Relay", "Draw"),
      content: "CLEAR",
    };

    socket?.send(JSON.stringify(msg));
  };

  const endDraw = () => {
    isDrawing = false;

    sendDrawData(0, 0, false, true);
  };

  const changePenSize = (index: number) => {
    if (!ctx) return;

    currentSize = index;

    ctx.lineWidth = SIZES[index];
  };

  const changePenColor = (index: number) => {
    if (!ctx) return;

    currentColor = index;

    ctx.fillStyle = COLORS[index];
    ctx.strokeStyle = COLORS[index];
  };

  const clearCanvas = () => {
    if (!ctx) return;
    ctx.clearRect(0, 0, canvasEle.width, canvasEle.height);
    sendClearBoard();
  };

  const handleDrawMsgs = (msg: string) => {
    if (msg.length === 0) return;
    if (msg === "CLEAR") {
      return ctx?.clearRect(0, 0, canvasEle.width, canvasEle.height);
    }
    let json: IDrawMsg = JSON.parse(msg.slice(1));

    if (json.beginning) {
      // ctx?.moveTo(json.x, json.y);
      ctx?.beginPath();
      isOtherDrawing = true;
    } else if (json.end) {
      isOtherDrawing = false;
    }
    drawFromData(json);
  };

  $: handleDrawMsgs(drawMsg);

  onMount(() => {
    ctx = canvasEle.getContext("2d");
    if (!ctx) return;
    ctx.imageSmoothingEnabled = true;
    ctx.lineCap = "round";
    changePenSize(0);
    changePenColor(0);
  });
</script>

<div class="container">
  {#if allowedToDraw}
    <p>You are drawing</p>
  {/if}
  <canvas
    width="600"
    height="600"
    bind:this={canvasEle}
    on:mousedown={startDraw}
    on:mousemove={draw}
    on:mouseup={endDraw}
  />
  {#if allowedToDraw}
    <div class="settings">
      <div class="colors">
        {#each COLORS as color, i}
          <button
            style="background: {color}"
            class:selected={currentColor === i}
            on:click={() => changePenColor(i)}
          />
        {/each}
      </div>

      <div class="sizes">
        {#each SIZES as size, i}
          <button
            style="width: {size}px"
            class="sizeBtn"
            class:selected={i === currentSize}
            on:click={() => changePenSize(i)}
          />
        {/each}
      </div>
      <button on:click={clearCanvas}>Clear</button>
    </div>
  {/if}
</div>

<style>
  .container {
    border: 1px solid black;
    width: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }

  canvas {
    border: 1px solid blue;
    width: 100%;
    max-width: 800px;
    aspect-ratio: 1;
  }

  .settings {
    padding: 1rem;
    display: flex;
    justify-content: space-between;
    gap: 2rem;
  }

  .settings .sizes {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 1rem;
  }

  .sizeBtn {
    border-radius: 50%;
    background: black;
    aspect-ratio: 1;

    position: relative;
  }

  .sizeBtn.selected::before {
    content: "";
    position: absolute;
    inset: -4px;
    border-radius: 50%;
    border: 2px solid black;
  }

  .settings .colors {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .colors > button {
    border-radius: 50%;
    width: 20px;
    aspect-ratio: 1;
    border: 2px solid white;
    position: relative;
  }

  .colors > button::before {
    content: "";
    position: absolute;
    inset: -4px;
    border-radius: 50%;
    border: 2px solid black;
  }

  .colors > button.selected {
    transform: scale(1.2);
  }
</style>
