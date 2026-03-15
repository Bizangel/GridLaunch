import { useUIState } from '../store/ui-store'
import { GAMES } from '../data'
import styles from './ConfirmBar.module.css'

export function ConfirmBar() {
  const selectedGameId = useUIState((s) => s.selectedGameId)
  const changeGame = useUIState((s) => s.changeGame)

  const game = GAMES.find((g) => g.id === selectedGameId)
  if (!game) return null

  return (
    <div className={styles.bar}>
      <div className={styles.gameInfo}>
        <div className={styles.dot} />
        <div>
          <div className={styles.name}>{game.name}</div>
          <div className={styles.sub}>
            up to {game.maxPlayers} players · controllers unlocked
          </div>
        </div>
      </div>
      <button className={styles.changeBtn} onClick={changeGame}>
        change B
      </button>
    </div>
  )
}