import type { Action, ActionResponse } from "../action";
import type { Views } from "../view";
import { get, post } from "./request";

export function view(token: string): Promise<Views> {
    return get("/api/view", token);
}

export function submit(token: string, action: Action): Promise<ActionResponse> {
    return post("/api/action", action, token);
}
