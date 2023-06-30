export interface PromptView {
    type: "prompt",
    name: string,
    details: string[],
    prompt: string,
    submit_button: string,
    disabled: boolean,
}

export interface Instance {
    id: string,
    view: PromptView
}

export type Instances = Instance[];
