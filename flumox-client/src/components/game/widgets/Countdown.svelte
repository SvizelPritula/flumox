<script lang="ts">
  import type { CountdownView } from "../../../lib/view";
  import Details from "../../Details.svelte";
  import Timer from "../../Timer.svelte";

  export let view: CountdownView;
  export let obsolete: boolean;
</script>

<Details name={view.name} open={!obsolete}>
  {#each view.details as paragraph}
    <p>{paragraph}</p>
  {/each}

  <div class="timer" role="timer" aria-atomic="true" aria-live="polite">
    {#if view.value.type == "done"}
      {view.value.text}
    {:else}
      <Timer time={view.value.type == "time" ? view.value.time : null} />
    {/if}
  </div>
</Details>

<style>
  .timer {
    font-size: 2rem;
    text-align: center;
  }
</style>
