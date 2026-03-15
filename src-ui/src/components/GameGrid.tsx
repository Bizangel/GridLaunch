import { useState } from 'react'
import { useUIState } from '../store/ui-store'
import { GAMES } from '../data'
import styles from './GameGrid.module.css'

export function GameGrid() {
  const phase = useUIState((s) => s.phase)
  const selectedGameId = useUIState((s) => s.selectedGameId)
  const confirmGame = useUIState((s) => s.confirmGame)

  // In phase 1, track the focused (highlighted) game before confirmation
  const [focusedId, setFocusedId] = useState<number | null>(null)

  const isPhase1 = phase === 'select-game'

  return (
    <div className={styles.grid}>
      {GAMES.map((game) => {
        const isSelected = game.id === selectedGameId
        const isFocused = game.id === focusedId && isPhase1
        const isDimmed = !isPhase1 && !isSelected

        return (
          <div
            key={game.id}
            className={[
              styles.card,
              isSelected && styles.selected,
              isFocused && styles.focused,
              isDimmed && styles.dimmed,
            ]
              .filter(Boolean)
              .join(' ')}
            onClick={() => {
              if (!isPhase1) return
              setFocusedId(game.id)
            }}
            onDoubleClick={() => {
              if (!isPhase1) return
              confirmGame(game.id)
            }}
          >
            <div className={styles.thumb} style={{ background: game.coverColor }} />
            {isSelected && <div className={styles.selectedBadge}>selected</div>}
            <div className={styles.info}>
              <div className={styles.title}>{game.name}</div>
              <div className={styles.desc}>{game.description}</div>
            </div>
          </div>
        )
      })}

      {/* Phase 1: confirm bar shown inside game pane below grid */}
      {isPhase1 && (
        <div className={styles.confirmRow}>
          <div className={styles.confirmGame}>
            <div className={`${styles.dot} ${focusedId ? styles.dotActive : ''}`} />
            <div>
              <div className={`${styles.confirmName} ${focusedId ? styles.confirmNameActive : ''}`}>
                {focusedId ? GAMES.find((g) => g.id === focusedId)?.name : 'no game selected'}
              </div>
              <div className={styles.confirmHint}>
                {focusedId ? 'press A or double-click to confirm' : 'browse and confirm to continue'}
              </div>
            </div>
          </div>
          <button
            className={`${styles.confirmBtn} ${focusedId ? styles.confirmBtnActive : ''}`}
            disabled={!focusedId}
            onClick={() => focusedId && confirmGame(focusedId)}
          >
            confirm A
          </button>
        </div>
      )}
    </div>
  )
}