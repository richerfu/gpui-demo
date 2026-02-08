use gpui::{AnyElement, IntoElement, ParentElement, Styled, Window};
use gpui_component::{
    accordion::Accordion,
    button::{Button, ButtonVariants},
    collapsible::Collapsible,
    v_flex, Sizable,
};

use crate::ComponentGallery;

pub fn render(
    view: &mut ComponentGallery,
    _window: &mut Window,
    cx: &mut gpui::Context<ComponentGallery>,
) -> AnyElement {
    view.card(
        "Disclosure",
        v_flex()
            .gap_3()
            .child(
                Accordion::new("accordion")
                    .item(|this| {
                        this.open(true)
                            .title("Is it shadcn style?")
                            .child("Yes, with subtle borders and muted backgrounds.")
                    })
                    .item(|this| {
                        this.title("Works on OHOS")
                            .child("This demo avoids gpui-component-story dependency.")
                    }),
            )
            .child(
                Collapsible::new()
                    .open(view.collapsible_open)
                    .child("Tap to reveal details")
                    .content("This content appears when the collapsible is open.")
                    .child(
                        Button::new("toggle")
                            .label(if view.collapsible_open {
                                "Show less"
                            } else {
                                "Show more"
                            })
                            .xsmall()
                            .link()
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.collapsible_open = !this.collapsible_open;
                                cx.notify();
                            })),
                    ),
            ),
        cx,
    )
    .into_any_element()
}
