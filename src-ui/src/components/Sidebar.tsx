import { useUIState } from '../store/ui-store'
import { ControllerList } from './ControllerList'
import { ProfileList } from './ProfileList'
import styles from './Sidebar.module.css'

export function Sidebar() {
  const phase = useUIState((s) => s.phase)
  const players = useUIState((s) => s.players)

  const readyCount = players.filter((p) => p?.state === 'ready').length
  const canLaunch = readyCount >= 2

  return (
    <aside className={styles.sidebar}>
      {phase === 'select-game' && (
        <div className={styles.lockMsg}>select a game to unlock</div>
      )}

      <div className={styles.section}>
        <div className={styles.sectionTitle}>controllers</div>
        <ControllerList />
      </div>

      <div className={styles.section} style={{ flex: 1, overflowY: 'auto' }}>
        <ProfileList />
      </div>

      <div className={styles.launchArea}>
        <button
          className={`${styles.launchBtn} ${canLaunch ? styles.launchBtnActive : ''}`}
          disabled={!canLaunch}
        >
          {canLaunch ? `Launch (${readyCount} players)` : 'Launch'}
        </button>
      </div>
    </aside>
  )
}