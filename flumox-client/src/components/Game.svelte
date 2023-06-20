<script lang="ts">
  import type { TeamInfo } from "../lib/team";
  import { session } from "../stores";
  import Toasts from "./Toasts.svelte";

  export let team: TeamInfo;
</script>

<header>
  {team.gameName}
</header>

<Toasts />

<main>
  <p>
    You are <b>{team.name}</b>.
  </p>
  <button on:click={() => ($session = null)}>Log out</button>
</main>

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
