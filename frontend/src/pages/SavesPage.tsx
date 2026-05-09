import type { SaveFileSummary } from "../types/launcher";
import { Card } from "../components/ui/card";
import { Button } from "../components/ui/button";
import { Spinner } from "../components/ui/Spinner";

type Props = {
  saves: SaveFileSummary[];
  isLoading: boolean;
  busyExportPath: string | null;
  busyDeletePath: string | null;
  busyDeleteRomPath: string | null;
  onExport: (savePath: string) => void;
  onDelete: (savePath: string) => void;
  onDeleteAllForRom: (romPath: string) => void;
};

export function SavesPage({
  saves,
  isLoading,
  busyExportPath,
  busyDeletePath,
  busyDeleteRomPath,
  onExport,
  onDelete,
  onDeleteAllForRom,
}: Props) {
  return (
    <Card id="content">
      <div className="settingsHeaderRow">
        <div>
          <h2 className="settingsHeader">Save Files</h2>
          <p className="muted">View, export, and delete your `.sav` files</p>
        </div>
      </div>

      {isLoading ? <div className="muted">Loading saves…</div> : null}
      {!isLoading && saves.length === 0 ? <div className="muted">No save files found yet.</div> : null}

      {!isLoading && saves.length > 0 ? (
        <div className="savesList">
          {saves.map((save) => (
            <div className="saveRow" key={save.savePath}>
              <div>
                <div className="saveTitle">{save.romName}</div>
                <div className="muted">
                  {save.kind === "backup" ? "Backup" : "Save"} · {formatBytes(save.sizeBytes)} ·{" "}
                  {formatRelative(save.modifiedUnixSecs)}
                </div>
                <div className="savePath">{save.savePath}</div>
              </div>
              <div className="saveActions">
                <Button
                  variant="secondary"
                  disabled={busyExportPath === save.savePath}
                  onClick={() => onExport(save.savePath)}
                >
                  {busyExportPath === save.savePath ? (
                    <>
                      <Spinner /> Exporting…
                    </>
                  ) : (
                    "Download"
                  )}
                </Button>
                <Button
                  variant="destructive"
                  disabled={busyDeletePath === save.savePath}
                  onClick={() => onDelete(save.savePath)}
                >
                  {busyDeletePath === save.savePath ? (
                    <>
                      <Spinner /> Deleting…
                    </>
                  ) : (
                    "Delete"
                  )}
                </Button>
                <Button
                  variant="ghost"
                  disabled={!save.romPath || busyDeleteRomPath === save.romPath}
                  onClick={() => save.romPath && onDeleteAllForRom(save.romPath)}
                >
                  {busyDeleteRomPath === save.romPath ? (
                    <>
                      <Spinner /> Deleting all…
                    </>
                  ) : (
                    "Delete all (game)"
                  )}
                </Button>
              </div>
            </div>
          ))}
        </div>
      ) : null}
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

function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}
