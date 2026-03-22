import { create } from 'zustand'
import { produce } from 'immer'
import type { Controller, Game, Phase, PlayerSlot, SplitOrientation } from '../types'

type State = {
  phase: Phase
  games: Game[]
  selectedGameName: string | null   // uses name as stable key
  splitOrientation: SplitOrientation
  connectedControllers: Controller[]
  players: [PlayerSlot, PlayerSlot, PlayerSlot, PlayerSlot]
  activePickerIdx: number | null
  gameCursor: number
  profileCursor: number
  sideCursor: number
}

type Actions = {
  syncHandlers: (handlers: import('../types').GameHandler[]) => void
  confirmGame: (gameName: string) => void
  changeGame: () => void
  toggleOrientation: () => void
  syncControllers: (gamepads: Record<string, string>) => void
  joinController: (devPath: string) => void
  unjoinByDevPath: (devPath: string) => void
  unjoinPlayer: (slotIdx: number) => void
  pickProfile: (profileId: number) => void
  pickSide: (sideIndex: number) => void
  setPickerActive: (slotIdx: number) => void
  setGameCursor: (index: number) => void
  moveProfileCursor: (delta: number, profileCount: number) => void
  moveSideCursor: (delta: number, sideCount: number) => void
  startLaunching: () => void
  returnFromLaunch: () => void
}

const emptyPlayers: State['players'] = [null, null, null, null]

function nextActiveIdx(players: State['players'], excludeIdx: number | null): number | null {
  const picking = players.findIndex((p, i) => i !== excludeIdx && p?.state === 'picking')
  if (picking !== -1) return picking
  const pickingSide = players.findIndex((p, i) => i !== excludeIdx && p?.state === 'picking-side')
  return pickingSide === -1 ? null : pickingSide
}

export const useUIState = create<State & Actions>((set) => ({
  phase: 'select-game',
  games: [],
  selectedGameName: null,
  splitOrientation: 'Horizontal',
  connectedControllers: [],
  players: emptyPlayers,
  activePickerIdx: null,
  gameCursor: 0,
  profileCursor: 0,
  sideCursor: 0,

  syncHandlers: (handlers) =>
    set((s) => ({
      games: handlers.map((h) => ({
        name:           h.name,
        description:    h.description,
        maxPlayers:     h.max_players,
        imageBase64:    h.image_base_64,
        executableArgs: h.executable_args,
      })),
      // Clamp cursor in case the new list is shorter
      gameCursor: Math.min(s.gameCursor, Math.max(0, handlers.length - 1)),
    })),

  confirmGame: (gameName) =>
    set({ phase: 'join-players', selectedGameName: gameName }),

  changeGame: () =>
    set((s) => ({
      phase:            'select-game',
      selectedGameName: null,
      players:          emptyPlayers,
      activePickerIdx:  null,
      gameCursor:       s.selectedGameName !== null
        ? Math.max(0, s.games.findIndex((g) => g.name === s.selectedGameName))
        : 0,
    })),

  toggleOrientation: () =>
    set((s) => ({
      splitOrientation: s.splitOrientation === 'Horizontal' ? 'Vertical' : 'Horizontal',
    })),

  syncControllers: (gamepads) =>
    set(
      produce((draft: State) => {
        const incoming = Object.entries(gamepads).map(
          ([devPath, name]): Controller => ({ devPath, name }),
        )
        const incomingPaths = new Set(incoming.map((c) => c.devPath))
        draft.players.forEach((player, slotIdx) => {
          if (player && !incomingPaths.has(player.devPath)) {
            draft.players[slotIdx] = null
            if (draft.activePickerIdx === slotIdx) {
              draft.activePickerIdx = nextActiveIdx(draft.players, slotIdx)
            }
          }
        })
        draft.connectedControllers = incoming
      }),
    ),

  joinController: (devPath) =>
    set(
      produce((draft: State) => {
        if (draft.players.some((p) => p?.devPath === devPath)) return
        const slot = draft.players.findIndex((p) => p === null)
        if (slot === -1) return
        draft.players[slot] = { devPath, profileId: null, sideIndex: null, state: 'picking' }
        draft.activePickerIdx = slot
        draft.profileCursor = 0
      }),
    ),

  unjoinByDevPath: (devPath) =>
    set(
      produce((draft: State) => {
        const slotIdx = draft.players.findIndex((p) => p?.devPath === devPath)
        if (slotIdx === -1) return
        draft.players[slotIdx] = null
        if (draft.activePickerIdx === slotIdx) {
          draft.activePickerIdx = nextActiveIdx(draft.players, slotIdx)
        }
      }),
    ),

  unjoinPlayer: (slotIdx) =>
    set(
      produce((draft: State) => {
        draft.players[slotIdx] = null
        if (draft.activePickerIdx === slotIdx) {
          draft.activePickerIdx = nextActiveIdx(draft.players, slotIdx)
        }
      }),
    ),

  pickProfile: (profileId) =>
    set(
      produce((draft: State) => {
        if (draft.activePickerIdx === null) return
        const slot = draft.players[draft.activePickerIdx]
        if (!slot) return
        slot.profileId = profileId
        slot.state = 'picking-side'
        draft.sideCursor = 0
      }),
    ),

  pickSide: (sideIndex) =>
    set(
      produce((draft: State) => {
        if (draft.activePickerIdx === null) return
        const slot = draft.players[draft.activePickerIdx]
        if (!slot) return
        slot.sideIndex = sideIndex
        slot.state = 'ready'
        draft.activePickerIdx = nextActiveIdx(draft.players, draft.activePickerIdx)
        draft.profileCursor = 0
        draft.sideCursor = 0
      }),
    ),

  setPickerActive: (slotIdx) =>
    set(
      produce((draft: State) => {
        const slot = draft.players[slotIdx]
        if (!slot) return
        slot.state = 'picking'
        slot.profileId = null
        slot.sideIndex = null
        draft.activePickerIdx = slotIdx
        draft.profileCursor = 0
      }),
    ),

  setGameCursor: (index) =>
    set({ gameCursor: index }),

  moveProfileCursor: (delta, profileCount) =>
    set((s) => ({
      profileCursor: (s.profileCursor + delta + profileCount) % profileCount,
    })),

  moveSideCursor: (delta, sideCount) =>
    set((s) => ({
      sideCursor: (s.sideCursor + delta + sideCount) % sideCount,
    })),

  startLaunching: () =>
    set({ phase: 'launching' }),

  returnFromLaunch: () =>
    set({
      phase:            'select-game',
      selectedGameName: null,
      players:          emptyPlayers,
      activePickerIdx:  null,
      gameCursor:       0,
      profileCursor:    0,
      sideCursor:       0,
    }),
}))