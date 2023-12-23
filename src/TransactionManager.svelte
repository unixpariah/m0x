<script>
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  onMount(async () => {
    await listen("click", (event) => {
      console.log(`Received click event with payload: ${event.payload}`);
    });
  });
</script>

<div>hi</div>
