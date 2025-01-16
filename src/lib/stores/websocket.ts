import { writable } from 'svelte/store';
import type { UserId } from '../../../types';

export const wsStatus = writable<'connecting' | 'open' | 'closed'>('closed');
export const wsMessages = writable<any[]>([]);

export interface WebSocketParams {
  userId: UserId;
}

class WebSocketClient {
  private ws: WebSocket | null = null;
  private url: string;
  private params?: Record<string, string>;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectTimeout = 3000;

  constructor(url: string, params?: Record<string, string>) {
    this.url = url;
    this.params = params;
  }

  connect(params?: Record<string, string>) {
    try {
      let url = this.url;
      if (params) {
        const paramsString = new URLSearchParams(params).toString();
        url = url + '?' + paramsString;
      }
      this.ws = new WebSocket(url);
      wsStatus.set('connecting');

      this.ws.onopen = () => {
        console.log('WebSocket 连接成功');
        wsStatus.set('open');
        this.reconnectAttempts = 0;
      };

      this.ws.onmessage = (event) => {
        const message = JSON.parse(event.data);
        wsMessages.update(messages => [...messages, message]);
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
      this.reconnectAttempts++;
      console.log(`尝试重新连接... (${this.reconnectAttempts}/${this.maxReconnectAttempts})`);
      setTimeout(() => this.connect(), this.reconnectTimeout);
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
    this.ws?.close(1000, 'close');
  }
}

export const wsClient = new WebSocketClient('ws://192.168.1.4:8080/ws'); 