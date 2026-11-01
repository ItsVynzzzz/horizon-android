//! AmberLauncher's look for the native launcher: warm black, glass plates with a thin white edge,
//! pill-shaped controls and one accent (Amber's default preset, "Navy"). Colours and shapes only:
//! nothing here decides what a control does.
use horizon_client::ui_theme::{Ui, color::hex};

/// The launcher's palette. Names it shares with `horizon_client::ui_theme::color` mean the same
/// thing there (page, text, accent), so a screen switches look by changing one import. The game's
/// own menus and the on-screen keyboard keep the design system's palette.
pub(crate) mod color {
    use super::hex;

    /// The page behind everything.
    pub const BG: u32 = hex(0x0e0e0d, 255);
    /// A dialog's body, opaque so nothing behind it shows through.
    pub const PANEL: u32 = hex(0x141312, 255);
    /// Glass, lowest level (white 6%): the card behind a page's content.
    pub const GLASS: u32 = hex(0xffffff, 15);
    /// Glass, second level (white 9%): buttons and fields.
    pub const RAISED: u32 = hex(0xffffff, 23);
    /// A plate under a finger (white 15%).
    pub const HOVER: u32 = hex(0xffffff, 38);
    /// The thin edge of every glass plate (white 14%), and the stronger one of a switch that is
    /// off or an unselected radio (white 25%).
    pub const EDGE: u32 = hex(0xffffff, 36);
    pub const EDGE_STRONG: u32 = hex(0xffffff, 64);
    /// Rules between rows (white 12%).
    pub const BORDER: u32 = hex(0xffffff, 31);
    /// Text, and secondary text.
    pub const TEXT: u32 = hex(0xf4f3ee, 255);
    pub const MUTED: u32 = hex(0xb9b5bc, 255);
    /// The accent, Amber's "Navy" preset: the fill of the open page and the main action, its edge,
    /// the light tone for accent text, progress and the focus ring, and the label on the fill.
    /// Amber's other presets, as fill / edge / soft: green 2fb86c / 9addb8 / 8df0b8, purple
    /// 6b4bd1 / b9a8ec / c9b8ff, teal 14a3a8 / 8fd6d9 / 8fe6ea.
    pub const ACCENT: u32 = hex(0x1f3380, 255);
    pub const ACCENT_EDGE: u32 = hex(0x8f99c0, 255);
    pub const ACCENT_SOFT: u32 = hex(0xa9bdff, 255);
    pub const ON_ACCENT: u32 = hex(0xffffff, 255);
}

/// A card's corner radius (Amber's `AmberCard`).
pub(crate) const CARD: f32 = 24.0;

/// The radius that turns a control `h` tall into a pill (Amber's `CircleShape`); a taller plate
/// stops at the large card's 28.
pub(crate) fn pill(h: f32) -> f32 {
    (h / 2.0).min(28.0)
}

/// A glass plate: `fill`, then a thin white edge (Amber's `amberGlass`, without the blur).
pub(crate) fn glass(ui: &mut Ui, x: f32, y: f32, w: f32, h: f32, radius: f32, fill: u32) {
    ui.rounded(x, y, w, h, radius, fill);
    ui.border(x, y, w, h, radius, color::EDGE);
}
