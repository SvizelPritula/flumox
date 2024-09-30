<script lang="ts">
  export let duration: number | null;

  function split(value: number) {
    value = Math.max(duration, 0);
    value = Math.ceil(value / 1000);

    let seconds = value % 60;
    value = Math.floor(value / 60);

    let minutes = value % 60;
    value = Math.floor(value / 60);

    let hours = value;
    return { seconds, minutes, hours };
  }

  $: comps = duration != null ? split(duration) : null;

  function pad(number: number): string {
    return number.toString().padStart(2, "0");
  }
</script>

{#if comps == null}
  <b>--</b>&thinsp;:&thinsp;<b>--</b>
{:else if comps.hours == 0}
  <b>{pad(comps.minutes)}</b>&thinsp;:&thinsp;<b>{pad(comps.seconds)}</b>
{:else}
  <b>
    {comps.hours}
  </b>&thinsp;:&thinsp;<b>
    {pad(comps.minutes)}
  </b>&thinsp;:&thinsp;<b>
    {pad(comps.seconds)}
  </b>
{/if}

<style>
  b {
    font-variant-numeric: tabular-nums;
  }
</style>
