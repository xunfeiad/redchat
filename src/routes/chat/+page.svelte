<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import type { Response, WebSocketMessage, TextContent} from "../../../types";
  import { wsClient, wsStatus, wsMessages } from "$lib/stores/websocket";
    import { get } from "svelte/store";
  // import { mediaDevices } from '@tauri-apps/api/window';

  interface Message {
    id?: number;
    content: string;
    type: "text" | "image" | "file";
    isSelf: boolean;
    timestamp: Date;
    status: "sending" | "sent" | "failed";
  }

  interface Contact {
    id: number;
    name: string;
    avatar: string;
    lastMessage?: string;
    unread: number;
    lastMessageTime: Date;
    online?: boolean;
    groupId?: string;
    groupName?: string;
    groupAvatar?: string;
    groupLastMessage?: string;
    groupLastMessageTime?: Date;
  }

  let messages: Message[] = [];
  let contacts: Contact[] = [];
  let currentContact: Contact | null = null;
  // 输入框
  let messageInput = "";
  let chatContainer: HTMLElement;
  let errorMessage = "";

  // WebRTC 相关状态
  let localStream: MediaStream | null = null;
  let remoteStream: MediaStream | null = null;
  let peerConnection: RTCPeerConnection | null = null;

  // 视频元素引用
  let localVideo: HTMLVideoElement | null = null;
  let remoteVideo: HTMLVideoElement | null = null;
  let userInfo = localStorage.getItem('userInfo');

  $: contacts;
  $: localStream;
  $: remoteStream;
  $: peerConnection;
  $: userInfo;
  $: userId = userInfo && JSON.parse(userInfo).id;

  const get_contacts = async () => {
    try {
      let res: Response<Contact[]> = await invoke("get_contacts", {
        userId: userId,
      });
      contacts = [...res.data];
      console.log("Contacts loaded:", contacts);
    } catch (error) {
      console.error("Failed to load contacts:", error);
      errorMessage = "加载联系人失败";
    }
  };
  onMount(async () => {
    console.log(wsClient);
    localVideo = document.createElement("video");
    remoteVideo = document.createElement("video");
    wsClient.connect({ userId: userId });
    // 订阅 wsState 的变化
    wsStatus.subscribe((state) => {
      console.log("WebSocket state changed:", state);

      if (state === "open") {
        const authMessage = {
          type: "auth",
          content: {
            userId: userId,
            // token: 'your-auth-token'
          },
        };

        wsClient.send(authMessage);
        console.log("Authentication message sent:", authMessage);
      }
    });
    wsMessages.subscribe((message: WebSocketMessage<TextContent>) => {
      console.log("Received message:", message);
      if(get(wsStatus) === 'open' && message.type === 'text'){
        messages = [...messages, {
          // id: message.content?.receiverId,
          content: message.content?.message,
          type: "text",
          isSelf: false,
          timestamp: new Date(),
          status: "sent",
        }]
      }
    });
    try {
      await get_contacts();
      console.log("Contacts loaded:", contacts);
    } catch (error) {
      console.error("Failed to load contacts:", error);
      errorMessage = "加载联系人失败";
    }
  });
  async function sendMessage() {
    if (!messageInput.trim() || !currentContact) return;

    const newMessage: Message = {
      id: Date.now(),
      content: messageInput,
      type: "text",
      isSelf: true,
      timestamp: new Date(),
      status: "sending",
    };

    messages = [...messages, newMessage];

    // 滚动到底部
    setTimeout(() => {
      chatContainer?.scrollTo({
        top: chatContainer.scrollHeight,
        behavior: "smooth",
      });
    }, 100);

    try {
      // 这里添加实际的消息发送逻辑
      const res: Response<object> = await invoke("send_message", {
        userId: userId,
        contactId: currentContact.id,
        message: messageInput,
      });

      // 更新消息状态为已发送
      if (res.code === 0) {
        messages = messages.map((msg) =>
          msg.id === newMessage.id ? { ...msg, status: "sent" as const } : msg,
        );
        console.log({
          type: "text",
          content: {
            receiver_id: currentContact.id,
            message: messageInput,
            group_id: null,
          },
        });
        wsClient.send({
          type: "text",
          content: {
            receiverId: currentContact.id,
            message: messageInput,
            groupId: null,
          },
        });
      }
    } catch (error) {
      // 更新消息状态为发送失败
      console.error("Failed to send message:", error);
      messages = messages.map((msg) =>
        msg.id === newMessage.id ? { ...msg, status: "failed" as const } : msg,
      );
    }
  }

  // 选择联系人
  async function selectContact(contact: Contact) {
    currentContact = contact;
    // 清除未读消息
    await invoke("clear_unread", {
      userId: userId,
      contactId: contact.id,
    });
    await get_contacts();
    // 这里可以加载历史消息
    loadMessages(contact.id);
  }

  // 加载历史消息
  async function loadMessages(contactId: number) {
    console.log("loadMessages", contactId);
    const res: Response<Message[]> = await invoke("get_messages", {
      userId: userId,
      contactId: contactId,
    });
    console.log(res);
    if (res.code === 0) {
      messages = [...res.data];
    }
  }

  // 处理按键事件
  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Enter" && !event.shiftKey) {
      event.preventDefault();
      sendMessage();
    }
  }

  // 初始化 WebRTC
  async function initWebRTC() {
    try {
      localStream = await navigator.mediaDevices.getUserMedia({
        video: true,
        audio: true
      });
      localVideo!.srcObject = localStream;

      peerConnection = new RTCPeerConnection({
        iceServers: [{ urls: 'stun:stun.l.google.com:19302' }]
      });

      // 添加本地流
      localStream.getTracks().forEach(track => {
        peerConnection!.addTrack(track, localStream!);
      });

      // 处理远程流
      peerConnection.ontrack = (event) => {
        remoteStream = event.streams[0];
        remoteVideo!.srcObject = remoteStream;
      };
    } catch (error) {
      console.error('WebRTC 初始化失败:', error);
    }
  }

  // 开始视频通话
  async function startVideoCall() {
    if (!currentContact) return;
    await initWebRTC();
    
    // 创建并发送 offer
    const offer = await peerConnection!.createOffer();
    await peerConnection!.setLocalDescription(offer);

    // 通过 WebSocket 发送 offer
    wsClient.send({
      type: 'webrtc',
      content: {
        receiverId: currentContact.id,
        sdp: offer.sdp
      }
    });
  }

  // 开始语音通话
  async function startVoiceCall() {
    if (!currentContact) return;
    await initWebRTC();
    
    // 关闭视频轨道
    localStream?.getVideoTracks().forEach(track => track.enabled = false);
    
    // 创建并发送 offer
    const offer = await peerConnection!.createOffer();
    await peerConnection!.setLocalDescription(offer);

    wsClient.send({
      type: 'webrtc',
      content: {
        receiverId: currentContact.id,
        sdp: offer.sdp
      }
    });
  }

  // 处理 WebSocket 消息
  // wsClient.onMessage = async (message) => {
  //   const data = JSON.parse(message);
    
  //   if (data.type === 'webrtc') {
  //     const { type, sdp } = data.content;
      
  //     if (type === 'offer') {
  //       await initWebRTC();
  //       await peerConnection!.setRemoteDescription(new RTCSessionDescription(sdp));
        
  //       const answer = await peerConnection!.createAnswer();
  //       await peerConnection!.setLocalDescription(answer);
        
  //       wsClient.send({
  //         type: 'webrtc',
  //         content: {
  //           type: 'answer',
  //           receiverId: data.senderId,
  //           sdp: answer
  //         }
  //       });
  //     } else if (type === 'answer') {
  //       await peerConnection!.setRemoteDescription(new RTCSessionDescription(sdp));
  //     }
  //   }
  // };

  onDestroy(() => {
    wsClient.close();
    localStream?.getTracks().forEach(track => track.stop());
    peerConnection?.close();
  });
</script>

<div class="chat-container">
  <!-- 联系人列表 -->
  <div class="contacts-panel">
    <div class="search-box">
      <input type="text" placeholder="搜索" />
    </div>
    <div class="contacts-list">
      {#each contacts as contact}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="contact-item"
          class:active={currentContact?.id === contact.id}
          on:click={() => selectContact(contact)}
        >
          <div class="avatar">
            {#if contact.avatar?.startsWith("http") || contact.avatar?.startsWith("data:image/")}
              <img src={contact.avatar} alt={contact.name} class="avatar-img" />
            {:else}
              <div class="avatar-placeholder">
                {contact.name[0]}
              </div>
            {/if}
            {#if contact.online}
              <span class="online-status"></span>
            {/if}
          </div>
          <div class="contact-info">
            <div class="contact-header">
              <span class="name">{contact.name}</span>
              {#if contact.unread > 0}
                <span class="unread">{contact.unread}</span>
              {/if}
            </div>
            {#if contact.lastMessage}
              <div class="last-message">{contact.lastMessage}</div>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  </div>

  <!-- 聊天区域 -->
  <div class="chat-panel">
    {#if currentContact}
      <div class="chat-header">
        <div class="contact-info">
          <div class="avatar">
            {#if currentContact.avatar}
              <img src={currentContact.avatar} alt={currentContact.name} class="avatar-img" />
            {:else}
              <div class="avatar-placeholder">
                {currentContact.name[0]}
              </div>
            {/if}
          </div>
          <div class="contact-details">
            <h2>{currentContact.name}</h2>
            <span class="status">{currentContact.online ? '在线' : '离线'}</span>
          </div>
        </div>
        
        <div class="call-actions">
          <button class="call-btn voice" on:click={startVoiceCall} title="语音通话">
            <i class="fas fa-phone">语音通话</i>
          </button>
          <button class="call-btn video" on:click={startVideoCall} title="视频通话">
            <i class="fas fa-video">视频通话</i>
          </button>
        </div>
      </div>

      <div class="messages" bind:this={chatContainer}>
        {#each messages as message}
          <div class="message" class:self={message.isSelf}>
            <div class="message-content">
              {message.content}
              <div class="message-status">
                {#if message.status === "sending"}
                  发送中...
                {:else if message.status === "failed"}
                  发送失败
                {/if}
              </div>
            </div>
            <div class="message-time">
              {new Date(message.timestamp).toLocaleTimeString("zh-CN", {
                hour: "2-digit",
                minute: "2-digit",
              })}
            </div>
          </div>
        {/each}
      </div>

      <div class="input-area">
        <textarea
          bind:value={messageInput}
          on:keydown={handleKeydown}
          placeholder="输入消息..."
          rows="3"
        ></textarea>
        <button on:click={sendMessage} disabled={!messageInput.trim()}>
          发送
        </button>
      </div>
    {:else}
      <div class="no-chat">请选择一个联系人开始聊天</div>
    {/if}
  </div>
</div>

<style>
  .chat-container {
    display: grid;
    grid-template-columns: 300px 1fr;
    height: 100vh;
    background: #f5f5f5;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, sans-serif;
  }

  .contacts-panel {
    background: white;
    border-right: 1px solid #e0e0e0;
    display: flex;
    flex-direction: column;
    box-shadow: 2px 0 5px rgba(0, 0, 0, 0.05);
  }

  .search-box {
    padding: 1rem;
    border-bottom: 1px solid #e0e0e0;
    background: #f8f8f8;
  }

  .search-box input {
    width: 100%;
    padding: 0.75rem 1rem;
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    background: white;
    font-size: 0.9rem;
    transition: all 0.2s;
  }

  .search-box input:focus {
    outline: none;
    border-color: #1976d2;
    box-shadow: 0 0 0 2px rgba(25, 118, 210, 0.1);
  }

  .contacts-list {
    overflow-y: auto;
    flex: 1;
  }

  .contact-item {
    display: flex;
    padding: 1rem;
    gap: 1rem;
    cursor: pointer;
    transition: all 0.2s;
    border-bottom: 1px solid #f0f0f0;
  }

  .contact-item:hover {
    background: #f8f8f8;
  }

  .contact-item.active {
    background: #e3f2fd;
  }

  .avatar {
    position: relative;
    width: 48px;
    height: 48px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    flex-shrink: 0;
    overflow: hidden;
  }

  .online-status {
    position: absolute;
    bottom: 2px;
    right: 2px;
    width: 12px;
    height: 12px;
    background: #4caf50;
    border-radius: 50%;
    border: 2px solid white;
    box-shadow: 0 0 0 1px #4caf50;
  }

  .contact-info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .contact-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.25rem;
  }

  .name {
    font-weight: 600;
    color: #333;
  }

  .unread {
    background: #ff4444;
    color: white;
    padding: 0.25rem 0.5rem;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 600;
    min-width: 20px;
    text-align: center;
  }

  .last-message {
    color: #666;
    font-size: 0.875rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .chat-panel {
    display: flex;
    flex-direction: column;
    background: white;
  }

  .chat-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem 1.5rem;
    border-bottom: 1px solid #e0e0e0;
    background: white;
  }

  .contact-info {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .contact-details {
    display: flex;
    flex-direction: column;
  }

  .contact-details h2 {
    margin: 0;
    font-size: 1.1rem;
  }

  .status {
    font-size: 0.85rem;
    color: #666;
  }

  .call-actions {
    display: flex;
    gap: 0.75rem;
  }

  .call-btn {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    border: none;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s;
    color: white;
    font-size: 1rem;
  }

  .call-btn:hover {
    transform: translateY(-2px);
  }

  .voice {
    background: #4CAF50;
  }

  .voice:hover {
    background: #43A047;
  }

  .video {
    background: #2196F3;
  }

  .video:hover {
    background: #1E88E5;
  }

  .avatar {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    overflow: hidden;
  }

  .avatar-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .avatar-placeholder {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #1976d2;
    color: white;
    font-size: 1.2rem;
    font-weight: 500;
  }

  .messages {
    flex: 1;
    overflow-y: auto;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    background: #f8f8f8;
  }

  .message {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    max-width: 70%;
    animation: fadeIn 0.2s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .message.self {
    align-self: flex-end;
    align-items: flex-end;
  }

  .message-content {
    background: white;
    padding: 0.75rem 1rem;
    border-radius: 16px;
    position: relative;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
    line-height: 1.4;
  }

  .message.self .message-content {
    background: #1976d2;
    color: white;
  }

  .message-status {
    font-size: 0.75rem;
    color: #666;
    margin-top: 0.25rem;
  }

  .message.self .message-status {
    color: #999;
  }

  .message-time {
    font-size: 0.75rem;
    color: #999;
    margin-top: 0.25rem;
  }

  .input-area {
    padding: 1rem 1.5rem;
    border-top: 1px solid #e0e0e0;
    display: flex;
    gap: 1rem;
    background: white;
    box-shadow: 0 -1px 3px rgba(0, 0, 0, 0.05);
  }

  textarea {
    flex: 1;
    padding: 0.75rem 1rem;
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    resize: none;
    font-family: inherit;
    font-size: 0.95rem;
    line-height: 1.4;
    transition: all 0.2s;
  }

  textarea:focus {
    outline: none;
    border-color: #1976d2;
    box-shadow: 0 0 0 2px rgba(25, 118, 210, 0.1);
  }

  button {
    padding: 0.75rem 1.5rem;
    background: #1976d2;
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    align-self: flex-end;
    font-weight: 600;
    font-size: 0.95rem;
    transition: all 0.2s;
  }

  button:disabled {
    background: #e0e0e0;
    cursor: not-allowed;
  }

  button:hover:not(:disabled) {
    background: #1565c0;
    transform: translateY(-1px);
  }

  button:active:not(:disabled) {
    transform: translateY(0);
  }

  .no-chat {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #666;
    font-size: 1.25rem;
    background: #f8f8f8;
  }

  /* 滚动条样式 */
  ::-webkit-scrollbar {
    width: 6px;
  }

  ::-webkit-scrollbar-track {
    background: transparent;
  }

  ::-webkit-scrollbar-thumb {
    background: #ccc;
    border-radius: 3px;
  }

  ::-webkit-scrollbar-thumb:hover {
    background: #999;
  }

  .avatar-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    border-radius: 50%;
  }

  .avatar-placeholder {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #1976d2;
    color: white;
    font-size: 1.5rem;
    font-weight: 500;
    text-transform: uppercase;
    border-radius: 50%;
  }

  .video-container {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.9);
    z-index: 1000;
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 1rem;
  }

  .video-container video {
    max-width: 45%;
    border-radius: 8px;
  }

  .call-controls {
    display: flex;
    gap: 0.5rem;
  }

  .call-controls button {
    padding: 0.5rem;
    border-radius: 50%;
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
