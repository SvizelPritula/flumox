export type SessionToken = string;

export interface TeamInfo {
    name: string
};

export interface Session {
    token: SessionToken,
    team: TeamInfo
};
