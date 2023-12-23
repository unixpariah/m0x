<script lang="ts">
  export let searchQuery = "";
  export let setSearchQuery: (e: Event) => void;
</script>

<input
  type="text"
  bind:value={searchQuery}
  on:input={setSearchQuery}
  placeholder="Search..."
/>

<style>
  input {
    width: 376px;
    height: 20px;
    margin-top: 10px;
    margin-left: 0;
  }
</style>

