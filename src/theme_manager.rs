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
}

impl Global for ThemeManager {}

impl ThemeManager {
    pub fn init(cx: &mut gpui::App) {
        let window_appearance: WindowAppearance = cx.window_appearance();

        match window_appearance {
            WindowAppearance::Dark | WindowAppearance::VibrantDark => {
                cx.set_global(Self::dark());
            }
            WindowAppearance::Light | WindowAppearance::VibrantLight => {
                cx.set_global(Self::light());
            }
        }
    }

    /// Dark theme (Catppuccin Mocha style slate)
    fn dark() -> Self {
        Self {
            bg_app: rgba(0x1e1e2eff),
            panel_bg: rgba(0x181825ff),
            header_bg: rgba(0x11111bff),
            border_color: rgba(0x313244ff),
            text_main: rgba(0xcdd6f4ff),
            text_muted: rgba(0xa6adc8ff),
            accent: rgba(0x89b4faff),
            button_bg: rgba(0x313244ff),
            button_hover: rgba(0x45475aff),
            button_active: rgba(0x89b4faff),
        }
    }

    /// Light theme (Catppuccin Latte style)
    fn light() -> Self {
        Self {
            bg_app: rgba(0xeff1f5ff),
            panel_bg: rgba(0xe6e9efff),
            header_bg: rgba(0xdce0e8ff),
            border_color: rgba(0xccd0daff),
            text_main: rgba(0x4c4f69ff),
            text_muted: rgba(0x6c6f85ff),
            accent: rgba(0x1e66f5ff),
            button_bg: rgba(0xe6e9efff),
            button_hover: rgba(0xccd0daff),
            button_active: rgba(0x1e66f5ff),
        }
    }
}
