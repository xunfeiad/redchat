<script lang="ts">
  import { goto } from "$app/navigation";

  function goBack() {
    history.back();
  }
</script>

<button on:click={goBack} class="back-button">
  ← 返回
</button>

<style>
  .back-button {
    padding: 0.5rem 1rem;
    border: none;
    background: none;
    cursor: pointer;
    font-size: 1rem;
    color: #666;
  }
  
  .back-button:hover {
    color: #333;
  }
</style> 