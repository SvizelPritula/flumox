const expireTime = 80 * 1000;
const warningTime = 60 * 1000;

export interface Watchdog {
    feed(): void,
    stop(): void,
}

export function createWatchdog(warning: () => void, expire: () => void): Watchdog {
    let warningTimeout = null;
    let expireTimeout = null;

    function start() {
        warningTimeout = setTimeout(() => { warningTimeout = null; warning(); }, warningTime);
        expireTimeout = setTimeout(() => { expireTimeout = null; expire(); }, expireTime);
    }

    function stop() {
        if (warningTimeout != null) clearTimeout(warningTimeout);
        if (expireTimeout != null) clearTimeout(expireTimeout);
        warningTimeout = null;
        expireTimeout = null;
    }

    function feed() {
        if (expireTimeout != null) {
            stop();
            start();
        }
    }

    start();
    return { feed, stop };
}
