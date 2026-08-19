use gpui::prelude::*;
use gpui::{
    App, Image, ImageFormat, ImageSource, IntoElement, RenderOnce, Styled, Window, div, img, px,
};
use gpui_component::{StyledExt, h_flex, v_flex};
use primitives::ThemeController;
use std::sync::Arc;

#[derive(Default, IntoElement)]
pub struct DockHeader;

impl RenderOnce for DockHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeController>();

        let bytes: Vec<u8> = include_bytes!("../../public/icon_svg.svg").into();
        let image = ImageSource::Image(Arc::new(Image::from_bytes(ImageFormat::Svg, bytes)));

        h_flex()
            .w_full()
            .h(px(64.0))
            .rounded_r_2xl()
            .px_4()
            .items_center()
            .justify_between()
            .border_b_1()
            .border_color(theme.border_subtle)
            .bg(theme.bg_panel)
            .child(
                h_flex()
                    .items_center()
                    .gap_3()
                    .child(img(image).w(px(32.0)).h(px(32.0)).rounded_lg())
                    .child(
                        v_flex()
                            .gap_0p5()
                            .child(
                                div()
                                    .text_sm()
                                    .font_bold()
                                    .text_color(theme.text_main)
                                    .child("LaserScanner"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .italic()
                                    .font_medium()
                                    .text_color(theme.accent)
                                    .child("Aeon Logics"),
                            ),
                    ),
            )
    }
}
