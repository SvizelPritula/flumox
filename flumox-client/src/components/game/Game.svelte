<script lang="ts">
  import type { Instances } from "../../lib/view";
  import Widget from "./Widget.svelte";

  export let views: Instances;
  export let disabled: boolean;
</script>

{#each views as view (view.id)}
  <Widget {view} {disabled} on:action />
{/each}
