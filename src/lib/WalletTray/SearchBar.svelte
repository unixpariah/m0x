<script lang="ts">
  export let query = "";
  export let search: (e: Event) => void;
</script>

<input
  type="text"
  bind:value={query}
  on:input={search}
  placeholder="Search..."
/>

<style>
  input {
    width: 376px;
    height: 20px;
    margin-top: 10px;
  }
</style>
