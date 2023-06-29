import { session, view } from "../stores";

export type SessionToken = string;

export interface GameInfo {
    name: string,
};

export interface TeamInfo {
    name: string,
    game: GameInfo,
};

export interface Session {
    token: SessionToken,
    team: TeamInfo
};

export function logout() {
    session.set(null);
    view.set(null);
}
