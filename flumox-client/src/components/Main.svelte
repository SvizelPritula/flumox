<script lang="ts">
  import type { TeamInfo } from "../lib/team";
  import { online, session, view } from "../stores";
  import Toasts from "./Toasts.svelte";
  import Game from "./game/Game.svelte";
  import { type Action, submit } from "../lib/action";
  import { toast, type Toast } from "../lib/toast";
  import { onMount } from "svelte";
  import { sync } from "../lib/api/sync";
  import { loadingOnline, loadingOffline, statusOffline } from "$translations";
  import { appName } from "$translations";

  export let team: TeamInfo;
  let inFlight = false;

  onMount(() => {
    return sync(view, online, $session.token);
  });

  async function action(payload: Action) {
    inFlight = true;

    try {
      await submit(payload, $session.token);
    } catch (error) {
      toast(String(error), "danger");
    } finally {
      inFlight = false;
    }
  }

  $: toasts = <Toast[]>[
    !$online && {
      key: "offline",
      text: statusOffline,
      type: "status",
    },
  ].filter(Boolean);
</script>

<svelte:head>
  <title>{team.game.name} | {appName}</title>
</svelte:head>

<div>
  <header>
    {team.game.name} <button on:click={() => ($session = null)}>Log out</button>
  </header>

  <Toasts permanent={toasts} />

  <main>
    {#if $view != null}
      <Game
        views={$view}
        on:action={(e) => action(e.detail)}
        disabled={inFlight}
      />
    {:else}
      {$online ? loadingOnline : loadingOffline}
    {/if}
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
