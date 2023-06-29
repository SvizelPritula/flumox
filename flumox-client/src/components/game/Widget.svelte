<script lang="ts">
  import type { View } from "../../lib/view";
  import Prompt from "./widgets/Prompt.svelte";

  export let view: View;
  export let disabled: boolean;
</script>

<div>
  {#if view.type == "prompt"}
    <Prompt view={view.view} id={view.id} {disabled} on:action />
  {:else}
    Unknown view
  {/if}
</div>

<style>
  div {
    background-color: hsl(0, 0%, 10%);
    border-radius: 1rem;
    padding: 2rem;
    margin: 1rem 0;
  }
</style>
