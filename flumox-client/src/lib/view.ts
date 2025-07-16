export interface PromptView {
    type: "prompt",
    name: string,
    details: string[],
    prompt: string,
    submit_button: string | null,
    disabled: boolean,
    solution?: string,
    hints: Hint[],
    time: TimeSpent
}

export type Hint = {
    ident: string,
    name: string,
} & ({
    state: "unknown"
} | {
    state: "future",
    time: string
} | {
    state: "available",
    button: string | null
} | {
    state: "taken",
    content: string[]
});

export type TimeSpent = { type: "solving", since: string } | { type: "solved", after: string } | null;

export interface TextView {
    type: "text",
    heading: string | null,
    content: string[],
}

export interface CountdownView {
    type: "countdown",
    name: string | null,
    details: string[],
    value: {
        type: "unknown"
    } | {
        type: "time",
        time: string
    } | {
        type: "done",
        text: string
    },
}

export interface Instance {
    id: string,
    view: PromptView | TextView | CountdownView,
    obsolete: boolean
}

export interface InstanceDelta {
    id: string,
    view?: PromptView | TextView | CountdownView,
    obsolete: boolean
}

export type Instances = Instance[];
