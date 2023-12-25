<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher<{ selectedWalletIndex: number }>();

  function selectWallet(index: number) {
    dispatch("selectedWalletIndex", index);
  }

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

  const handleKeydown = (e: { key: string }) => {
    updateWalletItemWidth();

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
    <p
      style="width: {walletItemWidth}; {selectedWalletIndex === index &&
        'border-top: 1px solid transparent;'}"
    >
      {wallet.address.slice(0, 5)}
    </p>
  {/each}
</div>

<style>
  p {
    color: white;
    font-size: 0.6rem;
    float: left;
    border: 1px solid white;
    padding: 5px;
    margin: 0;
  }

  div {
    position: absolute;
    bottom: 0;
  }
</style>

