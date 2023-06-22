<script lang="ts">
  import { session } from "../stores";
  import Main from "./Main.svelte";
  import Login from "./Login.svelte";
</script>

{#if $session != null}
  <Main team={$session.team} />
{:else}
  <Login />
{/if}
