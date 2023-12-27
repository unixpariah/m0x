<script lang="ts">
  import Buffer from "./TransactionManager/Buffer.svelte";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  interface CachedBalances {
    [key: string]: string;
  }

  interface Wallet {
    name: string;
    address: string;
    key: string;
  }

  const cachedBalances: CachedBalances = {};

  let openWallets: Wallet[] = [];
  let walletItemWidth = "";
  let currentWallet: Wallet;
  let balance = "0";

  onMount(async () => {
    openWallets = await invoke("read_opened_wallets");
    openWallets.forEach((wallet) => {
      cachedBalances[wallet.address] = "0";
    });
    updateWalletItemWidth();
    currentWallet = openWallets[0];
    await fetchAndStoreBalance();

    await listen("update_wallet_list", (event: { payload: Wallet[] }) => {
      openWallets = event.payload;
      updateWalletItemWidth();
    });
  });

  const fetchAndStoreBalance = async () => {
    balance = cachedBalances[currentWallet.address];

    const rawBalance =
      ((await invoke("get_balance", { wallet: currentWallet })) as number) /
      10 ** 18;
    const formattedBalance = rawBalance.toString().slice(0, -13);

    if (rawBalance > 0) {
      balance = formattedBalance;
      cachedBalances[currentWallet.address] = formattedBalance;
      return;
    }

    balance = "0";
    cachedBalances[currentWallet.address] = "0";
  };

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

  $: {
    if (currentWallet) {
      (async () => {
        await fetchAndStoreBalance();
      })();
    }
  }
</script>

<p>{currentWallet?.name}</p>
<p>{currentWallet?.address}</p>
<p>{balance} ETH</p>
<Buffer
  on:selectedWalletIndex={updateIndex}
  {openWallets}
  {walletItemWidth}
  {updateWalletItemWidth}
/>

{#if openWallets.length === 0}
  <h2>No open wallets</h2>
{/if}

