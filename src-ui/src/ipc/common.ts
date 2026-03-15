// Events that can be sent from the UI
export type LaunchRequestedEvent = {
    type: 'LaunchRequested',
    splitscreen_type: string,
    gamepads: string[],
    users: string[]
}

export type WebViewReadyEvent = {
    type: 'WebViewReady'
}


export type FromWebViewEvent = LaunchRequestedEvent | WebViewReadyEvent;
export const sendIPCEvent = (event: FromWebViewEvent) => {
    window.ipc.postMessage(JSON.stringify(event));
}

// Events that are listened and handled by UI
export type GAMEPAD_BUTTONS = "A" | "B" | "X" | "Y" | "LB" | "RB" | "LS" | "RS" | "Start" | "Select" | "DpadLeft" | "DpadRight" | "DpadUp" | "DpadDown";
export type GamepadButtonPressedEvent = { type: 'AppGamepadButtonEvent', button: GAMEPAD_BUTTONS, release: boolean, gamepad_name: string, gamepad_devpath: string}

export type GamepadsUpdateEvent = {
    type: 'GamepadsUpdate',
    gamepads: Record<string, string>
}
export type ToWebViewEvent = GamepadButtonPressedEvent | GamepadsUpdateEvent
