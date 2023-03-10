import type { IMessageType } from "./types/server";

export const getMsgType = (category: string, type: string): IMessageType => {
    const obj: IMessageType = {};
    obj[category] = type;
    return obj
};
