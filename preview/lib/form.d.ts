// deno-lint-ignore-file
// deno-fmt-ignore-file
// @ts-nocheck
import { Problem } from "./fetch.d.ts";
export declare class FormError {
    element: HTMLElement;
    contents: HTMLElement;
    constructor(formId: string);
    clearError(): void;
    addError(error: string): void;
    setError(error: string): void;
}
export declare class Input {
    input: HTMLInputElement;
    error: HTMLElement;
    constructor(formId: string, inputId: string);
    getValue(): string;
    lock(): void;
    unlock(): void;
    clearError(): void;
    addError(error: string): void;
    setError(error: string): void;
}
export declare class Form {
    form: HTMLFormElement;
    formError: FormError;
    submitButton: HTMLButtonElement;
    inputs: Map<string, Input>;
    constructor(formId: string, inputIds: string[]);
    clearError(): void;
    lock(): void;
    unlock(): void;
    setInputErrors(problems: Problem[] | null): void;
    getValues(): Map<string, string>;
}
