export interface PromptView {
    type: "prompt",
    name: string,
    details: string[],
    prompt: string,
    submit_button: string,
    disabled: boolean,
}

export type View = PromptView;
export type Views = View[];
