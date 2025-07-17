export type Problem = {
  pointer: string | null;
  detail: string | null;
};

type ServerResponse<T> =
  | { status: "ok"; body: T }
  | { status: "clientError"; problems: Problem[] }
  | { status: "serverError" }
  | { status: "unauthorized" }
  | never;

export async function fetch<T>(
  method: "GET" | "POST" | "PUT" | "DELETE",
  url: string,
  additionalHeaders: [string, string][] | null,
  body: object | null,
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
  const token = localStorage.getItem("token");

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
      localStorage.setItem("token", bearer);
    }

    return {
      status: "ok",
      body: await response.json(),
    };
  } else if (response.status === 401) {
    return { status: "unauthorized" };
  } else if (response.status >= 400 && response.status < 500) {
    const body = await response.json();
    return {
      status: "clientError",
      problems: body.problems ?? [],
    };
  } else {
    return { status: "serverError" };
  }
}
