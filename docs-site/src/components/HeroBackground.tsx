'use client'

import { useId } from 'react'

export function HeroBackground(props: React.ComponentPropsWithoutRef<'svg'>) {
  let id = useId()

  return (
    <svg
      aria-hidden="true"
      viewBox="0 0 668 1069"
      width={668}
      height={1069}
      fill="none"
      {...props}
    >
      <defs>
        <clipPath id={`${id}-clip`}>
          <path fill="#fff" d="M0 0h668v1069H0z" />
        </clipPath>
      </defs>
      <g opacity=".15" clipPath={`url(#${id}-clip)`} strokeWidth={2}>
        {/* Vertical grid lines */}
        {[83, 183, 283, 383, 483, 583].map((x) => (
          <line
            key={x}
            x1={x}
            y1={0}
            x2={x}
            y2={1069}
            stroke="#a3e635"
            opacity={0.3}
          />
        ))}
        {/* Horizontal grid lines */}
        {[0, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000].map((y) => (
          <line
            key={y}
            x1={0}
            y1={y}
            x2={668}
            y2={y}
            stroke="#a3e635"
            opacity={0.15}
          />
        ))}
        {/* Accent nodes */}
        {[
          [83, 200],
          [183, 400],
          [283, 300],
          [383, 600],
          [483, 500],
          [583, 700],
          [183, 800],
          [383, 900],
        ].map(([cx, cy], i) => (
          <rect
            key={i}
            x={cx! - 6}
            y={cy! - 6}
            width={12}
            height={12}
            fill={i % 3 === 0 ? '#a3e635' : 'none'}
            fillOpacity={i % 3 === 0 ? 0.4 : 0}
            stroke="#a3e635"
            strokeOpacity={0.5}
          />
        ))}
        {/* Diagonal accents */}
        <line x1={83} y1={200} x2={183} y2={400} stroke="#a3e635" opacity={0.2} />
        <line x1={283} y1={300} x2={383} y2={600} stroke="#a3e635" opacity={0.2} />
        <line x1={483} y1={500} x2={583} y2={700} stroke="#a3e635" opacity={0.2} />
      </g>
    </svg>
  )
}
