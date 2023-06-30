<script lang="ts">
  import type { Instance } from "../../lib/view";
  import Prompt from "./widgets/Prompt.svelte";
  import { unknownView } from "$translations";

  export let view: Instance;
  export let disabled: boolean;
</script>

<div>
  {#if view.view.type == "prompt"}
    <Prompt view={view.view} id={view.id} {disabled} on:action />
  {:else}
    {unknownView}
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
