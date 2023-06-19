export type SessionToken = string;

export interface TeamInfo {
    name: string,
    gameName: string,
};

export interface Session {
    token: SessionToken,
    team: TeamInfo
};
