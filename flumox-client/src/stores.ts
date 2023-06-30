import { writable } from "svelte/store";
import { persistent } from "./lib/persistent";
import type { Session } from "./lib/team";
import type { Toast } from "./lib/toast";
import type { Instances } from "./lib/view";

export const session = persistent<Session | null>("session", null);
export const view = persistent<Instances | null>("view", null);
export const online = writable<boolean>(false);
export const toasts = writable<Toast[]>([]);
