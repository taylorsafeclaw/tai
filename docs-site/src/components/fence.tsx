'use client'

import { Fragment, useCallback, useState } from 'react'
import { Highlight } from 'prism-react-renderer'
import { Check, Copy, Terminal } from 'lucide-react'

const SHELL_LANGS = new Set(['shell', 'bash', 'sh', 'zsh', 'terminal'])

export function Fence({
  children,
  language,
}: {
  children: string
  language: string
}) {
  const lang = language ?? 'text'
  const code = children.trimEnd()
  const [copied, setCopied] = useState(false)
  const isShell = SHELL_LANGS.has(lang)

  const onCopy = useCallback(() => {
    navigator.clipboard.writeText(code)
    setCopied(true)
    setTimeout(() => setCopied(false), 1500)
  }, [code])

  return (
    <div className="not-prose group relative my-6 border-2 border-neutral-800 border-l-[3px] border-l-lime-400/60">
      {/* Top glow line — follows left accent */}
      <span
        className="pointer-events-none absolute inset-x-0 top-0 z-10 h-px"
        style={{
          background:
            'linear-gradient(to right, rgba(163,230,53,0.3), transparent 50%)',
        }}
        aria-hidden="true"
      />

      {/* Terminal chrome bar */}
      <div className="flex items-center justify-between border-b border-neutral-800/80 bg-[#0a0a0a] px-4 py-2">
        <div className="flex items-center gap-2">
          {isShell && (
            <Terminal
              size={12}
              strokeWidth={2}
              className="text-lime-400/50"
            />
          )}
          <span className="font-mono text-[10px] font-bold uppercase tracking-widest text-neutral-500">
            {lang !== 'text' ? lang : 'code'}
          </span>
        </div>
        <button
          onClick={onCopy}
          aria-label="Copy code"
          className="flex items-center gap-1.5 font-mono text-[10px] font-bold uppercase tracking-widest text-neutral-600 transition-colors hover:text-lime-400 select-none"
        >
          {copied ? (
            <>
              <Check size={11} strokeWidth={2.5} className="text-lime-400" />
              <span className="text-lime-400">Copied</span>
            </>
          ) : (
            <>
              <Copy size={11} strokeWidth={2.5} />
              <span>Copy</span>
            </>
          )}
        </button>
      </div>

      {/* Code body */}
      <Highlight
        code={code}
        language={lang}
        theme={{ plain: {}, styles: [] }}
      >
        {({ className, style, tokens, getTokenProps }) => (
          <pre
            className={`${className} relative overflow-x-auto bg-[#111] px-5 py-4 font-mono text-[13px] leading-relaxed`}
            style={style}
          >
            <code>
              {tokens.map((line, lineIndex) => {
                const hasContent = line.some((t) => !t.empty)
                return (
                  <Fragment key={lineIndex}>
                    {isShell && hasContent && (
                      <span className="mr-3 text-lime-400/30 select-none">
                        $
                      </span>
                    )}
                    {line
                      .filter((token) => !token.empty)
                      .map((token, tokenIndex) => (
                        <span
                          key={tokenIndex}
                          {...getTokenProps({ token })}
                        />
                      ))}
                    {'\n'}
                  </Fragment>
                )
              })}
            </code>
          </pre>
        )}
      </Highlight>
    </div>
  )
}
