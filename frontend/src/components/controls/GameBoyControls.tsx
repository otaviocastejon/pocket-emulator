type Props = {
  a: string;
  b: string;
  start: string;
  select: string;
  dpad: { up: string; down: string; left: string; right: string };
};

export function GameBoyControls({ a, b, start, select, dpad }: Props) {
  return (
    <div className="gbControls" aria-hidden="true">
      <p className="gbControlsCaption muted">Control layout</p>
      <div className="gbControlsPlate">
        <div className="gbControlsLayout">
          <div className="gbDpad">
            <span className="gbDpadKey gbDpadKey-up">{dpad.up}</span>
            <span className="gbDpadBtn gbDpad-up" />
            <span className="gbDpadBtn gbDpad-left" />
            <span className="gbDpadCenter" />
            <span className="gbDpadBtn gbDpad-right" />
            <span className="gbDpadBtn gbDpad-down" />
            <span className="gbDpadKey gbDpadKey-down">{dpad.down}</span>
            <span className="gbDpadKey gbDpadKey-left">{dpad.left}</span>
            <span className="gbDpadKey gbDpadKey-right">{dpad.right}</span>
          </div>

          <div className="gbFace">
            <div className="gbFaceBtnWrap">
              <span className="gbBtn gbBtn-b">B</span>
              <span className="gbBtnKey">{b}</span>
            </div>
            <div className="gbFaceBtnWrap">
              <span className="gbBtn gbBtn-a">A</span>
              <span className="gbBtnKey">{a}</span>
            </div>
          </div>
        </div>

        <div className="gbMenu">
          <div className="gbMenuBtnWrap">
            <span className="gbMenuPrinted">SELECT</span>
            <span className="gbBtn gbBtn-select" />
            <span className="gbBtnKey">{select}</span>
          </div>
          <div className="gbMenuBtnWrap">
            <span className="gbMenuPrinted">START</span>
            <span className="gbBtn gbBtn-start" />
            <span className="gbBtnKey">{start}</span>
          </div>
        </div>
      </div>
    </div>
  );
}
