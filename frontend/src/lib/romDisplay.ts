import type { RomSummary } from "../types/launcher";

export function formatRelative(unix: number | null): string {
  if (!unix) return "never played";
  const now = Math.floor(Date.now() / 1000);
  const d = Math.max(0, now - unix);
  if (d < 60) return "just now";
  if (d < 3600) return `${Math.floor(d / 60)} min ago`;
  if (d < 86400) return `${Math.floor(d / 3600)} h ago`;
  if (d < 86400 * 7) return `${Math.floor(d / 86400)} days ago`;
  return "last week";
}

export function hashPath(s: string): number {
  let h = 0;
  for (let i = 0; i < s.length; i++) {
    h = (h * 31 + s.charCodeAt(i)) >>> 0;
  }
  return h;
}

export function thumbGradient(path: string): string {
  const seed = hashPath(path);
  return `linear-gradient(135deg, hsl(${seed % 360} 85% 60%), hsl(${(seed + 80) % 360} 70% 38%))`;
}

export function trimTitle(title: string, max = 18): string {
  if (title.length <= max) return title;
  return `${title.slice(0, max - 1)}…`;
}

export function trimTitleWide(title: string, max = 48): string {
  if (title.length <= max) return title;
  return `${title.slice(0, max - 1)}…`;
}

export function getLastPlayedRom(roms: RomSummary[]): RomSummary | null {
  let best: RomSummary | null = null;
  let bestTs = 0;
  for (const rom of roms) {
    const ts = rom.lastPlayedUnixSecs ?? 0;
    if (ts > bestTs) {
      bestTs = ts;
      best = rom;
    }
  }
  return best;
}
