use gpui::prelude::*;
use gpui::{App, IntoElement, RenderOnce, Styled, Window, WindowControlArea, div, px};
use gpui_component::{Icon, IconName, Sizable, StyledExt};
use primitives::ThemeController;

#[derive(Clone, Copy)]
enum ControlKind {
    Minimize,
    Maximize,
    Restore,
    Close,
}

impl ControlKind {
    fn control_area(&self) -> WindowControlArea {
        match self {
            Self::Minimize => WindowControlArea::Min,
            Self::Maximize | Self::Restore => WindowControlArea::Max,
            Self::Close => WindowControlArea::Close,
        }
    }

    fn icon(&self) -> IconName {
        match self {
            Self::Minimize => IconName::WindowMinimize,
            Self::Maximize => IconName::WindowMaximize,
            Self::Restore => IconName::WindowRestore,
            Self::Close => IconName::WindowClose,
        }
    }
}

#[derive(IntoElement)]
struct WinButton {
    kind: ControlKind,
}

impl RenderOnce for WinButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeController>();
        let is_close = matches!(self.kind, ControlKind::Close);

        div()
            .id(match self.kind {
                ControlKind::Minimize => "win-min-btn",
                ControlKind::Maximize => "win-max-btn",
                ControlKind::Restore => "win-restore-btn",
                ControlKind::Close => "win-close-btn",
            })
            .flex()
            .w(px(46.0))
            .h_full()
            .items_center()
            .justify_center()
            .text_color(theme.text_muted)
            .window_control_area(self.kind.control_area())
            .hover(|style| {
                if is_close {
                    style.bg(gpui::rgb(0xe81123)).text_color(gpui::white())
                } else {
                    style.bg(theme.bg_surface).text_color(theme.text_main)
                }
            })
            .child(Icon::new(self.kind.icon()).small())
    }
}
#[derive(Default, IntoElement)]
pub struct DeckTitleBar;

impl RenderOnce for DeckTitleBar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeController>();
        let is_maximized = window.is_maximized();

        div()
            .w_full()
            .h(px(64.0))
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .rounded_bl_2xl()
            .bg(theme.bg_surface)
            .border_b_1()
            .border_color(theme.border_subtle)
            .overflow_hidden()
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .flex()
                    .items_center()
                    .px_4()
                    .window_control_area(WindowControlArea::Drag)
                    .child(
                        div()
                            .text_xs()
                            .font_medium()
                            .text_color(theme.text_muted)
                            .child("Optical Bill Telemetry & Extraction"),
                    ),
            )
            .child(
                div()
                    .h_full()
                    .flex()
                    .flex_row()
                    .child(WinButton {
                        kind: ControlKind::Minimize,
                    })
                    .child(WinButton {
                        kind: if is_maximized {
                            ControlKind::Restore
                        } else {
                            ControlKind::Maximize
                        },
                    })
                    .child(WinButton {
                        kind: ControlKind::Close,
                    }),
            )
    }
}
