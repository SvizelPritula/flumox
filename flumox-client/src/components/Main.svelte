<script lang="ts">
  import type { TeamInfo } from "../lib/team";
  import { session } from "../stores";
  import Toasts from "./Toasts.svelte";
  import Game from "./game/Game.svelte";
  import { submit, view } from "../lib/api/game";
  import type { Action } from "../lib/action";
  import { toast } from "../lib/toast";

  export let team: TeamInfo;
  let inFlight = false;

  $: state = view($session.token);

  async function action(payload: Action) {
    inFlight = true;

    try {
      let response = await submit($session.token, payload);

      if (response.result == "success") {
        if (response.toast != null) {
          toast(response.toast.text, response.toast.type);
        }

        state = view($session.token);
      } else if (response.result == "dispatch-failed") {
        toast(
          "Failed to process action due to game configuration being out of sync",
          "danger"
        );
      } else if (response.result == "not-possible") {
        toast(
          "Failed to process action due to game state being out of sync",
          "danger"
        );
      }
    } catch (error) {
      toast(String(error), "danger");
    } finally {
      inFlight = false;
    }
  }
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
      <Game {views} on:action={(e) => action(e.detail)} />
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
