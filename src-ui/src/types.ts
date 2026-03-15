export type SplitOrientation = 'horizontal' | 'vertical'

export type Phase = 'select-game' | 'join-players' | 'launching'

export type Game = {
  id: number
  name: string
  description: string
  maxPlayers: number
  coverColor: string
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

export type GamepadButtonPressedEvent = {
  type: 'AppGamepadButtonEvent'
  button: GAMEPAD_BUTTONS
  release: boolean
  gamepad_name: string
  gamepad_devpath: string
}

// Full snapshot of currently connected controllers.
// Record<devPath, gamepadName> — emitted on any change (add or remove).
export type GamepadsUpdateEvent = {
  type: 'GamepadsUpdate'
  gamepads: Record<string, string>
}

export type ToWebViewEvent = GamepadButtonPressedEvent | GamepadsUpdateEvent