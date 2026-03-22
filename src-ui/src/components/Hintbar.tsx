import { useUIState } from '../store/ui-store'
import styles from './Hintbar.module.css'

type Hint = { key: string; label: string; color?: string }

export function HintBar() {
  const phase           = useUIState((s) => s.phase)
  const players         = useUIState((s) => s.players)
  const activePickerIdx = useUIState((s) => s.activePickerIdx)
  const orientation     = useUIState((s) => s.splitOrientation)

  const joined    = players.filter(Boolean)
  const ready     = players.filter((p) => p?.state === 'ready')
  const openSlots = 4 - joined.length

  const activePicker     = activePickerIdx !== null ? players[activePickerIdx] : null
  const isPickingProfile = activePicker?.state === 'picking'
  const isPickingSide    = activePicker?.state === 'picking-side'

  const nextOrientation = orientation === 'Horizontal' ? 'Vertical' : 'Horizontal'

  const hints: Hint[] = []

  if (phase === 'select-game') {
    hints.push({ key: '↑↓←→', label: 'browse' })
    hints.push({ key: 'A', label: 'confirm game' })
  } else if (phase === 'join-players') {
    if (isPickingProfile) {
      hints.push({ key: '↑↓', label: 'pick profile' })
      hints.push({ key: 'A', label: 'confirm' })
    } else if (isPickingSide) {
      hints.push({ key: '← →', label: 'pick side' })
      hints.push({ key: 'A', label: 'confirm' })
    }
    hints.push({ key: 'B', label: joined.length > 0 ? 'unjoin / change game' : 'change game' })
    if (ready.length >= 2) {
      hints.push({ key: 'Start', label: 'launch' })
    }
  }

  // Select is always available (except while launching)
  if (phase !== 'launching') {
    hints.push({ key: 'Select', label: `→ ${nextOrientation}` })
  }

  return (
    <div className={styles.bar}>
      <div className={styles.hints}>
        {hints.map((h) => (
          <div key={h.key + h.label} className={styles.hint}>
            <span className={styles.key} style={h.color ? { background: `${h.color}22`, color: h.color } : undefined}>
              {h.key}
            </span>
            <span className={styles.label}>{h.label}</span>
          </div>
        ))}
      </div>

      {phase === 'join-players' && openSlots > 0 && (
        <div className={styles.slotsOpen}>
          {openSlots} slot{openSlots !== 1 ? 's' : ''} open
        </div>
      )}
    </div>
  )
}