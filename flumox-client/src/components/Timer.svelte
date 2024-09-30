<script lang="ts">
  import { onMount } from "svelte";
  import Time from "./Time.svelte";

  export let time: string | null;
  export let direction: "up" | "down" = "down";

  let callback: number | null = null;
  let duration: number | null = null;

  function update() {
    if (time == null) {
      duration = null;
      return;
    }

    let target = Date.parse(time);
    let now = Date.now();

    if (direction == "down") duration = Math.max(target - now, 0);
    else if (direction == "up") duration = Math.max(now - target, 0);
  }

  function frame() {
    update();
    callback = requestAnimationFrame(frame);
  }

  onMount(() => {
    callback = requestAnimationFrame(frame);
    return () => cancelAnimationFrame(callback);
  });
</script>

<Time {duration} />
