type Props = { color?: string; size?: number }

export function GamepadIcon({ color = 'currentColor', size = 14 }: Props) {
  return (
    <svg width={size} height={size} viewBox="0 0 16 16" fill="none" style={{ color }}>
      <rect x="1" y="4" width="14" height="8" rx="3" stroke="currentColor" strokeWidth="1.2" />
      <line x1="5" y1="8" x2="7" y2="8" stroke="currentColor" strokeWidth="1.2" strokeLinecap="round" />
      <line x1="6" y1="7" x2="6" y2="9" stroke="currentColor" strokeWidth="1.2" strokeLinecap="round" />
      <circle cx="10.5" cy="7.5" r="0.8" fill="currentColor" />
      <circle cx="12" cy="8.5" r="0.8" fill="currentColor" />
      <circle cx="9" cy="8.5" r="0.8" fill="currentColor" />
    </svg>
  )
}