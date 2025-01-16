<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from "@tauri-apps/api/core";

  let localVideo: HTMLVideoElement;
  let remoteVideo: HTMLVideoElement;
  let channelInput = "";
  let errorMessage = "";
  let isHost = true;

  // WebRTC 配置
  const configuration = {
    iceServers: [
      { urls: 'stun:stun.l.google.com:19302' }
    ]
  };

  async function startVideoChat() {
    try {
      // 1. 获取本地媒体流
      const stream = await navigator.mediaDevices.getUserMedia({
        video: true,
        audio: true
      });
      
      localVideo.srcObject = stream;

      // 2. 创建 SDP offer
      const pc = new RTCPeerConnection(configuration);
      
      // 添加本地流到连接
      stream.getTracks().forEach(track => {
        pc.addTrack(track, stream);
      });

      // 处理远程流
      pc.ontrack = (event) => {
        if (remoteVideo) {
          remoteVideo.srcObject = event.streams[0];
        }
      };

      if (isHost) {
        // 主机模式：创建 offer
        const offer = await pc.createOffer();
        await pc.setLocalDescription(offer);
        
        // 调用 Rust 后端连接
        await invoke("connect_rtc", {
          channel: channelInput,
          sdp: offer.sdp,
          isVideo: true,
          isAudio: true
        });
      } else {
        // 客户端模式：创建 answer
        const offer = await pc.createOffer();
        await pc.setLocalDescription(offer);
        
        // 调用 Rust 后端连接
        await invoke("connect_rtc", {
          channel: channelInput,
          sdp: offer.sdp,
          isVideo: true,
          isAudio: true
        });
      }

    } catch (error) {
      errorMessage = `视频聊天启动失败: ${error}`;
      console.error(errorMessage);
    }
  }

  onMount(() => {
    return () => {
      // 清理资源
      if (localVideo?.srcObject) {
        const stream = localVideo.srcObject as MediaStream;
        stream.getTracks().forEach(track => track.stop());
      }
      if (remoteVideo?.srcObject) {
        const stream = remoteVideo.srcObject as MediaStream;
        stream.getTracks().forEach(track => track.stop());
      }
    };
  });
</script>

<div class="video-chat">
  <div class="controls">
    <div class="mode-selector">
      <label>
        <input 
          type="radio" 
          bind:group={isHost} 
          value={true}
        > 主机模式
      </label>
      <label>
        <input 
          type="radio" 
          bind:group={isHost} 
          value={false}
        > 客户端模式
      </label>
    </div>

    <div class="channel-input">
      <input
        type="text"
        placeholder="输入房间号"
        bind:value={channelInput}
      >
      <button on:click={startVideoChat}>
        {isHost ? '创建房间' : '加入房间'}
      </button>
    </div>

    {#if errorMessage}
      <p class="error">{errorMessage}</p>
    {/if}
  </div>

  <div class="video-container">
    <div class="video-wrapper">
      <h3>本地视频</h3>
      <video
        bind:this={localVideo}
        autoplay
        playsinline
        muted
      >
        <track kind="captions" />
      </video>
    </div>

    <div class="video-wrapper">
      <h3>远程视频</h3>
      <video
        bind:this={remoteVideo}
        autoplay
        playsinline
      >
        <track kind="captions" />
      </video>
    </div>
  </div>
</div>

<style>
  .video-chat {
    padding: 1rem;
  }

  .controls {
    margin-bottom: 2rem;
  }

  .mode-selector {
    margin-bottom: 1rem;
  }

  .mode-selector label {
    margin-right: 1rem;
  }

  .channel-input {
    display: flex;
    gap: 1rem;
    margin-bottom: 1rem;
  }

  input[type="text"] {
    padding: 0.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    width: 200px;
  }

  button {
    padding: 0.5rem 1rem;
    background: #4CAF50;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  button:hover {
    background: #45a049;
  }

  .error {
    color: red;
    margin-top: 0.5rem;
  }

  .video-container {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 2rem;
  }

  .video-wrapper {
    text-align: center;
  }

  h3 {
    margin-bottom: 1rem;
    color: #333;
  }

  video {
    width: 100%;
    max-width: 640px;
    border-radius: 8px;
    background: #000;
  }
</style> 