use gpui::{AnyElement, IntoElement, ParentElement, Styled};
use gpui::{div, Window};
use gpui_component::{
    ActiveTheme as _, ElementExt as _, StyledExt as _, IconName, input::Input, v_flex,
};

use crate::ComponentGallery;

pub fn render(
    view: &mut ComponentGallery,
    _: &mut Window,
    cx: &mut gpui::Context<ComponentGallery>,
) -> AnyElement {
    v_flex()
        .gap_2()
        .child(
            div()
                .text_xl()
                .font_semibold()
                .child("GPUI Component Gallery"),
        )
        .child(
            div()
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child("Mobile-style shadcn UI demo on OpenHarmony"),
        )
        .child(Input::new(&view.input_state).prefix(IconName::Search).cleanable(true))
        .into_any_element()
}
