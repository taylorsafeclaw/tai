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

export function Logo({ className, ...props }: React.ComponentPropsWithoutRef<'span'>) {
  return (
    <span className="flex items-center gap-2" aria-label="tstack" {...props}>
      <Logomark className="h-7 w-7 text-lime-400" />
      <span className="font-mono text-base font-semibold tracking-tight text-neutral-900 dark:text-white">
        tstack
      </span>
    </span>
  )
}
