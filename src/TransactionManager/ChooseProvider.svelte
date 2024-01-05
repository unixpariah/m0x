<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { ethers } from "ethers";

  let providers: string[] = [];
  let isExpanded = false;
  let enterProvider = false;
  let expandedHeight = 52;
  let newProvider: string;

  onMount(async () => {
    providers = await invoke("get_providers");
    expandedHeight = 52 + (providers.length - 1) * 40;
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
  <div id="line"></div>
  {#each providers as provider, i}
    {#if i > 0}
      <button
        on:click={async () => {
          [providers[0], providers[i]] = [providers[i], providers[0]];
          await invoke("update_provider_list", { updatedProviders: providers });
        }}
        id="provider">{provider}</button
      >
    {/if}
  {/each}
  <button id="new-provider" on:click={() => (enterProvider = true)}
    >Add new provider</button
  >
</div>

{#if enterProvider}
  <input class="ignore-keys" type="text" on:input={addProvider} />
  <button
    on:click={async () => {
      newProvider = newProvider.replace(/\s/g, "");
      if (providers.includes(newProvider)) {
        return;
      }

      const provider = new ethers.JsonRpcProvider(newProvider);

      try {
        await provider.getNetwork();
      } catch (e) {
        return;
      }

      providers.push(newProvider);
      providers = providers;
      newProvider = "";
      enterProvider = false;
      expandedHeight = 52 + (providers.length - 1) * 40;

      await invoke("update_provider_list", { updatedProviders: providers });
    }}
  ></button>
{/if}

<style>
  #line {
    height: 0;
    border: 1px solid black;
  }

  #new-provider {
    margin-top: 10px;
    border: 0;
    border-radius: 0;
    background-color: white;
    width: 340px;
    height: 40px;
  }

  div {
    overflow: auto;
    overflow-x: hidden;
    transition: height 0.5s;
    background-color: red;
    width: 340px;
  }

  #provider {
    border: 0;
    border-radius: 0;
    background-color: white;
    width: 340px;
    height: 40px;
  }
</style>

