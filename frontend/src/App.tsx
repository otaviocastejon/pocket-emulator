import { useState } from "react";
import { PageHeader } from "./components/layout/PageHeader";
import { MyGamesPage } from "./pages/MyGamesPage";
import { SavesPage } from "./pages/SavesPage";
import { SettingsPage } from "./pages/SettingsPage";
import { GetRomsPage } from "./pages/GetRomsPage";
import { useLauncherData } from "./hooks/useLauncherData";
import { useToast } from "./hooks/useToast";
import { ToastStack } from "./components/feedback/ToastStack";
import { type TabKey } from "./types/launcher";

export function App() {
  const [activeTab, setActiveTab] = useState<TabKey>("games");
  const [isAddingRom, setIsAddingRom] = useState(false);
  const [isSavingSettings, setIsSavingSettings] = useState(false);
  const [isOpeningCatalog, setIsOpeningCatalog] = useState(false);
  const [busyFavoritePath, setBusyFavoritePath] = useState<string | null>(null);
  const [busyLaunchPath, setBusyLaunchPath] = useState<string | null>(null);
  const [busyExportSavePath, setBusyExportSavePath] = useState<string | null>(null);
  const [busyDeleteSavePath, setBusyDeleteSavePath] = useState<string | null>(null);
  const [busyDeleteRomPath, setBusyDeleteRomPath] = useState<string | null>(null);
  const {
    roms,
    saveFiles,
    selectedRom,
    loading,
    savesLoading,
    error,
    clearError,
    addRom,
    launch,
    toggleFavorite,
    saveSettings,
    openCatalog,
    exportSave,
    deleteSave,
    deleteSavesForRom,
  } = useLauncherData();
  const { toasts, pushToast, dismiss } = useToast();

  async function onAddRom() {
    setIsAddingRom(true);
    const ok = await addRom();
    setIsAddingRom(false);
    if (ok) {
      pushToast({ title: "ROM added", message: "Imported into your library", tone: "success" });
    }
  }

  async function onToggleFavorite(path: string) {
    setBusyFavoritePath(path);
    const ok = await toggleFavorite(path);
    setBusyFavoritePath(null);
    if (ok) {
      pushToast({ title: "Favorite updated", tone: "success", ttlMs: 1800 });
    }
  }

  async function onSaveSettings(input: {
    scale: number;
    autosaveEnabled: boolean;
    videoFilter: "Sharp" | "Smooth";
    audioMode: "Balanced" | "LowLatency";
    controlsEnv?: string;
  }) {
    setIsSavingSettings(true);
    const ok = await saveSettings(input);
    setIsSavingSettings(false);
    if (ok) {
      pushToast({ title: "Settings saved", tone: "success", ttlMs: 1800 });
    }
  }

  async function onOpenCatalog() {
    setIsOpeningCatalog(true);
    const ok = await openCatalog();
    setIsOpeningCatalog(false);
    if (ok) {
      pushToast({ title: "Catalog opened", message: "Check your browser", tone: "info" });
    }
  }

  async function onLaunch(path: string) {
    setBusyLaunchPath(path);
    const ok = await launch(path);
    setBusyLaunchPath(null);
    if (ok) {
      pushToast({ title: "Launching game", message: "Opening native game window", tone: "info", ttlMs: 1400 });
    }
  }

  async function onExportSave(savePath: string) {
    setBusyExportSavePath(savePath);
    const ok = await exportSave(savePath);
    setBusyExportSavePath(null);
    if (ok) {
      pushToast({ title: "Save exported", message: "Choose destination completed", tone: "success" });
    }
  }

  async function onDeleteSave(savePath: string) {
    const confirmed = window.confirm("Delete this save file?");
    if (!confirmed) return;
    setBusyDeleteSavePath(savePath);
    const ok = await deleteSave(savePath);
    setBusyDeleteSavePath(null);
    if (ok) {
      pushToast({ title: "Save deleted", tone: "success", ttlMs: 1800 });
    }
  }

  async function onDeleteAllForRom(romPath: string) {
    const confirmed = window.confirm("Delete all saves for this game (including backup)?");
    if (!confirmed) return;
    setBusyDeleteRomPath(romPath);
    const deletedCount = await deleteSavesForRom(romPath);
    setBusyDeleteRomPath(null);
    if (deletedCount != null) {
      pushToast({
        title: "Game saves deleted",
        message: deletedCount > 0 ? `Deleted ${deletedCount} file(s)` : "No files found to delete",
        tone: "success",
        ttlMs: 2200,
      });
    }
  }

  return (
    <div id="app">
      <aside id="sidebar">
        <div className="brand">LAUNCHER</div>
        <button className={`navRow ${activeTab === "games" ? "active" : ""}`} onClick={() => setActiveTab("games")}>
          🎮 My Games
        </button>
        <button className={`navRow ${activeTab === "saves" ? "active" : ""}`} onClick={() => setActiveTab("saves")}>
          💾 Saves
        </button>
        <button className={`navRow ${activeTab === "settings" ? "active" : ""}`} onClick={() => setActiveTab("settings")}>
          ⚙ Settings
        </button>
        <button className={`navRow ${activeTab === "getroms" ? "active" : ""}`} onClick={() => setActiveTab("getroms")}>
          🌐 Get ROMs
        </button>
        <div className="sidebarDivider" />
        <div className="sidebarLabel">LIBRARY</div>
        <div className="navRow navRow-muted static">📁 All ROMs</div>
        <div className="navRow navRow-muted static">📁 Game Boy</div>
        <div className="navRow navRow-muted static">📁 Game Boy Color</div>
      </aside>
      <main id="main">
        {activeTab !== "games" ? (
          <PageHeader activeTab={activeTab} romCount={roms.length} savesCount={saveFiles.length} />
        ) : null}

        {error ? (
          <div className="error">
            <span>{error}</span>
            <button className="errorClose" onClick={clearError}>
              Dismiss
            </button>
          </div>
        ) : null}
        {loading ? <div className="muted">Loading…</div> : null}

        {!loading && activeTab === "games" ? (
          <MyGamesPage
            roms={roms}
            onAddRom={() => void onAddRom()}
            onToggleFavorite={(path) => void onToggleFavorite(path)}
            onLaunch={(path) => void onLaunch(path)}
            isAddingRom={isAddingRom}
            busyFavoritePath={busyFavoritePath}
            busyLaunchPath={busyLaunchPath}
          />
        ) : null}

        {!loading && activeTab === "saves" ? (
          <SavesPage
            saves={saveFiles}
            isLoading={savesLoading}
            busyExportPath={busyExportSavePath}
            busyDeletePath={busyDeleteSavePath}
            busyDeleteRomPath={busyDeleteRomPath}
            onExport={(path) => void onExportSave(path)}
            onDelete={(path) => void onDeleteSave(path)}
            onDeleteAllForRom={(path) => void onDeleteAllForRom(path)}
          />
        ) : null}

        {!loading && activeTab === "settings" ? (
          <SettingsPage
            selectedRom={selectedRom}
            onSaveSettings={(input) => void onSaveSettings(input)}
            isSaving={isSavingSettings}
          />
        ) : null}

        {!loading && activeTab === "getroms" ? (
          <GetRomsPage onOpenCatalog={() => void onOpenCatalog()} isOpening={isOpeningCatalog} />
        ) : null}
      </main>
      <ToastStack toasts={toasts} onDismiss={dismiss} />
    </div>
  );
}
