import type { Writable } from "svelte/store";
import type { Instances } from "../view";
import { reconnecting } from "../connect/reconnect";
import { toast } from "../toast";
import { logout } from "../team";
import { errorMalformedMessage, errorServerRejected, warningSessionExpired } from "$translations";
import type { BadResponseType } from "./request";
import { getErrorMessageForType } from "../error";

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
    widgets: Instances
}

interface ErrorMessage {
    type: "error",
    reason: BadResponseType
}

type IncomingMessage = MalformedMessageMessage | UnknownTokenMessage | ViewMessage | ErrorMessage;

export function sync(view: Writable<Instances | null>, online: Writable<boolean>, token: string): () => void {
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
                        toast(errorServerRejected, "danger");
                        break;

                    case "unknown-token":
                        toast(warningSessionExpired, "warning");
                        logout();
                        break;

                    case "view":
                        online.set(true);
                        view.set(payload.widgets);
                        break;

                    case "error":
                        toast(getErrorMessageForType(payload.reason), "danger");
                        break;

                    default:
                        toast(errorMalformedMessage, "danger");
                }
            } catch (error) {
                console.error(error);
                toast(errorMalformedMessage, "danger");
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
