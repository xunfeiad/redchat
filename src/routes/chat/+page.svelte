<script lang="ts">
  import { onMount, onDestroy, tick } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import type {
    Response,
    WebSocketMessage,
    TextContent,
    AuthContent,
    WebRTCContent,
    DisconnectContent,
    Message,
    Contact,
    UserInfo,
  } from "../../../types";
  import audio from "$lib/assets/remind_audio.wav";
  import { wsClient, wsStatus, wsMessages } from "$lib/stores/websocket";
  import { get } from "svelte/store";

  // 消息
  let messages: Message[] = $state([]);
  // 联系人
  let contacts: Contact[] = $state([]);
  // 当前联系人
  let currentContact: Contact | null = $state(null);
  // 输入框
  let messageInput = $state("");
  let chatContainer: HTMLElement | null = $state(null);
  let errorMessage = $state("");

  // WebRTC 相关状态
  let localStream: MediaStream | null = $state(null);  
  const iceServers = [{ urls: "stun:stun.l.google.com:19302" }];

  let peers: Map<number, RTCPeerConnection> = $state(new Map());
  let peedingCandidates: Map<number, RTCIceCandidate[]> = $state(new Map());

  // 视频元素引用
  let localVideo: HTMLVideoElement | null = $state(null);
  let remoteVideo: HTMLVideoElement | null = $state(null);
  // 用户id
  let userId: number = $state(0);

  let userInfo: UserInfo | null = $state(null);
  // 是否显示来电
  let showIncomingCall = $state(false);
  // 来电用户名
  let callerName = $state("");
  // 来电类型
  let callType: "voice" | "video" | null = $state(null);
  // 是否是视频通话
  let isVideoCall = $state(false);

  const get_contacts = async () => {
    try {
      let res: Response<Contact[]> = await invoke("get_contacts", {
        userId: userId,
      });
      contacts = [...res.data];
      console.log("contacts received....:", contacts);
    } catch (error) {
      console.error("Failed to load contacts:", error);
      errorMessage = "加载联系人失败";
    }
  };

  async function getLocalStream(video: boolean, audio: boolean){
    localStream = await navigator.mediaDevices.getUserMedia({
      video,
      audio,
    });
    wsClient.connect();
  }

  async function createPeerConnection(){
    const peer = new RTCPeerConnection({ iceServers: iceServers });
    peer.onicecandidate = onIceCandidate;
    peer.ontrack = onAddStream;
    return peer;
  }

  async function onIceCandidate(event: RTCPeerConnectionIceEvent){
    if (event.candidate) {
        console.log('ICE candidate');
          wsClient.send({
              type: 'webrtc',
              content: {
                  receiverId: currentContact?.id,
                  senderName: userInfo?.nickname || userInfo?.username || "未知用户",
                  content: JSON.stringify(event.candidate),
                  sdpType: "candidate",
                  sid: userId
              }
          });
    }
  }

  async function onAddStream(event: RTCTrackEvent){
    console.log('Add stream');
    const newRemoteStreamElem = document.createElement('video');
    newRemoteStreamElem.autoplay = true;
    newRemoteStreamElem.srcObject = event.streams[0];
    remoteVideo = newRemoteStreamElem;
  }

  async function addIceCandidate(id: number){
    if (peedingCandidates.has(id)) {
        peedingCandidates.get(id)?.forEach(candidate => {
            peers.get(id)?.addIceCandidate(new RTCIceCandidate(candidate))
        });
    }
  }

  async function sendAnswer(id: number){
    console.log('Send answer');
    const answer = await peers.get(id)?.createAnswer();
    try{
      if(answer){
        await setAndSendLocalDescription(id, answer);
      }
    }catch(error){
      console.error('Send answer failed: ', error);
    }
  }

  async function setAndSendLocalDescription(id: number, sdp: RTCSessionDescriptionInit){
    peers.get(id)?.setLocalDescription(sdp);
    console.log('Local description set');
    wsClient.send({
        type: 'webrtc',
        content: {
            receiverId: currentContact?.id,
            senderName: userInfo?.nickname || userInfo?.username || "未知用户",
            content: sdp.sdp,
            sdpType: sdp.type,
            sid: userId
        }
    });
  }

  async function addPendingCandidates(sid: number){
    if (peedingCandidates.has(sid)) {
        peedingCandidates.get(sid)?.forEach(candidate => {
            peers.get(sid)?.addIceCandidate(new RTCIceCandidate(candidate))
        });
    }
  }

  async function handleSignalingMessage(message: WebRTCContent){
    console.log(message)
    const sid = message.sid;
    switch (message.sdpType) {
        case 'offer':
            const peer = await createPeerConnection();
            peers.set(sid, peer);
            peers.get(sid)?.setRemoteDescription(new RTCSessionDescription({
              type: message.sdpType,
              sdp: message.content,
            }));
            sendAnswer(sid);
            addPendingCandidates(sid);
            break;
        case 'answer':
            peers.get(sid)?.setRemoteDescription(new RTCSessionDescription({
              type: message.sdpType,
              sdp: message.content,
            }));
            break;
        case 'candidate':
            if(message.content){
              const candidate = JSON.parse(message.content);
              if (sid in peers) {
                  peers.get(sid)?.addIceCandidate(new RTCIceCandidate(candidate));
              } else {
                if (!(sid in peedingCandidates)) {
                  peedingCandidates.set(sid, []);
                }
                peedingCandidates.get(sid)?.push(candidate)
            }
            break;
    }
  }
}


  onMount(async () => {
    console.log(wsClient);
    userInfo = JSON.parse(localStorage.getItem("userInfo") || "{}") as UserInfo;
    userId = userInfo?.id || 0;

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
    wsMessages.subscribe(
      async (
        message: WebSocketMessage<TextContent | AuthContent | WebRTCContent>,
      ) => {
        console.log("Received message:", message);
        if (get(wsStatus) !== "open") return;

        switch (message.type) {
          case "text":
            messages = [
              ...messages,
              {
                content: (message.content as TextContent).message,
                type: "text",
                isSelf: false,
                timestamp: new Date(),
                status: "sent",
              },
            ];
            await get_contacts();
            break;

          case "auth":
            const authUserId = (message.content as AuthContent).userId;
            contacts = contacts.map((contact) =>
              contact.id === authUserId
                ? { ...contact, online: true }
                : contact,
            );
            break;
          case "webrtc":
            showIncomingCall = true;
            callerName =
              (message.content as WebRTCContent).senderName || "未知用户";
            let audioPlayer = document.getElementById(
              "remoteContactRemind",
            ) as HTMLAudioElement;
            audioPlayer.addEventListener("ended", () => {
              audioPlayer.currentTime = 0;
              audioPlayer.play();
            });
            audioPlayer.play();

            const rtc_type = (message.content as WebRTCContent).sdpType;
            switch (rtc_type) {
              case "offer":
                await handleSignalingMessage(message.content as WebRTCContent);
                break;
              case "answer":
                await handleSignalingMessage(message.content as WebRTCContent);
                break;
              case "candidate":
                await handleSignalingMessage(message.content as WebRTCContent);
                break;
            }
            break;
          case "disconnect":
            console.log("remote user disconnected.", message);
            const disConnectUserId = (message.content as DisconnectContent)
              .userId;
            contacts = contacts.map((contact) =>
              contact.id === disConnectUserId
                ? { ...contact, online: false }
                : contact,
            );
            break;
        }
      },
    );
    try {
      await get_contacts();
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
 

  // 开始视频通话
  async function startVideoCall() {
    if (!currentContact) return;
    await getLocalStream(true, true);
  }

  // 开始语音通话
  async function startVoiceCall() {
    if (!currentContact) return;
    await getLocalStream(false, true);
  }

  function handleRejectCall() {
    showIncomingCall = false;
    let audioPlayer = document.getElementById(
      "remoteContactRemind",
    ) as HTMLAudioElement;
    audioPlayer.pause();
    audioPlayer.currentTime = 0;
    // 处理拒绝通话逻辑...
  }
  async function handleAcceptCall() {
    showIncomingCall = false;
    let audioPlayer = document.getElementById(
      "remoteContactRemind",
    ) as HTMLAudioElement;
    audioPlayer.pause();
    audioPlayer.currentTime = 0;
  }

  function endCall() {
    localStream?.getTracks().forEach((track) => track.stop());
    for(const peer of peers.values()){
      peer?.close();
    }
    localStream = null;
    isVideoCall = false;
  }

  function toggleCamera() {
    const videoTrack = localStream?.getVideoTracks()[0];
    if (videoTrack) {
      videoTrack.enabled = !videoTrack.enabled;
    }
  }

  function toggleMic() {
    const audioTrack = localStream?.getAudioTracks()[0];
    if (audioTrack) {
      audioTrack.enabled = !audioTrack.enabled;
    }
  }

  onDestroy(() => {
    wsClient.close();
    localStream?.getTracks().forEach((track) => track.stop());
    for(const peer of peers.values()){
      peer?.close();
    }
  });
</script>

<audio id="remoteContactRemind" src={audio}></audio>

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
          onclick={() => selectContact(contact)}
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
              <img
                src={currentContact.avatar}
                alt={currentContact.name}
                class="avatar-img"
              />
            {:else}
              <div class="avatar-placeholder">
                {currentContact.name[0]}
              </div>
            {/if}
          </div>
          <div class="contact-details">
            <h2>{currentContact.name}</h2>
            <span class="status">{currentContact.online ? "在线" : "离线"}</span
            >
          </div>
        </div>

        <div class="call-actions">
          <button
            class="call-btn voice"
            onclick={startVoiceCall}
            title="语音通话"
          >
            <i class="fas fa-phone">语音通话</i>
          </button>
          <button
            class="call-btn video"
            onclick={startVideoCall}
            title="视频通话"
          >
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
          onkeydown={handleKeydown}
          placeholder="输入消息..."
          rows="3"
        ></textarea>
        <button onclick={sendMessage} disabled={!messageInput.trim()}>
          发送
        </button>
      </div>
    {:else}
      <div class="no-chat">请选择一个联系人开始聊天</div>
    {/if}
  </div>
</div>

{#if showIncomingCall}
  <div class="modal-backdrop">
    <div class="incoming-call-modal">
      <div class="call-avatar">
        <i class="fas fa-{callType === 'voice' ? 'phone' : 'video'}"
          >{callerName}</i
        >
      </div>
      <div class="call-info">
        <h3>{callerName}</h3>
        <p>正在发起{callType === "voice" ? "语音" : "视频"}通话...</p>
      </div>
      <div class="call-actions">
        <button class="reject-btn" onclick={handleRejectCall}>
          <i class="fas fa-phone-slash"></i>
          <span>拒绝</span>
        </button>
        <button class="accept-btn" onclick={handleAcceptCall}>
          <i class="fas fa-phone"></i>
          <span>接听</span>
        </button>
      </div>
    </div>
  </div>
{/if}

{#if localStream}
  <div class="video-call-modal">
    <div class="video-container">
      <div class="video-wrapper remote" id="remoteVideo">
        <video bind:this={remoteVideo} autoplay playsinline>
          <track kind="captions" srclang="en" label="English" />
        </video>
      </div>

      <div class="video-wrapper local" id="localVideo">
        <video bind:this={localVideo} autoplay playsinline>
          <track kind="captions" srclang="en" label="English" />
        </video>
      </div>
    </div>
    </div>

    <div class="call-controls">
      <button class="control-btn end" onclick={endCall} aria-label="结束通话">
        <i class="fas fa-phone-slash"></i>
      </button>
      {#if isVideoCall}
        <button
          class="control-btn camera"
          onclick={toggleCamera}
          aria-label="切换摄像头"
        >
          <i class="fas fa-video"></i>
        </button>
      {/if}
      <button
        class="control-btn mic"
        onclick={toggleMic}
        aria-label="切换麦克风"
      >
        <i class="fas fa-microphone"></i>
      </button>
    </div>
{/if}

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
    background: #4caf50;
  }

  .voice:hover {
    background: #43a047;
  }

  .video {
    background: #2196f3;
  }

  .video:hover {
    background: #1e88e5;
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

  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    backdrop-filter: blur(5px);
    animation: fadeIn 0.3s ease-out;
  }

  .incoming-call-modal {
    background: white;
    border-radius: 16px;
    padding: 2rem;
    width: 320px;
    text-align: center;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
    animation: slideIn 0.3s ease-out;
  }

  .call-avatar {
    width: 80px;
    height: 80px;
    background: #f0f2f5;
    border-radius: 50%;
    margin: 0 auto 1rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .call-avatar i {
    font-size: 2rem;
    color: #1976d2;
  }

  .call-info h3 {
    margin: 0;
    font-size: 1.25rem;
    color: #1a1a1a;
  }

  .call-info p {
    margin: 0.5rem 0 1.5rem;
    color: #666;
  }

  .call-actions {
    display: flex;
    gap: 1rem;
    justify-content: center;
  }

  .call-actions button {
    width: 120px;
    height: 48px;
    border: none;
    border-radius: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .accept-btn {
    background: #4caf50;
    color: white;
  }

  .accept-btn:hover {
    background: #43a047;
    transform: translateY(-2px);
  }

  .reject-btn {
    background: #f44336;
    color: white;
  }

  .reject-btn:hover {
    background: #e53935;
    transform: translateY(-2px);
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .video-call-modal {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: #000;
    z-index: 1100;
    display: flex;
    flex-direction: column;
  }

  .video-container {
    position: relative;
    width: 100%;
    height: 100%;
  }

  .video-wrapper {
    width: 100%;
    height: 100%;
  }

  .video-wrapper.remote {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
  }

  .video-wrapper.local {
    position: absolute;
    width: 240px;
    height: 180px;
    right: 20px;
    bottom: 100px;
    border-radius: 12px;
    overflow: hidden;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    border: 2px solid rgba(255, 255, 255, 0.2);
  }

  .video-wrapper video {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .call-controls {
    position: absolute;
    bottom: 40px;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    gap: 20px;
    padding: 16px 24px;
    background: rgba(0, 0, 0, 0.6);
    border-radius: 50px;
    backdrop-filter: blur(10px);
  }

  .control-btn {
    width: 56px;
    height: 56px;
    border-radius: 50%;
    border: none;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(255, 255, 255, 0.1);
    color: white;
    cursor: pointer;
    transition: all 0.2s;
  }

  .control-btn:hover {
    background: rgba(255, 255, 255, 0.2);
    transform: translateY(-2px);
  }

  .control-btn.end {
    background: #dc3545;
  }

  .control-btn.end:hover {
    background: #c82333;
  }

  .control-btn i {
    font-size: 24px;
  }
</style>
