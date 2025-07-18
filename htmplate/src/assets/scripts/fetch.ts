export type Problem = {
  pointer: string | null;
  detail: string | null;
};

export type ServerResponse<T> =
  | { status: "ok"; body: T }
  | { status: "clientError"; problems: Problem[] }
  | { status: "serverError" }
  | { status: "unauthorized" }
  | never;

export type LogoutConfig = {
  endpoint: string;
  redirect: string;
};

export type Header = [string, string];

export const TOKEN_KEY = "token";

export class FetchBuilder {
  #method: "GET" | "POST" | "PUT" | "DELETE";
  #url: string;
  #additionalHeaders: Header[] | null = null;
  #body: object | null = null;
  #logoutConfig: LogoutConfig | null = null;

  constructor(method: "GET" | "POST" | "PUT" | "DELETE", url: string) {
    this.#method = method;
    this.#url = url;
  }

  setBody(body: object | null): FetchBuilder {
    this.#body = body;
    return this;
  }

  setHeaders(headers: Header[] | null): FetchBuilder {
    this.#additionalHeaders = headers;
    return this;
  }

  setLogout(logoutConfig: LogoutConfig | null): FetchBuilder {
    this.#logoutConfig = logoutConfig;
    return this;
  }

  async fetch<T>(): Promise<ServerResponse<T>> {
    return await fetch(
      this.#method,
      this.#url,
      this.#additionalHeaders,
      this.#body,
      this.#logoutConfig,
    );
  }
}

export async function fetch<T>(
  method: "GET" | "POST" | "PUT" | "DELETE",
  url: string,
  additionalHeaders: Header[] | null,
  body: object | null,
  logoutConfig: LogoutConfig | null,
): Promise<ServerResponse<T>> {
  const headers = new Headers();

  if (additionalHeaders) {
    for (const header of additionalHeaders) {
      headers.append(header[0], header[1]);
    }
  }

  if (body) {
    headers.append("content-type", "application/json");
  }

  const token = localStorage.getItem(TOKEN_KEY);
  if (token) {
    headers.append("Authorization", token);
  }

  let bodyContent = null;
  if (body) {
    bodyContent = JSON.stringify(body);
  }

  const response = await self.fetch(url, {
    method,
    body: bodyContent,
    headers,
  }).catch(() => {
    return new Response(null, { status: 500 });
  });

  if (response.ok) {
    const bearer = response.headers.get("Authorization");
    if (bearer) {
      localStorage.setItem(TOKEN_KEY, bearer);
    }

    // deno-lint-ignore no-explicit-any
    let body: any = {};
    try {
      body = await response.json();
      // deno-lint-ignore no-empty
    } catch {}

    return {
      status: "ok",
      body,
    };
  } else if (response.status === 401) {
    if (logoutConfig) {
      await logout(logoutConfig.endpoint, additionalHeaders, logoutConfig.redirect);
    }

    return { status: "unauthorized" };
  } else if (response.status >= 400 && response.status < 500) {
    let body = { problems: [] };
    try {
      body = await response.json();
      // deno-lint-ignore no-empty
    } catch {}

    return {
      status: "clientError",
      problems: body.problems ?? [],
    };
  } else {
    return { status: "serverError" };
  }
}

export async function logout(
  endpoint: string,
  additionalHeaders: Header[] | null,
  redirect: string,
): Promise<never> {
  await new FetchBuilder("DELETE", endpoint).setHeaders(additionalHeaders).fetch();
  localStorage.removeItem(TOKEN_KEY);
  location.href = redirect;

  // Prevent further execution of JS
  // deno-lint-ignore no-empty
  while (true) {}
}
