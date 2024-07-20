export async function deflate(payload: Blob): Promise<string> {
    let decompressor = new DecompressionStream("deflate-raw");
    let decoder = new TextDecoderStream("utf-8");

    let reader = payload.stream().pipeThrough(decompressor).pipeThrough(decoder).getReader();

    let result = "";

    while (true) {
        let chunk = await reader.read();
        if (chunk.done) break;

        result += chunk.value;
    }

    return result;
}

export async function maybeDeflate(payload: Blob | string): Promise<string> {
    if (typeof payload == "string")
        return payload;
    else
        return await deflate(payload);
}

export function isDeflateSupported(): boolean {
    try {
        new DecompressionStream("deflate-raw");
        return true;
    } catch { }

    return false;
}
