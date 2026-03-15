import { useUIState } from '../store/ui-store'
import { GAMES } from '../data'
import styles from './LaunchingOverlay.module.css'

export function LaunchingOverlay() {
  const selectedGameId = useUIState((s) => s.selectedGameId)
  const game = GAMES.find((g) => g.id === selectedGameId)

  return (
    <div className={styles.overlay}>
      <div className={styles.card}>
        <div className={styles.gameName}>{game?.name ?? 'Game'}</div>
        <div className={styles.status}>launching session</div>
        <div className={styles.track}>
          <div className={styles.bar} />
        </div>
        <div className={styles.hint}>waiting for game to exit</div>
      </div>
    </div>
  )
}