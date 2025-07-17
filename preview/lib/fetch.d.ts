export type Problem = {
    pointer: string | null;
    detail: string | null;
};
type ServerResponse<T> = {
    status: "ok";
    body: T;
} | {
    status: "clientError";
    problems: Problem[];
} | {
    status: "serverError";
} | {
    status: "unauthorized";
} | never;
export declare function fetch<T>(method: "GET" | "POST" | "PUT" | "DELETE", url: string, additionalHeaders: [string, string][] | null, body: object | null): Promise<ServerResponse<T>>;
export {};
