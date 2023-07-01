<script lang="ts">
  import type { TextView } from "../../../lib/view";

  export let view: TextView;
</script>

{#if view.heading != null}
  <h2>{view.heading}</h2>
{/if}

{#each view.content as paragraph}
  <p>{paragraph}</p>
{/each}
