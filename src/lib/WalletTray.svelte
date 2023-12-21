<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import CreateWalletsComponent from "./WalletTray/CreateWallets.svelte";
  import WalletsComponent from "./WalletTray/Wallets.svelte";
  import PasswordInputComponent from "./WalletTray/PasswordInput.svelte";
  import SearchBarComponent from "./WalletTray/SearchBar.svelte";

  interface Wallet {
    name: string;
    address: string;
    key: string;
  }

  let trayHeight = 55;
  let isTrayExpanded = false;
  let isCreatingWallet = false;
  let isImportingWallet = false;
  let isCreatingPassword = false;
  let selectedKeyType: string;
  let enteredPassword: string;
  let enteredName: string;
  let passwordLength = 12;
  let walletList: Wallet[] = [];
  let searchQuery = "";
  let importedKey: string;

  const toggleTrayHeight = () => {
    isTrayExpanded = !isTrayExpanded;
    trayHeight = isTrayExpanded ? 430 : 55;
  };

  const closePasswordInput = async (isImporting?: boolean) => {
    if (isImporting) {
      isImportingWallet = false;
      isCreatingPassword = false;
      await invoke("import_wallet", {
        keyType: selectedKeyType,
        password: enteredPassword,
        key: importedKey,
        name: enteredName,
      });
      return;
    }
    isCreatingWallet = false;
    await invoke("generate_wallet", {
      keyType: selectedKeyType,
      length: passwordLength,
      password: enteredPassword,
      name: enteredName,
    });
    enteredName = "";
    enteredPassword = "";
    refreshWalletList();
  };

  const refreshWalletList = async () => {
    const loadedWallets: Wallet[] = await invoke("read_wallets");
    walletList = loadedWallets.filter((wallet: Wallet) =>
      wallet.name.includes(searchQuery),
    );
  };

  const createNewWallet = async (type: string) => {
    selectedKeyType = type;
    isCreatingWallet = true;
  };

  const importNewWallet = async (type: string) => {
    selectedKeyType = type;
    isImportingWallet = true;
  };

  const setName = (event: Event) => {
    const target = event.target as HTMLInputElement;
    enteredName = target.value;
  };

  const setPassword = (event: Event) => {
    const target = event.target as HTMLInputElement;
    enteredPassword = target.value;
  };

  const setImportedKey = (event: Event) => {
    const target = event.target as HTMLInputElement;
    importedKey = target.value;
  };

  const setSearchQuery = (event: Event) => {
    const target = event.target as HTMLInputElement;
    searchQuery = target.value;
    refreshWalletList();
  };

  refreshWalletList();
</script>

<main class="container">
  <CreateWalletsComponent
    {createNewWallet}
    {trayHeight}
    {isTrayExpanded}
    {toggleTrayHeight}
    {importNewWallet}
  />
  <SearchBarComponent {searchQuery} {setSearchQuery} />
  <WalletsComponent {walletList} />
  {#if isCreatingWallet}
    <PasswordInputComponent
      {setName}
      {setPassword}
      {closePasswordInput}
      {selectedKeyType}
    />
  {/if}
  {#if isImportingWallet}
    <input type="text" on:input={(e) => setImportedKey(e)} />
    <button on:click={() => (isCreatingPassword = true)}></button>
    {#if isCreatingPassword}
      <PasswordInputComponent
        {setName}
        {setPassword}
        closePasswordInput={async () => await closePasswordInput(true)}
        {selectedKeyType}
      />
    {/if}
  {/if}
</main>
