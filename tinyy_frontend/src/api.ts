const BASE_URL = "https://tinyy.io";

export interface TinyUrl {
    tinyUrl: string,
    url: string
}

export enum TinyStatus {
    Ok = "Ok",                                   // 200
    GenericServerError = "GenericServerError",   // 5xx
    InvalidHttpUrl = "InvalidHttpUrl",           // 422
    UniqueCodeViolation = "UniqueCodeViolation"  // 409
}

export interface TinyStatusSpec {
    status: TinyStatus,
    tinyUrl?: TinyUrl
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

export async function createTinyUrl(url: string): Promise<TinyStatusSpec> {
    let resp = await post("/", {"url": url});

    switch (resp.status) {
        case 200: {
            let resp_json = await resp.json();
            return {
                status: TinyStatus.Ok,
                tinyUrl: {
                    tinyUrl: `${BASE_URL}/${resp_json.code}`,
                    url: resp_json.url
                }
            };
        }
        case 500: return { status: TinyStatus.GenericServerError };
        case 422: return { status: TinyStatus.InvalidHttpUrl };
        case 409: return { status: TinyStatus.UniqueCodeViolation };

        default: return { status: TinyStatus.GenericServerError };
    }
}
