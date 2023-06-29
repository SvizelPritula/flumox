export type Connect = (retry: Reconnect) => Disconnect | null;
export type Reconnect = () => void;
export type Disconnect = () => void;

const DELAY = 3000;

export function reconnecting(connect: Connect, onDisconnect: Disconnect): Disconnect {
    let close = null;

    function start() {
        let disconnected = false;

        function retry() {
            if (disconnected)
                return;

            disconnected = true;

            onDisconnect();

            if (close != null)
                close();

            close = null;

            setTimeout(start, DELAY);
        }

        close = connect(retry);
    }

    start();

    return () => {
        onDisconnect();

        if (close != null)
            close();

        close = null;
    }
}
