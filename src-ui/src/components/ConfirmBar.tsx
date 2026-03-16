import { useUIState } from '../store/ui-store'
import styles from './ConfirmBar.module.css'

export function ConfirmBar() {
  const games            = useUIState((s) => s.games)
  const selectedGameName = useUIState((s) => s.selectedGameName)
  const changeGame       = useUIState((s) => s.changeGame)

  const game = games.find((g) => g.name === selectedGameName)
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