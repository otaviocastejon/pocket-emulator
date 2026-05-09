import { Card } from "../components/ui/card";
import { Button } from "../components/ui/button";
import { Spinner } from "../components/ui/Spinner";

type Props = {
  onOpenCatalog: () => void;
  isOpening: boolean;
};

export function GetRomsPage({ onOpenCatalog, isOpening }: Props) {
  return (
    <Card id="content" className="catalogPage">
      <div className="settingsHeaderRow">
        <div>
          <h2 className="settingsHeader">Get ROMs</h2>
          <p className="muted settingsSub">Use legal backups from cartridges you own.</p>
        </div>
        <span className="listPlatform">WEB</span>
      </div>
      <div className="settingsPanel">
        <div className="settingRow">
          <div>
            <div className="settingLabel">ROM Catalog</div>
            <div className="muted">Browse Game Boy and Game Boy Color libraries in your browser.</div>
          </div>
          <Button onClick={onOpenCatalog} disabled={isOpening}>
            {isOpening ? (
              <>
                <Spinner />
                Opening…
              </>
            ) : (
              "Open ROM Catalog"
            )}
          </Button>
        </div>
        <div className="catalogLegal muted">
          Tip: import downloaded files from `My Games` using the `Add ROM` button.
        </div>
      </div>
    </Card>
  );
}
