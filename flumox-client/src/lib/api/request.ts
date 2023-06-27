export function assertResponseOk(response: Response) {
    if (!response.ok) {
        throw new Error(`Bad status code: ${response.status} ${response.statusText}`);
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

    assertResponseOk(response);

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

    assertResponseOk(response);

    return await response.json();
}
