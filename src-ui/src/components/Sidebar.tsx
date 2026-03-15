import { useUIState } from '../store/ui-store'
import { launchSession } from '../ipc/launchSession'
import { ControllerList } from './ControllerList'
import { ProfileList } from './ProfileList'
import { SidePicker } from './SidePicker'
import styles from './Sidebar.module.css'

function StartIcon() {
  return (
    <svg width="1em" height="1em" viewBox="0 0 16 16" fill="none" style={{ display: 'block', flexShrink: 0 }}>
      <circle cx="8" cy="8" r="6.5" stroke="currentColor" strokeWidth="1.2"/>
      <polygon points="6.5,5.5 11,8 6.5,10.5" fill="currentColor"/>
    </svg>
  )
}

export function Sidebar() {
  const phase           = useUIState((s) => s.phase)
  const players         = useUIState((s) => s.players)
  const activePickerIdx = useUIState((s) => s.activePickerIdx)

  const joined     = players.filter((p) => p !== null)
  const readyCount = joined.filter((p) => p!.state === 'ready').length
  const anyPicking = joined.some((p) => p!.state === 'picking' || p!.state === 'picking-side')
  const canLaunch  = readyCount >= 2 && !anyPicking

  const activePicker   = activePickerIdx !== null ? players[activePickerIdx] : null
  const showSidePicker = activePicker?.state === 'picking-side'

  return (
    <aside className={styles.sidebar}>
      {phase === 'select-game' && (
        <div className={styles.lockMsg}>select a game to unlock</div>
      )}

      <div className={styles.section}>
        <div className={styles.sectionTitle}>controllers</div>
        <ControllerList />
      </div>

      {showSidePicker ? (
        <SidePicker />
      ) : (
        <div className={styles.section} style={{ flex: 1, overflowY: 'auto', minHeight: 0 }}>
          <ProfileList />
        </div>
      )}

      <div className={styles.launchArea}>
        <button
          className={`${styles.launchBtn} ${canLaunch ? styles.launchBtnActive : ''}`}
          disabled={!canLaunch}
          onClick={launchSession}
        >
          <StartIcon />
          {canLaunch ? `Launch (${readyCount} players)` : 'Launch'}
        </button>
      </div>
    </aside>
  )
}