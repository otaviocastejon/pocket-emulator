export type TabKey =
  | "games"
  | "saves"
  | "settings"
  | "getroms"
  | "library-all"
  | "library-gb"
  | "library-gbc";

export function isLibraryTab(tab: TabKey): boolean {
  return tab === "library-all" || tab === "library-gb" || tab === "library-gbc";
}

export function libraryTabTitle(tab: TabKey): string | null {
  switch (tab) {
    case "library-all":
      return "All ROMs";
    case "library-gb":
      return "Game Boy";
    case "library-gbc":
      return "Game Boy Color";
    default:
      return null;
  }
}

export function filterRomsForTab(roms: RomSummary[], tab: TabKey): RomSummary[] {
  switch (tab) {
    case "library-gb":
      return roms.filter((rom) => rom.extension === "gb");
    case "library-gbc":
      return roms.filter((rom) => rom.extension === "gbc");
    case "library-all":
    case "games":
      return roms;
    default:
      return roms;
  }
}

export type VideoFilter = "Sharp" | "Smooth";
export type AudioMode = "Balanced" | "LowLatency";

export type ProfileSummary = {
  scale: number;
  controlsEnv: string;
  autosaveEnabled: boolean;
  videoFilter: VideoFilter;
  audioMode: AudioMode;
};

export type RomSummary = {
  path: string;
  name: string;
  extension: string;
  favorite: boolean;
  lastPlayedUnixSecs: number | null;
  profile: ProfileSummary;
};

export type SaveFileSummary = {
  romPath: string;
  romName: string;
  savePath: string;
  kind: "save" | "backup";
  sizeBytes: number;
  modifiedUnixSecs: number | null;
};

