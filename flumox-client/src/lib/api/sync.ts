import type { Writable } from "svelte/store";
import type { Instance, InstanceDelta, Instances } from "../view";
import { reconnecting } from "../connect/reconnect";
import { toast } from "../toast";
import { logout } from "../team";
import { errorMalformedMessage, errorServerRejected, warningSessionExpired } from "$translations";
import type { BadResponseType } from "./request";
import { getErrorMessageForType } from "../error";
import { isDeflateSupported, maybeDeflate } from "../deflate";
import { createWatchdog } from "../connect/watchdog";

interface LoginMessage {
    type: "auth",
    token: string,
    compress: boolean,
}

interface PingMessage {
    type: "ping"
}

type OutgoingMessage = LoginMessage | PingMessage;

interface MalformedMessageMessage {
    type: "malformed-message"
}

interface UnknownTokenMessage {
    type: "unknown-token"
}

interface ViewMessage {
    type: "view",
    widgets: InstanceDelta[]
}

interface PongMessage {
    type: "pong"
}

interface ErrorMessage {
    type: "error",
    reason: BadResponseType
}

type IncomingMessage = MalformedMessageMessage | UnknownTokenMessage | ViewMessage | PongMessage | ErrorMessage;

export function sync(view: Writable<Instances | null>, online: Writable<boolean>, token: string): () => void {
    online.set(false);

    return reconnecting(retry => {
        let url = new URL("/api/sync", window.location.href);
        url.protocol = url.protocol == 'https:' ? 'wss' : 'ws';

        let socket = new WebSocket(url);

        let watchdog = createWatchdog(() => {
            socket.send(JSON.stringify(<OutgoingMessage>{
                type: "ping",
            }));
        }, retry);

        let oldInstances: Instances = [];

        socket.addEventListener("open", () => {
            socket.send(JSON.stringify(<OutgoingMessage>{
                type: "auth",
                token,
                compress: isDeflateSupported()
            }));
        });

        socket.addEventListener("message", async event => {
            try {
                let data = await maybeDeflate(event.data);
                let payload: IncomingMessage = JSON.parse(data);

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

                        let instances = applyDelta(payload.widgets, oldInstances);

                        oldInstances = instances;
                        view.set(instances);
                        break;

                    case "pong":
                        break

                    case "error":
                        toast(getErrorMessageForType(payload.reason), "danger");
                        break;

                    default:
                        toast(errorMalformedMessage, "danger");
                }

                watchdog.feed();
            } catch (error) {
                console.error(error);
                toast(errorMalformedMessage, "danger");
            }
        });

        socket.addEventListener("error", () => {
            retry();
        });

        socket.addEventListener("close", retry);

        return () => { socket.close(); watchdog.stop(); };
    }, () => {
        online.set(false);
    });
}

function applyDelta(delta: InstanceDelta[], old: Instance[]): Instance[] {
    let oldMap = new Map();

    for (let { id, view } of old) {
        oldMap.set(id, view);
    }

    let instances: Instance[] = [];

    for (let { id, view, obsolete } of delta) {
        if (view != null) {
            instances.push({ id, view, obsolete })
        } else {
            let oldView = oldMap.get(id);

            if (oldView != null)
                instances.push({ id, view: oldView, obsolete });
        }
    }

    return instances;
}
