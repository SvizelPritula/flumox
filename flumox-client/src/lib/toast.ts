import { toasts } from "../stores";

export type ToastType = "danger" | "warning" | "success" | "status";

export interface Toast {
    text: string,
    type: ToastType,
    key: string
}

const TOAST_EXPIRATION_TIME = 10 * 1000;

let nextId: number = 0;

export function dismiss(key: string) {
    toasts.update(toasts => toasts.filter(t => t.key != key))
}

export function toast(text: string, type: ToastType) {
    let key = `\0${nextId++}`;
    let toast = { text, type, key };

    setTimeout(() => {
        dismiss(toast.key);
    }, TOAST_EXPIRATION_TIME);

    toasts.update(toasts => [...toasts, toast]);
}
