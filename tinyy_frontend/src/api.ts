const BASE_URL = "https://tinyy.io";

export interface TinyUrl {
    tinyUrl: string,
    url: string
}

export async function get(route: string): Promise<Response> {
    try {
        return await fetch(`${BASE_URL}${route}`);
    } catch(e) {
        return new Promise(() => null);
    }
}

export async function post(route: string, data: Record<string, unknown>): Promise<Response> {
    try {
        return await fetch(`${BASE_URL}${route}`, {
            method: "POST",
            headers: {"Content-Type": "application/json"},
            body: JSON.stringify(data)
        })
    } catch(e) {
        return new Promise(() => null);
    }
}

export async function createTinyUrl(url: string): Promise<TinyUrl> {
    let resp = await (await post("/", {"url": url})).json();

    return {
        "tinyUrl": `${BASE_URL}/${resp.code}`,
        "url": resp.url
    };
}
