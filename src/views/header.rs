use std::sync::Arc;

use crate::{theme_manager::ThemeManager, views::TabView};
use gpui::{
    Context, Entity, Image, ImageSource, InteractiveElement, IntoElement, ParentElement, Render,
    StatefulInteractiveElement, Styled, Window, div, img, px,
};
use gpui_component::{Icon, IconName, StyledExt, TitleBar};

pub struct Header {
    tab_view: Entity<TabView>,
}

impl Header {
    pub fn new(tab_view: Entity<TabView>, cx: &mut Context<Self>) -> Self {
        cx.observe(&tab_view, |_this, _tab_view, cx| {
            cx.notify();
        })
        .detach();

        Self { tab_view }
    }

    fn nav_pill(
        &self,
        target_idx: usize,
        label: &'static str,
        icon: IconName,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.global::<ThemeManager>();
        let is_active = self.tab_view.read(cx).active_tab == target_idx;
        let tab_view = self.tab_view.clone();

        div()
            .id(label)
            .flex()
            .items_center()
            .rounded_md()
            .text_sm()
            .font_medium()
            .cursor_pointer()
            .py_1()
            .px_3()
            .gap_1()
            .bg(if is_active {
                theme.button_bg
            } else {
                gpui::rgba(0x00000000)
            })
            .text_color(if is_active {
                theme.accent
            } else {
                theme.text_muted
            })
            .border_1()
            .border_color(if is_active {
                theme.accent
            } else {
                gpui::rgba(0x00000000)
            })
            .hover(|s| {
                if !is_active {
                    s.bg(theme.button_hover).text_color(theme.text_main)
                } else {
                    s
                }
            })
            .child(Icon::new(icon).text_xl())
            .child(label)
            .on_mouse_down(gpui::MouseButton::Left, move |_event, _window, cx| {
                cx.stop_propagation();
            })
            .on_click(cx.listener(move |_this, _event, _window, cx| {
                tab_view.update(cx, |tv, cx| {
                    tv.set_tab(target_idx, cx);
                });
            }))
    }
}

impl Render for Header {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Header>) -> impl IntoElement {
        let theme = cx.global::<ThemeManager>();
        let bytes: Vec<u8> = include_bytes!("../../public/icon.ico").into();
        let image = ImageSource::Image(Arc::new(Image::from_bytes(gpui::ImageFormat::Ico, bytes)));

        TitleBar::new()
            .w_full()
            .h(px(50.0))
            .bg(theme.header_bg)
            .border_b_1()
            .border_color(theme.border_color)
            .child(
                div()
                    .child(img(image).w(px(32.)))
                    .size_full()
                    .flex()
                    .gap_1()
                    .items_center()
                    .pr(px(75.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .justify_center()
                            .child(
                                div()
                                    .font_bold()
                                    .italic()
                                    .text_color(theme.text_main)
                                    .child("Rapid Bill Scanner"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_bold()
                                    .italic()
                                    .text_color(theme.text_muted)
                                    .child("Aeon Logics"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .p_1_3()
                            .gap_2()
                            .child(self.nav_pill(0, "Automation", IconName::Bot, cx))
                            .child(self.nav_pill(1, "PreScanned", IconName::File, cx))
                            .child(self.nav_pill(2, "Settings", IconName::Settings, cx)),
                    ),
            )
    }
}
