use gpui::prelude::FluentBuilder;
use gpui::{App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px};
use gpui_component::{Icon, IconName, StyledExt, h_flex, v_flex};
use primitives::ThemeController;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(IntoElement, Clone)]
pub struct ScanMonitor {
    pub total_scanned: usize,
    pub total_amount: u32,
    pub is_active: bool,
}

impl ScanMonitor {
    pub fn new(total_scanned: usize, total_amount: u32, is_active: bool) -> Self {
        Self {
            total_scanned,
            total_amount,
            is_active,
        }
    }
}

impl RenderOnce for ScanMonitor {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeController>();

        let elapsed_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);

        // 1. Horizontal Laser Position
        let cycle = (elapsed_ms % 1600) as f32 / 1600.0;
        let pos_pct = if cycle < 0.5 {
            cycle * 2.0 * 88.0 + 6.0
        } else {
            (1.0 - cycle) * 2.0 * 88.0 + 6.0
        };

        // 2. Vertical Grid Pulse Position
        let vert_cycle = (elapsed_ms % 2400) as f32 / 2400.0;
        let vert_pos_pct = vert_cycle * 90.0 + 5.0;

        // 3. Telemetry Hex Generators
        let hex_val1 = (elapsed_ms / 120) % 0xFFFF;
        let hex_val2 = (elapsed_ms / 80) % 0xFF;

        v_flex()
            .flex_1()
            .h_full()
            .w_full()
            .p_6()
            .gap_6()
            .rounded_xl()
            .border_1()
            .border_color(theme.border)
            .bg(theme.bg_panel)
            .justify_between()
            // --- Top HUD Header ---
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .items_center()
                    .child(
                        h_flex()
                            .items_center()
                            .gap_2()
                            .child(Icon::new(IconName::Bot))
                            .child(
                                div()
                                    .text_sm()
                                    .font_bold()
                                    .text_color(theme.text_main)
                                    .child("LaserScanner Barcode HUD"),
                            ),
                    )
                    .child(
                        h_flex()
                            .items_center()
                            .gap_2()
                            .px_3()
                            .py_1()
                            .rounded_full()
                            .bg(if self.is_active {
                                theme.accent
                            } else {
                                theme.bg_panel
                            })
                            .child(div().size_2().rounded_full().bg(if self.is_active {
                                theme.accent
                            } else {
                                theme.text_muted
                            }))
                            .child(
                                div()
                                    .text_xs()
                                    .font_bold()
                                    .text_color(if self.is_active {
                                        theme.accent
                                    } else {
                                        theme.text_muted
                                    })
                                    .child(if self.is_active {
                                        "LASER ARMED"
                                    } else {
                                        "STANDBY"
                                    }),
                            ),
                    ),
            )
            // --- Center Animated Stage ---
            .child(
                div()
                    .relative()
                    .w_full()
                    .flex_1()
                    .rounded_lg()
                    .bg(theme.bg_app)
                    .border_1()
                    .border_color(theme.border_color)
                    .overflow_hidden()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .absolute()
                            .size_full()
                            .border_1()
                            .border_color(theme.border_color)
                            .opacity(0.1),
                    )
                    .child(
                        div()
                            .absolute()
                            .top_3()
                            .left_3()
                            .size(px(16.0))
                            .border_t_2()
                            .border_l_2()
                            .border_color(if self.is_active {
                                theme.accent
                            } else {
                                theme.border_color
                            }),
                    )
                    .child(
                        div()
                            .absolute()
                            .top_3()
                            .right_3()
                            .size(px(16.0))
                            .border_t_2()
                            .border_r_2()
                            .border_color(if self.is_active {
                                theme.accent
                            } else {
                                theme.border_color
                            }),
                    )
                    .child(
                        div()
                            .absolute()
                            .bottom_3()
                            .left_3()
                            .size(px(16.0))
                            .border_b_2()
                            .border_l_2()
                            .border_color(if self.is_active {
                                theme.accent
                            } else {
                                theme.border_color
                            }),
                    )
                    .child(
                        div()
                            .absolute()
                            .bottom_3()
                            .right_3()
                            .size(px(16.0))
                            .border_b_2()
                            .border_r_2()
                            .border_color(if self.is_active {
                                theme.accent
                            } else {
                                theme.border_color
                            }),
                    )
                    // Live Telemetry Hex Overlay (Top Left inside viewport)
                    .child(
                        v_flex()
                            .absolute()
                            .top_4()
                            .left_6()
                            .gap_0p5()
                            .font_family("Consolas")
                            .text_xs()
                            .text_color(theme.text_muted)
                            .opacity(0.7)
                            .child(format!("HEX: 0x{:04X}", hex_val1))
                            .child(format!("FREQ: {}00 Hz", hex_val2))
                            .child(if self.is_active {
                                "STREAM: RAW_HID"
                            } else {
                                "STREAM: OFF"
                            }),
                    )
                    // Live Telemetry Status (Top Right inside viewport)
                    .child(
                        v_flex()
                            .absolute()
                            .top_4()
                            .right_6()
                            .items_end()
                            .gap_0p5()
                            .font_family("Consolas")
                            .text_xs()
                            .text_color(theme.text_muted)
                            .opacity(0.7)
                            .child(format!("T_STAMP: {}", elapsed_ms % 100000))
                            .child("AUTO_EXEC: YES")
                            .child(if self.is_active {
                                "LNK_STATUS: OK"
                            } else {
                                "LNK_STATUS: WAIT"
                            }),
                    )
                    // Dual-Axis Animated Sweeping Lasers
                    .when(self.is_active, |this| {
                        this
                            // Horizontal Primary Red Laser
                            .child(
                                div()
                                    .absolute()
                                    .left_0()
                                    .right_0()
                                    .top(gpui::relative((pos_pct - 2.0).max(0.0) / 100.0))
                                    .h(px(12.0))
                                    .bg(theme.accent)
                                    .opacity(0.12),
                            )
                            .child(
                                div()
                                    .absolute()
                                    .left_0()
                                    .right_0()
                                    .top(gpui::relative(pos_pct / 100.0))
                                    .h(px(2.0))
                                    .bg(theme.accent),
                            )
                            // Vertical Matrix Scanner Column
                            .child(
                                div()
                                    .absolute()
                                    .top_0()
                                    .bottom_0()
                                    .left(gpui::relative(vert_pos_pct / 100.0))
                                    .w(px(2.0))
                                    .bg(theme.accent)
                                    .opacity(0.25),
                            )
                    })
                    // Center Animated Radar Reticle
                    .child(
                        v_flex()
                            .items_center()
                            .gap_4()
                            .child(
                                div()
                                    .relative()
                                    .size(px(144.0))
                                    .rounded_full()
                                    .border_2()
                                    .border_color(if self.is_active {
                                        theme.accent
                                    } else {
                                        theme.border_color
                                    })
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    // Crosshair Horizontal & Vertical
                                    .child(div().absolute().w_full().h_0p5().bg(theme.border_color))
                                    .child(div().absolute().h_full().w_0p5().bg(theme.border_color))
                                    .child(
                                        div()
                                            .relative()
                                            .size(px(96.0))
                                            .rounded_full()
                                            .border_1()
                                            .border_color(if self.is_active {
                                                theme.accent
                                            } else {
                                                theme.border_color
                                            })
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            // Glowing Inner Core Dot
                                            .child(div().size(px(24.0)).rounded_full().bg(
                                                if self.is_active {
                                                    theme.accent
                                                } else {
                                                    theme.text_muted
                                                },
                                            )),
                                    ),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_family("Consolas")
                                    .font_bold()
                                    .text_color(if self.is_active {
                                        theme.accent
                                    } else {
                                        theme.text_muted
                                    })
                                    .child(if self.is_active {
                                        ">> PASS BARCODE THROUGH LASER BEAM <<"
                                    } else {
                                        "!! ENGINE IDLE - CLICK ACTIVATE !!"
                                    }),
                            ),
                    )
                    // Bottom Bouncing Audio Equalizer Bar Visualization
                    .child(h_flex().absolute().bottom_4().gap_1().items_end().children(
                        (0..16).map(|i| {
                            let bar_h = if self.is_active {
                                (((elapsed_ms / 50 + i * 17) % 24) + 6) as f32
                            } else {
                                4.0
                            };
                            div()
                                .w(px(3.0))
                                .h(px(bar_h))
                                .rounded_sm()
                                .bg(if self.is_active {
                                    theme.accent
                                } else {
                                    theme.border_color
                                })
                        }),
                    )),
            )
            // --- Bottom Stats Cards ---
            .child(
                h_flex()
                    .w_full()
                    .gap_4()
                    .child(
                        v_flex()
                            .flex_1()
                            .p_4()
                            .rounded_lg()
                            .bg(theme.bg_app)
                            .border_1()
                            .border_color(theme.border_color)
                            .child(
                                div()
                                    .text_xs()
                                    .font_bold()
                                    .text_color(theme.text_muted)
                                    .child("SCANNED BATCH"),
                            )
                            .child(
                                div()
                                    .text_2xl()
                                    .font_black()
                                    .text_color(theme.text_main)
                                    .child(format!("{} Bills", self.total_scanned)),
                            ),
                    )
                    .child(
                        v_flex()
                            .flex_1()
                            .p_4()
                            .rounded_lg()
                            .bg(theme.bg_app)
                            .border_1()
                            .border_color(theme.border_color)
                            .child(
                                div()
                                    .text_xs()
                                    .font_bold()
                                    .text_color(theme.text_muted)
                                    .child("BATCH TOTAL"),
                            )
                            .child(
                                div()
                                    .text_2xl()
                                    .font_black()
                                    .text_color(theme.accent)
                                    .child(format!("Rs. {}", self.total_amount)),
                            ),
                    ),
            )
    }
}
