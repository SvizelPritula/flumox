export interface PromptView {
    name: string,
    details: string[],
    prompt: string,
    submit_button: string,
    disabled: boolean,
}

export interface Metadata {
    id: string
}

export type View = { type: "prompt", view: PromptView } & Metadata;
export type Views = View[];
