'use client'

import { Button } from '@/components/Button'
import { BlurFade } from '@/components/ui/blur-fade'
import { BorderBeam } from '@/components/ui/border-beam'
import { FlickeringGrid } from '@/components/ui/flickering-grid'
import { AnimatedSpan, Terminal } from '@/components/ui/terminal'

const WORKFLOW_LINES = [
  { text: '$ tstack install', delay: 0, color: 'text-white' },
  { text: '  ✓ plugin registered', delay: 400, color: 'text-lime-400' },
  { text: '  ✓ 24 commands loaded', delay: 600, color: 'text-lime-400' },
  { text: '  ✓ 6 agents ready', delay: 800, color: 'text-lime-400' },
  { text: '', delay: 1000, color: '' },
  { text: '$ /task "add dark mode toggle"', delay: 1200, color: 'text-white' },
  {
    text: '  → sonnet analyzing scope...',
    delay: 1600,
    color: 'text-neutral-500',
  },
  { text: '  ✓ 2 files changed', delay: 2200, color: 'text-lime-400' },
  { text: '  ✓ lint pass', delay: 2600, color: 'text-lime-400' },
  { text: '  ✓ build pass', delay: 2800, color: 'text-lime-400' },
  { text: '  ✓ test pass', delay: 3000, color: 'text-lime-400' },
  {
    text: '  ✓ committed: feat: add dark mode toggle',
    delay: 3400,
    color: 'text-lime-400',
  },
  { text: '', delay: 3600, color: '' },
  { text: '$ /feature "auth system"', delay: 4000, color: 'text-white' },
  { text: '  → opus planning...', delay: 4400, color: 'text-neutral-500' },
  {
    text: '  → dispatching agent team:',
    delay: 4800,
    color: 'text-neutral-500',
  },
  { text: '    schema ▓▓▓▓▓▓▓▓▓▓ done', delay: 5400, color: 'text-lime-400' },
  { text: '    backend ▓▓▓▓▓▓▓▓▓ done', delay: 6000, color: 'text-lime-400' },
  { text: '    frontend ▓▓▓▓▓▓▓▓ done', delay: 6600, color: 'text-lime-400' },
  { text: '    tests ▓▓▓▓▓▓▓▓▓▓ done', delay: 7200, color: 'text-lime-400' },
  { text: '  ✓ PR #42 created', delay: 7800, color: 'text-lime-400' },
]

function TerminalWorkflow() {
  return (
    <div className="relative">
      <div className="absolute -inset-px bg-lime-400/5" />
      <div className="absolute -inset-[2px] border-2 border-lime-400/20" />
      <Terminal
        className="relative h-[360px] w-full max-w-none border-2 border-neutral-800 bg-[#0a0a0a]"
        sequence
        startOnView={false}
      >
        {WORKFLOW_LINES.map((line, i) => (
          <AnimatedSpan
            key={i}
            className={`${line.color || 'text-transparent'} leading-relaxed`}
          >
            {line.text || '\u00A0'}
          </AnimatedSpan>
        ))}
        <AnimatedSpan>
          <span className="inline-block h-4 w-2 animate-pulse bg-lime-400" />
        </AnimatedSpan>
      </Terminal>
    </div>
  )
}

export function Hero() {
  return (
    <div className="relative overflow-hidden bg-[#0a0a0a] dark:-mt-19 dark:-mb-32 dark:pt-19 dark:pb-32">
      {/* Background */}
      <div
        className="pointer-events-none absolute inset-0 z-0"
        style={{
          maskImage:
            'linear-gradient(to bottom, black 0%, black 55%, transparent 100%)',
          WebkitMaskImage:
            'linear-gradient(to bottom, black 0%, black 55%, transparent 100%)',
        }}
      >
        <FlickeringGrid
          color="rgb(163, 230, 53)"
          flickerChance={0.15}
          maxOpacity={0.08}
          squareSize={3}
          gridGap={8}
          className="h-full w-full"
        />
      </div>

      {/* Bottom fade overlay for smooth transition to docs content */}
      <div
        className="pointer-events-none absolute right-0 bottom-0 left-0 z-[1] h-40"
        style={{
          background:
            'linear-gradient(to bottom, transparent 0%, rgb(10, 10, 10) 100%)',
        }}
        aria-hidden
      />

      <div className="relative z-10 py-16 sm:px-2 lg:relative lg:px-0 lg:py-24">
        <div className="mx-auto grid max-w-2xl grid-cols-1 items-center gap-x-12 gap-y-16 px-4 lg:max-w-8xl lg:grid-cols-2 lg:px-8 xl:gap-x-16 xl:px-12">
          {/* Left column: text */}
          <div className="relative z-10 md:text-center lg:text-left">
            <div className="relative">
              <BlurFade delay={0} direction="up">
                <p className="font-display text-5xl font-bold tracking-tighter text-white uppercase sm:text-6xl lg:text-7xl">
                  Stop prompting.
                  <br />
                  <span className="text-lime-400">Start shipping.</span>
                </p>
              </BlurFade>
              <BlurFade delay={0.1} direction="up">
                <p className="mt-5 font-sans text-lg tracking-tight text-neutral-400 sm:text-xl">
                  Three tiers. Quality gates. Agent teams.
                  <br />A structured dev workflow for Claude Code.
                </p>
              </BlurFade>
              <BlurFade delay={0.2} direction="up">
                <div className="mt-10 flex gap-4 md:justify-center lg:justify-start">
                  <div className="relative">
                    <BorderBeam
                      size={80}
                      duration={4}
                      colorFrom="#a3e635"
                      colorTo="#65a30d"
                      borderWidth={2}
                      pathRound={0}
                    />
                    <Button href="/">
                      <span className="font-mono text-sm font-bold tracking-wider uppercase">
                        Get started →
                      </span>
                    </Button>
                  </div>
                  <Button
                    href="https://github.com/taylorsafeclaw/tstack"
                    variant="secondary"
                  >
                    <span className="font-mono text-sm font-bold tracking-wider uppercase">
                      GitHub ↗
                    </span>
                  </Button>
                </div>
              </BlurFade>
            </div>
          </div>

          {/* Right column: terminal */}
          <BlurFade
            delay={0.15}
            direction="left"
            className="relative lg:static xl:pl-10"
          >
            <TerminalWorkflow />
          </BlurFade>
        </div>
      </div>
    </div>
  )
}
