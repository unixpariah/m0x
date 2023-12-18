<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import CreateWallets from "./WalletTray/CreateWallets.svelte";
  import Wallets from "./WalletTray/Wallets.svelte";

  interface Wallet {
    address: string;
    key: string;
  }

  let divHeight = 55;
  let isExpanded = false;
  let createPassword = false;
  let keyType: string;
  let password = "";
  let length = 12;

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
    });
    wallets = getWallets();
  };

  const getWallets = async (): Promise<Wallet[]> => {
    return await invoke("read_wallets");
  };

  const createNewWallet = async (type: string) => {
    keyType = type;
    createPassword = true;
  };

  let wallets: Promise<Wallet[]> = getWallets();

  function passwordCreator(event: Event) {
    const target = event.target as HTMLInputElement;
    password = target.value;
  }
</script>

<main class="container" lang="ts">
  <CreateWallets {createNewWallet} {divHeight} {isExpanded} {toggleDivHeight} />

  {#if createPassword}
    <div id="overlay">
      <input
        id="password-input"
        type="text"
        on:input={(e) => passwordCreator(e)}
      />
      {#if keyType === "mnemonic"}
        <div id="choose-lenght">
          <button
            on:click={() => {
              length = 12;
            }}
            id="length-button">12</button
          >
          <button
            on:click={() => {
              length = 24;
            }}
            id="length-button">24</button
          >
        </div>
      {/if}
      <button
        id="password-button"
        on:click={async () => {
          await closePasswordWindow();
        }}
      ></button>
    </div>
  {/if}

  <Wallets {wallets} />
</main>

<style>
  #length-button {
    border-radius: 0;
    border: 1px solid black;
    border-left: 0;
    width: 151px;
    height: 40px;
    padding: 0;
    float: left;
  }

  #choose-lenght {
    width: 302px;
    height: 40px;
    background-color: white;
    margin-left: calc(50% - 150px);
  }

  #overlay {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: 5;
    border-radius: 10px;
    background-color: rgba(30, 30, 30, 0.5);
  }

  #password-input {
    background-color: grey;
    border-top-right-radius: 5px;
    border-top-left-radius: 5px;
    margin-left: calc(50% - 150px);
    margin-top: 120px;
    width: 300px;
    height: 100px;
    border-bottom: 3px solid black;
    border: 0;
  }

  #password-button {
    cursor: pointer;
    width: 302px;
    height: 40px;
    border-bottom-left-radius: 5px;
    border-bottom-right-radius: 5px;
    margin-left: calc(50% - 150px);
    border: 0;
  }
</style>
