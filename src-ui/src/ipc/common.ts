import type { FromWebViewEvent } from "../types";

export const sendIPCEvent = (event: FromWebViewEvent) => {
    window.ipc.postMessage(JSON.stringify(event));
}