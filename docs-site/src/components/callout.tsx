import { cn } from '@/lib/utils'

import { Icon } from '@/components/icon'

const styles = {
  note: {
    container:
      'border-2 border-lime-400/30 bg-lime-400/5 dark:bg-lime-400/5',
    title: 'text-lime-600 dark:text-lime-400',
    body: 'text-neutral-700 prose-a:text-lime-600 prose-code:text-lime-600 dark:text-neutral-300 dark:prose-code:text-neutral-300',
  },
  warning: {
    container:
      'border-2 border-amber-400/30 bg-amber-400/5 dark:bg-amber-400/5',
    title: 'text-amber-600 dark:text-amber-400',
    body: 'text-neutral-700 prose-a:text-amber-600 prose-code:text-amber-600 dark:text-neutral-300 dark:prose-code:text-neutral-300',
  },
}

const icons = {
  note: (props: { className?: string }) => <Icon icon="lightbulb" {...props} />,
  warning: (props: { className?: string }) => (
    <Icon icon="warning" color="amber" {...props} />
  ),
}

export function Callout({
  title,
  children,
  type = 'note',
}: {
  title: string
  children: React.ReactNode
  type?: keyof typeof styles
}) {
  const IconComponent = icons[type]

  return (
    <div className={cn('my-8 flex p-6', styles[type].container)}>
      <IconComponent className="h-8 w-8 flex-none" />
      <div className="ml-4 flex-auto">
        <p
          className={cn('not-prose font-mono text-base font-bold', styles[type].title)}
        >
          {title}
        </p>
        <div className={cn('prose mt-2.5', styles[type].body)}>
          {children}
        </div>
      </div>
    </div>
  )
}
