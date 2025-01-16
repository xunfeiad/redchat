export interface Response<T>{
    code: number;
    message: string;
    data: T;
}

export interface WebSocketMessage{
    type: "text" | "auth" | "webrtc";
    content: {
        receiverId?: number;
        message?: string;
        groupId?: number;
        userId?: number;
        token?: string;
        sdp?: string;
        senderName?: string;
    };
}
export type UserId = number;