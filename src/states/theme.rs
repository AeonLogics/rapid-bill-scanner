use gpui::{App, Global, Rgba, WindowAppearance, rgba};
use gpui_component::Theme;

#[derive(Clone, Copy)]
pub struct ThemeManager {
    // -------------------------------------------------------------------------
    // Surfaces & Layers
    // -------------------------------------------------------------------------
    pub bg_app: Rgba,
    pub panel_bg: Rgba,
    pub header_bg: Rgba,
    pub surface_elevated: Rgba,
    pub overlay_bg: Rgba,

    // -------------------------------------------------------------------------
    // Borders & Lines
    // -------------------------------------------------------------------------
    pub border_color: Rgba,
    pub border_variant: Rgba,
    pub border_focused: Rgba,

    // -------------------------------------------------------------------------
    // Typography
    // -------------------------------------------------------------------------
    pub text_main: Rgba,
    pub text_muted: Rgba,
    pub text_disabled: Rgba,
    pub text_on_accent: Rgba,

    // -------------------------------------------------------------------------
    // Buttons & Interactivity
    // -------------------------------------------------------------------------
    pub button_bg: Rgba,
    pub button_hover: Rgba,
    pub button_active: Rgba,
    pub selection_bg: Rgba,

    // -------------------------------------------------------------------------
    // Primary Accents
    // -------------------------------------------------------------------------
    pub accent: Rgba,
    pub accent_hover: Rgba,
    pub accent_active: Rgba,

    // -------------------------------------------------------------------------
    // Semantic States (Status & Notifications)
    // -------------------------------------------------------------------------
    pub status_paid: Rgba,    // Success / Green
    pub status_unpaid: Rgba,  // Danger / Red
    pub status_warning: Rgba, // Late Fee / Yellow or Orange
    pub status_info: Rgba,    // Info / Cyan or Blue

    // -------------------------------------------------------------------------
    // Full Brand Spectrum (For Badges, Tags, & Charts)
    // -------------------------------------------------------------------------
    pub cyan: Rgba,
    pub green: Rgba,
    pub orange: Rgba,
    pub pink: Rgba,
    pub purple: Rgba,
    pub red: Rgba,
    pub yellow: Rgba,
    pub blue: Rgba,
}

impl Global for ThemeManager {}

impl ThemeManager {
    pub fn init(cx: &mut App) {
        let window_appearance: WindowAppearance = cx.window_appearance();
        let active_theme = match window_appearance {
            WindowAppearance::Dark | WindowAppearance::VibrantDark => Self::dracula(),
            WindowAppearance::Light | WindowAppearance::VibrantLight => Self::catppuccin_latte(),
        };
        cx.set_global(active_theme);
    }

    /// Official Dracula Spec
    pub fn dracula() -> Self {
        Self {
            // Surfaces
            bg_app: rgba(0x282a36ff),    // Dracula Background (#282a36)
            panel_bg: rgba(0x21222cff),  // Mantle (#21222c)
            header_bg: rgba(0x191a21ff), // Crust (#191a21)
            surface_elevated: rgba(0x343746ff), // Lighter Surface
            overlay_bg: rgba(0x191a21d9), // Backdrop Blur / Modal (85% opacity)

            // Borders
            border_color: rgba(0x44475aff), // Selection / Current Line (#44475a)
            border_variant: rgba(0x6272a4ff), // Comment border (#6272a4)
            border_focused: rgba(0xbd93f9ff), // Dracula Purple Focus (#bd93f9)

            // Typography
            text_main: rgba(0xf8f8f2ff),      // Foreground (#f8f8f2)
            text_muted: rgba(0x6272a4ff),     // Comment Slate (#6272a4)
            text_disabled: rgba(0x44475aff),  // Muted Selection (#44475a)
            text_on_accent: rgba(0x282a36ff), // Dark Text for Bright Accents

            // Buttons & Controls
            button_bg: rgba(0x44475aff),     // Base Button
            button_hover: rgba(0x6272a4ff),  // Hover State
            button_active: rgba(0x343746ff), // Clicked State
            selection_bg: rgba(0x44475aff),  // Highlight Selection

            // Primary Accent
            accent: rgba(0xbd93f9ff),        // Dracula Purple (#bd93f9)
            accent_hover: rgba(0xd6bbfaff),  // Lighter Purple
            accent_active: rgba(0xa575f8ff), // Deeper Purple

            // Semantics
            status_paid: rgba(0x50fa7bff),    // Green (#50fa7b)
            status_unpaid: rgba(0xff5555ff),  // Red (#ff5555)
            status_warning: rgba(0xffb86cff), // Orange (#ffb86c)
            status_info: rgba(0x8be9fdff),    // Cyan (#8be9fd)

            // Full Color Spectrum
            cyan: rgba(0x8be9fdff),
            green: rgba(0x50fa7bff),
            orange: rgba(0xffb86cff),
            pink: rgba(0xff79c6ff),
            purple: rgba(0xbd93f9ff),
            red: rgba(0xff5555ff),
            yellow: rgba(0xf1fa8cff),
            blue: rgba(0x8be9fdff),
        }
    }

    /// Official Catppuccin Latte Spec
    pub fn catppuccin_latte() -> Self {
        Self {
            // Surfaces
            bg_app: rgba(0xeff1f5ff),           // Latte Base (#eff1f5)
            panel_bg: rgba(0xe6e9efff),         // Latte Mantle (#e6e9ef)
            header_bg: rgba(0xdce0e8ff),        // Latte Crust (#dce0e8)
            surface_elevated: rgba(0xccd0daff), // Surface2 (#ccd0da)
            overlay_bg: rgba(0xdce0e8d9),       // Backdrop Blur / Modal (85% opacity)

            // Borders
            border_color: rgba(0xccd0daff),   // Surface2 (#ccd0da)
            border_variant: rgba(0xbcc0ccff), // Surface1 (#bcc0cc)
            border_focused: rgba(0x8839efff), // Mauve Focus (#8839ef)

            // Typography
            text_main: rgba(0x4c4f69ff),      // Text Ink (#4c4f69)
            text_muted: rgba(0x8c8fa1ff),     // Subtext0 (#8c8fa1)
            text_disabled: rgba(0x9ca0b0ff),  // Overlay0 (#9ca0b0)
            text_on_accent: rgba(0xeff1f5ff), // Light Text for Dark Accents

            // Buttons & Controls
            button_bg: rgba(0xdce0e8ff),     // Base Button
            button_hover: rgba(0xbcc0ccff),  // Surface1
            button_active: rgba(0xacb0beff), // Surface0
            selection_bg: rgba(0xacb0beff),  // Highlight Selection

            // Primary Accent
            accent: rgba(0x8839efff),        // Mauve (#8839ef)
            accent_hover: rgba(0x9a5bf1ff),  // Lighter Mauve
            accent_active: rgba(0x7721ebff), // Deeper Mauve

            // Semantics
            status_paid: rgba(0x40a02bff),    // Green (#40a02b)
            status_unpaid: rgba(0xd20f39ff),  // Red (#d20f39)
            status_warning: rgba(0xfe640bff), // Peach (#fe640b)
            status_info: rgba(0x1e66f5ff),    // Blue (#1e66f5)

            // Full Color Spectrum
            cyan: rgba(0x179299ff),   // Teal (#179299)
            green: rgba(0x40a02bff),  // Green (#40a02b)
            orange: rgba(0xfe640bff), // Peach (#fe640b)
            pink: rgba(0xea76cbff),   // Pink (#ea76cb)
            purple: rgba(0x8839efff), // Mauve (#8839ef)
            red: rgba(0xd20f39ff),    // Red (#d20f39)
            yellow: rgba(0xdf8e1dff), // Yellow (#df8e1d)
            blue: rgba(0x1e66f5ff),   // Blue (#1e66f5)
        }
    }
}
