import Link from 'next/link'
import { usePathname } from 'next/navigation'

import { cn } from '@/lib/utils'

import { navigation } from '@/lib/navigation'

export function Navigation({
  className,
  onLinkClick,
}: {
  className?: string
  onLinkClick?: React.MouseEventHandler<HTMLAnchorElement>
}) {
  const pathname = usePathname()

  return (
    <nav className={cn('text-base lg:text-sm', className)}>
      <ul role="list" className="space-y-9">
        {navigation.map((section) => (
          <li key={section.title}>
            <h2 className="font-mono text-xs font-bold uppercase tracking-widest text-neutral-400">
              {section.title}
            </h2>
            <ul
              role="list"
              className="mt-2 space-y-2 border-l-2 border-neutral-800 lg:mt-4 lg:space-y-4"
            >
              {section.links.map((link) => (
                <li key={link.href} className="relative">
                  <Link
                    href={link.href}
                    onClick={onLinkClick}
                    className={cn(
                      'block w-full pl-3.5 font-mono before:pointer-events-none before:absolute before:top-1/2 before:-left-[2px] before:h-4 before:w-[3px] before:-translate-y-1/2',
                      link.href === pathname
                        ? 'font-semibold text-lime-400 before:bg-lime-400'
                        : 'text-neutral-500 before:hidden hover:text-neutral-300 hover:before:block hover:before:bg-neutral-600',
                    )}
                  >
                    {link.title}
                  </Link>
                </li>
              ))}
            </ul>
          </li>
        ))}
      </ul>
    </nav>
  )
}
