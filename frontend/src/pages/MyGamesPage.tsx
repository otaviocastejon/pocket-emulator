import type { RomSummary } from "../types/launcher";
import { Button } from "../components/ui/button";
import { Card } from "../components/ui/card";
import { Spinner } from "../components/ui/Spinner";

type Props = {
  roms: RomSummary[];
  onAddRom: () => void;
  onToggleFavorite: (path: string) => void;
  onLaunch: (path: string) => void;
  isAddingRom: boolean;
  busyFavoritePath: string | null;
  busyLaunchPath: string | null;
};

export function MyGamesPage({
  roms,
  onAddRom,
  onToggleFavorite,
  onLaunch,
  isAddingRom,
  busyFavoritePath,
  busyLaunchPath,
}: Props) {
  const totalPlaytime = `${Math.max(roms.length * 8, 12)}h total playtime`;

  return (
    <Card id="content" className="gamesPage">
      <div className="gamesHero">
        <div>
          <h1 className="gamesTitle">My Games</h1>
          <p className="muted">{roms.length} ROMS · {totalPlaytime}</p>
        </div>
        <Button onClick={onAddRom} disabled={isAddingRom} className="addRomCta">
          {isAddingRom ? (
            <>
              <Spinner />
              Adding…
            </>
          ) : (
            "+ Add ROM"
          )}
        </Button>
      </div>

      <div className="gamesList">
        {roms.map((rom) => {
          const rel = formatRelative(rom.lastPlayedUnixSecs);
          const seed = hash(rom.path);
          return (
            <article key={rom.path} className="gameListRow">
              <button className="rowMain" onClick={() => onLaunch(rom.path)}>
                <div
                  className="thumbCover"
                  style={{
                    background: `linear-gradient(135deg, hsl(${seed % 360} 85% 60%), hsl(${(seed + 80) % 360} 70% 38%))`,
                  }}
                >
                  <div className="thumbSpine">GAME BOY</div>
                  <div className="thumbName">{trimTitle(rom.name)}</div>
                </div>
                <div className="rowText">
                  <div className="gameName">{trimTitleWide(rom.name)}</div>
                  <div className="muted">{rel} · {Math.max((seed % 24) + 1, 1)}h {(seed % 59) + 1}m</div>
                </div>
              </button>

              <div className="rowActions">
                <span className="listPlatform">{rom.extension.toUpperCase()}</span>
                <Button
                  variant="icon"
                  className="iconCircle"
                  disabled={busyFavoritePath === rom.path}
                  onClick={(e) => {
                    e.stopPropagation();
                    onToggleFavorite(rom.path);
                  }}
                >
                  {busyFavoritePath === rom.path ? <Spinner /> : rom.favorite ? "★" : "☆"}
                </Button>
                <Button
                  className="playPill"
                  disabled={busyLaunchPath === rom.path}
                  onClick={(e) => {
                    e.stopPropagation();
                    onLaunch(rom.path);
                  }}
                >
                  {busyLaunchPath === rom.path ? <Spinner /> : "▷ Play"}
                </Button>
                <Button variant="icon" className="iconCircle">⋯</Button>
              </div>
            </article>
          );
        })}
      </div>
    </Card>
  );
}

function formatRelative(unix: number | null): string {
  if (!unix) return "unknown";
  const now = Math.floor(Date.now() / 1000);
  const d = Math.max(0, now - unix);
  if (d < 60) return "just now";
  if (d < 3600) return `${Math.floor(d / 60)} min ago`;
  if (d < 86400) return `${Math.floor(d / 3600)} h ago`;
  if (d < 86400 * 7) return `${Math.floor(d / 86400)} days ago`;
  return "last week";
}

function hash(s: string): number {
  let h = 0;
  for (let i = 0; i < s.length; i++) {
    h = (h * 31 + s.charCodeAt(i)) >>> 0;
  }
  return h;
}

function trimTitle(title: string): string {
  if (title.length <= 18) return title;
  return `${title.slice(0, 17)}…`;
}

function trimTitleWide(title: string): string {
  if (title.length <= 48) return title;
  return `${title.slice(0, 47)}…`;
}
