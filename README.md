# Lumen

A fullscreen, console-style launcher for Windows games and apps — built with
Tauri 2 (Rust) and SvelteKit 2 / Svelte 5. Navigate with keyboard or gamepad,
launch anything from one place, track playtime.

## Features

- **Library**: drag & drop `.exe`, `.lnk`, `.url` files or folders; icons are
  extracted automatically from executables. Kinds: `exe`, `file`, `url`,
  `steam`, `epic`, `uwp`, `folder`.
- **Home screen**: rows for Recent, Favourites, Games, Apps and custom
  categories; 10 animated carousel modes (swipe, coverflow, orbit, wheel…).
- **Playtime tracking**: measured for direct `.exe` launches; for Steam/Epic/
  launcher stubs set *Track process* (e.g. `game.exe`) in the entry editor.
- **Themes**: background (dynamic / image / video / gradient), fonts, tiles,
  animation, custom CSS. Saved as JSON in the data folder.
- **Input**: keyboard and any XInput/standard gamepad, with on-screen keyboard
  for search. Global hotkey (default `Ctrl+Shift+L`) brings Lumen back.

### Controls

| Action        | Keyboard          | Gamepad   |
|---------------|-------------------|-----------|
| Navigate      | Arrows            | D-pad / left stick |
| Launch        | Enter / Space     | A         |
| Back          | Esc / Backspace   | B         |
| Search        | `/`               | X         |
| Options       | Tab / `O`         | Start     |
| Quick menu    | `M`               | Select    |

## Development

Requirements: Node 20+, Rust stable, WebView2 (preinstalled on Windows 10/11).

```bash
npm install
npm run tauri dev      # run the app
npm run check          # svelte-check (types + a11y)
npm run tauri build    # installer in src-tauri/target/release/bundle
```

Rust unit tests: `cd src-tauri && cargo test --lib`.

## Releases

Releases are built automatically by GitHub Actions
([`.github/workflows/release.yml`](.github/workflows/release.yml)).
To publish a new version, push a tag:

```bash
git tag v0.2.0
git push origin v0.2.0
```

The workflow type-checks, runs tests, builds the Windows installers (`.exe`
NSIS setup and `.msi`) and attaches them to a GitHub Release named after the
tag. The tag's version is used for the build, so you don't need to edit
`tauri.conf.json` by hand. Tags with a suffix (e.g. `v0.2.0-beta.1`) are
published as pre-releases. You can also start a release by hand from the
**Actions** tab ("Run workflow"); it uses the version in `tauri.conf.json`.

Every push to `main` and every pull request also runs the CI workflow
(type check, frontend build, Rust check and tests).

## Data

Stored in `%APPDATA%\com.lumen.launcher\`:

- `library.json` — entries and stats
- `settings.json` — app settings
- `themes\*.json` — themes
- `media\<entry-id>\` — extracted icons

Writes are atomic (temp file + rename). If a file fails to parse it is copied
to `<name>.corrupt-<timestamp>` before being replaced, so data is never
silently lost.

## Project layout

```
src/                     SvelteKit frontend (SPA, adapter-static)
  lib/api.ts             typed wrappers around Tauri commands
  lib/stores.ts          app state + launch helper
  lib/input.ts           keyboard/gamepad → actions
  lib/anim/              carousel animation modes + spring physics
  lib/components/        UI components
  routes/                home, library, editor/[id], settings, theme
src-tauri/src/
  commands.rs            Tauri commands (library, launch, tracking)
  storage.rs             JSON persistence
  models.rs              data model
  lib.rs                 app setup: plugins, tray, global hotkey
```
