<script lang="ts">
  export let trayHeight: number;
  export let isTrayExpanded: boolean;
  export let toggleTrayHeight: () => void;
  export let createNewWallet: (type: string) => Promise<void>;
  export let importNewWallet: (type: string) => Promise<void>;
</script>

<div style={`height: ${trayHeight}px;`} id="expanding-div">
  <button on:click={toggleTrayHeight} class:clicked={isTrayExpanded}>
    {!isTrayExpanded ? "New wallet" : "Hide menu"}
  </button>

  <div id="line"></div>
  <h3>Import:</h3>

  <button class="method-button" on:click={() => importNewWallet("private_key")}
    >Private key</button
  >
  <button
    class="method-button"
    on:click={() => importNewWallet("import_mnemonic")}>Seed phrase</button
  >
  <h3>Create:</h3>
  <button class="method-button" on:click={() => createNewWallet("private_key")}
    >Private key</button
  >
  <button class="method-button" on:click={() => createNewWallet("mnemonic")}
    >Seed phrase</button
  >
</div>

<style>
  .method-button {
    cursor: pointer;
    margin-left: 10px;
    margin-top: 10px;
    border: 0;
    background-color: black;
    color: white;
    width: 360px;
    height: 60px;
    border-radius: 10px;
    font-weight: 500;
    font-size: 20px;
  }

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

