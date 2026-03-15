import { useCallback } from 'react'
import { useUIState } from './store/ui-store'
import { useWebViewEventHandler } from './hooks/useWebViewEventHandler'
import { useOnWebviewLoaded } from './hooks/useOnWebviewLoaded'
import { useGamepadInput } from './hooks/useGamepadInput'
import { sendIPCEvent } from './ipc/common'
import { TopBar } from './components/TopBar'
import { GameGrid } from './components/GameGrid'
import { ConfirmBar } from './components/ConfirmBar'
import { Sidebar } from './components/Sidebar'
import { HintBar } from './components/Hintbar'
import { LaunchingOverlay } from './components/LaunchingOverlay'
import styles from './App.module.css'
import type { GamepadButtonPressedEvent, GamepadsUpdateEvent } from './types'

function App() {
  const phase           = useUIState((s) => s.phase)
  const returnFromLaunch = useUIState((s) => s.returnFromLaunch)
  const { handleButtonEvent, handleGamepadsUpdate } = useGamepadInput()

  useOnWebviewLoaded(useCallback(() => {
    sendIPCEvent({ type: 'WebViewReady' })
  }, []))

  useWebViewEventHandler(
    'AppGamepadButtonEvent',
    useCallback((ev: GamepadButtonPressedEvent) => handleButtonEvent(ev), [handleButtonEvent]),
  )

  useWebViewEventHandler(
    'GamepadsUpdate',
    useCallback((ev: GamepadsUpdateEvent) => handleGamepadsUpdate(ev), [handleGamepadsUpdate]),
  )

  useWebViewEventHandler(
    'LaunchReturned',
    useCallback(() => {
      console.log('Launch returned — resetting UI')
      returnFromLaunch()
    }, [returnFromLaunch]),
  )

  return (
    <div className={styles.launcher}>
      <TopBar />

      <div className={styles.body}>
        <main className={styles.gamePane}>
          <div className={styles.sectionLabel}>installed games</div>
          <GameGrid />
          {phase === 'join-players' && <ConfirmBar />}
        </main>

        <Sidebar />
      </div>

      <HintBar />

      {phase === 'launching' && <LaunchingOverlay />}
    </div>
  )
}

export default App