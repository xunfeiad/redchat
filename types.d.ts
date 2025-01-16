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
    receiverId: number;
    senderName: string;
    sdp: string;
}

export interface DisconnectContent {
    userId: number;
}

export type UserId = number;