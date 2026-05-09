import { invoke } from "@tauri-apps/api/core";
import type { RomSummary, SaveFileSummary } from "../types/launcher";

export type SaveProfileRequest = {
  path: string;
  scale: number;
  controlsEnv: string;
  autosaveEnabled: boolean;
  videoFilter: "Sharp" | "Smooth";
  audioMode: "Balanced" | "LowLatency";
};

export const tauriApi = {
  listRoms: () => invoke<RomSummary[]>("list_roms"),
  listSaveFiles: () => invoke<SaveFileSummary[]>("list_save_files"),
  pickAndImportRom: () => invoke("pick_and_import_rom"),
  launchRom: (path: string) => invoke("launch_rom", { request: { path } }),
  toggleFavorite: (path: string) => invoke<boolean>("toggle_favorite", { path }),
  saveProfile: (request: SaveProfileRequest) => invoke("save_profile", { request }),
  openRomCatalog: () => invoke("open_rom_catalog"),
  exportSaveFile: (savePath: string) => invoke("export_save_file", { savePath }),
  deleteSaveFile: (savePath: string) => invoke("delete_save_file", { savePath }),
  deleteSavesForRom: (romPath: string) => invoke<number>("delete_saves_for_rom", { romPath }),
};
