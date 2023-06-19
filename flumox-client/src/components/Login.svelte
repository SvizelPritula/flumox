<script lang="ts">
  import { login } from "../lib/api/session";
  import type { Toaster } from "../lib/toast";
  import { session } from "../stores";
  import Toasts from "./Toasts.svelte";
  import { button, input, label } from "../styles/forms.module.css";

  let code = "";
  let inFlight = false;

  let toast: Toaster;

  async function submit() {
    inFlight = true;

    try {
      let result = await login(code);

      if (result.result == "success") {
        $session = { team: result.team, token: result.token };
      } else {
        toast("Incorrect access code", "danger");
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
  <Toasts bind:toast />

  <form on:submit|preventDefault={submit}>
    <h1>Login</h1>

    <label class={label}>
      <div>Access code</div>
      <input
        bind:value={code}
        type="password"
        autocomplete="current-password"
        disabled={inFlight}
        class={input}
      />
    </label>

    <button type="submit" disabled={inFlight} class={button}>Submit</button>
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
