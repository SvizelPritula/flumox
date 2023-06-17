import { writable, type Writable } from "svelte/store";

function localStorageSupported() {
    const testKey = "__detect";

    try {
        localStorage.setItem(testKey, testKey);
        localStorage.removeItem(testKey);
        return true;
    } catch {
        return false;
    }
}

function localStore<T>(key: string, initial: T): Writable<T> {
    function parse(value: string | null): T {
        if (value == null) {
            return initial;
        } else {
            try {
                return JSON.parse(value);
            } catch (error) {
                console.error(`Failed to parse storage key ${key}`, error);
                return initial;
            }
        }
    }

    function read(): T {
        return parse(localStorage.getItem(key));
    }

    function write(value: T) {
        localStorage.setItem(key, JSON.stringify(value));
    }

    let store = writable(read(), (set) => {
        function handler(event: StorageEvent) {
            if (event.key == key) {
                set(parse(event.newValue));
            }
        }

        set(read());

        window.addEventListener("storage", handler);
        return () => window.removeEventListener("storage", handler);
    });

    return {
        set(value) {
            write(value);
            store.set(value);
        },
        subscribe(run, invalidate) {
            return store.subscribe(run, invalidate);
        },
        update(updater) {
            store.update((value) => {
                let result = updater(value);
                write(result);
                return result;
            });
        },
    }
}

export function persistent<T>(key: string, initial: T): Writable<T> {
    if (localStorageSupported()) {
        return localStore(key, initial);
    } else {
        return writable(initial);
    }
}