import type { RomSummary } from "../types/launcher";
import { Button } from "../components/ui/button";
import { Card } from "../components/ui/card";
import { Spinner } from "../components/ui/Spinner";
import {
  formatRelative,
  hashPath,
  thumbGradient,
  trimTitle,
  trimTitleWide,
} from "../lib/romDisplay";

type Props = {
  title?: string;
  roms: RomSummary[];
  onAddRom: () => void;
  onToggleFavorite: (path: string) => void;
  onLaunch: (path: string) => void;
  isAddingRom: boolean;
  busyFavoritePath: string | null;
  busyLaunchPath: string | null;
};

export function MyGamesPage({
  title = "My Games",
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
          <h1 className="gamesTitle">{title}</h1>
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
        {roms.length === 0 ? (
          <p className="muted gamesEmpty">No ROMs in this library yet. Use Add ROM to import one.</p>
        ) : null}
        {roms.map((rom) => {
          const rel = formatRelative(rom.lastPlayedUnixSecs);
          const seed = hashPath(rom.path);
          return (
            <article key={rom.path} className="gameListRow">
              <button className="rowMain" onClick={() => onLaunch(rom.path)}>
                <div className="thumbCover" style={{ background: thumbGradient(rom.path) }}>
                  <div className="thumbSpine">GAME BOY</div>
                  <div className="thumbName">{trimTitle(rom.name)}</div>
                </div>
                <div className="rowText">
                  <div className="gameName">{trimTitleWide(rom.name)}</div>
                  <div className="muted">{rel} · {Math.max((seed % 24) + 1, 1)}h {(seed % 59) + 1}m</div>
                </div>
              </button>

              <div className="rowActions">
                <span className={`listPlatform listPlatform--${rom.extension}`}>
                  {rom.extension.toUpperCase()}
                </span>
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
