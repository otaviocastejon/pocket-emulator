import {
  Save,
  Upload,
  Camera,
  Rewind,
  FastForward,
  Settings,
  Power,
  Star,
  Search,
  Gamepad2,
  Play,
  Pause,
  Volume2,
  Keyboard,
  HardDrive,
  ChevronRight,
  CheckCircle2,
  AlertTriangle,
  Info,
  XCircle,
  Plus,
  Trash2,
  MoreHorizontal,
  Heart,
  Clock,
  Folder,
  Maximize2,
  Minus,
  X,
  Terminal,
  Shield,
  Zap,
  PackageOpen,
  MonitorSmartphone,
} from "lucide-react";

import gamePixelQuest from "@/assets/game-pixel-quest.jpg";
import gameStarRacer from "@/assets/game-star-racer.jpg";
import gameDungeon from "@/assets/game-dungeon.jpg";
import gameForest from "@/assets/game-forest.jpg";
import gameBlocks from "@/assets/game-blocks.jpg";
import gameKart from "@/assets/game-kart.jpg";

import { Section } from "@/components/guide/Section";
import { Swatch } from "@/components/guide/Swatch";
import { TokenRow } from "@/components/guide/TokenRow";

const NAV = [
  { id: "foundations", label: "Foundations" },
  { id: "color", label: "Color" },
  { id: "typography", label: "Typography" },
  { id: "spacing", label: "Spacing" },
  { id: "elevation", label: "Elevation" },
  { id: "components", label: "Components" },
  { id: "patterns", label: "Patterns" },
  { id: "voice", label: "Voice" },
  { id: "tauri", label: "Tauri + Vite" },
];

const GAMES = [
  { title: "Pixel Quest", system: "GBC", playtime: "12h 24m", cover: gamePixelQuest, last: "Yesterday" },
  { title: "Star Racer", system: "GB", playtime: "3h 02m", cover: gameStarRacer, last: "2 days ago" },
  { title: "Dungeon Keeper", system: "GBC", playtime: "21h 48m", cover: gameDungeon, last: "Today" },
  { title: "Forest Tales", system: "GBC", playtime: "8h 15m", cover: gameForest, last: "Last week" },
  { title: "Block Puzzle", system: "GB", playtime: "1h 30m", cover: gameBlocks, last: "1h ago" },
  { title: "Kart Mayhem", system: "GBC", playtime: "5h 12m", cover: gameKart, last: "3 days ago" },
];

export function StyleGuide() {
  return (
    <div className="grain min-h-screen text-foreground">
      {/* Top bar */}
      <header className="sticky top-0 z-40 border-b border-border bg-background/70 backdrop-blur-xl">
        <div className="mx-auto flex max-w-7xl items-center justify-between px-6 py-4">
          <div className="flex items-center gap-3">
            <div className="flex h-9 w-9 items-center justify-center rounded-lg bg-gradient-to-br from-primary to-dmg-red">
              <Gamepad2 className="h-5 w-5 text-primary-foreground" />
            </div>
            <div>
              <p className="text-sm font-bold leading-none">PocketEmulator</p>
              <p className="font-pixel text-[8px] uppercase tracking-widest text-muted-foreground">
                Design System v1.0
              </p>
            </div>
          </div>
          <nav className="hidden items-center gap-1 lg:flex">
            {NAV.map((n) => (
              <a
                key={n.id}
                href={`#${n.id}`}
                className="rounded-md px-3 py-1.5 text-xs font-medium text-muted-foreground transition-colors hover:bg-surface-2 hover:text-foreground"
              >
                {n.label}
              </a>
            ))}
          </nav>
          <button className="hidden items-center gap-2 rounded-lg border border-border bg-surface-2 px-3 py-1.5 text-xs text-muted-foreground transition-colors hover:text-foreground md:flex">
            <Search className="h-3.5 w-3.5" />
            Search tokens
            <kbd className="ml-2 rounded border border-border bg-background px-1.5 py-0.5 font-mono text-[10px]">⌘K</kbd>
          </button>
        </div>
      </header>

      <main className="mx-auto max-w-7xl px-6">
        {/* Hero */}
        <section className="relative overflow-hidden py-20">
          <div className="grid items-center gap-12 lg:grid-cols-2">
            <div>
              <p className="font-pixel text-[10px] uppercase tracking-widest text-accent">
                PocketEmulator · UI/UX Guide
              </p>
              <h1 className="mt-4 text-5xl font-extrabold leading-[1.05] tracking-tight md:text-6xl">
                A <span className="gradient-text">nostalgic</span> system,
                <br />
                built for modern desktop.
              </h1>
              <p className="mt-6 max-w-xl text-lg leading-relaxed text-muted-foreground">
                The complete design language for a Game Boy emulator that feels
                instant, friendly, and keeps your save files safe. Tokens,
                components, and patterns shared across launcher and gameplay.
              </p>
              <div className="mt-8 flex flex-wrap gap-3">
                <a
                  href="#foundations"
                  className="inline-flex items-center gap-2 rounded-lg bg-primary px-5 py-3 text-sm font-semibold text-primary-foreground shadow-md transition-transform hover:-translate-y-0.5"
                >
                  Explore the system <ChevronRight className="h-4 w-4" />
                </a>
                <a
                  href="#components"
                  className="inline-flex items-center gap-2 rounded-lg border border-border bg-surface-2 px-5 py-3 text-sm font-semibold text-foreground transition-colors hover:bg-surface-3"
                >
                  Components
                </a>
              </div>

              <div className="mt-10 grid grid-cols-3 gap-6 border-t border-border pt-8">
                <Stat label="Tokens" value="86" />
                <Stat label="Components" value="32" />
                <Stat label="Patterns" value="14" />
              </div>
            </div>

            {/* Hero visual: stylized launcher preview */}
            <div className="relative">
              <div className="surface-card overflow-hidden shadow-lg">
                <div className="flex items-center gap-1.5 border-b border-border bg-surface-2 px-4 py-3">
                  <span className="h-3 w-3 rounded-full bg-dmg-red/80" />
                  <span className="h-3 w-3 rounded-full bg-dmg-yellow/80" />
                  <span className="h-3 w-3 rounded-full bg-success/80" />
                  <span className="ml-3 font-pixel text-[9px] uppercase tracking-widest text-muted-foreground">
                    PocketEmulator — My Games
                  </span>
                </div>
                <div className="p-5">
                  <div className="mb-4 flex items-center justify-between">
                    <h3 className="text-sm font-semibold">Continue playing</h3>
                    <span className="text-xs text-muted-foreground">6 games</span>
                  </div>
                  <div className="grid grid-cols-3 gap-3">
                    {GAMES.slice(0, 6).map((g) => (
                      <div key={g.title} className="group">
                        <div className="aspect-square overflow-hidden rounded-lg border border-border">
                          <img
                            src={g.cover}
                            alt={g.title}
                            width={512}
                            height={512}
                            loading="lazy"
                            className="h-full w-full object-cover transition-transform group-hover:scale-105"
                          />
                        </div>
                        <p className="mt-1.5 truncate text-[11px] font-medium">{g.title}</p>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
              <div className="absolute -bottom-6 -right-6 hidden rounded-2xl border border-border bg-surface-2 p-4 shadow-lg md:block">
                <div className="lcd-screen flex h-24 w-40 items-center justify-center rounded-md">
                  <span className="font-pixel pixel-shadow text-sm text-lcd-ink">
                    READY
                  </span>
                </div>
              </div>
            </div>
          </div>
        </section>

        {/* Foundations */}
        <Section
          id="foundations"
          eyebrow="01 · Foundations"
          title="Principles that shape every screen"
          description="Five rules we apply to every decision — from tokens to copy."
        >
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-5">
            {[
              { icon: Play, title: "Instant play", desc: "One-click resume. No menus between user and game." },
              { icon: HardDrive, title: "Progress safety", desc: "Saves are visible, reassuring, and never silent." },
              { icon: Search, title: "Discoverability", desc: "Find features in the UI — not in shortcut docs." },
              { icon: Settings, title: "Progressive disclosure", desc: "Basic first. Power tools, one tap deeper." },
              { icon: Heart, title: "Consistency", desc: "Same grammar across launcher, settings, and game." },
            ].map(({ icon: Icon, title, desc }) => (
              <div key={title} className="surface-card p-5">
                <div className="mb-4 inline-flex h-9 w-9 items-center justify-center rounded-lg bg-accent/15 text-accent">
                  <Icon className="h-4 w-4" />
                </div>
                <h4 className="text-sm font-semibold">{title}</h4>
                <p className="mt-2 text-xs leading-relaxed text-muted-foreground">{desc}</p>
              </div>
            ))}
          </div>
        </Section>

        {/* Color */}
        <Section
          id="color"
          eyebrow="02 · Color"
          title="A neutral surface, a retro accent"
          description="Dark, low-chroma neutrals carry content. Accents borrow from DMG hardware: LCD green, classic red, deep blue, and warm yellow."
        >
          <div className="space-y-10">
            <div>
              <h3 className="mb-4 text-sm font-semibold uppercase tracking-wider text-muted-foreground">
                Surfaces & text
              </h3>
              <div className="grid gap-4 md:grid-cols-3 lg:grid-cols-6">
                <Swatch name="Background" token="--background" value="oklch(0.16 0.025 280)" bgClass="bg-background" />
                <Swatch name="Surface" token="--surface" value="oklch(0.21 0.025 280)" bgClass="bg-surface" />
                <Swatch name="Surface 2" token="--surface-2" value="oklch(0.25 0.028 280)" bgClass="bg-surface-2" />
                <Swatch name="Surface 3" token="--surface-3" value="oklch(0.30 0.030 280)" bgClass="bg-surface-3" />
                <Swatch name="Border" token="--border" value="oklch(0.32 0.025 280)" bgClass="bg-border" />
                <Swatch name="Foreground" token="--foreground" value="oklch(0.96 0.01 280)" bgClass="bg-foreground" textClass="text-background" />
              </div>
            </div>

            <div>
              <h3 className="mb-4 text-sm font-semibold uppercase tracking-wider text-muted-foreground">
                Brand & retro accents
              </h3>
              <div className="grid gap-4 md:grid-cols-3 lg:grid-cols-6">
                <Swatch name="Primary" token="--primary" value="oklch(0.62 0.22 350)" bgClass="bg-primary" textClass="text-primary-foreground" note="CTAs, focus rings" />
                <Swatch name="Accent" token="--accent" value="oklch(0.78 0.16 145)" bgClass="bg-accent" textClass="text-accent-foreground" note="LCD green, highlights" />
                <Swatch name="DMG Red" token="--dmg-red" value="oklch(0.62 0.24 27)" bgClass="bg-dmg-red" textClass="text-primary-foreground" />
                <Swatch name="DMG Blue" token="--dmg-blue" value="oklch(0.55 0.18 250)" bgClass="bg-dmg-blue" textClass="text-primary-foreground" />
                <Swatch name="DMG Yellow" token="--dmg-yellow" value="oklch(0.85 0.15 90)" bgClass="bg-dmg-yellow" textClass="text-background" />
                <Swatch name="LCD Deep" token="--lcd-deep" value="oklch(0.62 0.14 145)" bgClass="bg-lcd-deep" textClass="text-background" />
              </div>
            </div>

            <div>
              <h3 className="mb-4 text-sm font-semibold uppercase tracking-wider text-muted-foreground">
                Semantic states
              </h3>
              <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
                <Swatch name="Success" token="--success" value="oklch(0.74 0.16 150)" bgClass="bg-success" textClass="text-success-foreground" note="Save complete" />
                <Swatch name="Warning" token="--warning" value="oklch(0.82 0.16 80)" bgClass="bg-warning" textClass="text-warning-foreground" note="Unsaved changes" />
                <Swatch name="Destructive" token="--destructive" value="oklch(0.62 0.24 27)" bgClass="bg-destructive" textClass="text-destructive-foreground" note="Delete ROM" />
                <Swatch name="Info" token="--info" value="oklch(0.7 0.14 230)" bgClass="bg-info" textClass="text-info-foreground" note="Tip / hint" />
              </div>
            </div>
          </div>
        </Section>

        {/* Typography */}
        <Section
          id="typography"
          eyebrow="03 · Typography"
          title="Inter for UI. Press Start 2P for personality."
          description="Inter handles every readable surface. Press Start 2P appears only in small accent moments — eyebrows, LCD readouts, status badges — never in long copy."
        >
          <div className="grid gap-6 lg:grid-cols-3">
            <div className="surface-card p-8 lg:col-span-2">
              <p className="font-pixel text-[10px] uppercase tracking-widest text-accent">Display / 5xl</p>
              <p className="mt-2 text-5xl font-extrabold tracking-tight">Resume Pixel Quest</p>
              <div className="mt-6 border-t border-border pt-6">
                <p className="font-pixel text-[10px] uppercase tracking-widest text-accent">H2 / 3xl</p>
                <p className="mt-2 text-3xl font-bold tracking-tight">Recently played</p>
              </div>
              <div className="mt-6 border-t border-border pt-6">
                <p className="font-pixel text-[10px] uppercase tracking-widest text-accent">H3 / xl</p>
                <p className="mt-2 text-xl font-semibold">Per-game settings</p>
              </div>
              <div className="mt-6 border-t border-border pt-6">
                <p className="font-pixel text-[10px] uppercase tracking-widest text-accent">Body / base</p>
                <p className="mt-2 text-base leading-relaxed text-muted-foreground">
                  Your save file was backed up automatically before the last
                  load. PocketEmulator keeps three rolling backups per game, so
                  you can always recover the last good state.
                </p>
              </div>
              <div className="mt-6 border-t border-border pt-6">
                <p className="font-pixel text-[10px] uppercase tracking-widest text-accent">Caption / xs</p>
                <p className="mt-2 text-xs text-muted-foreground">
                  Last played · 2 hours ago · Autosave on
                </p>
              </div>
              <div className="mt-6 border-t border-border pt-6">
                <p className="font-pixel text-[10px] uppercase tracking-widest text-accent">Mono</p>
                <p className="mt-2 font-mono text-sm">~/Library/PocketEmulator/saves/pixel_quest.sav</p>
              </div>
            </div>

            <div className="space-y-4">
              <div className="surface-card p-5">
                <p className="text-xs uppercase tracking-wider text-muted-foreground">Display</p>
                <p className="mt-2 text-2xl font-bold">Inter</p>
                <p className="mt-1 text-xs text-muted-foreground">400 / 500 / 600 / 700 / 800</p>
                <p className="mt-4 text-2xl">Aa Bb Cc 0123</p>
              </div>
              <div className="surface-card p-5">
                <p className="text-xs uppercase tracking-wider text-muted-foreground">Pixel</p>
                <p className="font-pixel mt-2 text-base">Press Start 2P</p>
                <p className="mt-1 text-xs text-muted-foreground">Eyebrows, LCD, badges only</p>
                <p className="font-pixel mt-4 text-sm pixel-shadow">AA BB 01 23</p>
              </div>
              <div className="surface-card p-5">
                <p className="text-xs uppercase tracking-wider text-muted-foreground">Mono</p>
                <p className="font-mono mt-2 text-base font-semibold">JetBrains Mono</p>
                <p className="mt-1 text-xs text-muted-foreground">Paths, tokens, hex codes</p>
                <p className="font-mono mt-4 text-sm">aa bb 01 23 →</p>
              </div>
            </div>
          </div>
        </Section>

        {/* Spacing */}
        <Section
          id="spacing"
          eyebrow="04 · Spacing & Radius"
          title="A 4px rhythm, soft 12px corners"
          description="Multiples of 4 keep components aligned across launcher rows, settings panels, and the in-game overlay."
        >
          <div className="grid gap-6 lg:grid-cols-2">
            <div className="surface-card overflow-hidden">
              <div className="border-b border-border bg-surface-2 px-5 py-3">
                <p className="text-sm font-semibold">Spacing scale</p>
              </div>
              <div className="px-2 py-2">
                {[
                  { token: "space-1", value: "4px", w: "w-1" },
                  { token: "space-2", value: "8px", w: "w-2" },
                  { token: "space-3", value: "12px", w: "w-3" },
                  { token: "space-4", value: "16px", w: "w-4" },
                  { token: "space-6", value: "24px", w: "w-6" },
                  { token: "space-8", value: "32px", w: "w-8" },
                  { token: "space-12", value: "48px", w: "w-12" },
                  { token: "space-16", value: "64px", w: "w-16" },
                ].map((s) => (
                  <TokenRow
                    key={s.token}
                    token={s.token}
                    value={s.value}
                    preview={<div className={`h-3 ${s.w} rounded-sm bg-accent`} />}
                  />
                ))}
              </div>
            </div>

            <div className="surface-card overflow-hidden">
              <div className="border-b border-border bg-surface-2 px-5 py-3">
                <p className="text-sm font-semibold">Radius scale</p>
              </div>
              <div className="px-2 py-2">
                {[
                  { token: "radius-sm", value: "8px", r: "rounded-sm" },
                  { token: "radius-md", value: "10px", r: "rounded-md" },
                  { token: "radius-lg", value: "12px", r: "rounded-lg" },
                  { token: "radius-xl", value: "16px", r: "rounded-xl" },
                  { token: "radius-2xl", value: "20px", r: "rounded-2xl" },
                  { token: "radius-3xl", value: "24px", r: "rounded-3xl" },
                ].map((s) => (
                  <TokenRow
                    key={s.token}
                    token={s.token}
                    value={s.value}
                    preview={<div className={`h-10 w-10 ${s.r} bg-primary`} />}
                  />
                ))}
              </div>
            </div>
          </div>
        </Section>

        {/* Elevation */}
        <Section
          id="elevation"
          eyebrow="05 · Elevation & Effects"
          title="Subtle depth, one signature glow"
          description="Shadows are soft and warm-cool; the only loud effect is the LCD glow — used sparingly to celebrate gameplay."
        >
          <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-4">
            <ElevationCard label="shadow-sm" desc="Inputs, list rows" shadow="shadow-sm" />
            <ElevationCard label="shadow-md" desc="Cards, popovers" shadow="shadow-md" />
            <ElevationCard label="shadow-lg" desc="Modals, command bar" shadow="shadow-lg" />
            <div className="surface-card p-5">
              <div
                className="lcd-screen mb-4 flex h-24 items-center justify-center rounded-md"
              >
                <span className="font-pixel pixel-shadow text-base text-lcd-ink">
                  GAME ON
                </span>
              </div>
              <p className="text-sm font-semibold">LCD glow</p>
              <p className="mt-1 text-xs text-muted-foreground">Reserved for the in-game canvas and hero moments only.</p>
            </div>
          </div>
        </Section>

        {/* Components */}
        <Section
          id="components"
          eyebrow="06 · Components"
          title="The building blocks"
          description="Every reusable piece — buttons, badges, list rows, toggles, keybind inputs, toasts, and the in-game control bar."
        >
          <div className="space-y-8">
            {/* Buttons */}
            <ComponentBlock title="Buttons" hint="Primary for one-tap launches. Secondary for everything else.">
              <div className="flex flex-wrap items-center gap-3">
                <button className="inline-flex items-center gap-2 rounded-lg bg-primary px-4 py-2.5 text-sm font-semibold text-primary-foreground shadow-md transition-transform hover:-translate-y-0.5">
                  <Play className="h-4 w-4" /> Resume
                </button>
                <button className="inline-flex items-center gap-2 rounded-lg border border-border bg-surface-2 px-4 py-2.5 text-sm font-semibold transition-colors hover:bg-surface-3">
                  <Folder className="h-4 w-4" /> Add ROM
                </button>
                <button className="inline-flex items-center gap-2 rounded-lg bg-accent px-4 py-2.5 text-sm font-semibold text-accent-foreground transition-colors hover:opacity-90">
                  <Save className="h-4 w-4" /> Save state
                </button>
                <button className="inline-flex items-center gap-2 rounded-lg bg-destructive px-4 py-2.5 text-sm font-semibold text-destructive-foreground transition-colors hover:opacity-90">
                  <Trash2 className="h-4 w-4" /> Delete
                </button>
                <button className="inline-flex items-center gap-2 rounded-lg px-4 py-2.5 text-sm font-semibold text-muted-foreground transition-colors hover:bg-surface-2 hover:text-foreground">
                  Cancel
                </button>
                <button className="inline-flex h-9 w-9 items-center justify-center rounded-lg border border-border bg-surface-2 transition-colors hover:bg-surface-3">
                  <Settings className="h-4 w-4" />
                </button>
              </div>
            </ComponentBlock>

            {/* Badges */}
            <ComponentBlock title="Badges & status" hint="Pixel-cased for retro ID tags; sentence-case for state.">
              <div className="flex flex-wrap items-center gap-3">
                <span className="font-pixel inline-flex items-center rounded-md bg-accent/15 px-2 py-1 text-[9px] uppercase tracking-widest text-accent">GBC</span>
                <span className="font-pixel inline-flex items-center rounded-md bg-dmg-blue/15 px-2 py-1 text-[9px] uppercase tracking-widest text-dmg-blue">GB</span>
                <span className="inline-flex items-center gap-1.5 rounded-full bg-success/15 px-2.5 py-1 text-xs font-medium text-success">
                  <CheckCircle2 className="h-3 w-3" /> Saved
                </span>
                <span className="inline-flex items-center gap-1.5 rounded-full bg-warning/15 px-2.5 py-1 text-xs font-medium text-warning">
                  <AlertTriangle className="h-3 w-3" /> Unsaved
                </span>
                <span className="inline-flex items-center gap-1.5 rounded-full bg-destructive/15 px-2.5 py-1 text-xs font-medium text-destructive">
                  <XCircle className="h-3 w-3" /> Save error
                </span>
                <span className="inline-flex items-center gap-1.5 rounded-full bg-info/15 px-2.5 py-1 text-xs font-medium text-info">
                  <Info className="h-3 w-3" /> Tip
                </span>
              </div>
            </ComponentBlock>

            {/* Game card */}
            <ComponentBlock title="Game cards" hint="Cover-led, with system, last play, and one-tap resume.">
              <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                {GAMES.slice(0, 3).map((g) => (
                  <article key={g.title} className="group surface-card overflow-hidden transition-transform hover:-translate-y-1">
                    <div className="relative aspect-[4/3] overflow-hidden">
                      <img src={g.cover} alt={g.title} loading="lazy" width={512} height={384} className="h-full w-full object-cover transition-transform group-hover:scale-105" />
                      <div className="absolute inset-0 bg-gradient-to-t from-background/90 via-background/0 to-transparent" />
                      <span className="font-pixel absolute right-3 top-3 rounded-md bg-background/70 px-2 py-1 text-[9px] uppercase tracking-widest text-accent backdrop-blur">
                        {g.system}
                      </span>
                      <button className="absolute bottom-3 right-3 inline-flex h-10 w-10 items-center justify-center rounded-full bg-primary text-primary-foreground shadow-lg transition-transform group-hover:scale-110">
                        <Play className="h-4 w-4 fill-current" />
                      </button>
                    </div>
                    <div className="p-4">
                      <h4 className="text-sm font-semibold">{g.title}</h4>
                      <div className="mt-2 flex items-center gap-3 text-xs text-muted-foreground">
                        <span className="inline-flex items-center gap-1"><Clock className="h-3 w-3" /> {g.playtime}</span>
                        <span>·</span>
                        <span>{g.last}</span>
                      </div>
                    </div>
                  </article>
                ))}
              </div>
            </ComponentBlock>

            {/* Library row */}
            <ComponentBlock title="Library row" hint="Compact alternative for long libraries. Right-side quick actions.">
              <div className="surface-card divide-y divide-border overflow-hidden">
                {GAMES.slice(0, 4).map((g) => (
                  <div key={g.title} className="flex items-center gap-4 px-4 py-3 transition-colors hover:bg-surface-2">
                    <img src={g.cover} alt="" loading="lazy" width={48} height={48} className="h-12 w-12 rounded-md object-cover" />
                    <div className="min-w-0 flex-1">
                      <p className="truncate text-sm font-semibold">{g.title}</p>
                      <p className="truncate text-xs text-muted-foreground">{g.last} · {g.playtime}</p>
                    </div>
                    <span className="font-pixel hidden rounded-md bg-surface-3 px-2 py-1 text-[9px] uppercase tracking-widest text-muted-foreground sm:inline">{g.system}</span>
                    <button className="inline-flex h-8 w-8 items-center justify-center rounded-md text-muted-foreground hover:bg-surface-3 hover:text-warning">
                      <Star className="h-4 w-4" />
                    </button>
                    <button className="inline-flex h-8 items-center gap-1.5 rounded-md bg-primary px-3 text-xs font-semibold text-primary-foreground">
                      <Play className="h-3.5 w-3.5" /> Play
                    </button>
                    <button className="inline-flex h-8 w-8 items-center justify-center rounded-md text-muted-foreground hover:bg-surface-3 hover:text-foreground">
                      <MoreHorizontal className="h-4 w-4" />
                    </button>
                  </div>
                ))}
              </div>
            </ComponentBlock>

            {/* Form controls */}
            <ComponentBlock title="Form controls" hint="Toggles, sliders, segmented tabs, keybind inputs.">
              <div className="grid gap-6 lg:grid-cols-2">
                <div className="surface-card space-y-5 p-5">
                  <FormRow label="Autosave" hint="Every 30 seconds while playing">
                    <Toggle on />
                  </FormRow>
                  <FormRow label="Rewind buffer" hint="Hold last 60 seconds">
                    <Toggle on />
                  </FormRow>
                  <FormRow label="Color filter">
                    <Segmented options={["Off", "DMG", "Pocket", "Light"]} active={1} />
                  </FormRow>
                  <FormRow label="Display scale">
                    <Slider value={3} max={5} />
                  </FormRow>
                  <FormRow label="Audio mode">
                    <Segmented options={["Mono", "Stereo"]} active={1} />
                  </FormRow>
                </div>

                <div className="surface-card p-5">
                  <p className="mb-4 text-sm font-semibold">Keybinds</p>
                  <div className="space-y-2">
                    {[
                      { action: "A button", key: "X" },
                      { action: "B button", key: "Z" },
                      { action: "Start", key: "Enter" },
                      { action: "Select", key: "Shift" },
                      { action: "Fast forward", key: "Space" },
                      { action: "Save state", key: "F5" },
                    ].map((k) => (
                      <div key={k.action} className="flex items-center justify-between rounded-lg border border-border bg-surface-2 px-3 py-2">
                        <span className="text-sm">{k.action}</span>
                        <button className="inline-flex items-center gap-1.5 rounded-md border border-border bg-background px-2.5 py-1 font-mono text-xs hover:border-primary hover:text-primary">
                          <Keyboard className="h-3 w-3" />
                          {k.key}
                        </button>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            </ComponentBlock>

            {/* Toasts */}
            <ComponentBlock title="Toasts" hint="Bottom-right. 3s auto-dismiss. One action max.">
              <div className="flex flex-col gap-3">
                <Toast icon={CheckCircle2} tone="success" title="Save state created" message="Slot 3 · Pixel Quest · just now" />
                <Toast icon={AlertTriangle} tone="warning" title="Unsaved progress" message="Last battery save was 4 minutes ago." action="Save now" />
                <Toast icon={XCircle} tone="destructive" title="Couldn't write save" message="Restored from backup automatically." action="Show file" />
                <Toast icon={Info} tone="info" title="New shortcut available" message="Press F8 to take a screenshot." />
              </div>
            </ComponentBlock>
          </div>
        </Section>

        {/* Patterns */}
        <Section
          id="patterns"
          eyebrow="07 · Patterns"
          title="In-game overlay & launcher shell"
          description="Two flagship surfaces that share grammar — same colors, same radii, same spacing rhythm — so users feel at home in both."
        >
          <div className="grid gap-8 lg:grid-cols-5">
            {/* Launcher mock */}
            <div className="surface-card overflow-hidden lg:col-span-3">
              <div className="flex items-center gap-1.5 border-b border-border bg-surface-2 px-4 py-3">
                <span className="h-3 w-3 rounded-full bg-dmg-red/80" />
                <span className="h-3 w-3 rounded-full bg-dmg-yellow/80" />
                <span className="h-3 w-3 rounded-full bg-success/80" />
                <span className="ml-3 font-pixel text-[9px] uppercase tracking-widest text-muted-foreground">
                  Launcher · My Games
                </span>
              </div>
              <div className="grid grid-cols-[180px_1fr]">
                <aside className="border-r border-border bg-surface/60 p-3">
                  <SidebarItem icon={Gamepad2} label="My Games" active />
                  <SidebarItem icon={Star} label="Favorites" />
                  <SidebarItem icon={Clock} label="Recent" />
                  <SidebarItem icon={HardDrive} label="Saves" />
                  <SidebarItem icon={Settings} label="Settings" />
                  <div className="my-3 h-px bg-border" />
                  <p className="px-2 text-[10px] uppercase tracking-wider text-muted-foreground">Library</p>
                  <SidebarItem icon={Folder} label="All ROMs" muted />
                  <SidebarItem icon={Folder} label="Game Boy" muted />
                  <SidebarItem icon={Folder} label="Game Boy Color" muted />
                </aside>
                <div className="p-5">
                  <div className="mb-5 flex items-center justify-between">
                    <div>
                      <h3 className="text-lg font-bold">My Games</h3>
                      <p className="text-xs text-muted-foreground">6 ROMs · 51h total playtime</p>
                    </div>
                    <button className="inline-flex items-center gap-1.5 rounded-lg bg-primary px-3 py-1.5 text-xs font-semibold text-primary-foreground">
                      <Plus className="h-3.5 w-3.5" /> Add ROM
                    </button>
                  </div>
                  <div className="grid grid-cols-3 gap-3">
                    {GAMES.map((g) => (
                      <div key={g.title} className="group">
                        <div className="aspect-square overflow-hidden rounded-lg border border-border">
                          <img src={g.cover} alt={g.title} loading="lazy" width={256} height={256} className="h-full w-full object-cover transition-transform group-hover:scale-105" />
                        </div>
                        <p className="mt-1.5 truncate text-[11px] font-semibold">{g.title}</p>
                        <p className="text-[10px] text-muted-foreground">{g.last}</p>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            </div>

            {/* In-game overlay mock */}
            <div className="surface-card overflow-hidden lg:col-span-2">
              <div className="flex items-center gap-1.5 border-b border-border bg-surface-2 px-4 py-3">
                <span className="h-3 w-3 rounded-full bg-dmg-red/80" />
                <span className="h-3 w-3 rounded-full bg-dmg-yellow/80" />
                <span className="h-3 w-3 rounded-full bg-success/80" />
                <span className="ml-3 font-pixel text-[9px] uppercase tracking-widest text-muted-foreground">
                  Now playing · Pixel Quest
                </span>
              </div>
              <div className="relative bg-background p-5">
                <div className="lcd-screen mx-auto flex aspect-[10/9] w-full items-center justify-center rounded-lg">
                  <div className="text-center">
                    <p className="font-pixel text-[8px] uppercase tracking-widest text-lcd-ink/70">▶ playing</p>
                    <p className="font-pixel pixel-shadow mt-3 text-2xl text-lcd-ink">PIXEL QUEST</p>
                    <p className="font-pixel mt-2 text-[8px] text-lcd-ink/80">Lvl 12 · 02:14:36</p>
                  </div>
                </div>

                {/* Control bar */}
                <div className="mt-4 flex items-center justify-between gap-2 rounded-xl border border-border bg-surface-2/80 p-2 backdrop-blur">
                  <div className="flex items-center gap-1">
                    <OverlayBtn icon={Pause} label="Pause" />
                    <OverlayBtn icon={Save} label="Save" tone="accent" />
                    <OverlayBtn icon={Upload} label="Load" />
                    <OverlayBtn icon={Rewind} label="Rewind" />
                    <OverlayBtn icon={FastForward} label="Speed" />
                    <OverlayBtn icon={Camera} label="Snap" />
                  </div>
                  <div className="flex items-center gap-1">
                    <OverlayBtn icon={Settings} label="Settings" />
                    <OverlayBtn icon={Power} label="Quit" tone="destructive" />
                  </div>
                </div>

                <p className="mt-3 flex items-center gap-1.5 text-[11px] text-muted-foreground">
                  <Volume2 className="h-3 w-3" /> Stereo · Autosave on · Backup 4m ago
                </p>
              </div>
            </div>
          </div>
        </Section>

        {/* Voice */}
        <Section
          id="voice"
          eyebrow="08 · Voice & Microcopy"
          title="Plain, warm, never alarming"
          description="Every label assumes the player isn't a developer. Save messages are reassuring, not technical."
        >
          <div className="grid gap-4 md:grid-cols-2">
            <VoiceCard
              tone="do"
              title="Do"
              examples={[
                "Saved. You're good to keep playing.",
                "Resume Pixel Quest",
                "We backed up your last save automatically.",
                "Add a ROM to get started",
              ]}
            />
            <VoiceCard
              tone="dont"
              title="Don't"
              examples={[
                "WriteFile() succeeded at 0x4FA1",
                "Continue session [Y/n]",
                "Save corrupted. ENOENT on backup path.",
                "Drag a .gb or .gbc file into the application window",
              ]}
            />
          </div>
        </Section>

        {/* Tauri + Vite */}
        <Section
          id="tauri"
          eyebrow="09 · Tauri + Vite"
          title="Designed for a native desktop shell"
          description="PocketEmulator ships as a Tauri 2 app with a Vite-powered React frontend. The system accounts for the native window chrome, OS conventions, IPC boundaries, and the Rust ↔ webview bridge."
        >
          <div className="grid gap-6 lg:grid-cols-2">
            <div className="surface-card overflow-hidden">
              <div className="border-b border-border bg-surface-2 px-5 py-3">
                <p className="text-sm font-semibold">Custom titlebar (decorations: false)</p>
                <p className="mt-0.5 text-xs text-muted-foreground">
                  Use <span className="font-mono">data-tauri-drag-region</span> on the draggable area, never on buttons.
                </p>
              </div>
              <div className="p-4">
                <div className="overflow-hidden rounded-lg border border-border">
                  <div data-tauri-drag-region className="flex items-center justify-between bg-surface-2 px-3 py-2">
                    <div className="flex items-center gap-2">
                      <div className="flex h-5 w-5 items-center justify-center rounded bg-gradient-to-br from-primary to-dmg-red">
                        <Gamepad2 className="h-3 w-3 text-primary-foreground" />
                      </div>
                      <span className="text-xs font-medium">PocketEmulator</span>
                      <span className="font-pixel text-[8px] uppercase tracking-widest text-muted-foreground">Pixel Quest · GBC</span>
                    </div>
                    <div className="flex items-center gap-1">
                      <WinBtn icon={Minus} label="Minimize" />
                      <WinBtn icon={Maximize2} label="Maximize" />
                      <WinBtn icon={X} label="Close" tone="destructive" />
                    </div>
                  </div>
                  <div className="lcd-screen flex h-32 items-center justify-center">
                    <span className="font-pixel pixel-shadow text-xs text-lcd-ink">WEBVIEW · 60 FPS</span>
                  </div>
                </div>
                <p className="mt-3 text-xs text-muted-foreground">
                  Min size <span className="font-mono">960 × 600</span> · respects OS button order (left on macOS, right on Win/Linux).
                </p>
              </div>
            </div>

            <div className="surface-card overflow-hidden">
              <div className="border-b border-border bg-surface-2 px-5 py-3">
                <p className="text-sm font-semibold">Platform-aware tokens</p>
              </div>
              <div className="p-2">
                <TokenRow token="--titlebar-height" value="32px (win/linux) · 28px (macOS)" description="Reserved drag region. Layouts offset by this value." preview={<MonitorSmartphone className="h-4 w-4 text-accent" />} />
                <TokenRow token="--safe-area-traffic-lights" value="78px on macOS" description="Left padding when window controls overlay content." preview={<div className="h-3 w-20 rounded-sm bg-surface-3" />} />
                <TokenRow token="--mod-key" value="⌘ on macOS · Ctrl elsewhere" description="Resolved via @tauri-apps/plugin-os at runtime." preview={<kbd className="rounded border border-border bg-background px-1.5 py-0.5 font-mono text-[10px]">⌘ / Ctrl</kbd>} />
                <TokenRow token="--app-data-dir" value="appDataDir() from @tauri-apps/api/path" description="ROMs, saves, config never use hardcoded paths." preview={<Folder className="h-4 w-4 text-muted-foreground" />} />
                <TokenRow token="--font-render" value="-webkit-font-smoothing: antialiased" description="Sharper UI text inside WKWebView / WebView2." preview={<span className="text-xs">Aa</span>} />
              </div>
            </div>
          </div>

          <div className="mt-6 grid gap-6 lg:grid-cols-3">
            <div className="surface-card p-5 lg:col-span-2">
              <div className="mb-3 flex items-center gap-2">
                <Terminal className="h-4 w-4 text-accent" />
                <p className="text-sm font-semibold">IPC contract — frontend ↔ Rust core</p>
              </div>
              <p className="text-xs text-muted-foreground">
                Every Rust command is a typed wrapper. Components consume hooks that mirror the command surface — never <span className="font-mono">invoke</span> inline.
              </p>
              <pre className="mt-4 overflow-x-auto rounded-lg border border-border bg-background/60 p-4 font-mono text-[11px] leading-relaxed text-foreground">
{`// src/lib/ipc.ts
import { invoke } from "@tauri-apps/api/core";

export const ipc = {
  loadRom:     (path: string)  => invoke<RomMeta>("load_rom", { path }),
  saveState:   (slot: number)  => invoke<void>("save_state",  { slot }),
  listLibrary: ()              => invoke<RomMeta[]>("list_library"),
  setPalette:  (id: PaletteId) => invoke<void>("set_palette", { id }),
};

// src-tauri/src/commands.rs
#[tauri::command]
pub async fn load_rom(path: String) -> Result<RomMeta, AppError> { /* ... */ }`}
              </pre>
              <div className="mt-4 grid gap-3 sm:grid-cols-3">
                <Bullet icon={Shield} title="Scoped allowlist" body="Only $APPDATA & user-picked ROM dirs." />
                <Bullet icon={Zap} title="Non-blocking" body="Long calls async; UI stays at 60 FPS." />
                <Bullet icon={PackageOpen} title="Typed errors" body="Rust AppError → toast tone mapping." />
              </div>
            </div>

            <div className="surface-card p-5">
              <p className="text-sm font-semibold">Vite config essentials</p>
              <ul className="mt-3 space-y-2 text-xs text-muted-foreground">
                <li><span className="font-mono text-foreground">server.port: 1420</span> — matches <span className="font-mono">tauri.conf.json</span> devUrl</li>
                <li><span className="font-mono text-foreground">server.strictPort: true</span> — fail fast if taken</li>
                <li><span className="font-mono text-foreground">clearScreen: false</span> — keep cargo logs visible</li>
                <li><span className="font-mono text-foreground">envPrefix: ["VITE_", "TAURI_"]</span></li>
                <li><span className="font-mono text-foreground">build.target: "es2021"</span> — WebView2 / WKWebView baseline</li>
                <li>No SSR — bundle ships as static assets inside the binary</li>
              </ul>
            </div>
          </div>

          <div className="mt-6 grid gap-4 md:grid-cols-2 lg:grid-cols-4">
            <Bullet icon={MonitorSmartphone} title="Native menus" body="File / Edit / View / Game / Window / Help via tauri::menu — mirrored by an in-app ⌘K palette." />
            <Bullet icon={Folder} title="File dialogs" body="plugin-dialog open() for ROM picking. Never a raw path field." />
            <Bullet icon={Shield} title="CSP & capabilities" body="Strict CSP. Capabilities split per-window: launcher vs gameplay." />
            <Bullet icon={Zap} title="Performance budget" body="Frame ≤ 16 ms · cold launch ≤ 1.2 s · idle CPU ≤ 2 %." />
          </div>
        </Section>

        <footer className="border-t border-border py-10 text-center text-xs text-muted-foreground">
          <p className="font-pixel text-[9px] uppercase tracking-widest text-accent">
            PocketEmulator · Design System v1.0
          </p>
          <p className="mt-2">Tokens, components, and patterns for the friendliest way to play classic Game Boy.</p>
        </footer>
      </main>
    </div>
  );
}

/* ---------- Helpers ---------- */

function Stat({ label, value }: { label: string; value: string }) {
  return (
    <div>
      <p className="text-3xl font-extrabold tracking-tight">{value}</p>
      <p className="mt-1 font-pixel text-[9px] uppercase tracking-widest text-muted-foreground">
        {label}
      </p>
    </div>
  );
}

function ElevationCard({ label, desc, shadow }: { label: string; desc: string; shadow: string }) {
  return (
    <div className="surface-card p-5">
      <div className={`mb-4 h-24 rounded-xl bg-surface-3 ${shadow}`} />
      <p className="font-mono text-xs">{label}</p>
      <p className="mt-1 text-xs text-muted-foreground">{desc}</p>
    </div>
  );
}

function ComponentBlock({
  title,
  hint,
  children,
}: {
  title: string;
  hint: string;
  children: React.ReactNode;
}) {
  return (
    <div>
      <div className="mb-4 flex items-baseline justify-between gap-4">
        <h3 className="text-lg font-semibold">{title}</h3>
        <p className="text-xs text-muted-foreground">{hint}</p>
      </div>
      <div className="surface-card p-6">{children}</div>
    </div>
  );
}

function Toggle({ on }: { on?: boolean }) {
  return (
    <span
      className={`relative inline-flex h-6 w-10 items-center rounded-full transition-colors ${
        on ? "bg-accent" : "bg-surface-3"
      }`}
    >
      <span
        className={`inline-block h-5 w-5 transform rounded-full bg-background shadow transition-transform ${
          on ? "translate-x-[18px]" : "translate-x-0.5"
        }`}
      />
    </span>
  );
}

function Segmented({ options, active }: { options: string[]; active: number }) {
  return (
    <div className="inline-flex rounded-lg border border-border bg-surface-2 p-0.5">
      {options.map((o, i) => (
        <button
          key={o}
          className={`rounded-md px-3 py-1.5 text-xs font-medium transition-colors ${
            i === active
              ? "bg-primary text-primary-foreground"
              : "text-muted-foreground hover:text-foreground"
          }`}
        >
          {o}
        </button>
      ))}
    </div>
  );
}

function Slider({ value, max }: { value: number; max: number }) {
  const pct = (value / max) * 100;
  return (
    <div className="flex w-44 items-center gap-3">
      <div className="relative h-1.5 flex-1 rounded-full bg-surface-3">
        <div className="absolute inset-y-0 left-0 rounded-full bg-accent" style={{ width: `${pct}%` }} />
        <div className="absolute top-1/2 h-4 w-4 -translate-y-1/2 rounded-full border-2 border-accent bg-background" style={{ left: `calc(${pct}% - 8px)` }} />
      </div>
      <span className="font-mono text-xs text-muted-foreground">{value}x</span>
    </div>
  );
}

function FormRow({ label, hint, children }: { label: string; hint?: string; children: React.ReactNode }) {
  return (
    <div className="flex items-center justify-between gap-4">
      <div>
        <p className="text-sm font-medium">{label}</p>
        {hint && <p className="mt-0.5 text-xs text-muted-foreground">{hint}</p>}
      </div>
      {children}
    </div>
  );
}

function Toast({
  icon: Icon,
  tone,
  title,
  message,
  action,
}: {
  icon: React.ElementType;
  tone: "success" | "warning" | "destructive" | "info";
  title: string;
  message: string;
  action?: string;
}) {
  const tones: Record<string, string> = {
    success: "border-success/40 bg-success/10",
    warning: "border-warning/40 bg-warning/10",
    destructive: "border-destructive/40 bg-destructive/10",
    info: "border-info/40 bg-info/10",
  };
  const iconTones: Record<string, string> = {
    success: "text-success",
    warning: "text-warning",
    destructive: "text-destructive",
    info: "text-info",
  };
  return (
    <div className={`flex max-w-md items-start gap-3 rounded-xl border p-3 backdrop-blur ${tones[tone]}`}>
      <Icon className={`mt-0.5 h-4 w-4 flex-shrink-0 ${iconTones[tone]}`} />
      <div className="min-w-0 flex-1">
        <p className="text-sm font-semibold">{title}</p>
        <p className="mt-0.5 text-xs text-muted-foreground">{message}</p>
      </div>
      {action && (
        <button className="rounded-md px-2 py-1 text-xs font-semibold text-foreground hover:bg-background/40">
          {action}
        </button>
      )}
    </div>
  );
}

function SidebarItem({
  icon: Icon,
  label,
  active,
  muted,
}: {
  icon: React.ElementType;
  label: string;
  active?: boolean;
  muted?: boolean;
}) {
  return (
    <button
      className={`mt-0.5 flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-left text-xs transition-colors ${
        active
          ? "bg-primary/15 font-semibold text-primary"
          : muted
            ? "text-muted-foreground/70 hover:bg-surface-2 hover:text-foreground"
            : "text-muted-foreground hover:bg-surface-2 hover:text-foreground"
      }`}
    >
      <Icon className="h-3.5 w-3.5" />
      {label}
    </button>
  );
}

function OverlayBtn({
  icon: Icon,
  label,
  tone,
}: {
  icon: React.ElementType;
  label: string;
  tone?: "accent" | "destructive";
}) {
  const toneClass =
    tone === "accent"
      ? "text-accent hover:bg-accent/15"
      : tone === "destructive"
        ? "text-destructive hover:bg-destructive/15"
        : "text-muted-foreground hover:bg-surface-3 hover:text-foreground";
  return (
    <button
      title={label}
      className={`inline-flex h-9 w-9 items-center justify-center rounded-lg transition-colors ${toneClass}`}
    >
      <Icon className="h-4 w-4" />
    </button>
  );
}

function VoiceCard({
  tone,
  title,
  examples,
}: {
  tone: "do" | "dont";
  title: string;
  examples: string[];
}) {
  const isDo = tone === "do";
  return (
    <div className={`surface-card overflow-hidden border-l-4 ${isDo ? "border-l-success" : "border-l-destructive"}`}>
      <div className="flex items-center gap-2 border-b border-border bg-surface-2 px-5 py-3">
        {isDo ? <CheckCircle2 className="h-4 w-4 text-success" /> : <XCircle className="h-4 w-4 text-destructive" />}
        <p className="text-sm font-semibold">{title}</p>
      </div>
      <ul className="divide-y divide-border">
        {examples.map((e) => (
          <li key={e} className={`px-5 py-3 text-sm ${isDo ? "text-foreground" : "text-muted-foreground line-through decoration-destructive/40"}`}>
            {e}
          </li>
        ))}
      </ul>
    </div>
  );
}

function WinBtn({
  icon: Icon,
  label,
  tone,
}: {
  icon: React.ElementType;
  label: string;
  tone?: "destructive";
}) {
  const hover =
    tone === "destructive"
      ? "hover:bg-destructive hover:text-destructive-foreground"
      : "hover:bg-surface-3 hover:text-foreground";
  return (
    <button
      title={label}
      aria-label={label}
      className={`inline-flex h-6 w-6 items-center justify-center rounded text-muted-foreground transition-colors ${hover}`}
    >
      <Icon className="h-3 w-3" />
    </button>
  );
}

function Bullet({
  icon: Icon,
  title,
  body,
}: {
  icon: React.ElementType;
  title: string;
  body: string;
}) {
  return (
    <div className="rounded-lg border border-border bg-surface-2 p-3">
      <div className="mb-1.5 flex items-center gap-2">
        <Icon className="h-3.5 w-3.5 text-accent" />
        <p className="text-xs font-semibold">{title}</p>
      </div>
      <p className="text-[11px] leading-relaxed text-muted-foreground">{body}</p>
    </div>
  );
}
