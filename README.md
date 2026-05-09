# PocketEmulator

A work-in-progress **DMG (original Game Boy)** emulator written in Rust, with a `pixels` + `winit` desktop UI and a stubbed APU (no sound yet).

## Build

```bash
cargo build --release
```

## Run

```bash
cargo run --release -- path/to/game.gb
```

- `--scale N` — integer window scale (default `4`)
- `--info` — print cartridge header from a ROM and exit
- `--no-autosave` — disable periodic autosave
- `--package` — build and copy runnable binary to `dist/`
- `--regression` — run regression suite from `tests/regression_manifest.json`
- `--regression-dir PATH` — folder containing regression ROMs (default `roms/regression`)

### Controls (keyboard)

| Game Boy | Key        |
|----------|------------|
| D-Pad    | Arrow keys |
| A        | Z          |
| B        | X          |
| Start    | Enter      |
| Select   | Shift      |
| Fast-forward (hold) | Space |
| Rewind SRAM snapshot | F7 |
| Screenshot (PPM) | F12 |
| Quit     | Esc        |

## ROM history + save storage

The emulator now keeps user data in an app-data folder (per OS user), not mixed into project files:

- **macOS:** `~/Library/Application Support/com/pocketemulator/pocketemulator`
- **Linux:** `~/.local/share/com/pocketemulator/pocketemulator`
- **Windows:** `%LOCALAPPDATA%\com\pocketemulator\pocketemulator`

Inside that folder:
- `state.json` — recently played ROMs (used by launcher memory)
- `saves/*.sav` — battery-backed saves, one per ROM path (hashed file names)

Battery-backed SRAM is loaded on startup and saved on exit.

Extra save controls:
- `F5` = write `.sav` immediately
- `F9` = reload `.sav` from disk
- `F2` = pick another ROM and switch game
- `F6` = open save folder in Finder
- autosave every ~10 seconds while running (can be toggled in launcher UI)
- crash recovery fallback reads `.sav.bak` if `.sav` is unavailable
- CLI override: `--no-autosave`

## Launcher UX

- `cargo run --release -- --menu` opens the launcher directly
- `My Games` shows recent ROMs from `state.json`
- click a game in `My Games` to launch it immediately
- favorites (`★`) and search are available in `My Games`
- `Settings` tab stores per-game profile values (scale, autosave, controls, video/audio mode)
- profile values are loaded automatically when launching a known game

## Cheats, screenshots, and experimental link play

- Optional cheats file: place `<romname>.cht` beside ROM, one patch per line:
  - `ADDR:VALUE` in hex (example `C000:42`)
- `F12` writes a screenshot as `P6 .ppm` into app data `screenshots/`
- Experimental UDP serial link (advanced). Prefer `POCKETEMU_*`; `MYGAMEBOY_*` still works:
  - `POCKETEMU_LINK_BIND=127.0.0.1:7001` (legacy: `MYGAMEBOY_LINK_BIND`)
  - `POCKETEMU_LINK_PEER=127.0.0.1:7002` (legacy: `MYGAMEBOY_LINK_PEER`)

## Packaging / executable

Create an easy-to-run binary in `dist/`:

```bash
cargo run --release -- --package
```

That produces a native executable for your current OS.

On macOS, it now also creates a **double-clickable app bundle**:
- `dist/PocketEmulator.app`
- plus raw binary: `dist/pocketemulator`
- app icon is always bundled from embedded `assets/icon.png` bytes
- packaging enforces `.icns` generation (`PocketEmulator.icns`) and fails if it cannot be created

You can launch it from Finder like any regular app.

### macOS: "PocketEmulator can't be opened"

Unsigned local builds are often blocked by Gatekeeper the first time. Try in order:

1. **Right‑click** `PocketEmulator.app` → **Open** → confirm **Open** (only needed once).
2. **System Settings → Privacy & Security** — scroll down and click **Open Anyway** if macOS shows a block for this app.
3. If you copied the `.app` from a download or another disk, clear quarantine, then open again:
   ```bash
   xattr -cr dist/PocketEmulator.app
   ```
4. Re-sign the bundle (also runs automatically after `--package` when `codesign` is available):
   ```bash
   codesign --force --deep -s - dist/PocketEmulator.app
   ```

From Terminal you can confirm the binary runs:

```bash
dist/PocketEmulator.app/Contents/MacOS/PocketEmulator --menu
```

### Windows `.exe`

To produce a Windows `.exe`, build on Windows or cross-compile with a Windows target toolchain:

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

Output file:
`target/x86_64-pc-windows-gnu/release/pocketemulator.exe`

## Test ROMs (optional)

Place open-source test ROMs under `roms/tests/` (gitignored). Examples:

- [Blargg’s test ROMs](https://github.com/retrio/gb-test-roms) — `cpu_instrs.gb`, `instr_timing.gb`, …
- [dmg-acid2](https://github.com/mattcurrie/dmg-acid2) — PPU regression

Run `cargo test` — tests skip automatically if the ROM file is missing.

## Project layout

- `src/cpu/` — SM83 CPU (generated opcode tables in `opcodes_gen.rs` / `cb_gen.rs`)
- `src/ppu.rs` — LCD timing + scanline renderer
- `src/cartridge/` — MBC0 / MBC1 / MBC3 / MBC5 + `.sav` loading
- `src/frontend/desktop.rs` — window + input

Regenerate opcode tables after editing generators:

```bash
python3 tools/gen_cpu_opcodes.py
python3 tools/gen_cb.py
```

## Legal

You must supply your own ROM dumps. This repository does not include commercial games.
