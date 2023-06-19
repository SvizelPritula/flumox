import type { Session, SessionToken, TeamInfo } from "../team";
import { assertResponseOk } from "./errors";

type LoginResult = { result: "success", token: SessionToken, team: TeamInfo } | { result: "incorrect-code" };

export async function login(accessCode: string): Promise<LoginResult> {
    let response = await fetch("/api/login", {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ access_code: accessCode }),
    });

    assertResponseOk(response);

    return await response.json();
}
