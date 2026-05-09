import type { RomSummary } from "../types/launcher";
import { Card } from "../components/ui/card";
import { Badge } from "../components/ui/badge";
import { TableRow } from "../components/ui/table";
import { Button } from "../components/ui/button";
import { Spinner } from "../components/ui/Spinner";

type Props = {
  recent: RomSummary[];
  onLaunch: (path: string) => void;
  busyLaunchPath: string | null;
};

export function RecentPage({ recent, onLaunch, busyLaunchPath }: Props) {
  return (
    <Card id="content">
      {recent.length === 0 ? <p className="muted">No recent games yet.</p> : null}
      {recent.map((rom) => (
        <TableRow key={rom.path}>
          <div>{rom.name}</div>
          <div>
            <Badge>{rom.extension.toUpperCase()}</Badge>
          </div>
          <div>
            <span className="muted">
              {rom.lastPlayedUnixSecs
                ? new Date(rom.lastPlayedUnixSecs * 1000).toLocaleString()
                : "-"}
            </span>
          </div>
          <div>
            <Button onClick={() => onLaunch(rom.path)} disabled={busyLaunchPath === rom.path}>
              {busyLaunchPath === rom.path ? (
                <>
                  <Spinner />
                  Opening…
                </>
              ) : (
                "Play"
              )}
            </Button>
          </div>
        </TableRow>
      ))}
    </Card>
  );
}
