export function assertResponseOk(response: Response) {
    if (!response.ok) {
        throw new Error(`Bad status code: ${response.status} ${response.statusText}`);
    }
}
