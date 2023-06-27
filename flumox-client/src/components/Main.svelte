<script lang="ts">
  import type { TeamInfo } from "../lib/team";
  import { session } from "../stores";
  import Toasts from "./Toasts.svelte";
  import Game from "./game/Game.svelte";
  import { view } from "../lib/api/game";

  export let team: TeamInfo;

  $: state = view($session.token);
</script>

<div>
  <header>
    {team.game.name} <button on:click={() => ($session = null)}>Log out</button>
  </header>

  <Toasts />

  <main>
    {#await state}
      <p>Loading...</p>
    {:then views}
      <Game {views} />
    {:catch}
      <p>Failed to load</p>
    {/await}
  </main>
</div>

<style>
  header,
  main {
    width: 100%;
  }

  header {
    min-height: 3rem;
    background-color: hsl(0, 0%, 15%);
    padding: 0.5rem 2rem;
    font-size: 1.5rem;
  }

  main {
    padding: 0 2rem;
  }
</style>
