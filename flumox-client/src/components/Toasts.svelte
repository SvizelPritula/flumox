<script lang="ts">
  import { dismiss, type Toast } from "../lib/toast";
  import { toasts } from "../stores";
  import { toastClose } from "$translations";
  import Cross from "./icons/Cross.svelte";

  export let permanent: Toast[] = [];

  $: all = <[Toast, boolean][]>[
    ...permanent.map((t) => [t, false]),
    ...$toasts.map((t) => [t, true]),
  ];
</script>

<div class="toasts">
  {#each all as [{ text, type, key }, dismissable] (key)}
    <div class="toast {type}" role="alert">
      <div class="text">{text}</div>
      {#if dismissable}
        <button
          class="dismiss"
          aria-label={toastClose}
          on:click={() => dismiss(key)}
        >
          <Cross />
        </button>
      {/if}
    </div>
  {/each}
</div>

<style>
  .toasts,
  .toast {
    width: 100%;
  }

  .toasts {
    position: sticky;
    min-height: 2rem;
    top: 0;
  }

  .toast {
    min-height: 2rem;
    padding: 0.4rem 1rem;
    display: flex;
    flex-direction: row;
    align-items: center;
  }

  .toast.danger {
    background-color: hsl(0, 100%, 30%);
  }

  .toast.warning {
    background-color: hsl(35, 100%, 30%);
  }

  .toast.success {
    background-color: hsl(140, 100%, 30%);
  }

  .toast.status {
    background-color: hsl(245, 100%, 30%);
    text-align: center;
  }

  .text {
    flex-grow: 1;
  }

  .dismiss {
    background-color: transparent;
    border: none;
    border-radius: 0.2rem;
    width: 1rem;
    height: 1rem;
    padding: 0;
    color: hsl(0, 0%, 80%);
  }

  .dismiss:hover {
    color: hsl(0, 0%, 100%);
  }

  .dismiss:focus-within {
    outline: 0.2rem solid white;
    outline-offset: 0.1rem;
  }

  .icon {
    width: 100%;
    height: 100%;
  }
</style>
