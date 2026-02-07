use gpui::{AnyElement, IntoElement, ParentElement, Styled, Window, Keystroke, px};
use gpui_component::{
    ActiveTheme as _,
    Sizable,
    hover_card::HoverCard,
    kbd::Kbd,
    label::Label,
    link::Link,
    popover::Popover,
    text::markdown,
    v_flex, h_flex,
};
use gpui_component::button::{Button, ButtonVariants};

use crate::ComponentGallery;

pub fn render(
    view: &mut ComponentGallery,
    window: &mut Window,
    cx: &mut gpui::Context<ComponentGallery>,
) -> AnyElement {
    let is_compact = window.bounds().size.width <= px(680.);
    let mut content = v_flex().gap_3();
    let row = if is_compact {
        v_flex()
            .gap_2()
            .child(
                Button::new("tooltip")
                    .label("Tooltip")
                    .tooltip("This is a tooltip"),
            )
            .child(
                Popover::new("popover")
                    .trigger(Button::new("pop").label("Popover").outline())
                    .content(|_, _, _| {
                        v_flex()
                            .gap_2()
                            .p_3()
                            .child("Quick actions")
                            .child(Button::new("copy").label("Copy link").small())
                    }),
            )
            .child(
                HoverCard::new("hover")
                    .trigger(Button::new("hover-trigger").label("Hover").ghost())
                    .child(
                        v_flex()
                            .gap_1()
                            .child("Hover card content")
                            .child(
                                gpui::div()
                                    .text_sm()
                                    .text_color(cx.theme().muted_foreground)
                                    .child("Additional info"),
                            ),
                    ),
            )
            .into_any_element()
    } else {
        h_flex()
            .gap_3()
            .items_center()
            .child(
                Button::new("tooltip")
                    .label("Tooltip")
                    .tooltip("This is a tooltip"),
            )
            .child(
                Popover::new("popover")
                    .trigger(Button::new("pop").label("Popover").outline())
                    .content(|_, _, _| {
                        v_flex()
                            .gap_2()
                            .p_3()
                            .child("Quick actions")
                            .child(Button::new("copy").label("Copy link").small())
                    }),
            )
            .child(
                HoverCard::new("hover")
                    .trigger(Button::new("hover-trigger").label("Hover").ghost())
                    .child(
                        v_flex()
                            .gap_1()
                            .child("Hover card content")
                            .child(
                                gpui::div()
                                    .text_sm()
                                    .text_color(cx.theme().muted_foreground)
                                    .child("Additional info"),
                            ),
                    ),
            )
            .into_any_element()
    };
    content = content
        .child(row)
        .child(
            if is_compact {
                v_flex()
                    .gap_2()
                    .items_center()
                    .child(Kbd::new(Keystroke::parse("cmd-k").unwrap()))
                    .child(Label::new("Command Palette"))
                    .child(Link::new("link").href("https://gpui.rs").child("Docs"))
                    .into_any_element()
            } else {
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(Kbd::new(Keystroke::parse("cmd-k").unwrap()))
                    .child(Label::new("Command Palette"))
                    .child(Link::new("link").href("https://gpui.rs").child("Docs"))
                    .into_any_element()
            },
        );
    let show_markdown = false;
    if show_markdown && view.markdown_ready {
        content = content.child(markdown(
            "**Markdown** is supported here: _inline emphasis_, lists, and more.",
        ));
    }

    view.card(
        "Overlay & Helpers",
        content,
        cx,
    )
    .into_any_element()
}
