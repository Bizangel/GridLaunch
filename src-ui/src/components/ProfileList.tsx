import { useUIState } from '../store/ui-store'
import { PROFILES, PLAYER_COLORS, PLAYER_LABELS } from '../data'
import styles from './ProfileList.module.css'

function hexToRgba(hex: string, alpha: number) {
  const r = parseInt(hex.slice(1, 3), 16)
  const g = parseInt(hex.slice(3, 5), 16)
  const b = parseInt(hex.slice(5, 7), 16)
  return `rgba(${r},${g},${b},${alpha})`
}

export function ProfileList() {
  const phase = useUIState((s) => s.phase)
  const players = useUIState((s) => s.players)
  const activePickerIdx = useUIState((s) => s.activePickerIdx)
  const profileCursor = useUIState((s) => s.profileCursor)
  const pickProfile = useUIState((s) => s.pickProfile)

  const isLocked = phase === 'select-game'
  const activePicker = activePickerIdx !== null ? players[activePickerIdx] : null
  const activeColor = activePickerIdx !== null ? PLAYER_COLORS[activePickerIdx] : undefined
  const activeLabel = activePickerIdx !== null ? PLAYER_LABELS[activePickerIdx] : undefined

  function ownerOf(profileId: number): number | null {
    const idx = players.findIndex((p) => p?.profileId === profileId)
    return idx === -1 ? null : idx
  }

  // Build the list of available profiles in order, maintaining original indices
  const takenIds = new Set(
    players
      .filter((p, i) => p !== null && i !== activePickerIdx && p.profileId !== null)
      .map((p) => p!.profileId),
  )
  const availableProfiles = PROFILES.filter((p) => !takenIds.has(p.id))

  return (
    <div className={`${styles.section} ${isLocked ? styles.locked : ''}`}>
      <div className={styles.title}>
        profiles
        {activePicker && activeLabel && (
          <span className={styles.pickerHint} style={{ color: activeColor }}>
            {' '}· {activeLabel} choosing
          </span>
        )}
      </div>

      <div className={styles.list}>
        {PROFILES.map((profile) => {
          const ownerIdx = ownerOf(profile.id)
          const isTaken = ownerIdx !== null
          const ownerColor = isTaken ? PLAYER_COLORS[ownerIdx!] : undefined
          const ownerLabel = isTaken ? PLAYER_LABELS[ownerIdx!] : undefined
          const isPickable = !!activePicker && !isTaken

          // Cursor highlight — find this profile's position in the available list
          const availableIdx = availableProfiles.findIndex((p) => p.id === profile.id)
          const isCursorOn = isPickable && availableIdx === profileCursor

          return (
            <div
              key={profile.id}
              className={[
                styles.row,
                isTaken && styles.taken,
                isPickable && styles.pickable,
                isCursorOn && styles.cursorOn,
              ].filter(Boolean).join(' ')}
              style={
                isCursorOn && activeColor
                  ? { borderColor: hexToRgba(activeColor, 0.6), background: hexToRgba(activeColor, 0.08) }
                  : isTaken && ownerColor
                  ? { borderColor: hexToRgba(ownerColor, 0.45), background: hexToRgba(ownerColor, 0.05) }
                  : undefined
              }
              onClick={() => isPickable && pickProfile(profile.id)}
            >
              <div
                className={styles.avatar}
                style={
                  isCursorOn && activeColor
                    ? { background: hexToRgba(activeColor, 0.2), color: activeColor }
                    : isTaken && ownerColor
                    ? { background: hexToRgba(ownerColor, 0.2), color: ownerColor }
                    : undefined
                }
              >
                {profile.initials}
              </div>

              <div className={styles.info}>
                <div className={styles.name}>{profile.name}</div>
              </div>

              {isTaken && ownerColor && (
                <span
                  className={styles.ownerBadge}
                  style={{ background: hexToRgba(ownerColor, 0.18), color: ownerColor }}
                >
                  {ownerLabel}
                </span>
              )}

              {isCursorOn && (
                <span className={styles.selectHint} style={{ color: activeColor }}>▶</span>
              )}
            </div>
          )
        })}
      </div>
    </div>
  )
}