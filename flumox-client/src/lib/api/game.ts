import type { Action, ActionResponse } from "../action";
import type { Instances } from "../view";
import { get, post } from "./request";

export function view(token: string): Promise<Instances> {
    return get("/api/view", token);
}

export function submit(token: string, action: Action): Promise<ActionResponse> {
    return post("/api/action", action, token);
}
