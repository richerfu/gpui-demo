use gpui::{AnyElement, IntoElement, ParentElement, Styled, Window, div};
use gpui_component::{
    StyledExt as _,
    rating::Rating,
    slider::Slider,
    stepper::{Stepper, StepperItem},
    v_flex,
};

use crate::ComponentGallery;

pub fn render(
    view: &mut ComponentGallery,
    _window: &mut Window,
    cx: &mut gpui::Context<ComponentGallery>,
) -> AnyElement {
    let slider_value = view.slider_state.read(cx).value().start();

    view.card(
        "Sliders & Rating",
        v_flex()
            .gap_3()
            .child(
                v_flex()
                    .gap_2()
                    .child(div().child("Volume"))
                    .child(Slider::new(&view.slider_state))
                    .child(div().text_sm().child(format!("{}%", slider_value as i32)))
                    .child(
                        Rating::new("rating")
                            .value(view.rating_value)
                            .max(5)
                            .on_click(cx.listener(|this, value, _, cx| {
                                this.rating_value = *value;
                                cx.notify();
                            })),
                    ),
            )
            .child(
                Stepper::new("stepper")
                    .selected_index(view.stepper_step)
                    .on_click(cx.listener(|this, step, _, cx| {
                        this.stepper_step = *step;
                        cx.notify();
                    }))
                    .item(StepperItem::new().child("Login"))
                    .item(StepperItem::new().child("Verify"))
                    .item(StepperItem::new().child("Done")),
            ),
        cx,
    )
    .into_any_element()
}
