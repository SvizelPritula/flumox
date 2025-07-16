<script lang="ts">
  export let name: string | null;
  export let open: boolean;
</script>

{#if name == null}
  <slot />
{:else}
  <details {open}>
    <summary><h2>{name}</h2></summary>

    <slot />
  </details>
{/if}

<style>
  summary {
    font-size: 1.5rem;
  }

  h2 {
    display: inline;
    font-size: inherit;
  }
</style>
