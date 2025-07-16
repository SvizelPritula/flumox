<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { Hint } from "../../../lib/view";
  import { button } from "../../../styles/forms.module.css";
  import type { Action } from "../../../lib/action";
  import Timer from "../../Timer.svelte";
  import { takeHintButton } from "$translations";

  export let hint: Hint;
  export let widget: string;
  export let disabled: boolean;

  const dispatch = createEventDispatcher<{ action: Action }>();

  function take() {
    dispatch("action", {
      widget,
      type: "hint",
      ident: hint.ident,
    });
  }
</script>

<h3>{hint.name}</h3>

{#if hint.state == "taken"}
  {#each hint.content as paragraph}
    <p>{paragraph}</p>
  {/each}
{:else if hint.state == "available"}
  <form on:submit|preventDefault={take}>
    <button type="submit" {disabled} class={button}>
      {hint.button ?? takeHintButton}
    </button>
  </form>
{:else}
  <div class="timer" role="timer" aria-atomic="true" aria-live="polite">
    <Timer time={hint.state == "future" ? hint.time : null} />
  </div>
{/if}

<style>
  .timer {
    font-size: 1.5rem;
    text-align: center;
  }
</style>
