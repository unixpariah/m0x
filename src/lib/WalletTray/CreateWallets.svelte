<script lang="ts">
  import ExportMethod from "./CreateWallets/ExportMethod.svelte";

  export let divHeight: number;
  export let isExpanded: boolean;
  export let toggleDivHeight: () => void;
  export let createNewWallet: (type: string) => Promise<void>;
</script>

<div style={`height: ${divHeight}px;`} id="expanding-div">
  <button on:click={toggleDivHeight} class:clicked={isExpanded}>
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

<style>
  #expanding-div {
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

  button {
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

  button:hover {
    background: #18181c;
  }

  button.clicked {
    border-bottom-right-radius: 0;
    border-bottom-left-radius: 0;
  }

  @media (min-width: 768px) {
    button {
      padding: 24px;
    }
  }
</style>
