<script lang="ts">
  import type { View } from "../../lib/view";
  import Prompt from "./widgets/Prompt.svelte";

  export let view: View;
</script>

<div>
  {#if view.type == "prompt"}
    <Prompt view={view.view} />
  {:else}
    Unknown view
  {/if}
</div>
