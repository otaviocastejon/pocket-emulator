export type TabKey = "games" | "saves" | "settings" | "getroms";

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

