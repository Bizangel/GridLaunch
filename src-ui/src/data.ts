import type { Profile } from './types'

export const PROFILES: Profile[] = [
  { id: 0, name: 'Player One',   saveSlot: 'slot_1', initials: 'P1' },
  { id: 1, name: 'Giluxe',       saveSlot: 'slot_4', initials: 'GX' },
  { id: 2, name: 'Guest',        saveSlot: 'slot_0', initials: 'GU' },
  { id: 3, name: 'Speedrunner',  saveSlot: 'slot_7', initials: 'SR' },
]

export const PLAYER_COLORS = ['#00e5a0', '#00bfff', '#ff6b6b', '#ffd166'] as const
export const PLAYER_LABELS = ['P1', 'P2', 'P3', 'P4'] as const