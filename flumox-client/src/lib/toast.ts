export type ToastType = "danger" | "warning" | "success";
export type Toaster = (text: string, type: ToastType) => void;

export interface Toast {
    text: string,
    type: ToastType,
    key: string
}

export const TOAST_EXPIRATION_TIME = 10 * 1000;
