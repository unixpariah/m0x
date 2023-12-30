<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  let data = ["0", "0"];

  onMount(async () => {
    data = await invoke("get_data");
    setInterval(async () => {
      data = await invoke("get_data");
    }, 10000);
  });
</script>

<div>
  <p>{data[0]}</p>
  <p>{data[1]}</p>
  <p>{data[2]}</p>
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
