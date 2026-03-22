import { useUIState } from '../store/ui-store'
import styles from './TopBar.module.css'

export function TopBar() {
  const orientation = useUIState((s) => s.splitOrientation)
  const toggleOrientation = useUIState((s) => s.toggleOrientation)
  const controllerCount = useUIState((s) => s.connectedControllers.length)

  return (
    <header className={styles.topbar}>
      <div className={styles.logo}>
        <div className={styles.logoIcon}>
          <span className={styles.logoHalf} />
          <span className={`${styles.logoHalf} ${styles.logoHalfDim}`} />
        </div>
        SplitPlay
      </div>

      <button className={styles.splitToggle} onClick={toggleOrientation} title="Toggle split orientation">
        <div className={`${styles.splitIcon} ${orientation === 'Horizontal' ? styles.splitH : styles.splitV}`}>
          <div className={styles.splitPane1} />
          <div className={styles.splitPane2} />
        </div>
        <span>{orientation}</span>
      </button>

      <div className={styles.ctrlCount}>
        <span className={styles.statusDot} />
        {controllerCount} controller{controllerCount !== 1 ? 's' : ''}
      </div>
    </header>
  )
}