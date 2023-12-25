<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  interface Wallet {
    name: string;
    address: string;
    key: string;
  }

  export let openWallets: Wallet[];
  export let walletWidth: string;
  let chosenWallet = 0;

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
