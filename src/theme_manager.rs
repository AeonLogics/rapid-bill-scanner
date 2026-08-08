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

    /// Deep Pitch Dark + Purple Accent Theme
    fn dark() -> Self {
        Self {
            bg_app: rgba(0x000010ff),
            panel_bg: rgba(0x0a0914ff),
            header_bg: rgba(0x05040cff),
            border_color: rgba(0x26233aff),
            text_main: rgba(0xe0def4ff),
            text_muted: rgba(0x908caaff),
            accent: rgba(0xcba6f7ff),
            button_bg: rgba(0x1f1d2eff),
            button_hover: rgba(0x2a283eff),
            button_active: rgba(0xcba6f7ff),
        }
    }

    /// Light theme (Soft Lavender)
    fn light() -> Self {
        Self {
            bg_app: rgba(0xf4f0f8ff),
            panel_bg: rgba(0xe9e3f0ff),
            header_bg: rgba(0xded6e8ff),
            border_color: rgba(0xccc3d8ff),
            text_main: rgba(0x4a405aff),
            text_muted: rgba(0x6e637eff),
            accent: rgba(0x8839efff),
            button_bg: rgba(0xe9e3f0ff),
            button_hover: rgba(0xccc3d8ff),
            button_active: rgba(0x8839efff),
        }
    }
}
