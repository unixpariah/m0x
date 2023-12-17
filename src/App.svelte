<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import ExportMethod from "./lib/ExportMethod.svelte";

  let divHeight = 55;
  let isExpanded = false;

  const newWallet = async (keyType: String) => {
    const wallet = await invoke("generate_wallet", {
      keyType: keyType,
      length: 12,
    });
    console.log(wallet);
  };

  const toggleDivHeight = () => {
    isExpanded = !isExpanded;
    divHeight = isExpanded ? 350 : 55;
  };
</script>

<main class="container">
  <div style={`height: ${divHeight}px;`} class="expanding-div">
    <button
      on:click={toggleDivHeight}
      class:clicked={isExpanded}
      class="new-wallet"
    >
      {!isExpanded ? "Create new wallets" : "Hide menu"}
    </button>

    <div id="line"></div>
    <ExportMethod text="Import with private key" />
    <ExportMethod text="Import with seed phrase" />
    <ExportMethod
      on:click={() => {
        newWallet("private_key");
      }}
      text="Generate private key"
    />
    <ExportMethod
      on:click={() => {
        newWallet("mnemonic");
      }}
      text="Generate seed phrase"
    />
  </div>
</main>

<style>
  #line {
    margin-top: 0.5px;
    height: 3px;
    background-color: black;
  }

  .new-wallet {
    align-items: center;
    background: #1d1d1f;
    border: 0;
    border-radius: 8px;
    box-sizing: border-box;
    color: white;
    cursor: pointer;
    display: flex;
    font-size: 1rem;
    justify-content: center;
    line-height: 1.5rem;
    padding: 15px;
    position: relative;
    text-align: left;
    transition: 0.2s;
    user-select: none;
    -webkit-user-select: none;
    touch-action: manipulation;
    white-space: pre;
    width: 382px;
    word-break: normal;
    word-spacing: normal;
    z-index: 2;
  }

  .new-wallet:hover {
    background: #18181c;
  }

  .new-wallet.clicked {
    border-bottom-right-radius: 0;
    border-bottom-left-radius: 0;
  }

  .expanding-div {
    background-color: #1d1d1f;
    transition: height 0.5s;
    overflow: hidden;
    width: 382px;
    position: relative;
    z-index: 1;
    box-shadow:
      -10px -10px 30px 0 #171719,
      10px 10px 30px 0 #2a1f62;
    border-radius: 10px;
  }

  @media (min-width: 768px) {
    .new-wallet {
      padding: 24px;
    }
  }
</style>

