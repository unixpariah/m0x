<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  interface Wallet {
    name: string;
    address: string;
    key: string;
  }

  let openWallets: Wallet[] = [];
  let walletWidth = "";
  let chosenWallet = 0;

  onMount(async () => {
    openWallets = await invoke("read_opened_wallets");
    walletWidth = `${
      (window.innerWidth - 12 * openWallets.length) / openWallets.length
    }px`;
    await listen("update_wallet_list", (event: { payload: Wallet[] }) => {
      openWallets = event.payload;
      walletWidth = `${
        (window.innerWidth - 12 * openWallets.length) / openWallets.length
      }px`;
    });
  });

  const handleKeydown = (e: { key: string }) => {
    switch (e.key) {
      case "H":
        if (chosenWallet <= 0) {
          chosenWallet = openWallets.length - 1;
          return;
        }
        chosenWallet--;
        break;
      case "L":
        if (chosenWallet >= openWallets.length - 1) {
          chosenWallet = 0;
          return;
        }
        chosenWallet++;
        break;
      case "d":
        invoke("close_wallet", { index: chosenWallet });
        if (chosenWallet >= openWallets.length - 1) {
          chosenWallet = openWallets.length - 2;
        }
        break;
      case "D":
        invoke("close_wallet");
      default:
        break;
    }
  };
</script>

<svelte:window on:keydown={handleKeydown} />

<div>
  {#each openWallets as wallet}
    {#if openWallets[chosenWallet].address === wallet.address}
      <p style="width: {walletWidth}; border-top: 1px solid transparent;">
        {wallet.address.slice(0, 5)}
      </p>
    {:else}
      <p style="width: {walletWidth};">{wallet.address.slice(0, 5)}</p>
    {/if}
  {/each}
</div>

{#if openWallets.length === 0}
  <h2>no open wallets</h2>
{/if}

<style>
  p {
    color: white;
    font-size: 0.6rem;
    float: left;
    border: 1px solid white;
    padding: 5px;
    margin: 0;
  }

  div {
    position: absolute;
    bottom: 0;
  }
</style>
