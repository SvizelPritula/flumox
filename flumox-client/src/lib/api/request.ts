export type BadResponseType = "request" | "database" | "config" | "server" | "unknown";

export class BadResponseError extends Error {
    type: BadResponseType;

    constructor(code: number, text: string, type: BadResponseType) {
        super(`Server responded with status code ${code}: ${text}`);

        this.name = "BadResponseError";
        this.type = type;
    }
}

interface InternalErrorResponse {
    reason: BadResponseType
}

export async function assertResponseOk(response: Response) {
    if (!response.ok) {
        let status = response.status;
        let type: BadResponseType = "unknown";

        if (status >= 400 && status < 500) {
            type = "request";
        } else if (status >= 500) {
            type = "server";

            if (status == 500) {
                try {
                    let payload: InternalErrorResponse = await response.json();
                    type = payload.reason;
                } catch (error) {
                    console.error(error);
                }
            }
        }

        throw new BadResponseError(status, response.statusText, type);
    }
}

const auth_header = "x-auth-token";

export async function get(path: string, token?: string) {
    let headers = new Headers();

    if (token != null)
        headers.append(auth_header, token);

    let response = await fetch(path, {
        method: "GET",
        headers,
    });

    await assertResponseOk(response);

    return await response.json();
}

export async function post(path: string, body: any, token?: string) {
    let headers = new Headers();

    if (token != null)
        headers.append(auth_header, token);

    headers.append("content-type", "application/json");

    let response = await fetch(path, {
        method: "POST",
        headers,
        body: JSON.stringify(body),
    });

    await assertResponseOk(response);

    return await response.json();
}
