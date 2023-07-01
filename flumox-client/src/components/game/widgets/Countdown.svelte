<script lang="ts">
  import type { CountdownView } from "../../../lib/view";
  import Timer from "../../Timer.svelte";

  export let view: CountdownView;
</script>

{#if view.name != null}
  <h2>{view.name}</h2>
{/if}

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

<style>
  .timer {
    font-size: 2rem;
    text-align: center;
  }
</style>
