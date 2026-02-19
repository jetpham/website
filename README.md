# jetpham.com

<div align="center">
  <img src="src/app/icon0.svg" alt="jetpham.com icon" width="200" height="200">
</div>

Jet Pham's personal website. This website comes with a long story. The domain was originally registered in highschool by my teamate on my robotics team as a joke. The site was originally a filesystem full of memes and random files. Once I was in college, the domain expired and I registered it myself.

The site originally contained a blog. It was made in Next.js plainly with plain colors and no real style. I posted a few blogs about my life but eventually lost motivaiton and didn't like sharing it with other people after having been inspired by so many other cool websites.

I started to become more obsessed with Rust and rewrote my website from being a blog into a static linktree site made in rust via WASM. It was in ASCII style using a modified fork of ratzilla and had a fun implementation of Conways Game of Life in the background.

After leaving that website alone, I started to make more web based projects in Next.js. I realized I could properly make this website awesome and still keep the interesting style in the site while making it more performant, responsive, and accessible. This is the state that you see the website in now.

I have some awesome features packed in this site now that represent all the cool things I'm interested in:

- ANSI rendering of my name in CSS!
- Terminal style text, font, and colors just like BBS
- Rust WASM implementation of Conway's Game of Life running in the background
- List of socials and contact info

Let me know if you have any feedback about the site!

## Tech Stack

- [Next.js 16](https://nextjs.org) with Turbo mode
- [Tailwind CSS v4](https://tailwindcss.com)
- [TypeScript](https://www.typescriptlang.org/)
- [React 19](https://react.dev/)
- Rust + WebAssembly (for Conway's Game of Life)
- [Bun](https://bun.sh) (package manager)

## Development

### Prerequisites

- Bun
- Rust (for building the Conway's Game of Life WASM module)
- wasm-pack (or use the install script)

### Getting Started

1. Clone the repository

2. Build the Rust WASM module:

   ```bash
   bun run build:wasm
   ```

   Or use the install script:

   ```bash
   ./install.sh
   ```

3. Install dependencies:

   ```bash
   bun install
   ```

4. Start the development server:

   ```bash
   bun run dev
   ```

The site will be available at `http://localhost:3000`.

## Project Structure

```
src/ - Next.js app router pages
cgol/ - Rust WASM module for Conway's Game of Life
public/ - Static assets
```
