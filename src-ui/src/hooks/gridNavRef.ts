/**
 * Stable ref to the grid's scroll container (.gridScroll in App.module.css).
 * GameGrid writes scrollTop directly against this rather than relying on
 * scrollIntoView, which breaks when overflow:hidden sits between the card
 * and the scrollable ancestor.
 */
export const gridScrollContainerRef = {
  el: null as HTMLElement | null,
}

import type { GridDirection } from './useGridNav'

export const gridNavRef = {
  navigate: null as ((dir: GridDirection) => void) | null,
}