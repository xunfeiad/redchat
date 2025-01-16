export interface Response<T>{
    code: number;
    message: string;
    data: T;
}

export interface WebSocketMessage<T extends TextContent | AuthContent | WebRTCContent> {
    type: "text" | "auth" | "webrtc";
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
    sdp: string;
}

export type UserId = number;