use crate::ui::ternary;
use gpui::prelude::*;
use gpui::{App, IntoElement, RenderOnce, Window, div, hsla, px};
use gpui_component::{Icon, IconName, StyledExt, h_flex, v_flex};
use primitives::{LaserBill, ThemeController};
use std::time::Instant;

#[derive(Clone)]
struct PipelineStep {
    icon: IconName,
    label: &'static str,
    x_pct: f32,
    y_pos: f32,
}

fn get_animation_progress() -> f32 {
    static START_TIME: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();
    let start = START_TIME.get_or_init(Instant::now);
    start.elapsed().as_secs_f32()
}

#[derive(IntoElement)]
pub struct ProceduralScanner {
    is_running: bool,
    last_bill: Option<LaserBill>,
}

impl ProceduralScanner {
    pub fn new(is_running: bool, laser_bill: Option<LaserBill>) -> Self {
        Self {
            is_running,
            last_bill: laser_bill,
        }
    }
}

impl RenderOnce for ProceduralScanner {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeController>();

        if self.is_running {
            window.on_next_frame(|_window, cx| {
                cx.refresh_windows();
            });
        }

        let time = get_animation_progress();
        let active = self.is_running;

        let accent_color = theme.accent;
        let trace_color = theme.border_subtle;

        let pipeline_steps = vec![
            PipelineStep {
                icon: IconName::Network,
                label: "DETECT STREAM",
                x_pct: 0.08,
                y_pos: 35.0,
            },
            PipelineStep {
                icon: IconName::Cpu,
                label: "DECODE BILL",
                x_pct: 0.36,
                y_pos: 155.0,
            },
            PipelineStep {
                icon: IconName::MemoryStick,
                label: "EXTRACT BILL TYPE",
                x_pct: 0.64,
                y_pos: 35.0,
            },
            PipelineStep {
                icon: IconName::ExternalLink,
                label: "SIMULATE KEYPRESS",
                x_pct: 0.92,
                y_pos: 155.0,
            },
        ];

        let canvas_steps = pipeline_steps.clone();

        v_flex()
            .w_full()
            .flex_1()
            .min_h(px(360.0))
            .bg(theme.bg_sunken)
            .border_1()
            .border_color(theme.border_subtle)
            .rounded_xl()
            .relative()
            .overflow_hidden()
            .justify_between()
            .p_5()
            .child(
                h_flex().w_full().justify_between().items_center().child(
                    h_flex()
                        .items_center()
                        .gap_2p5()
                        .child(div().size(px(8.0)).rounded_full().bg(ternary(
                            active,
                            theme.accent,
                            theme.text_muted,
                        )))
                        .child(
                            div()
                                .text_xs()
                                .font_semibold()
                                .text_color(ternary(active, theme.accent, theme.text_muted))
                                .child(ternary(active, "Laser streaming... ", "Laser idle")),
                        ),
                ),
            )
            // --- HUD TELEMETRY BAR ---
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .items_center()
                    .px_3p5()
                    .py_2()
                    .my_1()
                    .bg(theme.bg_surface)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .rounded_lg()
                    .shadow_sm()
                    .child(
                        h_flex()
                            .gap_6()
                            .items_center()
                            // Reference Number
                            .child(
                                v_flex()
                                    .gap_0p5()
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_semibold()
                                            .text_color(theme.text_muted)
                                            .child("LAST REF NO"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_family("Consolas")
                                            .font_bold()
                                            .text_color(theme.text_main)
                                            .child(match &self.last_bill {
                                                Some(b) => b.reference.clone(),
                                                None => "--".to_string(),
                                            }),
                                    ),
                            )
                            .child(div().w(px(1.0)).h(px(18.0)).bg(theme.border_subtle))
                            // Amount
                            .child(
                                v_flex()
                                    .gap_0p5()
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_semibold()
                                            .text_color(theme.text_muted)
                                            .child("AMOUNT"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_family("Consolas")
                                            .font_bold()
                                            .text_color(theme.accent)
                                            .child(match &self.last_bill {
                                                Some(b) => format!("RS. {}", b.amount),
                                                None => "--".to_string(),
                                            }),
                                    ),
                            )
                            .child(div().w(px(1.0)).h(px(18.0)).bg(theme.border_subtle))
                            // Bill Type (Replaces Due Date)
                            .child(
                                v_flex()
                                    .gap_0p5()
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_semibold()
                                            .text_color(theme.text_muted)
                                            .child("BILL TYPE"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_family("Consolas")
                                            .font_bold()
                                            .text_color(theme.text_main)
                                            .child(match &self.last_bill {
                                                Some(b) => b.bill_type.to_string(),
                                                None => "--".to_string(),
                                            }),
                                    ),
                            ),
                    )
                    // Status Badge (Fixed: has_late_fee)
                    .child(match &self.last_bill {
                        Some(b) if b.has_late_fee => div()
                            .px_2p5()
                            .py_0p5()
                            .rounded_full()
                            .border_1()
                            .border_color(hsla(10.0 / 360.0, 0.80, 0.60, 0.4))
                            .bg(hsla(10.0 / 360.0, 0.80, 0.60, 0.12))
                            .text_xs()
                            .font_bold()
                            .text_color(hsla(10.0 / 360.0, 0.90, 0.65, 1.0))
                            .child("LATE FEE APPLIED"),
                        Some(_) => div()
                            .px_2p5()
                            .py_0p5()
                            .rounded_full()
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_sunken)
                            .text_xs()
                            .font_bold()
                            .text_color(theme.text_muted)
                            .child("STANDARD BILL"),
                        None => div()
                            .text_xs()
                            .font_family("Consolas")
                            .text_color(theme.text_muted)
                            .child("NO PAYLOAD"),
                    }),
            )
            .child(
                div()
                    .relative()
                    .w_full()
                    .h(px(210.0))
                    .my_1()
                    .child(gpui::canvas(
                        move |_bounds, _cx, _fa| {},
                        move |bounds, _state, window, _cx| {
                            let width: f32 = bounds.size.width.into();

                            let waypoints: Vec<(f32, f32)> = canvas_steps
                                .iter()
                                .map(|step| (width * step.x_pct, step.y_pos))
                                .collect();

                            let get_spline_pt =
                                |p0: (f32, f32), p1: (f32, f32), t: f32| -> (f32, f32) {
                                    let mid_x = p0.0 + (p1.0 - p0.0) * t;
                                    let smooth_t = (1.0 - (t * std::f32::consts::PI).cos()) * 0.5;
                                    let mid_y = p0.1 + (p1.1 - p0.1) * smooth_t;
                                    (mid_x, mid_y)
                                };

                            let samples = 260;
                            for i in 0..waypoints.len() - 1 {
                                let p0 = waypoints[i];
                                let p1 = waypoints[i + 1];

                                for s in 0..samples {
                                    let t = (s as f32) / (samples as f32);
                                    let (x, y) = get_spline_pt(p0, p1, t);

                                    let dot_size = 2.5;
                                    window.paint_quad(
                                        gpui::fill(
                                            gpui::Bounds::new(
                                                bounds.origin
                                                    + gpui::point(
                                                        px(x - dot_size / 2.0),
                                                        px(y - dot_size / 2.0),
                                                    ),
                                                gpui::size(px(dot_size), px(dot_size)),
                                            ),
                                            trace_color,
                                        )
                                        .corner_radii(px(dot_size / 2.0)),
                                    );
                                }
                            }

                            if !active {
                                return;
                            }

                            let total_segs = (waypoints.len() - 1) as f32;
                            let speed = 2.20;
                            let loop_prog = (time * speed) % total_segs;
                            let seg_idx = loop_prog as usize;
                            let seg_frac = loop_prog.fract();

                            let (ball_x, ball_y) =
                                get_spline_pt(waypoints[seg_idx], waypoints[seg_idx + 1], seg_frac);

                            let trail_pts = 28;
                            for t in 1..=trail_pts {
                                let offset = (t as f32) * 0.012;
                                let t_prog = (loop_prog - offset).max(0.0);
                                let t_idx = (t_prog as usize).min(waypoints.len() - 2);
                                let t_frac = t_prog.fract();

                                let (tx, ty) =
                                    get_spline_pt(waypoints[t_idx], waypoints[t_idx + 1], t_frac);

                                let fade = 1.0 - (t as f32 / trail_pts as f32);
                                let size = (13.0 * fade).max(2.0);

                                window.paint_quad(
                                    gpui::fill(
                                        gpui::Bounds::new(
                                            bounds.origin
                                                + gpui::point(
                                                    px(tx - size / 2.0),
                                                    px(ty - size / 2.0),
                                                ),
                                            gpui::size(px(size), px(size)),
                                        ),
                                        accent_color,
                                    )
                                    .corner_radii(px(size / 2.0)),
                                );
                            }

                            let ball_size = 14.0;
                            window.paint_quad(
                                gpui::fill(
                                    gpui::Bounds::new(
                                        bounds.origin
                                            + gpui::point(
                                                px(ball_x - ball_size / 2.0),
                                                px(ball_y - ball_size / 2.0),
                                            ),
                                        gpui::size(px(ball_size), px(ball_size)),
                                    ),
                                    accent_color,
                                )
                                .corner_radii(px(ball_size / 2.0)),
                            );
                        },
                    ))
                    .children(pipeline_steps.into_iter().map(|step| {
                        let is_top = step.y_pos < 90.0;
                        div()
                            .absolute()
                            .left(gpui::DefiniteLength::Fraction(step.x_pct))
                            .top(px(step.y_pos))
                            .ml(px(-22.0))
                            .mt(px(-22.0))
                            .flex()
                            .flex_col()
                            .items_center()
                            .child(
                                div()
                                    .size(px(44.0))
                                    .rounded_full()
                                    .bg(theme.bg_surface)
                                    .border_2()
                                    .border_color(if active {
                                        theme.accent
                                    } else {
                                        theme.border_subtle
                                    })
                                    .shadow_md()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(Icon::new(step.icon).size(px(20.0)).text_color(
                                        if active {
                                            theme.accent
                                        } else {
                                            theme.text_muted
                                        },
                                    )),
                            )
                            .child(
                                div()
                                    .absolute()
                                    .top(if is_top { px(-24.0) } else { px(50.0) })
                                    .whitespace_nowrap()
                                    .text_xs()
                                    .font_bold()
                                    .text_color(if active {
                                        theme.text_main
                                    } else {
                                        theme.text_muted
                                    })
                                    .child(step.label),
                            )
                    })),
            )
    }
}
