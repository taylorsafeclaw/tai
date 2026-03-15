import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
  integrations: [
    starlight({
      title: 'tstack',
      customCss: ['./src/styles/custom.css'],
      components: {
        Hero: './src/components/Hero.astro',
      },
      description: 'Personal dev framework for Claude Code',
      social: [
        { icon: 'github', label: 'GitHub', href: 'https://github.com/tstack-framework/tstack' },
      ],
      sidebar: [
        {
          label: 'Getting Started',
          items: [
            { label: 'Install', slug: 'getting-started/install' },
            { label: 'Quickstart', slug: 'getting-started/quickstart' },
          ],
        },
        {
          label: 'Tiers',
          items: [
            { label: 'Overview', slug: 'tiers/overview' },
            { label: 'Missions', slug: 'tiers/missions' },
          ],
        },
        {
          label: 'Reference',
          items: [
            { label: 'Commands', slug: 'reference/commands' },
            { label: 'Agents', slug: 'reference/agents' },
            { label: 'Skills', slug: 'reference/skills' },
            { label: 'Hooks', slug: 'reference/hooks' },
            { label: 'CLI', slug: 'reference/cli' },
          ],
        },
        {
          label: 'Guides',
          items: [
            { label: 'Quality Pipeline', slug: 'guides/quality-pipeline' },
            { label: 'Agent Teams', slug: 'guides/agent-teams' },
            { label: 'Extensions', slug: 'guides/extensions' },
            { label: 'Templates', slug: 'guides/templates' },
          ],
        },
      ],
    }),
  ],
});
