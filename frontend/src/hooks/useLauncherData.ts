import { useEffect, useMemo, useState } from "react";
import type { RomSummary, SaveFileSummary } from "../types/launcher";
import { tauriApi } from "../api/tauri";

export function useLauncherData() {
  const [roms, setRoms] = useState<RomSummary[]>([]);
  const [loading, setLoading] = useState(true);
  const [savesLoading, setSavesLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedPath, setSelectedPath] = useState<string | null>(null);
  const [saveFiles, setSaveFiles] = useState<SaveFileSummary[]>([]);

  useEffect(() => {
    void refreshRoms();
    void refreshSaveFiles();
  }, []);

  const selectedRom = useMemo(
    () => roms.find((r) => r.path === selectedPath) ?? roms[0] ?? null,
    [roms, selectedPath],
  );

  async function refreshRoms(): Promise<boolean> {
    setLoading(true);
    setError(null);
    try {
      const data = await tauriApi.listRoms();
      setRoms(data);
      if (data.length > 0 && !selectedPath) {
        setSelectedPath(data[0].path);
      }
      return true;
    } catch (e) {
      setError(String(e));
      return false;
    } finally {
      setLoading(false);
    }
  }

  async function addRom(): Promise<boolean> {
    try {
      await tauriApi.pickAndImportRom();
      const romsOk = await refreshRoms();
      const savesOk = await refreshSaveFiles();
      return romsOk && savesOk;
    } catch (e) {
      setError(String(e));
      return false;
    }
  }

  async function launch(path: string): Promise<boolean> {
    try {
      await tauriApi.launchRom(path);
      void refreshSaveFiles();
      return true;
    } catch (e) {
      setError(String(e));
      return false;
    }
  }

  async function toggleFavorite(path: string): Promise<boolean> {
    const prev = roms;
    setRoms((current) =>
      current.map((rom) =>
        rom.path === path ? { ...rom, favorite: !rom.favorite } : rom,
      ),
    );
    try {
      await tauriApi.toggleFavorite(path);
      await refreshRoms();
      return true;
    } catch (e) {
      setRoms(prev);
      setError(String(e));
      return false;
    }
  }

  async function refreshSaveFiles(): Promise<boolean> {
    setSavesLoading(true);
    try {
      const data = await tauriApi.listSaveFiles();
      setSaveFiles(data);
      return true;
    } catch (e) {
      setError(String(e));
      return false;
    } finally {
      setSavesLoading(false);
    }
  }

  async function exportSave(savePath: string): Promise<boolean> {
    try {
      await tauriApi.exportSaveFile(savePath);
      return true;
    } catch (e) {
      setError(String(e));
      return false;
    }
  }

  async function deleteSave(savePath: string): Promise<boolean> {
    try {
      await tauriApi.deleteSaveFile(savePath);
      await refreshSaveFiles();
      return true;
    } catch (e) {
      setError(String(e));
      return false;
    }
  }

  async function deleteSavesForRom(romPath: string): Promise<number | null> {
    try {
      const deleted = await tauriApi.deleteSavesForRom(romPath);
      await refreshSaveFiles();
      return deleted;
    } catch (e) {
      setError(String(e));
      return null;
    }
  }

  async function saveScale(scale: number): Promise<boolean> {
    if (!selectedRom) return false;
    try {
      await tauriApi.saveProfile({
        path: selectedRom.path,
        scale,
        controlsEnv: selectedRom.profile.controlsEnv,
        autosaveEnabled: selectedRom.profile.autosaveEnabled,
        videoFilter: selectedRom.profile.videoFilter,
        audioMode: selectedRom.profile.audioMode,
      });
      await refreshRoms();
      return true;
    } catch (e) {
      setError(String(e));
      return false;
    }
  }

  async function saveSettings(input: {
    scale: number;
    autosaveEnabled: boolean;
    videoFilter: "Sharp" | "Smooth";
    audioMode: "Balanced" | "LowLatency";
    controlsEnv?: string;
  }): Promise<boolean> {
    if (!selectedRom) return false;
    try {
      await tauriApi.saveProfile({
        path: selectedRom.path,
        scale: input.scale,
        controlsEnv: input.controlsEnv ?? selectedRom.profile.controlsEnv,
        autosaveEnabled: input.autosaveEnabled,
        videoFilter: input.videoFilter,
        audioMode: input.audioMode,
      });
      await refreshRoms();
      return true;
    } catch (e) {
      setError(String(e));
      return false;
    }
  }

  async function openCatalog(): Promise<boolean> {
    try {
      await tauriApi.openRomCatalog();
      return true;
    } catch (e) {
      setError(String(e));
      return false;
    }
  }

  return {
    roms,
    selectedRom,
    selectedPath,
    setSelectedPath,
    loading,
    savesLoading,
    saveFiles,
    error,
    setError,
    refreshRoms,
    addRom,
    launch,
    toggleFavorite,
    saveScale,
    saveSettings,
    openCatalog,
    refreshSaveFiles,
    exportSave,
    deleteSave,
    deleteSavesForRom,
    clearError: () => setError(null),
  };
}
