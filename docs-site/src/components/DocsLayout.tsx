'use client'

import { usePathname } from 'next/navigation'
import { type Node } from '@markdoc/markdoc'
import clsx from 'clsx'

import { DocsHeader } from '@/components/DocsHeader'
import { PrevNextLinks } from '@/components/PrevNextLinks'
import { Prose } from '@/components/Prose'
import { TableOfContents } from '@/components/TableOfContents'
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
  let pathname = usePathname()
  let isHomePage = pathname === '/'
  let tableOfContents = collectSections(nodes)

  return (
    <>
      <div
        className={clsx(
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
