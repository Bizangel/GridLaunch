import { useUIState } from '../store/ui-store'
import { PLAYER_COLORS, PLAYER_LABELS, PROFILES } from '../data'
import styles from './SidePicker.module.css'

function hexToRgba(hex: string, alpha: number) {
  const r = parseInt(hex.slice(1, 3), 16)
  const g = parseInt(hex.slice(3, 5), 16)
  const b = parseInt(hex.slice(5, 7), 16)
  return `rgba(${r},${g},${b},${alpha})`
}

// Labels for each side index depending on orientation and player count
function sideLabel(index: number, orientation: 'horizontal' | 'vertical', totalPlayers: number): string {
  if (totalPlayers <= 2) {
    if (orientation === 'horizontal') return index === 0 ? 'top' : 'bottom'
    return index === 0 ? 'left' : 'right'
  }
  // 3-4 players: short quadrant labels to avoid wrapping
  const labels = ['↖ top left', '↗ top right', '↙ bot left', '↘ bot right']
  return labels[index] ?? `slot ${index + 1}`
}

function totalSides(playerCount: number): number {
  return playerCount <= 2 ? 2 : 4
}

export function SidePicker() {
  const players = useUIState((s) => s.players)
  const activePickerIdx = useUIState((s) => s.activePickerIdx)
  const orientation = useUIState((s) => s.splitOrientation)
  const pickSide = useUIState((s) => s.pickSide)

  if (activePickerIdx === null) return null
  const activePicker = players[activePickerIdx]
  if (!activePicker || activePicker.state !== 'picking-side') return null

  const activeColor = PLAYER_COLORS[activePickerIdx]
  const activeLabel = PLAYER_LABELS[activePickerIdx]
  const profileName = activePicker.profileId !== null
    ? PROFILES.find((p) => p.id === activePicker.profileId)?.name ?? ''
    : ''

  const joinedCount = players.filter(Boolean).length
  const sides = totalSides(joinedCount)

  // Which side indices are already taken by other ready/picking-side players
  const takenSides = new Map<number, number>() // sideIndex → playerSlotIdx
  players.forEach((p, i) => {
    if (i !== activePickerIdx && p && p.sideIndex !== null) {
      takenSides.set(p.sideIndex, i)
    }
  })

  // Available sides — if only one left, auto-pick is handled by caller but we still show it
  const availableSides = Array.from({ length: sides }, (_, i) => i).filter(
    (i) => !takenSides.has(i),
  )

  const isGrid = sides === 4

  return (
    <div className={styles.container}>
      <div className={styles.who}>
        <div className={styles.dot} style={{ background: activeColor }} />
        <span style={{ color: activeColor }}>{activeLabel}</span>
        <span className={styles.whoSub}>· {profileName} · pick a side</span>
      </div>

      <div className={isGrid ? styles.gridLayout : orientation === 'horizontal' ? styles.stackH : styles.stackV}>
        {Array.from({ length: sides }, (_, sideIdx) => {
          const ownerSlotIdx = takenSides.get(sideIdx)
          const isTaken = ownerSlotIdx !== undefined
          const isAvailable = availableSides.includes(sideIdx)
          const ownerColor = isTaken ? PLAYER_COLORS[ownerSlotIdx!] : undefined
          const ownerLabel = isTaken ? PLAYER_LABELS[ownerSlotIdx!] : undefined

          return (
            <div
              key={sideIdx}
              className={[
                styles.slot,
                isTaken && styles.slotTaken,
                isAvailable && styles.slotAvailable,
              ].filter(Boolean).join(' ')}
              style={isAvailable ? { borderColor: hexToRgba(activeColor, 0.5), background: hexToRgba(activeColor, 0.07) } : undefined}
              onClick={() => isAvailable && pickSide(sideIdx)}
            >
              {isTaken && ownerColor ? (
                <>
                  <span
                    className={styles.ownerTag}
                    style={{ background: hexToRgba(ownerColor, 0.18), color: ownerColor }}
                  >
                    {ownerLabel}
                  </span>
                </>
              ) : (
                <span className={styles.slotLabel} style={{ color: activeColor }}>
                  {sideLabel(sideIdx, orientation, joinedCount)}
                </span>
              )}
            </div>
          )
        })}
      </div>

      {availableSides.length === 1 && (
        <div className={styles.hint}>only one side available</div>
      )}
      {availableSides.length > 1 && (
        <div className={styles.hint}>click or use ← → to choose</div>
      )}
    </div>
  )
}