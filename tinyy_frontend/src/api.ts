const BASE_URL = "https://tinyy.io";

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

export async function createTinyUrl(url: string): Promise<string> {
    let resp = await post("/", {"url": url});
    let code = (await resp.json()).code;

    return `${BASE_URL}/${code}`;
}
