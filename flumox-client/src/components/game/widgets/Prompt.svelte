<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { PromptView } from "../../../lib/view";
  import { button, input, label } from "../../../styles/forms.module.css";
  import type { Action } from "../../../lib/action";
  import Hint from "./Hint.svelte";

  export let view: PromptView;
  export let id: string;
  export let disabled: boolean;

  let answer = "";

  $: formDisabled = disabled || view.disabled;

  const dispatch = createEventDispatcher<{ action: Action }>();

  function submit() {
    dispatch("action", {
      widget: id,
      type: "answer",
      answer,
    });

    answer = "";
  }
</script>

<h2>{view.name}</h2>

{#each view.details as detail}
  <p>{detail}</p>
{/each}

<form on:submit|preventDefault={submit}>
  <label class={label}>
    <div>{view.prompt}</div>

    {#if view.solution == null}
      <input
        bind:value={answer}
        type="text"
        autocomplete="off"
        disabled={formDisabled}
        class={input}
      />
    {:else}
      <input
        value={view.solution}
        type="text"
        autocomplete="off"
        disabled={true}
        class={input}
      />
    {/if}
  </label>

  <button type="submit" disabled={formDisabled} class={button}>
    {view.submit_button}
  </button>
</form>

{#each view.hints as hint}
  <Hint {hint} disabled={formDisabled} widget={id} on:action />
{/each}
