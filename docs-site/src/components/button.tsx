import Link from 'next/link'
import { cn } from '@/lib/utils'

const variantStyles = {
  primary:
    'bg-lime-400 py-2 px-4 text-sm font-bold font-mono uppercase tracking-wider text-black hover:bg-lime-300 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-lime-400/50 active:bg-lime-500 border-2 border-lime-400',
  secondary:
    'bg-transparent py-2 px-4 text-sm font-bold font-mono uppercase tracking-wider text-white hover:bg-white/10 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-white/50 active:text-neutral-400 border-2 border-neutral-700 hover:border-neutral-500',
}

type ButtonProps = {
  variant?: keyof typeof variantStyles
} & (
  | React.ComponentPropsWithoutRef<typeof Link>
  | (React.ComponentPropsWithoutRef<'button'> & { href?: undefined })
)

export function Button({
  variant = 'primary',
  className,
  ...props
}: ButtonProps) {
  className = cn(variantStyles[variant], className)

  return typeof props.href === 'undefined' ? (
    <button className={className} {...props} />
  ) : (
    <Link className={className} {...props} />
  )
}
