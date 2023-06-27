import type { ToastType } from "./toast";

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
