<script lang="ts">
  import type { Views } from "../../lib/view";
  import Widget from "./Widget.svelte";

  export let views: Views;
</script>

{#each views as view (view.id)}
  <Widget {view} />
{/each}
