import { useUIState } from '../store/ui-store'
import styles from './LaunchingOverlay.module.css'

export function LaunchingOverlay() {
  const games            = useUIState((s) => s.games)
  const selectedGameName = useUIState((s) => s.selectedGameName)
  const game = games.find((g) => g.name === selectedGameName)

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