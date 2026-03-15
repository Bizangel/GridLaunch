import { useUIState } from '../store/ui-store'
import { PLAYER_COLORS, PLAYER_LABELS } from '../data'
import { GamepadIcon } from './GamepadIcon'
import styles from './ControllerList.module.css'

function hexToRgba(hex: string, alpha: number) {
  const r = parseInt(hex.slice(1, 3), 16)
  const g = parseInt(hex.slice(3, 5), 16)
  const b = parseInt(hex.slice(5, 7), 16)
  return `rgba(${r},${g},${b},${alpha})`
}

export function ControllerList() {
  const phase = useUIState((s) => s.phase)
  const connectedControllers = useUIState((s) => s.connectedControllers)
  const players = useUIState((s) => s.players)
  const unjoinByDevPath = useUIState((s) => s.unjoinByDevPath)
  const setPickerActive = useUIState((s) => s.setPickerActive)
  const joinController = useUIState((s) => s.joinController)

  const isLocked = phase === 'select-game'

  function handleClick(devPath: string) {
    if (isLocked) return
    const slotIdx = players.findIndex((p) => p?.devPath === devPath)
    if (slotIdx === -1) {
      joinController(devPath)
    } else if (players[slotIdx]?.state === 'picking') {
      unjoinByDevPath(devPath)
    } else if (players[slotIdx]?.state === 'ready') {
      setPickerActive(slotIdx)
    }
  }

  return (
    <div className={`${styles.list} ${isLocked ? styles.locked : ''}`}>
      {connectedControllers.map((ctrl) => {
        const slotIdx = players.findIndex((p) => p?.devPath === ctrl.devPath)
        const player = slotIdx !== -1 ? players[slotIdx] : null
        const color = player ? PLAYER_COLORS[slotIdx] : undefined
        const label = player ? PLAYER_LABELS[slotIdx] : undefined

        return (
          <div
            key={ctrl.devPath}
            className={[
              styles.row,
              !player && styles.unjoined,
            ].filter(Boolean).join(' ')}
            style={color ? { borderColor: hexToRgba(color, 0.5) } : undefined}
            onClick={() => handleClick(ctrl.devPath)}
          >
            {color && <div className={styles.stripe} style={{ background: color }} />}

            <div
              className={styles.icon}
              style={color ? { background: hexToRgba(color, 0.15) } : undefined}
            >
              <GamepadIcon color={color ?? '#6b7280'} size="var(--fs-md)" />
            </div>

            <div className={styles.info}>
              <div className={styles.name} style={color ? { color } : undefined}>
                {ctrl.name}
              </div>
              <span
                className={styles.badge}
                style={color ? { background: hexToRgba(color, 0.18), color } : undefined}
              >
                {!player && 'A to join'}
                {player?.state === 'picking' && `${label} · picking`}
                {player?.state === 'picking-side' && `${label} · side`}
                {player?.state === 'ready' && `${label} · ready`}
              </span>
            </div>
          </div>
        )
      })}

      {connectedControllers.length === 0 && (
        <div style={{ fontFamily: 'var(--font-mono)', fontSize: 'var(--fs-xs)', color: 'var(--mt)', padding: '4px 0' }}>
          no controllers detected
        </div>
      )}
    </div>
  )
}