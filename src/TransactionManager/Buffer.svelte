<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher<{ selectedWalletIndex: number }>();

  const selectWallet = (index: number) => {
    dispatch("selectedWalletIndex", index);
  };

  interface Wallet {
    name: string;
    address: string;
    key: string;
  }

  export let openWallets: Wallet[];
  export let walletItemWidth: string;
  export let updateWalletItemWidth: () => void;

  let selectedWalletIndex = 0;
  selectWallet(selectedWalletIndex);

  const handleKeydown = (e: { key: string; target: any }) => {
    updateWalletItemWidth();
    if ((e.target as HTMLElement).closest(".ignore-keys")) {
      return;
    }

    switch (e.key) {
      case "H":
        selectedWalletIndex =
          selectedWalletIndex <= 0
            ? openWallets.length - 1
            : selectedWalletIndex - 1;
        selectWallet(selectedWalletIndex);
        break;
      case "L":
        selectedWalletIndex =
          selectedWalletIndex >= openWallets.length - 1
            ? 0
            : selectedWalletIndex + 1;
        selectWallet(selectedWalletIndex);
        break;
      case "d":
        invoke("close_wallet", { index: selectedWalletIndex });
        selectedWalletIndex =
          selectedWalletIndex >= openWallets.length - 1
            ? openWallets.length - 2
            : selectedWalletIndex;

        selectWallet(selectedWalletIndex);

        if (openWallets.length === 1) {
          invoke("close_window");
        }
        break;
      case "D":
        invoke("close_wallet");
        break;
      default:
        break;
    }
  };
</script>

<svelte:window on:keydown={handleKeydown} />

<div>
  {#each openWallets as wallet, index}
    <button
      id="buffer"
      on:click={() => {
        selectedWalletIndex = index;
        dispatch("selectedWalletIndex", index);
      }}
      style="width: {walletItemWidth}; {selectedWalletIndex === index &&
        'border-top: 1px solid transparent; background-color: transparent;'}"
    >
      <button
        on:click={async () => {
          await invoke("close_wallet", { index });
          if (openWallets.length === 0) {
            invoke("close_window");
          }

          if (selectedWalletIndex > 0) {
            selectedWalletIndex = selectedWalletIndex - 1;
          }

          selectWallet(selectedWalletIndex);
        }}>X</button
      >
      {wallet.address.slice(0, 5)}
    </button>
  {/each}
</div>

<style>
  #buffer {
    text-align: left;
    color: white;
    font-size: 0.6rem;
    float: left;
    border: 1px solid white;
    padding: 5px;
    margin: 0;
  }

  button {
    cursor: pointer;
    background-color: grey;
    color: white;
    border-radius: 0;
    border: 0;
  }

  div {
    position: absolute;
    bottom: 0;
  }
</style>

