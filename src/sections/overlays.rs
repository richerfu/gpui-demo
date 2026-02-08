use gpui::{px, AnyElement, IntoElement, Keystroke, ParentElement, Styled, Window};
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::{
    h_flex, hover_card::HoverCard, kbd::Kbd, label::Label, link::Link, popover::Popover,
    text::markdown, v_flex, ActiveTheme as _, Sizable,
};

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
                    .tooltip("This is a tooltip")
                    .w_full(),
            )
            .child(
                Popover::new("popover")
                    .trigger(Button::new("pop").label("Popover").outline().w_full())
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
                    .trigger(Button::new("hover-trigger").label("Hover").ghost().w_full())
                    .child(
                        v_flex().gap_1().child("Hover card content").child(
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
                        v_flex().gap_1().child("Hover card content").child(
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
        .child(if is_compact {
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
        })
        .child(
            markdown(
                "**Markdown** demo:\n\n- 支持列表\n- 支持 _强调_\n- 支持 `inline code`\n\n> 已重新开启 markdown 渲染，用于验证当前修复是否生效。",
            )
            .w_full(),
        );

    let _ = view;
    view.card("Overlay & Helpers", content, cx)
        .into_any_element()
}
