import { useCallback } from 'react'
import { useUIState } from '../store/ui-store'
import { GAMES, PROFILES } from '../data'
import { launchSession } from '../ipc/launchSession'
import type { GamepadButtonPressedEvent, GamepadsUpdateEvent } from '../types'

export function useGamepadInput() {
  // ── State (read) ────────────────────────────────────────────────────────
  const phase           = useUIState((s) => s.phase)
  const players         = useUIState((s) => s.players)
  const activePickerIdx = useUIState((s) => s.activePickerIdx)
  const gameCursor      = useUIState((s) => s.gameCursor)
  const profileCursor   = useUIState((s) => s.profileCursor)
  const sideCursor      = useUIState((s) => s.sideCursor)

  // ── Actions (write) ─────────────────────────────────────────────────────
  const syncControllers   = useUIState((s) => s.syncControllers)
  const confirmGame       = useUIState((s) => s.confirmGame)
  const changeGame        = useUIState((s) => s.changeGame)
  const joinController    = useUIState((s) => s.joinController)
  const unjoinByDevPath   = useUIState((s) => s.unjoinByDevPath)
  const unjoinPlayer      = useUIState((s) => s.unjoinPlayer)
  const pickProfile       = useUIState((s) => s.pickProfile)
  const pickSide          = useUIState((s) => s.pickSide)
  const moveGameCursor    = useUIState((s) => s.moveGameCursor)
  const moveProfileCursor = useUIState((s) => s.moveProfileCursor)
  const moveSideCursor    = useUIState((s) => s.moveSideCursor)

  // ── Gamepad lifecycle ────────────────────────────────────────────────────

  const handleGamepadsUpdate = useCallback((ev: GamepadsUpdateEvent) => {
    syncControllers(ev.gamepads)
  }, [syncControllers])

  // ── Button input ─────────────────────────────────────────────────────────

  const handleButtonEvent = useCallback((ev: GamepadButtonPressedEvent) => {
    if (ev.release) return

    // All input is frozen while the game session is running
    if (phase === 'launching') return

    const { button, gamepad_devpath: devPath } = ev

    // ── Phase: select-game ───────────────────────────────────────────────
    // Any controller can navigate — no identity check needed here

    if (phase === 'select-game') {
      if (button === 'DpadRight' || button === 'DpadDown') {
        moveGameCursor(1, GAMES.length)
      } else if (button === 'DpadLeft' || button === 'DpadUp') {
        moveGameCursor(-1, GAMES.length)
      } else if (button === 'A') {
        confirmGame(GAMES[gameCursor].id)
      }
      return
    }

    // ── Phase: join-players ──────────────────────────────────────────────

    const mySlotIdx       = players.findIndex((p) => p?.devPath === devPath)
    const hasJoined       = mySlotIdx !== -1
    const myPlayer        = hasJoined ? players[mySlotIdx] : null
    const iAmActivePicker = activePickerIdx === mySlotIdx
    const activePicker    = activePickerIdx !== null ? players[activePickerIdx] : null

    // Start — any controller can trigger launch once all conditions are met
    // Conditions: ≥2 players, every joined player is 'ready', nobody mid-flow

    if (button === 'Start') {
      const joined = players.filter((p) => p !== null)
      const allReady = joined.length >= 2 && joined.every((p) => p!.state === 'ready')
      if (allReady) launchSession()
      return
    }

    // B ───────────────────────────────────────────────────────────────────

    if (button === 'B') {
      if (!hasJoined) {
        // Not joined: B goes back to game selection if no one has joined yet
        const anyJoined = players.some((p) => p !== null)
        if (!anyJoined) {
          changeGame()
          return
        }
        // Otherwise cancel another player's pending (not yet ready) join
        if (activePickerIdx !== null && activePicker?.state !== 'ready') {
          unjoinPlayer(activePickerIdx)
        }
        return
      }
      // Joined: leave
      unjoinByDevPath(devPath)
      return
    }

    // A ───────────────────────────────────────────────────────────────────

    if (button === 'A') {
      if (!hasJoined) {
        // Block joining while someone else is actively mid-flow.
        // They must finish (or be cancelled via B) before a new player can join.
        const someoneIsPicking = players.some(
          (p) => p !== null && (p.state === 'picking' || p.state === 'picking-side'),
        )
        if (someoneIsPicking) return
        joinController(devPath)
        return
      }

      // Joined but not the active picker — cannot act on behalf of others
      if (!iAmActivePicker) return

      if (myPlayer?.state === 'picking') {
        const takenIds = new Set(
          players
            .filter((p, i) => p !== null && i !== mySlotIdx && p.profileId !== null)
            .map((p) => p!.profileId),
        )
        const available = PROFILES.filter((p) => !takenIds.has(p.id))
        const target = available[profileCursor]
        if (target) pickProfile(target.id)
        return
      }

      if (myPlayer?.state === 'picking-side') {
        const joinedCount = players.filter(Boolean).length
        const sideCount = joinedCount <= 2 ? 2 : 4
        const takenSides = new Set(
          players
            .filter((p, i) => p !== null && i !== mySlotIdx && p.sideIndex !== null)
            .map((p) => p!.sideIndex),
        )
        const availableSides = Array.from({ length: sideCount }, (_, i) => i).filter(
          (i) => !takenSides.has(i),
        )
        const target = availableSides[sideCursor]
        if (target !== undefined) pickSide(target)
        return
      }

      return
    }

    // Dpad — only the active picker can move cursors ──────────────────────

    if (!iAmActivePicker) return

    if (myPlayer?.state === 'picking') {
      const takenIds = new Set(
        players
          .filter((p, i) => p !== null && i !== mySlotIdx && p.profileId !== null)
          .map((p) => p!.profileId),
      )
      const availableCount = PROFILES.filter((p) => !takenIds.has(p.id)).length
      if (availableCount === 0) return
      if (button === 'DpadUp')   moveProfileCursor(-1, availableCount)
      if (button === 'DpadDown') moveProfileCursor(1,  availableCount)
      return
    }

    if (myPlayer?.state === 'picking-side') {
      const joinedCount = players.filter(Boolean).length
      const sideCount = joinedCount <= 2 ? 2 : 4
      const takenSides = new Set(
        players
          .filter((p, i) => p !== null && i !== mySlotIdx && p.sideIndex !== null)
          .map((p) => p!.sideIndex),
      )
      const availableSideCount = Array.from({ length: sideCount }, (_, i) => i)
        .filter((i) => !takenSides.has(i)).length
      if (availableSideCount === 0) return
      // Accept both axes — Left/Up = previous, Right/Down = next
      if (button === 'DpadLeft'  || button === 'DpadUp')   moveSideCursor(-1, availableSideCount)
      if (button === 'DpadRight' || button === 'DpadDown') moveSideCursor(1,  availableSideCount)
      return
    }

  }, [
    phase, players, activePickerIdx, gameCursor, profileCursor, sideCursor,
    confirmGame, changeGame, joinController, unjoinByDevPath, unjoinPlayer,
    pickProfile, pickSide, moveGameCursor, moveProfileCursor, moveSideCursor,
  ])

  return { handleButtonEvent, handleGamepadsUpdate }
}