<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from "@tauri-apps/api/core";
    
    let videoElement: HTMLVideoElement;
    let remoteVideos: HTMLVideoElement[] = [];
    let errorMessage = '';
    let channelInput = '';
    let localStream: MediaStream | null = null;

    const constraints = {
        video: true,
        audio: true
    }
  
    async function startStream() {
      try {
        if (!navigator.mediaDevices?.getUserMedia) {
          throw new Error('浏览器不支持 getUserMedia API');
        }
  
        localStream = await getStream();
        videoElement.srcObject = localStream;
      } catch (error) {
        errorMessage = `获取媒体设备失败: ${error}`;
        console.error(errorMessage);
      }
    }

    const getStream = async (): Promise<MediaStream>=>{
        const stream = await navigator.mediaDevices.getUserMedia(constraints);
        stream
        return stream;
    }

  
    async function createConnection() {
      if (!channelInput) {
        errorMessage = "请输入频道名";
        return;
      }
  
      try {
        if (!localStream) {
          await startStream();
        }
  
        // 创建 SDP offer
        const pc = new RTCPeerConnection({
          iceServers: [{ urls: "stun:stun.l.google.com:19302" }]
        });
  
        // 添加本地流
        localStream?.getTracks().forEach(track => {
          pc.addTrack(track, localStream!);
        });
  
        // 创建 offer
        const offer = await pc.createOffer();
        await pc.setLocalDescription(offer);
        
        // 监听远程流
        pc.addEventListener("track", (event) => {
          const remoteVideo = document.createElement('video');
          remoteVideo.srcObject = event.streams[0];
          remoteVideos.push(remoteVideo);
        });

        // 创建数据通道
        const dataChannel = pc.createDataChannel('chat');
        console.log(dataChannel)
        // 监听数据通道状态
        dataChannel.onopen = () => {
          console.log('数据通道已打开');
          // 发送频道信息
          dataChannel.send(JSON.stringify({
            type: 'join',
            channel: channelInput
          }));
        };
        
        dataChannel.onmessage = (event) => {
            console.log(event);
          const data = JSON.parse(event.data);
          if (data.type === 'answer') {
            // 设置远程描述
            pc.setRemoteDescription(new RTCSessionDescription({
              type: 'answer',
              sdp: data.sdp
            }));
          }
        };
  
      } catch (error) {
        errorMessage = `连接失败: ${error}`;
        console.error(errorMessage);
      }
    }
  
    onMount(() => {
      return () => {
        if (localStream) {
          localStream.getTracks().forEach(track => track.stop());
        }
      };
    });
  </script>
  
  <div class="video-container">
    <div class="video-grid">
      <div class="video-box">
        <h3>本地视频</h3>
        <video 
          bind:this={videoElement}
          autoplay
          playsinline
          muted
        >
          <track kind="captions" />
        </video>
      </div>
  
      {#each remoteVideos as remoteVideo}
        <div class="video-box">
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
  
    <div class="controls">
      <input 
        type="text" 
        bind:value={channelInput}
        placeholder="输入频道名"
      />
      <button on:click={startStream}>
        开启摄像头
      </button>
      <button on:click={createConnection} disabled={!localStream}>
        建立连接
      </button>
    </div>
  
    {#if errorMessage}
      <p class="error">{errorMessage}</p>
    {/if}
  </div>
  
  <style>
    .video-container {
      padding: 1rem;
      display: flex;
      flex-direction: column;
      gap: 1rem;
    }
  
    .video-grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
      gap: 1rem;
    }
  
    .video-box {
      background: #f5f5f5;
      padding: 1rem;
      border-radius: 8px;
    }
  
    h3 {
      margin: 0 0 0.5rem 0;
      font-size: 1rem;
      color: #333;
    }
  
    video {
      width: 100%;
      max-width: 640px;
      border-radius: 8px;
      background: #000;
    }
  
    .controls {
      display: flex;
      gap: 1rem;
      align-items: center;
      flex-wrap: wrap;
    }
  
    input {
      padding: 0.5rem;
      border: 1px solid #ddd;
      border-radius: 4px;
      min-width: 200px;
    }
  
    button {
      padding: 0.5rem 1rem;
      border: none;
      border-radius: 4px;
      background: #4CAF50;
      color: white;
      cursor: pointer;
      font-size: 1rem;
    }
  
    button:disabled {
      background: #ccc;
      cursor: not-allowed;
    }
  
    button:hover:not(:disabled) {
      background: #45a049;
    }
  
    .error {
      color: #ff4444;
      margin-top: 1rem;
    }
  </style>