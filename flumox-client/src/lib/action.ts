import { toast, type ToastType } from "./toast";
import { submit as submitRequest } from "./api/game"
import { errorDispatchFailed, errorNotPossible, hintTaken, solutionCorrect, solutionIncorrect } from "$translations";

export interface AnswerAction {
    type: "answer",
    answer: string
}

export interface HintAction {
    type: "hint",
    ident: string
}

export interface Metadata {
    widget: string
}

export type Action = (AnswerAction | HintAction) & Metadata;

export type ActionResponse = {
    result: "success",
    toast?: {
        message: Message,
        type: ToastType
    }
} | { result: "not-possible" } | { result: "dispatch-failed" };

export type Message = {
    "type": "custom", value: string
} | { type: PredefinedMessage };
export type PredefinedMessage = "solution-correct" | "solution-incorrect" | "hint-taken";

export function messageToString(message: Message): string {
    if (message.type == "custom")
        return message.value;
    if (message.type == "solution-correct")
        return solutionCorrect;
    if (message.type == "solution-incorrect")
        return solutionIncorrect;
    if (message.type == "hint-taken")
        return hintTaken;

}

export async function submit(payload: Action, token: string) {
    let response = await submitRequest(token, payload);

    if (response.result == "success") {
        if (response.toast != null) {
            toast(messageToString(response.toast.message), response.toast.type);
        }
    } else if (response.result == "dispatch-failed") {
        toast(errorDispatchFailed, "danger");
    } else if (response.result == "not-possible") {
        toast(errorNotPossible, "danger");
    }
}
