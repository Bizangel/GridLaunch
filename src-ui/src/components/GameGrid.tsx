import { useEffect } from 'react'
import { useUIState } from '../store/ui-store'
import { GAMES } from '../data'
import { useGridNav } from '../hooks/useGridNav'
import { gridNavRef } from '../hooks/gridNavRef'
import styles from './GameGrid.module.css'

export function GameGrid() {
  const phase          = useUIState((s) => s.phase)
  const selectedGameId = useUIState((s) => s.selectedGameId)
  const gameCursor     = useUIState((s) => s.gameCursor)
  const confirmGame    = useUIState((s) => s.confirmGame)
  const setGameCursor  = useUIState((s) => s.setGameCursor)

  const isPhase1 = phase === 'select-game'

  const { setCardRef, navigate, cardRefs } = useGridNav(GAMES.length, gameCursor, setGameCursor)

  useEffect(() => {
    gridNavRef.navigate = navigate
    return () => { gridNavRef.navigate = null }
  }, [navigate])

  // Scroll focused card into view whenever cursor changes
  useEffect(() => {
    if (!isPhase1) return
    cardRefs.current[gameCursor]?.scrollIntoView({ block: 'nearest', inline: 'nearest' })
  }, [gameCursor, isPhase1])

  return (
    <div className={styles.grid}>
      {GAMES.map((game, idx) => {
        const isSelected = game.id === selectedGameId
        const isFocused  = isPhase1 && idx === gameCursor
        const isDimmed   = !isPhase1 && !isSelected

        return (
          <div
            key={game.id}
            ref={setCardRef(idx)}
            className={[
              styles.card,
              isSelected && styles.selected,
              isFocused  && styles.focused,
              isDimmed   && styles.dimmed,
            ].filter(Boolean).join(' ')}
            onClick={() => {
              if (!isPhase1) return
              setGameCursor(idx)
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
    </div>
  )
}