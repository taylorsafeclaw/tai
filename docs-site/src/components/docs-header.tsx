'use client'

import { usePathname } from 'next/navigation'

import { navigation } from '@/lib/navigation'

export function DocsHeader({ title }: { title?: string }) {
  const pathname = usePathname()
  const section = navigation.find((section) =>
    section.links.find((link) => link.href === pathname),
  )

  if (!title && !section) {
    return null
  }

  return (
    <header className="mb-9 space-y-1">
      {section && (
        <p className="font-mono text-xs font-bold uppercase tracking-widest text-lime-400">
          {section.title}
        </p>
      )}
      {title && (
        <h1 className="font-mono text-3xl font-bold tracking-tight text-neutral-900 dark:text-white">
          {title}
        </h1>
      )}
    </header>
  )
}
