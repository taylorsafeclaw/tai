'use client'

import { useEffect, useRef, useState } from 'react'
import { Star } from 'lucide-react'
import { Button } from '@/components/button'
import { BlurFade } from '@/components/ui/blur-fade'
import { BorderBeam } from '@/components/ui/border-beam'
import { FlickeringGrid } from '@/components/ui/flickering-grid'

// --- Constants ---
const SPINNER = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏']

// --- Types ---
interface TermLine {
  text: string
  className: string
}

// --- Helpers ---
function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

function charDelay(char: string, base: number): number {
  if (char === ' ') return base * 1.6
  if (char === '"' || char === "'") return base * 2.2
  if (char === '/') return base * 0.7
  return base + Math.random() * base * 0.5
}

function renderBar(progress: number, width = 18): string {
  const filled = progress * width
  const fullBlocks = Math.floor(filled)
  const partial = filled - fullBlocks
  const blocks = [' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉', '█']

  let bar = ''
  for (let i = 0; i < width; i++) {
    if (i < fullBlocks) {
      bar += '█'
    } else if (i === fullBlocks && partial > 0.05) {
      bar += blocks[Math.min(Math.round(partial * 8), 8)]
    } else {
      bar += '░'
    }
  }
  return bar
}

// --- Spinner Sub-component ---
function SpinnerLine({ text }: { text: string }) {
  const [frame, setFrame] = useState(0)
  useEffect(() => {
    const id = setInterval(() => setFrame((f) => (f + 1) % SPINNER.length), 80)
    return () => clearInterval(id)
  }, [])
  return (
    <div className="text-neutral-500">
      {'  '}
      <span className="text-lime-400/60">{SPINNER[frame]}</span> {text}
    </div>
  )
}

// --- Animated Terminal ---
function AnimatedTerminal() {
  const [lines, setLines] = useState<TermLine[]>([])
  const [typing, setTyping] = useState<TermLine | null>(null)
  const [progressLine, setProgressLine] = useState<TermLine | null>(null)
  const [spinnerText, setSpinnerText] = useState<string | null>(null)
  const [status, setStatus] = useState({ phase: 'READY', detail: '' })
  const [bodyOpacity, setBodyOpacity] = useState(1)
  const cancelRef = useRef(false)
  const bodyRef = useRef<HTMLDivElement>(null)

  // Auto-scroll
  useEffect(() => {
    const el = bodyRef.current
    if (el) el.scrollTop = el.scrollHeight
  }, [lines, typing, progressLine, spinnerText])

  useEffect(() => {
    cancelRef.current = false
    const dead = () => cancelRef.current

    const emit = (text: string, className = '') => {
      if (!dead()) setLines((p) => [...p, { text, className }])
    }

    const typeCmd = async (text: string, speed = 60) => {
      for (let i = 0; i <= text.length; i++) {
        if (dead()) return
        setTyping({ text: text.slice(0, i), className: 'text-white' })
        await sleep(charDelay(text[i] || '', speed))
      }
      if (dead()) return
      setTyping(null)
      emit(text, 'text-white')
    }

    const out = async (text: string, cls: string, ms = 250) => {
      if (dead()) return
      await sleep(ms)
      emit(text, cls)
    }

    const spin = async (text: string, duration: number) => {
      if (dead()) return
      setSpinnerText(text)
      await sleep(duration)
      if (dead()) return
      setSpinnerText(null)
    }

    const bar = async (label: string) => {
      const pad = label.padEnd(10)
      const steps = 30
      for (let i = 0; i <= steps; i++) {
        if (dead()) return
        const progress = i / steps
        const pct = Math.round(progress * 100)
          .toString()
          .padStart(3)
        const barStr = renderBar(progress, 18)
        setProgressLine({
          text: `  ${pad} ${barStr}  ${pct}%`,
          className: i < steps ? 'text-neutral-600' : 'text-lime-400',
        })
        await sleep(35 + Math.random() * 20)
      }
      if (dead()) return
      setProgressLine(null)
      emit(`  ${pad} ${'█'.repeat(18)}  done`, 'text-lime-400')
    }

    async function loop() {
      while (!dead()) {
        // Reset state
        setLines([])
        setTyping(null)
        setProgressLine(null)
        setSpinnerText(null)
        setStatus({ phase: 'READY', detail: '' })

        // Fade in
        setBodyOpacity(1)
        await sleep(800)

        // ─── Act 1: Install ───
        setStatus({ phase: 'INSTALL', detail: 'initializing' })
        await typeCmd('$ tstack install', 55)
        await spin('registering plugin\u2026', 800)
        await out('  \u2713 plugin registered', 'text-lime-400', 200)
        await out(
          '  \u2713 24 commands \u00B7 6 agents \u00B7 18 skills',
          'text-lime-400',
          350,
        )
        await sleep(900)
        emit('', '')

        // ─── Act 2: Task (sonnet) ───
        setStatus({ phase: 'TASK', detail: 'sonnet' })
        await typeCmd('$ /task "add dark mode toggle"', 48)
        await spin('sonnet \u00B7 analyzing scope\u2026', 1400)
        await out('  \u2713 2 files changed', 'text-lime-400', 200)
        await out(
          '  \u2713 lint \u00B7 build \u00B7 test passing',
          'text-lime-400',
          400,
        )
        await out(
          '  \u2713 commit: feat(ui): dark mode toggle',
          'text-lime-400',
          400,
        )
        await sleep(900)
        emit('', '')

        // ─── Act 3: Feature (agent team) ───
        setStatus({ phase: 'FEATURE', detail: 'opus \u2192 4 agents' })
        await typeCmd('$ /feature "user authentication"', 40)
        await spin('opus \u00B7 planning architecture\u2026', 1600)
        await out(
          '  \u2713 plan ready \u00B7 dispatching agents',
          'text-lime-400',
          200,
        )
        await sleep(300)
        emit('', '')

        await bar('schema')
        await bar('backend')
        await bar('frontend')
        await bar('tests')

        await sleep(400)
        await out(
          '  \u2713 PR #42 created \u2192 ready for review',
          'text-lime-400',
          500,
        )
        await sleep(900)
        emit('', '')

        // ─── Act 4: Ship ───
        setStatus({ phase: 'SHIP', detail: 'main' })
        await typeCmd('$ /ship', 70)
        await spin('running quality gates\u2026', 1000)
        await out('  \u2713 quality gates passed', 'text-lime-400', 200)
        await out('  \u2713 pushed to main', 'text-lime-400', 400)
        await out('  \u2713 deployed', 'text-lime-400', 400)
        setStatus({ phase: 'DEPLOYED', detail: '\u2713' })

        // Hold, then seamless transition
        await sleep(6000)
        if (dead()) return

        // Fade out
        setBodyOpacity(0)
        await sleep(700)
        if (dead()) return
      }
    }

    loop()
    return () => {
      cancelRef.current = true
    }
  }, [])

  return (
    <div className="relative">
      {/* Outer glow */}
      <div className="absolute -inset-px bg-lime-400/[0.04]" />
      <div className="absolute -inset-[2px] border-2 border-lime-400/20" />
      <div
        className="pointer-events-none absolute -inset-6 opacity-40"
        style={{
          background:
            'radial-gradient(ellipse at 50% 0%, rgba(163,230,53,0.12) 0%, transparent 65%)',
        }}
      />

      {/* Terminal */}
      <div className="relative overflow-hidden border-2 border-neutral-800 bg-[#0a0a0a]">
        {/* TUI Title bar */}
        <div className="flex items-center justify-between border-b border-neutral-800 px-4 py-2">
          <span className="font-mono text-[11px] font-bold tracking-widest text-lime-400 uppercase select-none">
            tstack
          </span>
          <span className="font-mono text-[10px] tracking-wider text-neutral-600 select-none">
            ~/myproject
          </span>
        </div>

        {/* Body */}
        <div
          ref={bodyRef}
          className="h-[340px] overflow-hidden px-5 py-4 font-mono text-[13px] leading-[1.85]"
          style={{
            opacity: bodyOpacity,
            transition: 'opacity 600ms ease-in-out',
          }}
        >
          {lines.map((line, i) => (
            <div key={i} className={line.className || 'h-[1.85em]'}>
              {line.text || '\u00A0'}
            </div>
          ))}

          {/* Active progress bar */}
          {progressLine && (
            <div className={progressLine.className}>{progressLine.text}</div>
          )}

          {/* Spinner */}
          {spinnerText && <SpinnerLine text={spinnerText} />}

          {/* Typing line with cursor */}
          {typing ? (
            <div className={typing.className}>
              {typing.text}
              <span className="ml-px inline-block h-[1.05em] w-[0.55em] translate-y-[2px] animate-pulse bg-lime-400" />
            </div>
          ) : (
            !progressLine &&
            !spinnerText &&
            bodyOpacity === 1 && (
              <div>
                <span className="inline-block h-[1.05em] w-[0.55em] translate-y-[2px] animate-pulse bg-lime-400" />
              </div>
            )
          )}
        </div>

        {/* Status bar */}
        <div className="flex items-center justify-between border-t border-neutral-800 px-4 py-1.5">
          <div className="flex items-center gap-2">
            <span
              className={`inline-block h-1.5 w-1.5 ${
                status.phase === 'DEPLOYED'
                  ? 'bg-lime-400'
                  : status.phase === 'READY'
                    ? 'bg-neutral-700'
                    : 'bg-lime-400 animate-pulse'
              }`}
            />
            <span className="font-mono text-[10px] font-bold tracking-widest text-neutral-500 uppercase select-none">
              {status.phase}
            </span>
          </div>
          {status.detail && (
            <span className="font-mono text-[10px] tracking-wider text-neutral-600 select-none">
              {status.detail}
            </span>
          )}
        </div>

        {/* Scanline texture */}
        <div
          className="pointer-events-none absolute inset-0 mix-blend-overlay"
          style={{
            opacity: 0.02,
            backgroundImage:
              'repeating-linear-gradient(0deg, transparent, transparent 2px, rgba(255,255,255,0.06) 2px, rgba(255,255,255,0.06) 4px)',
          }}
        />

        {/* Top vignette */}
        <div
          className="pointer-events-none absolute inset-x-0 top-0 h-20"
          style={{
            background:
              'linear-gradient(to bottom, rgba(163,230,53,0.02) 0%, transparent 100%)',
          }}
        />
      </div>
    </div>
  )
}

// --- GitHub Stars ---
function GitHubStars() {
  const [stars, setStars] = useState<number | null>(null)

  useEffect(() => {
    fetch('/api/github-stars')
      .then((res) => res.json())
      .then((data) => {
        if (typeof data.stars === 'number') {
          setStars(data.stars)
        }
      })
      .catch(() => {})
  }, [])

  return (
    <div className="mt-6 flex items-center gap-2 md:justify-center lg:justify-start">
      <Star className="h-4 w-4 fill-lime-400 text-lime-400" />
      <span className="font-mono text-sm tracking-wider text-neutral-400">
        {stars !== null ? `${stars.toLocaleString()} stars on GitHub` : 'Star on GitHub'}
      </span>
    </div>
  )
}

// --- Hero ---
export function Hero() {
  return (
    <div className="relative overflow-hidden bg-[#0a0a0a] dark:-mt-19 dark:pt-19">
      {/* Flickering grid background */}
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

      {/* Bottom fade to content */}
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
          {/* Left: copy */}
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
                  <div className="relative flex">
                    <BorderBeam
                      size={80}
                      duration={4}
                      colorFrom="#a3e635"
                      colorTo="#65a30d"
                      borderWidth={2}
                      pathRound={0}
                    />
                    <Button href="/docs/quickstart">
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
              <BlurFade delay={0.3} direction="up">
                <GitHubStars />
              </BlurFade>
            </div>
          </div>

          {/* Right: terminal */}
          <BlurFade
            delay={0.15}
            direction="left"
            className="relative lg:static xl:pl-10"
          >
            <AnimatedTerminal />
          </BlurFade>
        </div>
      </div>
    </div>
  )
}
