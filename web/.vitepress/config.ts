import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Monadclaw',
  description: 'A modular AI agent framework',
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/guide/' },
    ],
    sidebar: [
      {
        text: 'Guide',
        items: [
          { text: 'Introduction', link: '/guide/' },
          { text: 'Getting Started', link: '/guide/getting-started' },
        ],
      },
    ],
    socialLinks: [
      { icon: 'github', link: 'https://github.com/monadforge/monadclaw' },
    ],
  },
})
