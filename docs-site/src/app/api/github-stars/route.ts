import { NextResponse } from 'next/server'

const CACHE_DURATION = 3600 // 1 hour in seconds

export async function GET() {
  const res = await fetch(
    'https://api.github.com/repos/taylorsafeclaw/tstack',
    { next: { revalidate: CACHE_DURATION } } as RequestInit,
  )

  if (!res.ok) {
    return NextResponse.json({ stars: null }, { status: 502 })
  }

  const data = await res.json()
  return NextResponse.json({ stars: data.stargazers_count ?? null })
}
