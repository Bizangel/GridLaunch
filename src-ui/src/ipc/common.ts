
export type LaunchRequestedEvent = {
    type: 'LaunchRequested',
}

export type FromWebViewEvent = LaunchRequestedEvent;


export const sendIPCEvent = (event: FromWebViewEvent) => {
    const _ev = { type: 'FromWebViewEvent', event: event }
    window.ipc.postMessage(JSON.stringify(_ev));
}