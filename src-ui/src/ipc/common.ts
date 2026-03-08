// Events that can be sent from the UI
export type LaunchRequestedEvent = {
    type: 'LaunchRequested',
}

export type FromWebViewEvent = LaunchRequestedEvent;
export const sendIPCEvent = (event: FromWebViewEvent) => {
    window.ipc.postMessage(JSON.stringify(event));
}

// Events that are listened and handled by UI
export type GAMEPAD_BUTTONS = "A" | "B" | "X" | "Y" | "LB" | "RB" | "LS" | "RS" | "Start" | "Select" | "DpadLeft" | "DpadRight" | "DpadUp" | "DpadDown";
export type GamepadButtonPressed = { type: 'AppGamepadButtonEvent', button: GAMEPAD_BUTTONS, release: boolean }

export type ToWebViewEvent = GamepadButtonPressed
