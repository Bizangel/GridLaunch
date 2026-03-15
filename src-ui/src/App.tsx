import { useUIState } from './store/ui-store'
import { TopBar } from './components/TopBar'
import { GameGrid } from './components/GameGrid'
import { ConfirmBar } from './components/Confirmbar'
import { Sidebar } from './components/Sidebar'
import { HintBar } from './components/Hintbar'
import styles from './App.module.css'

function App() {
  const phase = useUIState((s) => s.phase)

  return (
    <div className={styles.launcher}>
      <TopBar />

      <div className={styles.body}>
        {/* Main game pane */}
        <main className={styles.gamePane}>
          <div className={styles.sectionLabel}>installed games</div>
          <GameGrid />
          {phase === 'join-players' && <ConfirmBar />}
        </main>

        {/* Right sidebar */}
        <Sidebar />
      </div>

      <HintBar />
    </div>
  )
}

export default App