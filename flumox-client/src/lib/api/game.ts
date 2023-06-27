import type { Views } from "../view";
import { get } from "./request";

export function view(token: string): Promise<Views> {
    return get("/api/view", token);
}
