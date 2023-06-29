export type Connect = (retry: Reconnect) => Disconnect | null;
export type Reconnect = () => void;
export type Disconnect = () => void;

const DELAY = 15000;

export function reconnecting(connect: Connect, onDisconnect: Disconnect): Disconnect {
    let close = null;
    let timeout = null;
    let closed = false;

    function start() {
        if (closed)
            return;

        timeout = null;
        let disconnected = false;

        function retry() {
            if (disconnected || closed)
                return;

            disconnected = true;

            onDisconnect();

            if (close != null)
                close();

            close = null;

            timeout = setTimeout(start, DELAY);
        }

        close = connect(retry);
    }

    function onOnline() {
        if (timeout != null) {
            clearInterval(timeout);
            start();
        }
    }

    start();

    window.addEventListener("online", onOnline);

    return () => {
        if (closed)
            return;
        closed = true;

        window.removeEventListener("online", onOnline);

        if (timeout != null)
            clearTimeout(timeout);

        onDisconnect();

        if (close != null)
            close();

        close = null;
    }
}
