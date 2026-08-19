use gpui::*;
use std::fmt::Display;
use std::sync::Arc;

#[derive(IntoElement)]
pub struct DropDown<T>
where
    T: Clone + Display + PartialEq + 'static,
{
    selected: T,
    options: Vec<T>,
    on_select: Arc<dyn Fn(&T, &mut Window, &mut Context<Self>) + 'static>,
}

impl<T> DropDown<T>
where
    T: Clone + Display + PartialEq + 'static,
{
    pub fn new(
        selected: T,
        options: Vec<T>,
        on_select: impl Fn(&T, &mut Window, &mut Context<Self>) + 'static,
    ) -> Self {
        Self {
            selected,
            options,
            on_select: Arc::new(on_select),
        }
    }
}

impl<T> RenderOnce for DropDown<T>
where
    T: Clone + Display + PartialEq + 'static,
{
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let selected_label = self.selected.to_string();

        div()
            .flex()
            .flex_col()
            .child(
                // Selected item trigger button
                div()
                    .px_3()
                    .py_1_5()
                    .bg(rgb(0x23272e))
                    .text_color(rgb(0xabb2bf))
                    .rounded_md()
                    .child(selected_label),
            )
            // Render options list dynamically
            .children(self.options.into_iter().map(|option| {
                let label = option.to_string();
                let is_active = option == self.selected;

                div()
                    .px_3()
                    .py_1()
                    .bg(if is_active {
                        rgb(0x2c313a)
                    } else {
                        rgb(0x1e2227)
                    })
                    .text_color(rgb(0x61afef))
                    .hover(|style| style.bg(rgb(0x3e4451)))
                    .child(label)
            }))
    }
}
