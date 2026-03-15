import { useUIState } from '../store/ui-store'
import { CONTROLLERS, PLAYER_COLORS, PLAYER_LABELS } from '../data'
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
  const players = useUIState((s) => s.players)
  const joinController = useUIState((s) => s.joinController)
  const unjoinPlayer = useUIState((s) => s.unjoinPlayer)
  const setPickerActive = useUIState((s) => s.setPickerActive)

  const isLocked = phase === 'select-game'

  function handleClick(controllerId: number) {
    if (isLocked) return
    const slotIdx = players.findIndex((p) => p?.controllerId === controllerId)
    if (slotIdx === -1) {
      joinController(controllerId)
    } else if (players[slotIdx]?.state === 'picking') {
      unjoinPlayer(slotIdx)
    } else {
      // re-open profile picking for this player
      setPickerActive(slotIdx)
    }
  }

  return (
    <div className={`${styles.list} ${isLocked ? styles.locked : ''}`}>
      {CONTROLLERS.map((ctrl) => {
        const slotIdx = players.findIndex((p) => p?.controllerId === ctrl.id)
        const player = slotIdx !== -1 ? players[slotIdx] : null
        const color = player ? PLAYER_COLORS[slotIdx] : undefined
        const label = player ? PLAYER_LABELS[slotIdx] : undefined

        return (
          <div
            key={ctrl.id}
            className={[
              styles.row,
              !player && styles.unjoined,
              player?.state === 'picking' && styles.picking,
              player?.state === 'ready' && styles.ready,
            ]
              .filter(Boolean)
              .join(' ')}
            style={
              color
                ? {
                    borderColor: hexToRgba(color, 0.5),
                  }
                : undefined
            }
            onClick={() => handleClick(ctrl.id)}
          >
            {color && (
              <div className={styles.stripe} style={{ background: color }} />
            )}

            <div
              className={styles.icon}
              style={color ? { background: hexToRgba(color, 0.15) } : undefined}
            >
              <GamepadIcon color={color ?? '#6b7280'} size={12} />
            </div>

            <div className={styles.info}>
              <div className={styles.name} style={color ? { color } : undefined}>
                {ctrl.name}
              </div>
              <div className={styles.sub}>
                {player
                  ? `${label} · ${player.state === 'picking' ? 'picking profile...' : 'ready'}`
                  : ctrl.devPath}
              </div>
            </div>

            <span
              className={styles.badge}
              style={
                color
                  ? {
                      background: hexToRgba(color, 0.18),
                      color,
                    }
                  : undefined
              }
            >
              {!player && 'A to join'}
              {player?.state === 'picking' && label}
              {player?.state === 'ready' && 'ready'}
            </span>
          </div>
        )
      })}
    </div>
  )
}