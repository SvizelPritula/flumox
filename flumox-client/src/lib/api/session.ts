import type { SessionToken, TeamInfo } from "../team";
import { post } from "./request";

type LoginResult = { result: "success", token: SessionToken, team: TeamInfo } | { result: "incorrect-code" };

export function login(accessCode: string): Promise<LoginResult> {
    return post("/api/login", { access_code: accessCode });
}
