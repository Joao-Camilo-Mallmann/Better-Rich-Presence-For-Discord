# Rules and Guidelines for Better Rich Presence for Discord Agent

## Styling & Design
- Always adhere to the high-fidelity Discord marketing and client UI design guidelines specified in `DESIGN.md`.
- Use curated, rich color palettes: Blurple (#5865f2), Electric Green (#35ed7e), Vibrant Magenta (#ec48bd), and deep dark indigo/black surfaces.
- Use `Space Grotesk` as the display font for headlines and `Inter` for body content.
- Ensure all interactive elements have responsive layouts, hover effects, and micro-animations.

## Components and Structure
- Maintain clean, modular React components using Tailwind CSS (v4) with explicit @theme utility variables.
- Keep UI components separated into `src/components` and page structures in `src/pages`.

## State and Themes
- Support interactive profile customizability mimicking Discord's native profile themes, banner gradients, and status overlays.
- Preserve the user's settings state cleanly.
