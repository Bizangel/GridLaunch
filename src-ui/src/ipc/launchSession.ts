import { useUIState } from '../store/ui-store'
import { sendIPCEvent } from '../ipc/common'

export function launchSession() {
  const s = useUIState.getState()
  const game = s.selectedGameName;
  if (game === null)
    return;

  const readyPlayers = s.players
    .filter((p): p is NonNullable<typeof p> => p !== null && p.state === 'ready')
    .map((p) => ({
      devPath:   p.devPath,
      profile:   s.profiles.find((pr) => pr.user  === p.profileUser)?.user ?? 'unknown',
      sideIndex: p.sideIndex,
    }))

  // sort by side-index as this determines the screen side and how it's ordered.
  readyPlayers.sort((a,b) => ((a.sideIndex ?? 0) - (b.sideIndex ?? 0)))

  sendIPCEvent({
    type:             'LaunchRequested',
    splitscreen_type: s.splitOrientation,
    gamepads:         readyPlayers.map((p) => p.devPath),
    users: readyPlayers.map((p) => p.profile),
    game: game,
  })

  useUIState.getState().startLaunching()
}