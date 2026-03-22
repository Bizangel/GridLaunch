import { useCallback, useEffect, useRef } from 'react'
import { useUIState } from './store/ui-store'
import { useWebViewEventHandler } from './hooks/useWebViewEventHandler'
import { useOnWebviewLoaded } from './hooks/useOnWebviewLoaded'
import { useGamepadInput } from './hooks/useGamepadInput'
import { sendIPCEvent } from './ipc/common'
import { gridScrollContainerRef } from './hooks/gridNavRef'
import { TopBar } from './components/TopBar'
import { GameGrid } from './components/GameGrid'
import { ConfirmGameBar } from './components/ConfirmGamebar'
import { ConfirmBar } from './components/ConfirmBar'
import { Sidebar } from './components/Sidebar'
import { HintBar } from './components/Hintbar'
import { LaunchingOverlay } from './components/LaunchingOverlay'
import styles from './App.module.css'
import type { GamepadButtonPressedEvent, GamepadsUpdateEvent, GameHandlersUpdateEvent, ProfilesUpdateEvent } from './types'

function App() {
  const phase = useUIState((s) => s.phase)
  const returnFromLaunch = useUIState((s) => s.returnFromLaunch)
  const syncProfiles = useUIState((s) => s.syncProfiles)
  const syncHandlers = useUIState((s) => s.syncHandlers)
  const { handleButtonEvent, handleGamepadsUpdate } = useGamepadInput()
  const gridScrollRef = useRef<HTMLDivElement>(null)

  // Keep the module-level ref in sync with the DOM element
  useEffect(() => {
    gridScrollContainerRef.el = gridScrollRef.current
    return () => { gridScrollContainerRef.el = null }
  }, [])

  useOnWebviewLoaded(useCallback(() => {
    sendIPCEvent({ type: 'WebViewReady' })
  }, []))

  useWebViewEventHandler(
    'GameHandlersUpdate',
    useCallback((ev: GameHandlersUpdateEvent) => syncHandlers(ev.handlers), [syncHandlers]),
  )

  useWebViewEventHandler(
    'AppGamepadButtonEvent',
    useCallback((ev: GamepadButtonPressedEvent) => handleButtonEvent(ev), [handleButtonEvent]),
  )

  useWebViewEventHandler(
    'GamepadsUpdate',
    useCallback((ev: GamepadsUpdateEvent) => handleGamepadsUpdate(ev), [handleGamepadsUpdate]),
  )

  useWebViewEventHandler(
    'ProfilesUpdate',
    useCallback((ev: ProfilesUpdateEvent) => {
      syncProfiles(ev.profiles)
    }, [syncProfiles])
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

          <div className={styles.gridScroll} ref={gridScrollRef}>
            <GameGrid />
          </div>

          {phase === 'select-game'  && <ConfirmGameBar />}
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