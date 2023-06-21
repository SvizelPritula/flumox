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
