<script lang="ts">
  export let passwordCreator: (event: Event) => void;
  export let setName: (event: Event) => void;
  export let closePasswordWindow: () => void;
  export let keyType: string;
</script>

<div id="overlay">
  <input type="text" on:input={(e) => setName(e)} />
  <input type="password" on:input={(e) => passwordCreator(e)} />
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
    on:click={() => {
      closePasswordWindow();
    }}
  ></button>
</div>

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

  input {
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

