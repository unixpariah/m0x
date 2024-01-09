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
  let throwError = false;

  const openWallet = async () => {
    try {
      await invoke("open_wallet", {
        wallet: usedWallet,
        password,
      });
      inputPassword = false;
      throwError = false;
    } catch (e) {
      throwError = true;
    }
    password = "";
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
    <input type="text" bind:value={password} on:input={(e) => setPassword(e)} />
    <button on:click={openWallet}></button>
  {/if}

  {#if throwError}
    <p>Wrong password</p>
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

