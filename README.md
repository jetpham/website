# jetpham.com

<div align="center">
  <img src="src/app/icon0.svg" alt="jetpham.com icon" width="200" height="200">
</div>

Jet Pham's personal website. This website comes with a long story. The domain was originally registered in highschool by my teamate on my robotics team as a joke. The site was originally a filesystem full of memes and random files. Once I was in college, the domain expired and I registered it myself.

The site originally contained a blog. It was made in Next.js plainly with plain colors and no real style. I posted a few blogs about my life but eventually lost motivaiton and didn't like sharing it with other people after having been inspired by so many other cool websites.

I started to become more obsessed with Rust and rewrote my website from being a blog into a static linktree site made in rust via WASM. It was in ASCII style using a modified fork of ratatui and had a fun implementation of Conways Game of Life in the background.

After leaving that website alone, I started to make more web based projects in Next.js. I realized I could properly make this website awesome and still keep the interesting style in the site while making it more performant, responsive, and accessible. This is the state that you see the website in now. Features like the Q+A are inspired directly from my friend Clover's website: ([paperclover.net](https://paperclover.net/)). Go check out her awesome site!

I have some awesome features packed in this site now that represent all the cool things I'm interested in:
- ANSI rendering of my name in css!
- Terminal style text, font, and colors just like BBS
(To be implemented)
- Rust WASM implementation of Conway's Game of Life with Rayon
- Super cool blog filled with stuff about me
- A sick Q+A inspired from ([paperclover.net](https://paperclover.net/))
- Projects page with info about projects I've made
- List of socials and contact info

Let me know if you have any feedback about the site!

## Tech Stack

- [Next.js 15](https://nextjs.org)
- [NextAuth.js v5](https://next-auth.js.org)
- [Prisma](https://prisma.io)
- [Tailwind CSS v4](https://tailwindcss.com)
- [tRPC](https://trpc.io)
- [TypeScript](https://www.typescriptlang.org/)
- [React Query](https://tanstack.com/query)

## Development

### Prerequisites

- Bun
- Docker

### Getting Started

1. Clone the repository
2. Install dependencies:
   ```bash
   bun install
   ```

3. Set up environment variables:
   ```bash
   cp .env.example .env.local
   # Edit .env.local with your configuration
   ```

4. Set up the database:
   ```bash
   bun run db:push
   ```

5. Start the development server:
   ```bash
   bun run dev
   ```

## Project Structure

```
src/
├── app/                 # Next.js App Router pages
│   ├── _components/     # Reusable UI components
│   ├── admin/          # Admin dashboard
│   └── api/            # API routes
├── server/             # Server-side code
│   ├── api/            # tRPC routers
│   └── auth/           # Authentication configuration
├── styles/             # Global styles
└── trpc/               # tRPC client configuration
```
