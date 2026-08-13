use gpui::{Global, Rgba, WindowAppearance, rgba};

#[derive(Clone, Copy)]
pub struct ThemeManager {
    pub bg_app: Rgba,
    pub panel_bg: Rgba,
    pub header_bg: Rgba,
    pub border_color: Rgba,
    pub text_main: Rgba,
    pub text_muted: Rgba,
    pub accent: Rgba,
    pub button_bg: Rgba,
    pub button_hover: Rgba,
    pub button_active: Rgba,

    // Status indicator colors for bill list
    pub status_paid: Rgba,
    pub status_unpaid: Rgba,
}

impl Global for ThemeManager {}

impl ThemeManager {
    pub fn init(cx: &mut gpui::App) {
        let window_appearance: WindowAppearance = cx.window_appearance();

        match window_appearance {
            WindowAppearance::Dark | WindowAppearance::VibrantDark => {
                cx.set_global(Self::dracula());
            }
            WindowAppearance::Light | WindowAppearance::VibrantLight => {
                cx.set_global(Self::catppuccin_latte());
            }
        }
    }

    /// Official Dracula Theme Spec
    pub fn dracula() -> Self {
        Self {
            bg_app: rgba(0x282a36ff),        // Dracula Background (#282a36)
            panel_bg: rgba(0x21222cff),      // Darker Mantle (#21222c)
            header_bg: rgba(0x191a21ff),     // Deep Crust (#191a21)
            border_color: rgba(0x44475aff),  // Current Line / Selection (#44475a)
            text_main: rgba(0xf8f8f2ff),     // Foreground (#f8f8f2)
            text_muted: rgba(0x6272a4ff),    // Comment Slate (#6272a4)
            accent: rgba(0xbd93f9ff),        // Dracula Purple (#bd93f9)
            button_bg: rgba(0x44475aff),     // Selection / Button Bg
            button_hover: rgba(0x6272a4ff),  // Hover State
            button_active: rgba(0xff79c6ff), // Dracula Pink Glow (#ff79c6)

            status_paid: rgba(0x50fa7bff),   // Dracula Green (#50fa7b)
            status_unpaid: rgba(0xff5555ff), // Dracula Red (#ff5555)
        }
    }

    /// Official Catppuccin Latte (Soft Warm Lavender/Mauve)
    pub fn catppuccin_latte() -> Self {
        Self {
            bg_app: rgba(0xeff1f5ff),        // Latte Base (#eff1f5)
            panel_bg: rgba(0xe6e9efff),      // Latte Mantle (#e6e9ef)
            header_bg: rgba(0xdce0e8ff),     // Latte Crust (#dce0e8)
            border_color: rgba(0xccd0daff),  // Latte Surface2 (#ccd0da)
            text_main: rgba(0x4c4f69ff),     // Latte Text Ink (#4c4f69)
            text_muted: rgba(0x8c8fa1ff),    // Latte Subtext0 (#8c8fa1)
            accent: rgba(0x8839efff),        // Latte Mauve/Purple (#8839ef)
            button_bg: rgba(0xdce0e8ff),     // Soft Button Base
            button_hover: rgba(0xbcc0ccff),  // Latte Surface1
            button_active: rgba(0xea76cbff), // Latte Pink (#ea76cb)

            status_paid: rgba(0x40a02bff),   // Latte Green (#40a02b)
            status_unpaid: rgba(0xd20f39ff), // Latte Red (#d20f39)
        }
    }
}
