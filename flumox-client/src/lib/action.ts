import { toast, type ToastType } from "./toast";
import { submit as submitRequest } from "./api/game"

export interface AnswerAction {
    type: "answer",
    answer: string
}

export interface Metadata {
    widget: string
}

export type Action = AnswerAction & Metadata;

export type ActionResponse = {
    result: "success",
    toast?: {
        text: string,
        type: ToastType
    }
} | { result: "not-possible" } | { result: "dispatch-failed" };

export async function submit(payload: Action, token: string) {
    let response = await submitRequest(token, payload);

    if (response.result == "success") {
        if (response.toast != null) {
            toast(response.toast.text, response.toast.type);
        }
    } else if (response.result == "dispatch-failed") {
        toast(
            "Failed to process action due to game configuration being out of sync",
            "danger"
        );
    } else if (response.result == "not-possible") {
        toast(
            "Failed to process action due to game state being out of sync",
            "danger"
        );
    }
}
