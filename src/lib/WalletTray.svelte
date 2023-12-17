<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import ExportMethod from "./WalletTray/ExportMethod.svelte";

  let divHeight = 55;
  let isExpanded = false;
  let createPassword = false;
  let keyType: string;
  let password = "";

  const createNewWallet = async (type: string) => {
    keyType = type;
    createPassword = true;
  };

  const toggleDivHeight = () => {
    isExpanded = !isExpanded;
    divHeight = isExpanded ? 430 : 55;
  };

  const closePasswordWindow = async () => {
    createPassword = false;
    await invoke("generate_wallet", {
      keyType,
      length: 12,
      password,
    });
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
    <h3>Import:</h3>
    <ExportMethod text="Private key" />
    <ExportMethod text="Seed phrase" />
    <h3>Create:</h3>
    <ExportMethod
      on:click={() => {
        createNewWallet("private_key");
      }}
      text="Private key"
    />
    <ExportMethod
      on:click={() => {
        createNewWallet("mnemonic");
      }}
      text="Seed phrase"
    />
  </div>

  {#if createPassword}
    <div id="overlay">
      <input
        id="password-input"
        type="text"
        on:input={(e) => (password = e.target.value)}
      />
      <button
        id="password-button"
        on:click={async () => {
          await closePasswordWindow();
        }}
      ></button>
    </div>
  {/if}
</main>

<style>
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

  h3 {
    color: white;
    margin-left: 10px;
    margin-bottom: 0;
  }

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

