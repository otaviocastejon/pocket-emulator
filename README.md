# PocketEmulator

## About

**PocketEmulator** is a **Game Boy (DMG) and Game Boy Color (GBC)** emulator. You get a **desktop launcher** to manage your ROM library, profiles, and save files, plus a **native gameplay window** with scaling, filters, and keyboard controls. The CPU and PPU are emulated in Rust; **audio is not implemented yet** (APU stubbed).

This project **does not** emulate Game Boy Advance.

---

## Tech stack

| Layer | Details |
|--------|---------|
| **Core emulator** | Rust — CPU (SM83), PPU, cartridges (MBC0/1/3/5), timers, serial (experimental link). |
| **Gameplay window** | `pixels` + `winit` — framebuffer, input, HUD. |
| **Launcher shell** | **Tauri 2** — desktop WebView hosting the UI. |
| **Launcher UI** | **React**, **TypeScript**, **Vite**. |
| **IPC** | Tauri commands between the UI and Rust (ROM library, settings, saves). |
| **Packaging** | Tauri bundler — `.app` on macOS, NSIS / `.exe` on Windows. |

---

## Quick start (downloads)

Prebuilt builds are attached to **[GitHub Releases](https://github.com/otaviocastejon/pocket-emulator/releases)** for **macOS** and **Windows**. Pick the latest version and download the artifact for your OS.

### macOS

1. Download **`PocketEmulator-macos.zip`** from the release and unzip it so **`PocketEmulator.app`** exists (for example in **`~/Downloads`**).
2. Follow **[Run the downloaded app on macOS](#run-the-downloaded-app-on-macos)** below — you usually need **`xattr`** once because the app is not signed/notarized.
3. Later launches: double‑click **`PocketEmulator.app`** in Finder. Optional CLI: `open ~/Downloads/PocketEmulator.app` or run **`PocketEmulator.app/Contents/MacOS/PocketEmulator`** for flags (e.g. `--menu`).

### Run the downloaded app on macOS

Downloads from GitHub are **quarantined** and **unsigned**, so Gatekeeper may block a normal double‑click or show **“damaged”**—that is usually quarantine, not a broken file.

If **`PocketEmulator.app`** is in your **Downloads** folder, open **Terminal** and run:

```bash
xattr -cr ~/Downloads/PocketEmulator.app
open ~/Downloads/PocketEmulator.app
```

- **`xattr -cr`** clears extended attributes (including quarantine) on the app bundle so macOS will allow it to run.
- **`open`** starts the app.

If the `.app` lives somewhere else, replace the path (for example `~/Desktop/PocketEmulator.app`). After the first successful launch, you can open it from Finder like any app.

**Alternatives if you prefer the GUI:** **System Settings → Privacy & Security → Open Anyway**, or **right‑click the app → Open**.

### Windows

1. Download the **Windows** asset from the same release page (installer **`.exe`** from NSIS and/or portable **`PocketEmulator.exe`**, depending on what was uploaded).
2. Run the **installer** and follow the prompts, **or** run **`PocketEmulator.exe`** directly.
3. If SmartScreen warns about an **unknown publisher**, choose **More info → Run anyway** (unsigned builds).

---

## Using the app

- The **launcher** opens by default: **My Games** (library), **Saves**, **Settings**, **Get ROMs** (catalog link).
- Supported ROM types: **`.gb`** / **`.gbc`** only.
- **Gameplay:** keyboard — D‑Pad arrows, **Z** (A), **X** (B), **Enter** (Start), **Shift** (Select), **Space** (fast‑forward hold), **Esc** quit. **F5** quick save, **F9** reload save, **F2** pick another ROM, **F12** screenshot (PPM in app data).

### Where data is stored

| OS | Path |
|----|------|
| **macOS** | `~/Library/Application Support/com/pocketemulator/pocketemulator` |
| **Windows** | `%LOCALAPPDATA%\com\pocketemulator\pocketemulator` |
| **Linux** (from source) | `~/.local/share/com/pocketemulator/pocketemulator` |

There you’ll find `state.json`, `saves/*.sav`, and screenshots.

---

## Building from source

### Prerequisites

- **Rust** (stable), **Node.js 20+**, repo cloned locally.

### Launcher + bundled app (recommended)

```bash
npm ci --prefix frontend
./build-app macos      # macOS → ./PocketEmulator.app
./build-app windows    # run on Windows → PocketEmulator.exe at repo root
```

### CLI only (game window / headless flows)

```bash
cargo build --release
cargo run --release -- path/to/game.gb
cargo run --release -- --menu          # open the launcher flow (see src/main.rs)
```

Common flags: `--scale N` (default `4`), `--info`, `--no-autosave`, `--package` (copy binary to `dist/`), `--regression`.

---

## Development

- **Test ROMs (optional):** place open ROMs under `roms/tests/` (gitignored). See [Blargg’s GB tests](https://github.com/retrio/gb-test-roms), [dmg-acid2](https://github.com/mattcurrie/dmg-acid2). Run `cargo test`.
- **Opcode tables:** after editing generators:
  ```bash
  python3 tools/gen_cpu_opcodes.py
  python3 tools/gen_cb.py
  ```

### Layout

- `src/cpu/` — SM83 core (`opcodes_gen.rs`, `cb_gen.rs`)
- `src/ppu.rs` — LCD / tiles / sprites
- `src/cartridge/` — MBC + `.sav`
- `src/frontend/desktop/` — native game window
- `frontend/` — launcher UI

---

## Legal

You must supply your own ROM dumps. This repository does not distribute copyrighted games.
