<script lang="ts">
  import { login } from "../lib/api/session";
  import { session } from "../stores";

  let code = "";

  function submit() {
    login(code)
      .then((result) => {
        if (result.result == "success") {
          $session = {
            token: result.token,
            team: result.team,
          };
        } else {
          console.log("Failed to log in.");
        }
      })
      .catch((error) => console.error(error));
  }
</script>

<main>
  <form on:submit|preventDefault={submit}>
    <input bind:value={code} type="password" />
    <button type="submit">Submit</button>
  </form>

  <pre>{JSON.stringify($session, null, 4)}</pre>
</main>
