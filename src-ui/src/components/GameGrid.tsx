import { useUIState } from '../store/ui-store'
import { GAMES } from '../data'
import styles from './GameGrid.module.css'

export function GameGrid() {
  const phase        = useUIState((s) => s.phase)
  const selectedGameId = useUIState((s) => s.selectedGameId)
  const gameCursor   = useUIState((s) => s.gameCursor)
  const confirmGame  = useUIState((s) => s.confirmGame)
  const moveGameCursor = useUIState((s) => s.moveGameCursor)

  const isPhase1 = phase === 'select-game'
  // gameCursor is an index into GAMES; derive the focused game id from it
  const focusedGame = isPhase1 ? GAMES[gameCursor] : null

  return (
    <div className={styles.grid}>
      {GAMES.map((game, idx) => {
        const isSelected = game.id === selectedGameId
        const isFocused  = isPhase1 && idx === gameCursor
        const isDimmed   = !isPhase1 && !isSelected

        return (
          <div
            key={game.id}
            className={[
              styles.card,
              isSelected && styles.selected,
              isFocused  && styles.focused,
              isDimmed   && styles.dimmed,
            ].filter(Boolean).join(' ')}
            onClick={() => {
              if (!isPhase1) return
              // Click moves cursor to this card
              moveGameCursor(idx - gameCursor, GAMES.length)
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

      {isPhase1 && (
        <div className={styles.confirmRow}>
          <div className={styles.confirmGame}>
            <div className={`${styles.dot} ${focusedGame ? styles.dotActive : ''}`} />
            <div>
              <div className={`${styles.confirmName} ${focusedGame ? styles.confirmNameActive : ''}`}>
                {focusedGame ? focusedGame.name : 'no game selected'}
              </div>
              <div className={styles.confirmHint}>
                press A to confirm · dpad to browse
              </div>
            </div>
          </div>
          <button
            className={`${styles.confirmBtn} ${focusedGame ? styles.confirmBtnActive : ''}`}
            onClick={() => focusedGame && confirmGame(focusedGame.id)}
          >
            confirm A
          </button>
        </div>
      )}
    </div>
  )
}