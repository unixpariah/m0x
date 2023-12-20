<script lang="ts">
  interface Wallet {
    address: string;
    key: string;
  }

  export let wallets: Wallet[];
</script>

<div>
  {#await wallets then wallets}
    {#each wallets as wallet}
      <div class="wallet-wrapper">
        <p class="wallet">{wallet.address}</p>
      </div>
    {/each}
  {/await}
</div>

<style>
  .wallet {
    color: white;
    font-size: 0.9rem;
  }

  .wallet-wrapper {
    background-color: grey;
    width: 380px;
    height: 30px;
  }
</style>
