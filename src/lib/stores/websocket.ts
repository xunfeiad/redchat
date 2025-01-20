import { get, writable } from 'svelte/store';
import type { UserId, WebSocketMessage, TextContent, AuthContent, WebRTCContent, UserInfo } from '../../../types';
import {goto} from '$app/navigation';
export const wsStatus = writable<'connecting' | 'open' | 'closed'>('closed');
export const wsMessages = writable<WebSocketMessage<TextContent | AuthContent | WebRTCContent>>();

export interface WebSocketParams {
  userId: UserId;
}

class WebSocketClient {
  private ws: WebSocket | null = null;
  private url: string;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectTimeout = 3000;

  constructor(url: string) {
    this.url = url;
  }
  getUserInfo(){
    const userInfo: UserInfo = JSON.parse(localStorage.getItem('userInfo') || '{}');
    if(!userInfo.id){
      goto('/login');
      return;
    }
    return userInfo;
  }

  connect() {
    try {
      let url = this.url;
      const userInfo: UserInfo | undefined = this.getUserInfo();
      const params = {
        userId: userInfo?.id.toString() || '',
      };
      const paramsString = new URLSearchParams(params).toString();
      url = url + '?' + paramsString;
      this.ws = new WebSocket(url);
      wsStatus.set('connecting');
      this.ws.onopen = () => {
        console.log('WebSocket 连接成功');
        if (get(wsStatus) === 'open') {
          return;
        }
        wsStatus.set('open');
        this.reconnectAttempts = 0;
      };

      this.ws.onmessage = (event) => {
        wsMessages.update(message => message = JSON.parse(event.data));
      };

      this.ws.onclose = () => {
        console.log('WebSocket 连接关闭');
        wsStatus.set('closed');
        this.reconnect();
      };

      this.ws.onerror = (error) => {
        console.error('WebSocket 错误:', error);
        this.ws?.close();
      };
    } catch (error) {
      console.error('WebSocket 连接失败:', error);
    }
  }

  private reconnect() {
    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      const userInfo: UserInfo | undefined = this.getUserInfo();
      this.reconnectAttempts++;
      console.log(`尝试重新连接... (${this.reconnectAttempts}/${this.maxReconnectAttempts})`);
      setTimeout(() =>{
        this.connect();
        this.send({
          type: 'auth',
          authMessage: {
            user_id: userInfo?.id.toString() || '',
          },
        });
      }, this.reconnectTimeout);
    }
  }

  send(message: any) {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(message));
      return true;
    }
    return false;
  }

  close() {
    this.ws?.close();
  }
}

export const wsClient = new WebSocketClient('ws://192.168.1.4:8080/ws'); 