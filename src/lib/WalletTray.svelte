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
  let createWallet = false;
  let importWallet = false;
  let createPassword = false;
  let keyType: string;
  let password = "";
  let name = "";
  let length = 12;
  let wallets: Wallet[] = [];
  let query = "";
  let key = "";

  const toggleDivHeight = () => {
    isExpanded = !isExpanded;
    divHeight = isExpanded ? 430 : 55;
  };

  const closePasswordWindow = async (isImport?: boolean) => {
    if (isImport) {
      importWallet = false;
      createPassword = false;
      await invoke("import_wallet", {
        keyType,
        password,
        key,
        name,
      });
      getWallets();
      return;
    }

    createWallet = false;
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
    createWallet = true;
  };

  const importNewWallet = async (type: string) => {
    keyType = type;
    importWallet = true;
  };

  const setName = (event: Event) => {
    const target = event.target as HTMLInputElement;
    name = target.value;
  };

  const passwordCreator = (event: Event) => {
    const target = event.target as HTMLInputElement;
    password = target.value;
  };

  const setKey = (event: Event) => {
    const target = event.target as HTMLInputElement;
    key = target.value;
  };

  const search = (event: Event) => {
    const target = event.target as HTMLInputElement;
    query = target.value;
    getWallets();
  };

  getWallets();
</script>

<main class="container">
  <CreateWallets
    {createNewWallet}
    {divHeight}
    {isExpanded}
    {toggleDivHeight}
    {importNewWallet}
  />
  <SearchBar {query} {search} />
  <Wallets {wallets} />
  {#if createWallet}
    <PasswordInput
      {setName}
      {passwordCreator}
      {closePasswordWindow}
      {keyType}
    />
  {/if}
  {#if importWallet}
    <input type="text" on:input={(e) => setKey(e)} />
    <button on:click={() => (createPassword = true)}></button>
    {#if createPassword}
      <PasswordInput
        {setName}
        {passwordCreator}
        closePasswordWindow={async () => await closePasswordWindow(true)}
        {keyType}
      />
    {/if}
  {/if}
</main>
