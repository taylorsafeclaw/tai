'use client'

import { Fragment, useCallback, useState } from 'react'
import { Highlight } from 'prism-react-renderer'
import { Check, Copy } from 'lucide-react'

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

  const onCopy = useCallback(() => {
    navigator.clipboard.writeText(code)
    setCopied(true)
    setTimeout(() => setCopied(false), 1500)
  }, [code])

  return (
    <div className="not-prose group relative my-6">
      {/* Copy button */}
      <button
        onClick={onCopy}
        aria-label="Copy code"
        className="absolute right-0 top-0 z-10 flex items-center gap-1.5 border-b-2 border-l-2 border-neutral-800 bg-[#0d0d0d] px-2.5 py-1 font-mono text-[10px] font-bold uppercase tracking-widest text-neutral-600 transition-colors hover:border-lime-400/40 hover:text-lime-400 select-none"
      >
        {copied ? (
          <>
            <Check size={12} strokeWidth={2.5} className="text-lime-400" />
            <span className="text-lime-400">Copied</span>
          </>
        ) : (
          <>
            <Copy size={12} strokeWidth={2.5} />
            <span>{lang !== 'text' ? lang : 'Copy'}</span>
          </>
        )}
      </button>

      <Highlight
        code={code}
        language={lang}
        theme={{ plain: {}, styles: [] }}
      >
        {({ className, style, tokens, getTokenProps }) => (
          <pre
            className={`${className} relative overflow-x-auto border-2 border-neutral-800 border-l-[3px] border-l-lime-400/40 bg-[#111] px-5 py-4 font-mono text-[13px] leading-relaxed`}
            style={style}
          >
            {/* Top glow line — follows left accent */}
            <span
              className="pointer-events-none absolute inset-x-0 top-0 h-px"
              style={{
                background:
                  'linear-gradient(to right, rgba(163,230,53,0.25), transparent 50%)',
              }}
              aria-hidden="true"
            />
            <code>
              {tokens.map((line, lineIndex) => (
                <Fragment key={lineIndex}>
                  {line
                    .filter((token) => !token.empty)
                    .map((token, tokenIndex) => (
                      <span key={tokenIndex} {...getTokenProps({ token })} />
                    ))}
                  {'\n'}
                </Fragment>
              ))}
            </code>
          </pre>
        )}
      </Highlight>
    </div>
  )
}
