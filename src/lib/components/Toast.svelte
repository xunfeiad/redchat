<script lang="ts">
    export let type: 'success' | 'error' = 'success';
    export let message: string = '';
    export let show: boolean = false;
  </script>
  
  {#if show}
    <div class="toast" class:success={type === 'success'} class:error={type === 'error'}>
      <div class="icon">
        {#if type === 'success'}
          ✓
        {:else}
          ✕
        {/if}
      </div>
      <span>{message}</span>
    </div>
  {/if}
  
  <style>
    .toast {
      position: fixed;
      top: 20px;
      left: 50%;
      transform: translateX(-50%);
      padding: 12px 24px;
      border-radius: 10px;
      display: flex;
      align-items: center;
      gap: 10px;
      box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
      animation: slideIn 0.3s ease-out;
      z-index: 1000;
    }
  
    .success {
      background-color: #34C759;
      color: white;
    }
  
    .error {
      background-color: #FF3B30;
      color: white;
    }
  
    .icon {
      font-size: 1.2em;
      font-weight: bold;
    }
  
    @keyframes slideIn {
      from {
        transform: translateX(-50%) translateY(-100%);
        opacity: 0;
      }
      to {
        transform: translateX(-50%) translateY(0);
        opacity: 1;
      }
    }
  </style>