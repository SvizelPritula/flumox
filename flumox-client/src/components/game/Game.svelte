<script lang="ts">
  import type { Views } from "../../lib/view";
  import Widget from "./Widget.svelte";

  export let views: Views;
  export let disabled: boolean;
</script>

{#each views as view (view.id)}
  <Widget {view} {disabled} on:action />
{/each}
