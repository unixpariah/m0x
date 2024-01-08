<script lang="ts">
  import Buffer from "./TransactionManager/Buffer.svelte";
  import ChainData from "./TransactionManager/ChainData.svelte";
  import ChooseProvider from "./TransactionManager/ChooseProvider.svelte";
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
    cachedBalances[openWallets[0]?.address] = "0";
    updateWalletItemWidth();
    currentWallet = openWallets[0];
    await fetchAndStoreBalance(currentWallet);

    await listen("update_wallet_list", (event: { payload: Wallet[] }) => {
      openWallets = event.payload;
      openWallets.forEach((wallet) => {
        cachedBalances[wallet.address] = "0";
      });
      updateWalletItemWidth();
    });
  });

  const fetchAndStoreBalance = async (wallet: Wallet) => {
    balance = cachedBalances[wallet.address];

    const rawBalance =
      ((await invoke("get_balance", { wallet })) as number) / 10 ** 18;
    const formattedBalance = rawBalance.toString().slice(0, -13);

    if (rawBalance > 0) {
      cachedBalances[wallet.address] = formattedBalance;
      if (currentWallet === wallet) {
        balance = formattedBalance;
      }
      return;
    }

    cachedBalances[wallet.address] = "0";
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
        await fetchAndStoreBalance(currentWallet);
      })();
    }
  }
</script>

<div>
  <p>{currentWallet?.name}</p>
  <p>{currentWallet?.address}</p>
  <p>{balance || 0} ETH</p>
</div>
<ChainData />
<ChooseProvider />
<Buffer
  on:selectedWalletIndex={updateIndex}
  {openWallets}
  {walletItemWidth}
  {updateWalletItemWidth}
/>

<style>
  p {
    color: white;
  }

  div {
    float: left;
  }
</style>

