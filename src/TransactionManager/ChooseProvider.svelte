<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let providers: string[] = [];
  let isExpanded = false;
  let enterProvider = false;
  let expandedHeight = 50 + providers.length * 50;
  let newProvider: string;

  onMount(async () => {
    providers = await invoke("get_providers");
  });

  const addProvider = (event: Event) => {
    const target = event.target as HTMLInputElement;
    newProvider = target.value;
  };
</script>

<button id="provider" on:click={() => (isExpanded = !isExpanded)}
  >{providers[0]}</button
>

<div style="height: {isExpanded ? expandedHeight : 0}px">
  {#each providers as provider, i}
    {#if i > 0}
      <button>{provider}</button>
    {/if}
  {/each}
  <button id="new-provider" on:click={() => (enterProvider = true)}
    >Add new provider</button
  >
</div>

{#if enterProvider}
  <input type="text" on:input={addProvider} />
  <button
    on:click={() => {
      providers.push(newProvider);
      newProvider = "";
      enterProvider = false;
      expandedHeight = 50 + providers.length * 50;
    }}
  ></button>
{/if}

<style>
  div {
    overflow: auto;
    transition: height 0.5s;
    background-color: red;
    width: 340px;
  }

  #new-provider {
    border: 0;
    border-radius: 0;
    background-color: white;
    width: 340px;
    height: 40px;
    margin-top: 10px;
  }

  #provider {
    border: 0;
    border-radius: 0;
    background-color: white;
    width: 340px;
    height: 40px;
  }
</style>

