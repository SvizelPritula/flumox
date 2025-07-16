<script lang="ts">
  import type { TextView } from "../../../lib/view";
  import Details from "../../Details.svelte";

  export let view: TextView;
  export let obsolete: boolean;
</script>

<Details name={view.heading} open={!obsolete}>
  {#each view.content as paragraph}
    <p>{paragraph}</p>
  {/each}
</Details>
