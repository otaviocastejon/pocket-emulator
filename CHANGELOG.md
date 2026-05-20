# Changelog

All notable changes to **PocketEmulator** are documented here.

## [0.1.6] — 2026-05-20

### Added

- **Home page** — Welcome screen with logo, intro, “continue last played,” quick how-to, open-source link, and signature.
- **Library filters** — Sidebar entries **All ROMs**, **Game Boy** (`.gb`), and **Game Boy Color** (`.gbc`) open filtered ROM lists.
- **Game Boy control diagram** on Settings — Visual D-pad and face buttons showing your current keybinds.
- **Platform badges** — Distinct styling for GB vs GBC tags in the library list.
- Shared ROM display helpers (`romDisplay.ts`) for thumbnails, titles, and “last played” text.

### Changed

- **My Games** is now the home tab; the full ROM grid lives under **Library** filters.
- **UI polish** — Accent/primary gradient panels across the launcher; layout fixes so Saves, Settings, and Get ROMs scroll correctly inside the main area.
- **Settings** — “Display filter” labels (**Sharp** / **Smooth**); removed unsupported Pocket/Light options.
- **Esc in-game** — Saves and closes only the gameplay window (no second app instance, no forced focus back to the launcher).

### Fixed

- **Game Boy Color compatibility** — CPU access to VRAM, OAM, and CGB palettes blocked during LCD mode 3 (draw), matching hardware behavior.
- **VRAM DMA (HDMA / GDMA)** — Latched VRAM bank for transfers; dedicated DMA write path; GDMA stalls the CPU for ~8 M-cycles per 16-byte block.
- Improves stability for demanding GBC titles (e.g. games using heavy VRAM DMA).

---

## [0.1.5] and earlier

See [GitHub Releases](https://github.com/otaviocastejon/pocket-emulator/releases) for prior builds.
