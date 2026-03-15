import type { Game, Profile } from './types'

export const GAMES: Game[] = [
  { id: 1, name: 'Portal 2',             description: 'Co-op puzzle platformer', maxPlayers: 2, coverColor: '#1a2a1a' },
  { id: 2, name: 'Stardew Valley',        description: 'Farm sim RPG',            maxPlayers: 4, coverColor: '#1a1a2a' },
  { id: 3, name: 'Rocket League',         description: 'Vehicular soccer',        maxPlayers: 4, coverColor: '#2a1a1a' },
  { id: 4, name: 'Hollow Knight',         description: 'Metroidvania action',     maxPlayers: 2, coverColor: '#15151f' },
  { id: 5, name: 'Celeste',              description: 'Precision platformer',     maxPlayers: 2, coverColor: '#1e1220' },
  { id: 6, name: "Don't Starve Together", description: 'Survival sandbox',        maxPlayers: 4, coverColor: '#1a1508' },
  { id: 7, name: 'Terraria',             description: 'Sandbox adventure',        maxPlayers: 4, coverColor: '#0f1f1f' },
  { id: 8, name: 'Cuphead', description: 'Run and gun action', maxPlayers: 2, coverColor: '#1f1020' },
  { id: 9, name: 'Cuphead 2', description: 'Run and gun action', maxPlayers: 4, coverColor: '#1f1021' },
  { id: 10, name: 'Cuphead 3', description: 'Run and gun action', maxPlayers: 4, coverColor: '#1f1022' },
  { id: 11, name: 'Stardew Valley',        description: 'Farm sim RPG',            maxPlayers: 4, coverColor: '#1a1a2a' },
  { id: 12, name: 'Fighting League',         description: 'Vehicular soccer',        maxPlayers: 4, coverColor: '#2a1a1b' },
  { id: 13, name: 'Shovel Knight',         description: 'Metroidvania action',     maxPlayers: 2, coverColor: '#15151f' },
  { id: 14, name: 'Skynight',              description: 'Precision platformer',     maxPlayers: 2, coverColor: '#1e1220' },
  { id: 15, name: "Don't Starve Alone", description: 'Survival sandbox',        maxPlayers: 4, coverColor: '#1a1508' },
  { id: 16, name: 'Terraria Overworld',             description: 'Sandbox adventure',        maxPlayers: 4, coverColor: '#0f1f1f' },

]

// export const CONTROLLERS: Controller[] = [
//   { id: 0, name: 'DualSense Edge',   devPath: '/dev/input/js0' },
//   { id: 1, name: 'Xbox Series X',    devPath: '/dev/input/js1' },
//   { id: 2, name: '8Bitdo Pro 2',     devPath: '/dev/input/js2' },
//   { id: 3, name: 'Steam Controller', devPath: '/dev/input/js3' },
//   { id: 4, name: 'Xbox 360 Controller', devPath: '/dev/input/js4' },
//   { id: 5, name: 'Keychron Link', devPath: '/dev/input/js5' },
// ]

export const PROFILES: Profile[] = [
  { id: 0, name: 'Player One',   saveSlot: 'slot_1', initials: 'P1' },
  { id: 1, name: 'Giluxe',       saveSlot: 'slot_4', initials: 'GX' },
  { id: 2, name: 'Guest',        saveSlot: 'slot_0', initials: 'GU' },
  { id: 3, name: 'Speedrunner', saveSlot: 'slot_7', initials: 'SR' },
  { id: 4, name: 'GameGod', saveSlot: 'slot_7', initials: 'SR' },
  { id: 5, name: 'Yet AnotherProfile', saveSlot: 'slot_7', initials: 'SR' },
  { id: 6, name: 'Player Two',   saveSlot: 'slot_1', initials: 'P1' },
  { id: 7, name: 'Giluxe+',       saveSlot: 'slot_4', initials: 'GX' },
  { id: 8, name: 'Guest2',        saveSlot: 'slot_0', initials: 'GU' },
  { id: 9, name: 'Slowrunner', saveSlot: 'slot_7', initials: 'SR' },
  { id: 10, name: 'GameGod', saveSlot: 'slot_7', initials: 'SR' },
  { id: 11, name: 'NotYet AnotherProfile',  saveSlot: 'slot_7', initials: 'SR' },
]

export const PLAYER_COLORS = ['#00e5a0', '#00bfff', '#ff6b6b', '#ffd166'] as const
export const PLAYER_LABELS = ['P1', 'P2', 'P3', 'P4'] as const