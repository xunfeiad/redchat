export interface Response<T>{
    code: number;
    message: string;
    data: T;
}

export type UserId = number;