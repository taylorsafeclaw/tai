import Link from 'next/link'

export default function NotFound() {
  return (
    <div className="max-w-2xl min-w-0 flex-auto px-4 py-16 lg:max-w-none lg:pr-0 lg:pl-8 xl:px-16">
      <div className="flex h-full flex-col items-center justify-center text-center">
        <p className="font-mono text-sm font-bold text-lime-400">
          404
        </p>
        <h1 className="mt-3 font-mono text-3xl font-bold tracking-tight text-white">
          Page not found
        </h1>
        <p className="mt-2 text-sm text-neutral-500">
          Sorry, we couldn&apos;t find the page you&apos;re looking for.
        </p>
        <Link
          href="/"
          className="mt-8 font-mono text-sm font-medium text-lime-400 hover:text-lime-300"
        >
          Go back home
        </Link>
      </div>
    </div>
  )
}
