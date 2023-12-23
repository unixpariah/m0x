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
</script>

<div>
  {#each openWallets as wallet}
    <p style="width: {walletWidth};">{wallet.address.slice(0, 5)}</p>
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

