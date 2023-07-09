export interface PromptView {
    type: "prompt",
    name: string,
    details: string[],
    prompt: string,
    submit_button: string,
    disabled: boolean,
    solution?: string,
    hints: Hint[]
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
    button: string
} | {
    state: "taken",
    content: string[]
})

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
    view: PromptView | TextView | CountdownView
}

export interface InstanceDelta {
    id: string,
    view?: PromptView
}

export type Instances = Instance[];
