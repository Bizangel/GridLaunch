import { useUIState } from '../store/ui-store'
import { GAMES } from '../data'
import styles from './GameGrid.module.css'

export function ConfirmGameBar() {
  const gameCursor = useUIState((s) => s.gameCursor)
  const confirmGame = useUIState((s) => s.confirmGame)
  const focusedGame = GAMES[gameCursor]

  return (
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
  )
}