<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { wsClient, wsStatus, wsMessages } from '$lib/stores/websocket';

  export let userId: number;

  $: if ($wsStatus === 'open') {
    // WebSocket 连接成功后发送用户认证信息
    wsClient.send({
      type: 'auth',
      authMessage: {
        user_id: userId
      }
    });
  }

  onMount(() => {
    wsClient.connect();
  });

  onDestroy(() => {
    wsClient.close();
  });
</script>

{#if $wsStatus === 'connecting'}
  <div class="ws-status connecting">正在连接...</div>
{:else if $wsStatus === 'closed'}
  <div class="ws-status closed">连接已断开</div>
{/if}

<style>
  .ws-status {
    position: fixed;
    top: 1rem;
    right: 1rem;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    font-size: 0.875rem;
    z-index: 1000;
  }

  .connecting {
    background: #fff3cd;
    color: #856404;
  }

  .closed {
    background: #f8d7da;
    color: #721c24;
  }
</style> 