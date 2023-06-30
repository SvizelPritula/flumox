<script lang="ts">
  import { logout, type TeamInfo } from "../lib/team";
  import { button } from "../styles/forms.module.css";
  import { settingsLoggedIn, settingsLogout } from "$translations";

  export let team: TeamInfo;
</script>

<p>
  {settingsLoggedIn} <b>{team.name}</b>.
</p>

<form on:submit={logout}>
  <button type="submit" class={button}>{settingsLogout}</button>
</form>
