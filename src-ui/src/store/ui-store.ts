import { create } from 'zustand'
import { produce } from 'immer'
import type { Phase, PlayerSlot, SplitOrientation } from '../types'

type State = {
  phase: Phase
  selectedGameId: number | null
  splitOrientation: SplitOrientation
  players: [PlayerSlot, PlayerSlot, PlayerSlot, PlayerSlot]
  activePickerIdx: number | null
}

type Actions = {
  confirmGame: (gameId: number) => void
  changeGame: () => void
  toggleOrientation: () => void
  joinController: (controllerId: number) => void
  unjoinPlayer: (slotIdx: number) => void
  pickProfile: (profileId: number) => void
  pickSide: (sideIndex: number) => void
  setPickerActive: (slotIdx: number) => void
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
  players: emptyPlayers,
  activePickerIdx: null,

  confirmGame: (gameId) =>
    set({ phase: 'join-players', selectedGameId: gameId }),

  changeGame: () =>
    set({ phase: 'select-game', players: emptyPlayers, activePickerIdx: null }),

  toggleOrientation: () =>
    set((s) => ({
      splitOrientation: s.splitOrientation === 'horizontal' ? 'vertical' : 'horizontal',
    })),

  joinController: (controllerId) =>
    set(
      produce((draft: State) => {
        if (draft.players.some((p) => p?.controllerId === controllerId)) return
        const slot = draft.players.findIndex((p) => p === null)
        if (slot === -1) return
        draft.players[slot] = { controllerId, profileId: null, sideIndex: null, state: 'picking' }
        draft.activePickerIdx = slot
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
        // activePickerIdx stays — same player now picks their side
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
      }),
    ),
}))