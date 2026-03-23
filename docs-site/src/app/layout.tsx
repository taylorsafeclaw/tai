import { type Metadata } from 'next'
import { Geist, JetBrains_Mono, Syne } from 'next/font/google'
import localFont from 'next/font/local'
import clsx from 'clsx'

import { Layout } from '@/components/Layout'

import '@/styles/tailwind.css'
import { cn } from '@/lib/utils'

const geist = Geist({ subsets: ['latin'], variable: '--font-sans' })

const syne = Syne({
  subsets: ['latin'],
  display: 'swap',
  variable: '--font-display',
})

const jetbrainsMono = JetBrains_Mono({
  subsets: ['latin'],
  display: 'swap',
  variable: '--font-mono',
})

// Use local version of Lexend so that we can use OpenType features
const lexend = localFont({
  src: '../fonts/lexend.woff2',
  display: 'swap',
  variable: '--font-lexend',
})

export const metadata: Metadata = {
  title: {
    template: '%s - tstack',
    default: 'tstack — Stop prompting. Start shipping.',
  },
  description:
    'A structured, opinionated dev workflow for Claude Code. Three tiers, quality gates, agent teams.',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html
      lang="en"
      className={cn(
        clsx(
          'dark h-full antialiased',
          geist.variable,
          syne.variable,
          lexend.variable,
          jetbrainsMono.variable,
        ),
        'font-sans',
      )}
    >
      <body className="flex min-h-full bg-white dark:bg-[#0a0a0a]">
        <Layout>{children}</Layout>
      </body>
    </html>
  )
}
