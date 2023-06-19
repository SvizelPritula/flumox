<script lang="ts">
  import { session } from "../stores";
  import Game from "./Game.svelte";
  import Login from "./Login.svelte";
</script>

{#if $session != null}
  <Game team={$session.team} />
{:else}
  <Login />
{/if}
