use gpui::{App, Global, Rgba, WindowAppearance, rgba};

#[derive(Clone, Copy)]
pub struct ThemeController {
    // Surface Hierarchy
    pub bg_app: Rgba,     // Base canvas floor
    pub bg_panel: Rgba,   // Structural sidebars and toolbars
    pub bg_surface: Rgba, // Elevated cards and container blocks
    pub bg_sunken: Rgba,  // Inputs, search bars, sunken wells
    pub bg_overlay: Rgba, // Modals, popovers, context menus
    pub bg_hover: Rgba,   // Subtle hover state
    pub bg_active: Rgba,  // Selected/pressed tab or row highlight

    // Structural Borders
    pub border: Rgba,
    pub border_subtle: Rgba,
    pub border_focused: Rgba,

    // Typography
    pub text_main: Rgba,
    pub text_muted: Rgba,
    pub text_disabled: Rgba,
    pub text_on_accent: Rgba,

    // Accents & Semantics
    pub accent: Rgba,
    pub accent_hover: Rgba,
    pub accent_active: Rgba,

    pub success: Rgba,
    pub warning: Rgba,
    pub error: Rgba,
    pub info: Rgba,

    // Spectrum
    pub cyan: Rgba,
    pub green: Rgba,
    pub orange: Rgba,
    pub purple: Rgba,
    pub red: Rgba,
    pub blue: Rgba,
}

impl Global for ThemeController {}

impl ThemeController {
    pub fn init(cx: &mut App) {
        let window_appearance: WindowAppearance = cx.window_appearance();
        let active_theme = match window_appearance {
            WindowAppearance::Dark | WindowAppearance::VibrantDark => Self::deep_void(),
            WindowAppearance::Light | WindowAppearance::VibrantLight => Self::github_light(),
        };
        cx.set_global(active_theme);
    }

    pub fn deep_void() -> Self {
        Self {
            bg_app: rgba(0x0a0c10ff),     // Slightly deeper base canvas
            bg_panel: rgba(0x13161fff),   // Lifted panel background so cards stand out
            bg_surface: rgba(0x1a1d28ff), // Clean surface contrast for inputs/cards
            bg_sunken: rgba(0x0e1017ff),  // Inset areas
            bg_overlay: rgba(0x222634ff),
            bg_hover: rgba(0x292d3eff),
            bg_active: rgba(0x32374cff),

            border: rgba(0x2d3245ff),        // Clear border definition
            border_subtle: rgba(0x202433ff), // Lifted subtle borders so panel edges pop
            border_focused: rgba(0x6366f1ff),

            text_main: rgba(0xf3f4f6ff),
            text_muted: rgba(0xa1a8b8ff), // Brighter muted text for high readability
            text_disabled: rgba(0x525b6eff),
            text_on_accent: rgba(0xffffffff),

            accent: rgba(0x6366f1ff),
            accent_hover: rgba(0x818cf8ff),
            accent_active: rgba(0x4f46e5ff),

            success: rgba(0x10b981ff),
            warning: rgba(0xf59e0bff),
            error: rgba(0xef4444ff),
            info: rgba(0x3b82f6ff),

            cyan: rgba(0x06b6d4ff),
            green: rgba(0x10b981ff),
            orange: rgba(0xf97316ff),
            purple: rgba(0x8b5cf6ff),
            red: rgba(0xef4444ff),
            blue: rgba(0x3b82f6ff),
        }
    }

    /// GitHub Light Spec (Border-less Background Tinting)
    pub fn github_light() -> Self {
        Self {
            // Surface Hierarchy
            bg_app: rgba(0xf1f5f9ff), // Cool Slate Canvas (gives contrast to white cards)
            bg_panel: rgba(0xf8fafcff), // Soft sidebar panel tint
            bg_surface: rgba(0xffffffff), // Pure white floating cards
            bg_hover: rgba(0xe2e8f0ff),
            bg_sunken: rgba(0xeff1f3ff),  // Sunken search wells / inputs
            bg_overlay: rgba(0xffffffff), // Floating context menu
            bg_active: rgba(0xddf4ffff),  // Soft GitHub blue selection fill

            // Structural Borders (Invisible or ultra-faint)
            border: rgba(0xd0d7de66),         // 40% transparent subtle line
            border_subtle: rgba(0xf0f6fcff),  // Almost invisible surface divider
            border_focused: rgba(0x0969daff), // GitHub classic blue focus

            // High Contrast GitHub Typography
            text_main: rgba(0x24292fff), // Sharp obsidian body text (#24292f)
            text_muted: rgba(0x57606aff), // Muted slate subtext (#57606a)
            text_disabled: rgba(0x8c959fff), // Dimmed labels
            text_on_accent: rgba(0xffffffff),

            // GitHub Classic Accents
            accent: rgba(0x0969daff),        // GitHub Royal Blue (#0969da)
            accent_hover: rgba(0x218bffff),  // Hover blue
            accent_active: rgba(0x0550aeff), // Deep blue

            // Functional Indicators
            success: rgba(0x1a7f37ff), // GitHub Green (#1a7f37)
            warning: rgba(0xbf8700ff), // GitHub Gold (#bf8700)
            error: rgba(0xcf222eff),   // GitHub Coral Red (#cf222e)
            info: rgba(0x0969daff),

            // Spectrum
            cyan: rgba(0x057d9fff),
            green: rgba(0x1a7f37ff),
            orange: rgba(0xd97706ff),
            purple: rgba(0x8250dfff),
            red: rgba(0xcf222eff),
            blue: rgba(0x0969daff),
        }
    }
}
