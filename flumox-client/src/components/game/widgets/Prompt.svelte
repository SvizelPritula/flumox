<script lang="ts">
  import { toast } from "../../../lib/toast";
  import type { PromptView } from "../../../lib/view";
  import { button, input, label } from "../../../styles/forms.module.css";

  export let view: PromptView;

  $: disabled = view.disabled;
</script>

<h2>{view.name}</h2>

{#each view.details as detail}
  <p>{detail}</p>
{/each}

<form on:submit|preventDefault={() => toast("Hello!", "warning")}>
  <label class={label}>
    <div>{view.prompt}</div>

    <input
      type="text"
      autocomplete="off"
      disabled={disabled}
      class={input}
    />
  </label>

  <button type="submit" disabled={disabled} class={button}>Submit</button>
</form>

<style>
  h2 {
    margin: 0;
    margin-bottom: 1rem;
  }
</style>
