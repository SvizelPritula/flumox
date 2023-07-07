<script lang="ts">
  import type { TeamInfo } from "../lib/team";
  import { online, session, view } from "../stores";
  import Toasts from "./Toasts.svelte";
  import Game from "./game/Game.svelte";
  import { type Action, submit } from "../lib/action";
  import { toast, type Toast } from "../lib/toast";
  import { onMount } from "svelte";
  import { sync } from "../lib/api/sync";
  import { getErrorMessage } from "../lib/error";
  import Settings from "./Settings.svelte";
  import Cross from "./icons/Cross.svelte";
  import {
    appName,
    settingsName,
    settingsOpen,
    settingsClose,
    loadingOnline,
    loadingOffline,
    statusOffline,
  } from "$translations";
  import Gear from "./icons/Gear.svelte";

  export let team: TeamInfo;
  let inFlight = false;
  let settingsActive = false;

  onMount(() => {
    return sync(view, online, $session.token);
  });

  async function action(payload: Action) {
    inFlight = true;

    try {
      await submit(payload, $session.token);
    } catch (error) {
      toast(getErrorMessage(error), "danger");
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

  $: screenName = !settingsActive ? team.game.name : settingsName;
</script>

<svelte:head>
  <title>{screenName} | {appName}</title>
</svelte:head>

<div>
  <header>
    <h1 class="name">{screenName}</h1>
    <button
      class="settings"
      aria-label={settingsActive ? settingsClose : settingsOpen}
      on:click={() => (settingsActive = !settingsActive)}
    >
      {#if !settingsActive}
        <Gear />
      {:else}
        <Cross />
      {/if}
    </button>
  </header>

  <Toasts permanent={toasts} />

  <main>
    {#if !settingsActive}
      {#if $view != null}
        <Game
          views={$view}
          on:action={(e) => action(e.detail)}
          disabled={inFlight}
        />
      {:else}
        {$online ? loadingOnline : loadingOffline}
      {/if}
    {:else}
      <Settings {team} />
    {/if}
  </main>
</div>

<style>
  header,
  main {
    width: 100%;
  }

  main {
    padding: 0 2rem 2rem;
  }

  header {
    min-height: 3rem;
    background-color: hsl(0, 0%, 15%);
    padding: 0.5rem 2rem;
    font-size: 1.5rem;
    display: flex;
    flex-direction: row;
    align-items: center;
  }

  h1 {
    font-size: unset;
    margin: unset;
    font-weight: bold;
    flex-grow: 1;
  }

  .settings {
    background-color: transparent;
    border: none;
    border-radius: 0.5rem;
    width: 1.5rem;
    height: 1.5rem;
    padding: 0;
    color: hsl(0, 0%, 80%);
  }

  .settings:hover {
    color: hsl(0, 0%, 100%);
  }

  .settings:focus-within {
    outline: 0.2rem solid white;
    outline-offset: 0.1rem;
  }
</style>
