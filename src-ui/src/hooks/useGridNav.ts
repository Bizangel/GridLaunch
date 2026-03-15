import { useRef, useCallback } from 'react'

export type GridDirection = 'left' | 'right' | 'up' | 'down'

/**
 * Provides refs for a list of grid cards and a navigate() function that
 * picks the spatially closest card in the given direction based on actual
 * rendered bounding rects — works regardless of how many columns the CSS
 * grid happens to produce at any viewport size.
 *
 * Usage:
 *   const { setCardRef, navigate } = useGridNav(count, cursor, onMove)
 *   <div ref={setCardRef(idx)}>...</div>
 */
export function useGridNav(
  count: number,
  cursor: number,
  onMove: (nextIdx: number) => void,
) {
  const cardRefs = useRef<(HTMLElement | null)[]>([])

  const setCardRef = useCallback(
    (idx: number) => (el: HTMLElement | null) => {
      cardRefs.current[idx] = el
    },
    [],
  )

  const navigate = useCallback(
    (dir: GridDirection) => {
      const rects = cardRefs.current.map((el) => el?.getBoundingClientRect() ?? null)
      const current = rects[cursor]
      if (!current) return

      const cx = current.left + current.width / 2
      const cy = current.top + current.height / 2

      let bestIdx = -1
      let bestScore = Infinity

      for (let i = 0; i < count; i++) {
        if (i === cursor) continue
        const r = rects[i]
        if (!r) continue

        const rx = r.left + r.width / 2
        const ry = r.top + r.height / 2
        const dx = rx - cx
        const dy = ry - cy

        // Only consider candidates in the correct half-plane for the direction,
        // with a small epsilon to handle cards that are nearly aligned
        const EPSILON = 4
        const inHalfPlane =
          (dir === 'right' && dx >  EPSILON) ||
          (dir === 'left'  && dx < -EPSILON) ||
          (dir === 'down'  && dy >  EPSILON) ||
          (dir === 'up'    && dy < -EPSILON)

        if (!inHalfPlane) continue

        // Score: primary axis distance + heavy penalty for off-axis drift.
        // This means "directly below" always wins over "diagonally below".
        const primary    = dir === 'left' || dir === 'right' ? Math.abs(dx) : Math.abs(dy)
        const offAxis    = dir === 'left' || dir === 'right' ? Math.abs(dy) : Math.abs(dx)
        const score      = primary + offAxis * 4

        if (score < bestScore) {
          bestScore = score
          bestIdx = i
        }
      }

      if (bestIdx !== -1) onMove(bestIdx)
    },
    [count, cursor, onMove],
  )

  return { setCardRef, navigate, cardRefs }
}