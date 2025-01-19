export interface Response<T>{
    code: number;
    message: string;
    data: T;
}

export interface WebSocketMessage<T extends TextContent | AuthContent | WebRTCContent | DisconnectContent> {
    type: "text" | "auth" | "webrtc" | "disconnect";
    content: T;
}

export interface TextContent {
    receiverId?: number;
    message: string;
    groupId?: number;
}

export interface AuthContent {
    userId: number;
    token?: string;
}   

export interface WebRTCContent {
    receiverId?: number;
    senderName?: string;
    callType?: "video" | "voice";
    sdpType: "offer" | "answer" | "candidate";
    content: string;
}

export interface DisconnectContent {
    userId: number;
}

export interface Contact {
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
export interface Message {
    id?: number;
    content: string;
    type: "text" | "image" | "file";
    isSelf: boolean;
    timestamp: Date;
    status: "sending" | "sent" | "failed";
  }

export interface UserInfo {
    id: number;
    username: string;
    password: string;
    avatar?: string;
    nickname?: string;
    phone?: string;
    email: string;
    gender?: string;
    birthday?: string;
    address?: string;
    signature?: string;
    online: boolean;
    is_deleted: boolean;
    created_at: Date;
    updated_at: Date;
    last_login_at: Date;
    status: number;
}

export type UserId = number;