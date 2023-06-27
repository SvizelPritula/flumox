<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { PromptView } from "../../../lib/view";
  import { button, input, label } from "../../../styles/forms.module.css";
  import type { Action } from "../../../lib/action";

  export let view: PromptView;
  export let id: string;
  let answer = "";

  $: disabled = view.disabled;

  const dispatch = createEventDispatcher<{ action: Action }>();

  function submit() {
    dispatch("action", {
      widget: id,
      type: "answer",
      answer,
    });
  }
</script>

<h2>{view.name}</h2>

{#each view.details as detail}
  <p>{detail}</p>
{/each}

<form on:submit|preventDefault={submit}>
  <label class={label}>
    <div>{view.prompt}</div>

    <input
      bind:value={answer}
      type="text"
      autocomplete="off"
      {disabled}
      class={input}
    />
  </label>

  <button type="submit" {disabled} class={button}>Submit</button>
</form>

<style>
  h2 {
    margin: 0;
    margin-bottom: 1rem;
  }
</style>
