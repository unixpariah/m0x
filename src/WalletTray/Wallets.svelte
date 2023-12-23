<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  interface Wallet {
    name: string;
    address: string;
    key: string;
  }

  let inputPassword = false;
  let password = "";
  let usedWallet: Wallet;

  const openWallet = async () => {
    await invoke("open_wallet", {
      wallet: usedWallet,
      password,
    });
  };

  const setPassword = (event: Event) => {
    const target = event.target as HTMLInputElement;
    password = target.value;
  };

  export let walletList: Wallet[];
</script>

<div>
  {#each walletList as wallet}
    <button
      class="wallet-wrapper"
      on:click={() => {
        inputPassword = true;
        usedWallet = wallet;
      }}
    >
      <p class="name">
        {wallet.name}
      </p>
      <p class="wallet">
        {wallet.address.slice(0, 5)}...{wallet.address.slice(
          wallet.address.length - 5,
        )}
      </p>
    </button>
  {/each}

  {#if inputPassword}
    <input type="text" on:input={(e) => setPassword(e)} />
    <button on:click={openWallet}></button>
  {/if}
</div>

<style>
  .wallet,
  .name {
    font-size: 0.9rem;
    margin: 0;
  }

  .wallet-wrapper {
    text-align: left;
    margin-top: 5px;
    background-color: grey;
    width: 380px;
    height: 35px;
    border: 0;
  }
</style>
