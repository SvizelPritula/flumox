import { BadResponseError, type BadResponseType } from "./api/request";

import {
    errorFetchNetwork,
    errorFetchParsing,
    errorFetchRequest,
    errorFetchDatabase,
    errorFetchConfig,
    errorFetchServer,
    errorFetchUnknown
} from "$translations";

export function getErrorMessageForType(type: BadResponseType): string {
    switch (type) {
        case "request":
            return errorFetchRequest;
        case "database":
            return errorFetchDatabase;
        case "config":
            return errorFetchConfig;
        case "server":
            return errorFetchServer;
        default:
            return errorFetchUnknown;
    }
}

export function getErrorMessage(error: any): string {
    if (error instanceof BadResponseError) {
        return getErrorMessageForType(error.type);
    } else if (error instanceof TypeError) {
        return errorFetchNetwork;
    } else if (error instanceof SyntaxError) {
        return errorFetchParsing;
    }

    return errorFetchUnknown;
}
