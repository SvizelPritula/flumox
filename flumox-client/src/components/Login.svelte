<script lang="ts">
  import { login } from "../lib/api/session";
  import { toast } from "../lib/toast";
  import { session } from "../stores";
  import Toasts from "./Toasts.svelte";
  import { button, input, label } from "../styles/forms.module.css";
  import {
    loginHeading,
    loginAccessCode,
    loginButton,
    loginIncorrect,
    loginSuccess,
  } from "$translations";

  let code = "";
  let inFlight = false;

  async function submit() {
    inFlight = true;

    try {
      let result = await login(code);

      if (result.result == "success") {
        toast(`${loginSuccess} ${result.team.name}.`, "success");
        $session = { team: result.team, token: result.token };
      } else {
        toast(loginIncorrect, "danger");
        code = "";
      }
    } catch (error) {
      toast(String(error), "danger");
    } finally {
      inFlight = false;
    }
  }
</script>

<main>
  <Toasts />

  <form on:submit|preventDefault={submit}>
    <h1>{loginHeading}</h1>

    <label class={label}>
      <div>{loginAccessCode}</div>
      <input
        bind:value={code}
        type="password"
        autocomplete="current-password"
        disabled={inFlight}
        class={input}
      />
    </label>

    <button type="submit" disabled={inFlight} class={button}>
      {loginButton}
    </button>
  </form>
</main>

<style>
  main {
    min-height: 100%;
    display: grid;
    grid-template-rows: 1fr min-content 1.2fr;
  }

  form {
    width: 100%;
    max-width: 25rem;
    margin: auto;
    padding: 1.5rem;
  }

  h1 {
    text-align: center;
    font-size: 2.2rem;
  }
</style>
