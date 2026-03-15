export type SplitOrientation = 'horizontal' | 'vertical'

export type Phase = 'select-game' | 'join-players'

export type Game = {
  id: number
  name: string
  description: string
  maxPlayers: number
  coverColor: string
}

export type Controller = {
  id: number
  name: string
  devPath: string
}

export type Profile = {
  id: number
  name: string
  saveSlot: string
  initials: string
}

export type PlayerSlot = {
  controllerId: number
  profileId: number | null
  state: 'picking' | 'ready'
} | null