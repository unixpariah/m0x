<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import CreateWallets from "./WalletTray/CreateWallets.svelte";
  import Wallets from "./WalletTray/Wallets.svelte";
  import PasswordInput from "./WalletTray/PasswordInput.svelte";
  import SearchBar from "./WalletTray/SearchBar.svelte";

  interface Wallet {
    name: string;
    address: string;
    key: string;
  }

  let divHeight = 55;
  let isExpanded = false;
  let createPassword = false;
  let keyType: string;
  let password = "";
  let name = "";
  let length = 12;
  let wallets: Wallet[] = [];
  let query = "";

  const toggleDivHeight = () => {
    isExpanded = !isExpanded;
    divHeight = isExpanded ? 430 : 55;
  };

  const closePasswordWindow = async () => {
    createPassword = false;
    await invoke("generate_wallet", {
      keyType,
      length,
      password,
      name,
    });
    getWallets();
    name = "";
    password = "";
  };

  const getWallets = async () => {
    const loadedWallets: Wallet[] = await invoke("read_wallets");
    wallets = [];
    loadedWallets.forEach((wallet: Wallet) => {
      if (wallet.name.includes(query)) {
        wallets.push(wallet);
      }
    });
  };

  const createNewWallet = async (type: string) => {
    keyType = type;
    createPassword = true;
  };

  const setName = (event: Event) => {
    const target = event.target as HTMLInputElement;
    name = target.value;
  };

  const passwordCreator = (event: Event) => {
    const target = event.target as HTMLInputElement;
    password = target.value;
  };

  const search = (event: Event) => {
    const target = event.target as HTMLInputElement;
    query = target.value;
    getWallets();
  };

  getWallets();
</script>

<main class="container">
  <CreateWallets {createNewWallet} {divHeight} {isExpanded} {toggleDivHeight} />
  <SearchBar {query} {search} />
  <Wallets {wallets} />
  {#if createPassword}
    <PasswordInput
      {setName}
      {passwordCreator}
      {closePasswordWindow}
      {keyType}
    />
  {/if}
</main>

