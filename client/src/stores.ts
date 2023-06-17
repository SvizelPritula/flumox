import { persistent } from "./lib/persistent";
import type { Session } from "./lib/team";

export const session = persistent<Session>("session", null);
