<script lang="ts">
  import { createEventDispatcher } from "svelte";
  export let text = "";
  const dispatch = createEventDispatcher();
</script>

<button class="butt" on:click={() => dispatch("click")}>{text}</button>

<style>
  .butt {
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
</style>

