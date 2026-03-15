import type { GridDirection } from './useGridNav'

/**
 * Module-level stable ref. GameGrid registers its navigate function here
 * on mount; useGamepadInput calls it on Dpad events.
 *
 * This is intentionally imperative — navigate() reads live DOM rects and
 * must not be reactive state. A ref is the correct primitive here.
 */
export const gridNavRef = {
  navigate: null as ((dir: GridDirection) => void) | null,
}