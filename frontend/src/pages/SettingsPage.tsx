import { useEffect, useState } from "react";
import type { RomSummary } from "../types/launcher";
import { GameBoyControls } from "../components/controls/GameBoyControls";
import { Card } from "../components/ui/card";
import { Button } from "../components/ui/button";
import { Spinner } from "../components/ui/Spinner";

type Props = {
  selectedRom: RomSummary | null;
  onSaveSettings: (input: {
    scale: number;
    autosaveEnabled: boolean;
    videoFilter: "Sharp" | "Smooth";
    audioMode: "Balanced" | "LowLatency";
    controlsEnv?: string;
  }) => void;
  isSaving: boolean;
};

type KeybindRow = {
  label: string;
  value: string;
};

export function SettingsPage({ selectedRom, onSaveSettings, isSaving }: Props) {
  const [scale, setScale] = useState(4);
  const [autosaveEnabled, setAutosaveEnabled] = useState(true);
  const [videoFilter, setVideoFilter] = useState<"Sharp" | "Smooth">("Sharp");
  const [audioMode, setAudioMode] = useState<"Balanced" | "LowLatency">("Balanced");
  const [rewindEnabled, setRewindEnabled] = useState(true);

  useEffect(() => {
    if (selectedRom) {
      setScale(selectedRom.profile.scale);
      setAutosaveEnabled(selectedRom.profile.autosaveEnabled);
      setVideoFilter(selectedRom.profile.videoFilter);
      setAudioMode(selectedRom.profile.audioMode);
    }
  }, [selectedRom]);

  const controls = parseControlMap(selectedRom?.profile.controlsEnv);
  const keybinds: KeybindRow[] = controls.rows;

  return (
    <Card id="content" className="settingsPage">
      {!selectedRom ? (
        <p className="muted">Import a ROM to edit per-game settings.</p>
      ) : (
        <>
          <div className="settingsHeaderRow">
            <div>
              <h3 className="settingsHeader">Runtime Profile</h3>
              <p className="muted settingsSub">{selectedRom.name} · {selectedRom.extension.toUpperCase()}</p>
            </div>
            <span className="listPlatform">LIVE</span>
          </div>
          <div className="settingsGrid">
            <div className="settingsPanel">
              <div className="settingRow">
                <div>
                  <div className="settingLabel">Autosave</div>
                  <div className="muted">Every 30 seconds while playing</div>
                </div>
                <button
                  className={`toggle ${autosaveEnabled ? "on" : ""}`}
                  disabled={isSaving}
                  onClick={() => setAutosaveEnabled((v) => !v)}
                />
              </div>

              <div className="settingRow">
                <div>
                  <div className="settingLabel">Rewind buffer</div>
                  <div className="muted">Hold last 60 seconds</div>
                </div>
                <button
                  className={`toggle ${rewindEnabled ? "on" : ""}`}
                  disabled={isSaving}
                  onClick={() => setRewindEnabled((v) => !v)}
                />
              </div>

              <div className="settingRow">
                <div>
                  <div className="settingLabel">Display filter</div>
                  <div className="muted">Sharp pixels or light smoothing when scaled up</div>
                </div>
                <div className="segment">
                  <button className={videoFilter === "Sharp" ? "active" : ""} onClick={() => setVideoFilter("Sharp")} disabled={isSaving}>
                    Sharp
                  </button>
                  <button className={videoFilter === "Smooth" ? "active" : ""} onClick={() => setVideoFilter("Smooth")} disabled={isSaving}>
                    Smooth
                  </button>
                </div>
              </div>

              <div className="settingRow">
                <div className="settingLabel">Display scale</div>
                <div className="scaleRow">
                  <input
                    type="range"
                    min={1}
                    max={10}
                    value={scale}
                    disabled={isSaving}
                    onChange={(e) => setScale(Number(e.target.value))}
                  />
                  <span className="muted">{scale}x</span>
                </div>
              </div>

              <div className="settingRow">
                <div className="settingLabel">Audio mode</div>
                <div className="segment">
                  <button className={audioMode === "LowLatency" ? "active" : ""} onClick={() => setAudioMode("LowLatency")} disabled={isSaving}>
                    Mono
                  </button>
                  <button className={audioMode === "Balanced" ? "active" : ""} onClick={() => setAudioMode("Balanced")} disabled={isSaving}>
                    Stereo
                  </button>
                </div>
              </div>
            </div>

            <div className="settingsPanel">
              <div className="keybindHeader">Keybinds</div>
              <div className="keybindList">
                {keybinds.map((k) => (
                  <div key={k.label} className="keybindRow">
                    <span>{k.label}</span>
                    <span className="keyChip">⌨ {k.value}</span>
                  </div>
                ))}
              </div>
              <GameBoyControls
                a={controls.a}
                b={controls.b}
                start={controls.start}
                select={controls.select}
                dpad={controls.dpad}
              />
            </div>
          </div>

          <div className="toolbar settingsActions">
            <Button
              onClick={() =>
                onSaveSettings({
                  scale,
                  autosaveEnabled,
                  videoFilter,
                  audioMode,
                })
              }
              disabled={isSaving}
            >
              {isSaving ? (
                <>
                  <Spinner />
                  Saving…
                </>
              ) : (
                "Save"
              )}
            </Button>
          </div>
        </>
      )}
    </Card>
  );
}

function parseControlMap(env?: string) {
  const defaults = ["X", "Z", "Enter", "Shift", "↑", "↓", "←", "→", "Space"];
  const values = (env ?? "")
    .split(",")
    .map((s) => s.trim())
    .filter(Boolean);
  const keys = values.length >= 9 ? values : defaults;
  const a = label(keys[0]);
  const b = label(keys[1]);
  const start = label(keys[2]);
  const select = label(keys[3]);
  return {
    a,
    b,
    start,
    select,
    dpad: {
      up: label(keys[4]),
      down: label(keys[5]),
      left: label(keys[6]),
      right: label(keys[7]),
    },
    rows: [
      { label: "A button", value: a },
      { label: "B button", value: b },
      { label: "Start", value: start },
      { label: "Select", value: select },
      { label: "Fast forward", value: label(keys[8]) },
      { label: "Save state", value: "F5" },
    ],
  };
}

function label(s: string): string {
  const m: Record<string, string> = {
    return: "Enter",
    lshift: "Shift",
    rshift: "Shift",
    space: "Space",
    up: "↑",
    down: "↓",
    left: "←",
    right: "→",
  };
  const lower = s.toLowerCase();
  if (m[lower]) return m[lower];
  if (s.length === 1) return s.toUpperCase();
  return s;
}
