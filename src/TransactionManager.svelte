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

  onMount(async () => {
    openWallets = await invoke("read_opened_wallets");
    await listen("update_wallet_list", (event: { payload: Wallet[] }) => {
      openWallets = event.payload;
    });
  });
</script>

{#each openWallets as item}
  <p>{item.address}</p>
{/each}
