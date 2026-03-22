export type SplitOrientation = 'horizontal' | 'vertical'

export type Phase = 'select-game' | 'join-players' | 'launching'

export type Game = {
  name: string
  description: string
  maxPlayers: number
  imageBase64: string | null
  executableArgs: string[]
}

export type Controller = {
  devPath: string
  name: string
}

export type Profile = {
  id: number
  name: string
  saveSlot: string
  initials: string
}

export type PlayerSlot = {
  devPath: string
  profileId: number | null
  sideIndex: number | null
  state: 'picking' | 'picking-side' | 'ready'
} | null

// ── IPC event types ────────────────────────────────────────────────────────

export type GAMEPAD_BUTTONS =
  | 'A' | 'B' | 'X' | 'Y'
  | 'LB' | 'RB' | 'LS' | 'RS'
  | 'Start' | 'Select'
  | 'DpadLeft' | 'DpadRight' | 'DpadUp' | 'DpadDown'
  | "LeftStickUp" | "LeftStickDown" | "LeftStickLeft" | "LeftStickRight"

export type GamepadButtonPressedEvent = {
  type: 'AppGamepadButtonEvent'
  button: GAMEPAD_BUTTONS
  release: boolean
  gamepad_name: string
  gamepad_devpath: string
}

export type GamepadsUpdateEvent = {
  type: 'GamepadsUpdate'
  gamepads: Record<string, string>
}

export type LaunchReturnedEvent = {
  type: 'LaunchReturned'
}

export type GameHandler = {
  name: string
  description: string
  image_base_64: string | null
  max_players: number
  executable_args: string[]
}

export type GameHandlersUpdateEvent = {
  type: 'GameHandlersUpdate'
  handlers: GameHandler[]
}

export type ToWebViewEvent =
  | GamepadButtonPressedEvent
  | GamepadsUpdateEvent
  | LaunchReturnedEvent
  | GameHandlersUpdateEvent

export type LaunchRequestedEvent = {
  type: 'LaunchRequested'
  splitscreen_type: string
  gamepads: string[]
  users: string[]
}

export type WebViewReadyEvent = {
  type: 'WebViewReady'
}

export type FromWebViewEvent = LaunchRequestedEvent | WebViewReadyEvent