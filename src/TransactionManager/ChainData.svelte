<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { ethers } from "ethers";

  const provider = new ethers.JsonRpcProvider("https://eth.llamarpc.com");
  let data: string[] = [];

  onMount(async () => {
    data = await invoke("get_data");
    provider.on("block", async () => {
      data = await invoke("get_data");
    });
  });
</script>

<div>
  <p>{data[0] || 0}</p>
  <p>{data[1] || 0}</p>
</div>

<style>
  div {
    position: absolute;
    top: 0;
    width: 150px;
    margin-left: calc(50% - 75px);
    color: white;
  }

  p {
    margin-top: 0;
    float: left;
    border: 1px solid white;
    padding-left: 5px;
    padding-right: 5px;
  }
</style>

