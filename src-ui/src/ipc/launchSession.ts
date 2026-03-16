import { useUIState } from '../store/ui-store'
import { PROFILES } from '../data'
import { sendIPCEvent } from '../ipc/common'

export function launchSession() {
  const s = useUIState.getState()

  const readyPlayers = s.players
    .filter((p): p is NonNullable<typeof p> => p !== null && p.state === 'ready')
    .map((p) => ({
      devPath:   p.devPath,
      profile:   PROFILES.find((pr) => pr.id === p.profileId)?.name ?? 'unknown',
      sideIndex: p.sideIndex,
    }))

  console.log('Launching:', {
    game:        s.selectedGameName,
    orientation: s.splitOrientation,
    players:     readyPlayers,
  })

  sendIPCEvent({
    type:             'LaunchRequested',
    splitscreen_type: s.splitOrientation,
    gamepads:         readyPlayers.map((p) => p.devPath),
    users:            readyPlayers.map((p) => p.profile),
  })

  useUIState.getState().startLaunching()
}