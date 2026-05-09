//! Column widths for the ROM library table (fixed tail + flex “Game” column).

use crate::desktop::launcher::theme::space_2;

/// Row height: token padding + badge / [`super::buttons`] small primary button.
pub(crate) const ROM_LIBRARY_ROW_HEIGHT: f32 = 36.0;

#[derive(Clone, Copy)]
pub(crate) struct RomLibraryColumnWidths {
    pub game: f32,
    pub plat: f32,
    pub played: f32,
    pub fav: f32,
    pub act: f32,
}

/// Fixed trailing columns + spacing gutters; remainder → `game`.
pub(crate) fn rom_library_column_widths(inner_row_width: f32) -> RomLibraryColumnWidths {
    const COL_PLAT: f32 = 76.0;
    const COL_PLAYED: f32 = 120.0;
    const COL_FAV: f32 = 40.0;
    /// Matches [`crate::desktop::launcher::components::buttons::action_button`] size `Sm` (88×26).
    const COL_ACT: f32 = 96.0;
    let gap = space_2();
    let gutters = 4.0 * gap;
    let fixed = COL_PLAT + COL_PLAYED + COL_FAV + COL_ACT + gutters;
    let mut game = (inner_row_width - fixed).max(96.0);
    let slack = inner_row_width - (game + fixed);
    game += slack;
    RomLibraryColumnWidths {
        game,
        plat: COL_PLAT,
        played: COL_PLAYED,
        fav: COL_FAV,
        act: COL_ACT,
    }
}
