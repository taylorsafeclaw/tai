'use client'

import { usePathname } from 'next/navigation'
import { type Node } from '@markdoc/markdoc'
import { cn } from '@/lib/utils'

import { DocsHeader } from '@/components/docs-header'
import { PrevNextLinks } from '@/components/prev-next-links'
import { Prose } from '@/components/prose'
import { TableOfContents } from '@/components/table-of-contents'
import { collectSections } from '@/lib/sections'

export function DocsLayout({
  children,
  frontmatter: { title },
  nodes,
}: {
  children: React.ReactNode
  frontmatter: { title?: string }
  nodes: Array<Node>
}) {
  const pathname = usePathname()
  const isHomePage = pathname === '/'
  const tableOfContents = collectSections(nodes)

  return (
    <>
      <div
        className={cn(
          'max-w-2xl min-w-0 flex-auto px-4 lg:max-w-none lg:pr-0 lg:pl-8 xl:px-16',
          isHomePage ? 'pt-8 pb-16' : 'py-16',
        )}
      >
        <article>
          <DocsHeader title={title} />
          <Prose>{children}</Prose>
        </article>
        <PrevNextLinks />
      </div>
      <TableOfContents tableOfContents={tableOfContents} />
    </>
  )
}
