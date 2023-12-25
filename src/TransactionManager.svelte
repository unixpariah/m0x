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

<Buffer {openWallets} {walletWidth} />

{#if openWallets.length === 0}
  <h2>no open wallets</h2>
{/if}

