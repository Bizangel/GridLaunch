import { create } from 'zustand'
import { produce } from 'immer'
import type { Controller, Phase, PlayerSlot, SplitOrientation } from '../types'

type State = {
  phase: Phase
  selectedGameId: number | null
  splitOrientation: SplitOrientation
  connectedControllers: Controller[]
  players: [PlayerSlot, PlayerSlot, PlayerSlot, PlayerSlot]
  activePickerIdx: number | null
  gameCursor: number      // index into GAMES array
  profileCursor: number
  sideCursor: number
}

type Actions = {
  confirmGame: (gameId: number) => void
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
  selectedGameId: null,
  splitOrientation: 'horizontal',
  connectedControllers: [],
  players: emptyPlayers,
  activePickerIdx: null,
  gameCursor: 0,
  profileCursor: 0,
  sideCursor: 0,

  confirmGame: (gameId) =>
    set({ phase: 'join-players', selectedGameId: gameId }),

  changeGame: () =>
    set({ phase: 'select-game', selectedGameId: null, players: emptyPlayers, activePickerIdx: null, gameCursor: 0 }),

  toggleOrientation: () =>
    set((s) => ({
      splitOrientation: s.splitOrientation === 'horizontal' ? 'vertical' : 'horizontal',
    })),

  // Receives the full current snapshot. Diffs against existing list:
  // - controllers no longer in the snapshot are removed and their players unjoined
  // - new controllers are added
  syncControllers: (gamepads) =>
    set(
      produce((draft: State) => {
        const incoming = Object.entries(gamepads).map(
          ([devPath, name]): Controller => ({ devPath, name }),
        )
        const incomingPaths = new Set(incoming.map((c) => c.devPath))

        // Unjoin any player whose controller is no longer present
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

  // Transition to launching — UI freezes, all input ignored
  startLaunching: () =>
    set({ phase: 'launching' }),

  // Game session ended — full reset back to initial state
  returnFromLaunch: () =>
    set({
      phase:          'select-game',
      selectedGameId: null,
      players:        emptyPlayers,
      activePickerIdx: null,
      gameCursor:     0,
      profileCursor:  0,
      sideCursor:     0,
    }),
}))