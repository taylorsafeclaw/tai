import Link from 'next/link'

import { Icon } from '@/components/Icon'

export function QuickLinks({ children }: { children: React.ReactNode }) {
  return (
    <div className="not-prose my-12 grid grid-cols-1 gap-6 sm:grid-cols-2">
      {children}
    </div>
  )
}

export function QuickLink({
  title,
  description,
  href,
  icon,
}: {
  title: string
  description: string
  href: string
  icon: React.ComponentProps<typeof Icon>['icon']
}) {
  return (
    <div className="group relative border-2 border-neutral-800 transition-colors hover:border-lime-400/50 dark:hover:bg-lime-400/5">
      <div className="relative overflow-hidden p-6">
        <Icon icon={icon} className="h-8 w-8" />
        <h2 className="mt-4 font-mono text-base font-bold text-slate-900 dark:text-white">
          <Link href={href}>
            <span className="absolute -inset-px" />
            {title}
          </Link>
        </h2>
        <p className="mt-1 text-sm text-slate-700 dark:text-neutral-400">
          {description}
        </p>
      </div>
    </div>
  )
}
