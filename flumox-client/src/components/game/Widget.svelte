<script lang="ts">
  import type { Instance } from "../../lib/view";
  import Prompt from "./widgets/Prompt.svelte";
  import { unknownView } from "$translations";
  import Text from "./widgets/Text.svelte";
  import Countdown from "./widgets/Countdown.svelte";

  export let view: Instance;
  export let disabled: boolean;
</script>

<div class:obsolete={view.obsolete}>
  {#if view.view.type == "prompt"}
    <Prompt
      view={view.view}
      id={view.id}
      {disabled}
      obsolete={view.obsolete}
      on:action
    />
  {:else if view.view.type == "text"}
    <Text view={view.view} obsolete={view.obsolete} />
  {:else if view.view.type == "countdown"}
    <Countdown view={view.view} obsolete={view.obsolete} />
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

    overflow-wrap: anywhere;
  }

  div > :global(h2) {
    margin-bottom: 1rem;
  }

  div > :global(*):first-child {
    margin-top: 0;
  }

  div > :global(*):last-child {
    margin-bottom: 0;
  }

  .obsolete {
    color: hsl(0, 0%, 40%);
    background-color: hsl(0, 0%, 7.5%);
  }
</style>
