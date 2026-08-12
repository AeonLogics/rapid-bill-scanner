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

    fn dark() -> Self {
        Self {
            bg_app: rgba(0x05060aff),        // Pure abyss background
            panel_bg: rgba(0x0e111aff),      // High-contrast elevated panel
            header_bg: rgba(0x161a26ff),     // Distinct header layer
            border_color: rgba(0x283047ff),  // Sharp glowing border
            text_main: rgba(0xf9fafbff),     // Pure crisp white
            text_muted: rgba(0x94a3b8ff),    // Slate grey text
            accent: rgba(0x38bdf8ff),        // Electric Sky Blue (#38bdf8)
            button_bg: rgba(0x1e293bff),     // Dark navy button
            button_hover: rgba(0x334155ff),  // Bright hover state
            button_active: rgba(0x38bdf8ff), // Sky blue active glow
        }
    }

    fn light() -> Self {
        Self {
            bg_app: rgba(0xf8fafcff),        // Ultra clean light slate
            panel_bg: rgba(0xffffffff),      // Pure white panels
            header_bg: rgba(0xf1f5f9ff),     // Soft slate header
            border_color: rgba(0xcbd5e1ff),  // Defined borders
            text_main: rgba(0x0f172aff),     // Dark ink text
            text_muted: rgba(0x64748bff),    // Secondary text
            accent: rgba(0x0284c7ff),        // Deep electric blue
            button_bg: rgba(0xf1f5f9ff),     // Soft button bg
            button_hover: rgba(0xe2e8f0ff),  // Clear hover state
            button_active: rgba(0x0284c7ff), // Active blue
        }
    }
}
