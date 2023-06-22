<script lang="ts">
  import type { Views } from "../../lib/view";
  import View from "./View.svelte";

  export let views: Views;
</script>

{#each views as view (view.id)}
  <View {view} />
{/each}
