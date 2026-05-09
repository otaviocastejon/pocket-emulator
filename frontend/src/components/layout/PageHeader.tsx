import type { TabKey } from "../../types/launcher";
import { Card } from "../ui/card";

type Props = {
  activeTab: TabKey;
  romCount: number;
  savesCount: number;
};

export function PageHeader({ activeTab, romCount, savesCount }: Props) {
  return (
    <Card id="header">
      <div className="eyebrow">LAUNCHER · {headerText(activeTab).toUpperCase()}</div>
      <h1>{headerText(activeTab)}</h1>
      <div className="muted">{headerSub(activeTab, romCount, savesCount)}</div>
    </Card>
  );
}

function headerText(tab: TabKey): string {
  switch (tab) {
    case "games":
      return "My Games";
    case "saves":
      return "Saves";
    case "settings":
      return "Settings";
    case "getroms":
      return "Get ROMs";
  }
}

function headerSub(tab: TabKey, romCount: number, savesCount: number): string {
  switch (tab) {
    case "games":
      return `${romCount} ROMs in library`;
    case "saves":
      return `${savesCount} save files available`;
    case "settings":
      return "Per-game runtime profile";
    case "getroms":
      return "Open browser catalog and import ROMs";
  }
}
