import { useCallback } from 'react'
import { sendIPCEvent } from './ipc/common'
import { useWebViewEventHandler } from './hooks/useWebViewEventHandler'
import { create } from 'zustand'
import { produce } from 'immer'

type GamepadPlayer = {
  name: string,
  dev_path: string,
}

type GamepadPlayersState = {
  players: [GamepadPlayer | null, GamepadPlayer | null, GamepadPlayer | null, GamepadPlayer | null],
  assign_player: (player: GamepadPlayer, idx: number) => void
}

const useGamepadStore = create<GamepadPlayersState>((set) => ({
  players: [null, null, null, null],
  assign_player: (player, idx) => {
    set(prev => produce(prev, draft => {
      const prev_idx = prev.players.findIndex(e => e?.dev_path == player.dev_path);
      if (prev_idx !== -1)
        draft.players[prev_idx] = null

      draft.players[idx] = player
    }))
  }
}))

function App() {

  const players = useGamepadStore(e => e.players)
  const assign_player = useGamepadStore(e => e.assign_player)

  const callbutton = useCallback(() => {
    sendIPCEvent({
      type: 'LaunchRequested', splitscreen_type: 'horizontal',
      gamepads: players.filter(e => e !== null).map(e => e.dev_path),
      users: players.filter(e => e !== null).map(e => e.name),
    })
  }, [players])

  useWebViewEventHandler('AppGamepadButtonEvent', useCallback((ev) => {


    if (ev.button == "X") {
      const player: GamepadPlayer = { name: "game-user", dev_path: ev.gamepad_devpath }
      assign_player(player, 0)
    }

    if (ev.button == "Y") {
      const player: GamepadPlayer = { name: "game-user-giluxe", dev_path: ev.gamepad_devpath }
      assign_player(player, 1)
    }
  }, [assign_player]))

  return (
    <>

      <div className='long-container'>
        <div className='gamepad-entry'>
          Controller1 Left - Press Y

          <div className='gamepad-name'>
            <p> {players[0]?.name} </p>
            <p> {players[0]?.dev_path} </p>
          </div>
        </div>
        <div className='gamepad-entry'>
          Controller2 Left - Press X
          <div className='gamepad-name'>
            <p> {players[1]?.name} </p>
            <p> {players[1]?.dev_path} </p>
          </div>
        </div>
      </div>


      <p>
        <button onClick={callbutton}>
        Launch
        </button>
      </p>
    </>
  )
}

export default App
