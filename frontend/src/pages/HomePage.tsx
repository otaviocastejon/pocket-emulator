import type { RomSummary } from "../types/launcher";
import { Button } from "../components/ui/button";
import { Card } from "../components/ui/card";
import { Spinner } from "../components/ui/Spinner";
import {
  formatRelative,
  getLastPlayedRom,
  thumbGradient,
  trimTitle,
  trimTitleWide,
} from "../lib/romDisplay";
import logoUrl from "../../../icons/icon.png";

type Props = {
  roms: RomSummary[];
  onAddRom: () => void;
  onLaunch: (path: string) => void;
  onBrowseLibrary: () => void;
  isAddingRom: boolean;
  busyLaunchPath: string | null;
};

export function HomePage({
  roms,
  onAddRom,
  onLaunch,
  onBrowseLibrary,
  isAddingRom,
  busyLaunchPath,
}: Props) {
  const lastPlayed = getLastPlayedRom(roms);

  return (
    <Card id="content" className="homePage">
      <header className="homeHero">
        <img className="homeLogo" src={logoUrl} alt="PocketEmulator" width={72} height={72} />
        <div className="homeHeroText">
          <p className="homeEyebrow">Welcome to</p>
          <h1 className="homeTitle">PocketEmulator</h1>
          <p className="homeTagline">
            A Game Boy and Game Boy Color emulator with a desktop launcher for your ROM library,
            saves, and per-game settings.
          </p>
          <p className="homeOpenSource">
            <span className="homeOssBadge">Free &amp; open source</span>
            Use, study, and contribute on{" "}
            <a
              className="homeOssLink"
              href="https://github.com/otaviocastejon/pocket-emulator"
              target="_blank"
              rel="noreferrer"
            >
              GitHub
            </a>
            .
          </p>
        </div>
      </header>

      <section className="homeSection">
        <h2 className="homeSectionTitle">Continue playing</h2>
        {lastPlayed ? (
          <article className="homeContinueCard">
            <div
              className="thumbCover homeContinueThumb"
              style={{ background: thumbGradient(lastPlayed.path) }}
            >
              <div className="thumbSpine">GAME BOY</div>
              <div className="thumbName">{trimTitle(lastPlayed.name)}</div>
            </div>
            <div className="homeContinueBody">
              <div className="gameName">{trimTitleWide(lastPlayed.name)}</div>
              <p className="muted homeContinueMeta">
                {formatRelative(lastPlayed.lastPlayedUnixSecs)}
                <span className={`listPlatform listPlatform--${lastPlayed.extension}`}>
                  {lastPlayed.extension.toUpperCase()}
                </span>
              </p>
            </div>
            <Button
              className="playPill homeContinuePlay"
              disabled={busyLaunchPath === lastPlayed.path}
              onClick={() => onLaunch(lastPlayed.path)}
            >
              {busyLaunchPath === lastPlayed.path ? <Spinner /> : "▷ Resume"}
            </Button>
          </article>
        ) : (
          <p className="muted homeContinueEmpty">
            No recent game yet. Add a ROM you own, then press Play to start.
          </p>
        )}
      </section>

      <section className="homeSection homeAbout">
        <h2 className="homeSectionTitle">How to use</h2>
        <ol className="homeSteps">
          <li>
            <strong>Add ROM</strong> — import a <code>.gb</code> or <code>.gbc</code> backup from cartridges you own.
          </li>
          <li>
            <strong>Play</strong> — launch from here or browse your library in the sidebar (All ROMs, Game Boy, Game Boy Color).
          </li>
          <li>
            <strong>Save &amp; settings</strong> — use Saves and Settings; in-game use arrow keys, Z/X, Enter, Shift (see README for shortcuts).
          </li>
        </ol>
        <p className="muted homeNote">
          Audio is not implemented yet. The gameplay window opens separately while this launcher stays open.
        </p>
      </section>

      <footer className="homeActions">
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
        <Button variant="secondary" onClick={onBrowseLibrary}>
          Browse library
        </Button>
        <p className="homeSignature">Made by Joao Otavio Castejon</p>
      </footer>
    </Card>
  );
}
