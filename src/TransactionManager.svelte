<script lang="ts">
  import Buffer from "./TransactionManager/Buffer.svelte";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  interface Wallet {
    name: string;
    address: string;
    key: string;
  }

  let openWallets: Wallet[] = [];
  let walletItemWidth = "";
  let currentWallet: Wallet;

  onMount(async () => {
    openWallets = await invoke("read_opened_wallets");
    updateWalletItemWidth();
    currentWallet = openWallets[0];

    await listen("update_wallet_list", (event: { payload: Wallet[] }) => {
      openWallets = event.payload;
      updateWalletItemWidth();
    });
  });

  const calculateWalletItemWidth = () => {
    return `${
      (window.innerWidth - 0.1 * openWallets.length) / openWallets.length
    }px`;
  };

  const updateWalletItemWidth = () => {
    walletItemWidth = calculateWalletItemWidth();
  };

  const updateIndex = (event: { detail: number }) => {
    currentWallet = openWallets[event.detail];
  };

  updateWalletItemWidth();
</script>

<p>{currentWallet?.name}</p>
<p>{currentWallet?.address}</p>
<Buffer
  on:selectedWalletIndex={updateIndex}
  {openWallets}
  {walletItemWidth}
  {updateWalletItemWidth}
/>

{#if openWallets.length === 0}
  <h2>No open wallets</h2>
{/if}

