import { useEffect } from 'react'
import { useUIState } from '../store/ui-store'
import { useGridNav } from '../hooks/useGridNav'
import { gridNavRef } from '../hooks/gridNavRef'
import styles from './GameGrid.module.css'

export function GameGrid() {
  const phase             = useUIState((s) => s.phase)
  const games             = useUIState((s) => s.games)
  const selectedGameName  = useUIState((s) => s.selectedGameName)
  const gameCursor        = useUIState((s) => s.gameCursor)
  const confirmGame       = useUIState((s) => s.confirmGame)
  const setGameCursor     = useUIState((s) => s.setGameCursor)

  const isPhase1 = phase === 'select-game'

  const { setCardRef, navigate, cardRefs } = useGridNav(games.length, gameCursor, setGameCursor)

  useEffect(() => {
    gridNavRef.navigate = navigate
    return () => { gridNavRef.navigate = null }
  }, [navigate])

  useEffect(() => {
    if (!isPhase1) return
    cardRefs.current[gameCursor]?.scrollIntoView({ block: 'nearest', inline: 'nearest' })
  }, [gameCursor, isPhase1, cardRefs])

  if (games.length === 0) {
    return (
      <div className={styles.empty}>
        <span>no games installed</span>
      </div>
    )
  }

  return (
    <div className={styles.grid}>
      {games.map((game, idx) => {
        const isSelected = game.name === selectedGameName
        const isFocused  = isPhase1 && idx === gameCursor
        const isDimmed   = !isPhase1 && !isSelected

        return (
          <div
            key={game.name}
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
              confirmGame(game.name)
            }}
          >
            {game.imageBase64 ? (
              <img
                className={styles.thumb}
                src={game.imageBase64}
                alt={game.name}
              />
            ) : (
              <div className={styles.thumbPlaceholder} />
            )}
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