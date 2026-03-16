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

export type LaunchReturnedEvent = {
    type: 'LaunchReturned'
}

export type GameHandler = {
    type: 'GameHandlersUpdate',
    name: string,
    description: string,
    image_base_64: string | null,
    max_players: number,
    executable_args: string[],
}

export type GameHandlersUpdateEvent = {
    type: 'GameHandlersUpdate',
    handlers: GameHandler[],
}

export type ToWebViewEvent = GamepadButtonPressedEvent | GamepadsUpdateEvent | LaunchReturnedEvent | GameHandlersUpdateEvent
