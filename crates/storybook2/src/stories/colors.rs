use crate::story::Story;
use gpui2::{px, Div, Render};
use ui::prelude::*;

pub struct ColorsStory;

impl Render for ColorsStory {
    type Element = Div<Self>;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        let color_scales = theme2::default_color_scales();

        Story::container(cx)
            .child(Story::title(cx, "Colors"))
            .child(
                div()
                    .id("colors")
                    .flex()
                    .flex_col()
                    .gap_1()
                    .overflow_y_scroll()
                    .text_color(gpui2::white())
                    .children(color_scales.into_iter().map(|(name, scale)| {
                        div()
                            .flex()
                            .child(
                                div()
                                    .w(px(75.))
                                    .line_height(px(24.))
                                    .child(name.to_string()),
                            )
                            .child(div().flex().gap_1().children(
                                (1..=12).map(|step| div().flex().size_6().bg(scale.step(cx, step))),
                            ))
                    })),
            )
    }
}