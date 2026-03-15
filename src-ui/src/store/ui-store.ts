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
  setPickerActive: (slotIdx: number) => void
}

const emptyPlayers: State['players'] = [null, null, null, null]

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
    set((s) => ({ splitOrientation: s.splitOrientation === 'horizontal' ? 'vertical' : 'horizontal' })),

  joinController: (controllerId) =>
    set(
      produce((draft: State) => {
        // Don't join if already joined
        if (draft.players.some((p) => p?.controllerId === controllerId)) return
        const slot = draft.players.findIndex((p) => p === null)
        if (slot === -1) return
        draft.players[slot] = { controllerId, profileId: null, state: 'picking' }
        draft.activePickerIdx = slot
      }),
    ),

  unjoinPlayer: (slotIdx) =>
    set(
      produce((draft: State) => {
        draft.players[slotIdx] = null
        if (draft.activePickerIdx === slotIdx) {
          // Hand focus to next picking player, if any
          const next = draft.players.findIndex((p, i) => i !== slotIdx && p?.state === 'picking')
          draft.activePickerIdx = next === -1 ? null : next
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
        slot.state = 'ready'
        // Advance focus to next picking player
        const next = draft.players.findIndex((p, i) => i !== draft.activePickerIdx && p?.state === 'picking')
        draft.activePickerIdx = next === -1 ? null : next
      }),
    ),

  setPickerActive: (slotIdx) =>
    set(
      produce((draft: State) => {
        const slot = draft.players[slotIdx]
        if (!slot) return
        slot.state = 'picking'
        slot.profileId = null
        draft.activePickerIdx = slotIdx
      }),
    ),
}))