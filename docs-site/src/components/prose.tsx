import { cn } from '@/lib/utils'

export function Prose<T extends React.ElementType = 'div'>({
  as,
  className,
  ...props
}: React.ComponentPropsWithoutRef<T> & {
  as?: T
}) {
  const Component = as ?? 'div'

  return (
    <Component
      className={cn(
        className,
        'prose max-w-none dark:text-neutral-400 dark:prose-invert',
        // headings
        'prose-headings:scroll-mt-28 prose-headings:font-mono prose-headings:font-bold lg:prose-headings:scroll-mt-34',
        // lead
        'prose-lead:text-neutral-500 dark:prose-lead:text-neutral-400',
        // links
        'prose-a:font-semibold dark:prose-a:text-lime-400',
        // link underline
        'dark:[--tw-prose-background:var(--color-neutral-900)] prose-a:no-underline prose-a:shadow-[inset_0_-2px_0_0_var(--tw-prose-background,#fff),inset_0_calc(-1*(var(--tw-prose-underline-size,4px)+2px))_0_0_var(--tw-prose-underline,var(--color-lime-400))] prose-a:hover:[--tw-prose-underline-size:6px] dark:prose-a:shadow-[inset_0_calc(-1*var(--tw-prose-underline-size,2px))_0_0_var(--tw-prose-underline,var(--color-lime-800))] dark:prose-a:hover:[--tw-prose-underline-size:6px]',
        // pre (fallback for non-Fence pre blocks)
        'prose-pre:bg-[#111] prose-pre:border-2 prose-pre:border-neutral-800 prose-pre:border-l-[3px] prose-pre:border-l-lime-400/40 dark:prose-pre:bg-[#111] dark:prose-pre:border-neutral-800 dark:prose-pre:border-l-lime-400/40',
        // hr
        'dark:prose-hr:border-neutral-800',
      )}
      {...props}
    />
  )
}
