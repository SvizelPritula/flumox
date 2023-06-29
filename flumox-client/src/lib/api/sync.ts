import type { Writable } from "svelte/store";
import type { Views } from "../view";
import { reconnecting } from "../connect/reconnect";
import { toast } from "../toast";
import { logout } from "../team";

interface LoginMessage {
    type: "auth",
    token: string
}

type OutgoingMessage = LoginMessage;

interface MalformedMessageMessage {
    type: "malformed-message"
}

interface UnknownTokenMessage {
    type: "unknown-token"
}

interface ViewMessage {
    type: "view",
    widgets: Views
}

type IncomingMessage = MalformedMessageMessage | UnknownTokenMessage | ViewMessage;

export function sync(view: Writable<Views | null>, online: Writable<boolean>, token: string): () => void {
    online.set(false);

    return reconnecting(retry => {
        let url = new URL("/api/sync", window.location.href);
        url.protocol = url.protocol.endsWith('s') ? 'wss' : 'ws';

        let socket = new WebSocket(url);

        socket.addEventListener("open", () => {
            socket.send(JSON.stringify(<OutgoingMessage>{
                type: "auth",
                token
            }));
        });

        socket.addEventListener("message", event => {
            try {
                let payload: IncomingMessage = JSON.parse(event.data);

                switch (payload.type) {
                    case "malformed-message":
                        toast("Server rejected message", "danger");
                        break;

                    case "unknown-token":
                        toast("Session expired", "warning");
                        logout();
                        break;

                    case "view":
                        online.set(true);
                        view.set(payload.widgets);
                        break;
                }
            } catch (error) {
                console.error(error);
                toast("Received malformed message", "danger");
            }
        });

        socket.addEventListener("error", () => {
            retry();
        });

        socket.addEventListener("close", retry);

        return () => socket.close();
    }, () => {
        online.set(false);
    });
}
