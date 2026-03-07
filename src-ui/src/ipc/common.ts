// Events that can be sent from the UI
export type LaunchRequestedEvent = {
    type: 'LaunchRequested',
}

export type FromWebViewEvent = LaunchRequestedEvent;


export const sendIPCEvent = (event: FromWebViewEvent) => {
    const _ev = { type: 'FromWebViewEvent', event: event }
    window.ipc.postMessage(JSON.stringify(_ev));
}

// Events that are listened and handled by UI

export type GamepadButtonPressed = { type: 'GamepadButtonPressed', button: string, release: boolean }

export type ToWebViewEvent = GamepadButtonPressed
