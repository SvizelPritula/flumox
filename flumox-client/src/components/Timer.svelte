<script lang="ts">
  import { onMount } from "svelte";

  export let time: string | null;

  let callback: number | null = null;

  let indeterminate: boolean = true;
  let hours: number = 0;
  let minutes: number = 0;
  let seconds: number = 0;

  function update() {
    indeterminate = time == null;
    if (indeterminate) return;

    let target = Date.parse(time);
    let now = Date.now();

    let value = Math.max(target - now, 0);
    value = Math.floor(value / 1000);

    seconds = value % 60;
    value = Math.floor(value / 60);

    minutes = value % 60;
    value = Math.floor(value / 60);

    hours = value;
  }

  function frame() {
    update();

    callback = requestAnimationFrame(frame);
  }

  onMount(() => {
    callback = requestAnimationFrame(frame);
    return () => cancelAnimationFrame(callback);
  });

  function pad(number: number): string {
    return number.toString().padStart(2, "0");
  }
</script>

{#if indeterminate}
  <b>--</b>&thinsp;:&thinsp;<b>--</b>
{:else if hours == 0}
  <b>{pad(minutes)}</b>&thinsp;:&thinsp;<b>{pad(seconds)}</b>
{:else}
  <b>
    {hours}
  </b>&thinsp;:&thinsp;<b>
    {pad(minutes)}
  </b>&thinsp;:&thinsp;<b>
    {pad(seconds)}
  </b>
{/if}

<style>
  b {
    font-variant-numeric: tabular-nums;
  }
</style>
