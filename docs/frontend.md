# 🎨 Front-End Architecture

The front-end follows a **framework-free, component-oriented CSS structure** built around reusable primitives.

- **Design tokens**: variables.css (colors, spacing, typography, shadows)
- **Themes**: themes.css using `[data-theme]` attributes
- **Components**: reusable styles in `components/`
- **Page-specific styles**: `pages/`
- **Base & typography layers**: global consistency

## 🎨 Theme System

The application supports multiple visual themes:

- Static assets referenced via fingerprinted filenames for cache safety
- Controlled via `data-theme` on `<html>`
- Persisted using `localStorage`
- Updated via lightweight JavaScript (`javascript.js`)
- Fully CSS-variable driven

Available themes:

- Night *(default)*
- Sunset
- Ocean
- Forest