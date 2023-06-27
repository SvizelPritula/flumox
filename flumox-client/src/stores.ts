import { writable } from "svelte/store";
import { persistent } from "./lib/persistent";
import type { Session } from "./lib/team";
import type { Toast } from "./lib/toast";

export const session = persistent<Session | null>("session", null);
export const toasts = writable<Toast[]>([]);
