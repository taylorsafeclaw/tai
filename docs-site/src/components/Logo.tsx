export function Logomark(props: React.ComponentPropsWithoutRef<'svg'>) {
  return (
    <svg aria-hidden="true" viewBox="0 0 36 36" fill="none" {...props}>
      <rect
        x="2"
        y="2"
        width="32"
        height="32"
        stroke="currentColor"
        strokeWidth="3"
        fill="none"
      />
      <path d="M10 18h16M18 10v16" stroke="currentColor" strokeWidth="3" />
    </svg>
  )
}

export function Logo(props: React.ComponentPropsWithoutRef<'svg'>) {
  void props
  return (
    <span className="flex items-center gap-2" aria-label="tstack">
      <Logomark className="h-7 w-7 text-lime-400" />
      <span className="font-display text-lg font-bold tracking-tighter text-slate-900 uppercase dark:text-white">
        tstack
      </span>
    </span>
  )
}
