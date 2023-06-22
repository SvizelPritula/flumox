<script lang="ts">
  import { onMount } from "svelte";
  import type { TeamInfo } from "../lib/team";
  import { session } from "../stores";
  import Toasts from "./Toasts.svelte";
  import type { Views } from "../lib/view";
  import Game from "./game/Game.svelte";

  export let team: TeamInfo;
  let view: Promise<Views> = new Promise(() => {});

  onMount(() => {
    view = fetch("/api/view", {
      headers: {
        "x-auth-token": $session.token,
      },
    }).then((result) => result.json());
  });
</script>

<header>
  {team.game.name} <button on:click={() => ($session = null)}>Log out</button>
</header>

<Toasts />

<main>
  {#await view}
    <p>Loading...</p>
  {:then views}
    <Game {views} />
  {:catch}
    <p>Failed to load</p>
  {/await}
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
