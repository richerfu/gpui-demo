use gpui::prelude::FluentBuilder as _;
use gpui::{px, AnyElement, IntoElement, ParentElement, Styled, Window};
use gpui_component::{
    button::{Button, ButtonGroup, ButtonVariant, ButtonVariants},
    h_flex,
    notification::Notification,
    v_flex, WindowExt as _,
};

use crate::ComponentGallery;

pub fn render(
    view: &mut ComponentGallery,
    window: &mut Window,
    cx: &mut gpui::Context<ComponentGallery>,
) -> AnyElement {
    let is_compact = window.bounds().size.width <= px(680.);
    view.card(
        "Buttons & Actions",
        v_flex()
            .gap_3()
            .child(if is_compact {
                v_flex()
                    .gap_2()
                    .child(Button::new("primary").label("Primary").primary().w_full())
                    .child(
                        Button::new("secondary")
                            .label("Secondary")
                            .with_variant(ButtonVariant::Secondary)
                            .w_full(),
                    )
                    .child(Button::new("outline").label("Outline").outline().w_full())
                    .child(Button::new("ghost").label("Ghost").ghost().w_full())
                    .into_any_element()
            } else {
                h_flex()
                    .gap_2()
                    .child(Button::new("primary").label("Primary").primary())
                    .child(
                        Button::new("secondary")
                            .label("Secondary")
                            .with_variant(ButtonVariant::Secondary),
                    )
                    .child(Button::new("outline").label("Outline").outline())
                    .child(Button::new("ghost").label("Ghost").ghost())
                    .into_any_element()
            })
            .child(
                ButtonGroup::new("actions")
                    .outline()
                    .compact()
                    .when(is_compact, |this| this.w_full())
                    .child(Button::new("left").label("Left"))
                    .child(Button::new("center").label("Center"))
                    .child(Button::new("right").label("Right")),
            )
            .child(if is_compact {
                v_flex()
                    .gap_2()
                    .child(Button::new("notify").label("Notify").w_full().on_click(
                        |_, window, cx| {
                            window.defer(cx, |window, cx| {
                                window.push_notification(
                                    Notification::new()
                                        .message("Notification triggered")
                                        .autohide(true),
                                    cx,
                                );
                            });
                        },
                    ))
                    .child(
                        Button::new("open-dialog")
                            .label("Open Dialog")
                            .primary()
                            .w_full()
                            .on_click(|_, window: &mut Window, cx| {
                                window.defer(cx, |window, cx| {
                                    window.open_dialog(cx, move |dialog, _, _| {
                                        dialog.title("Example Dialog").child(
                                            v_flex()
                                                .gap_2()
                                                .child("This is a dialog in GPUI Component.")
                                                .child(Button::new("ok").label("OK").primary()),
                                        )
                                    });
                                });
                            }),
                    )
                    .child(
                        Button::new("open-sheet")
                            .label("Open Sheet")
                            .outline()
                            .w_full()
                            .on_click(|_, window: &mut Window, cx| {
                                window.defer(cx, |window, cx| {
                                    window.open_sheet(cx, move |sheet, _, _| {
                                        sheet
                                            .title("Example Sheet")
                                            .child("This is a sheet drawer.")
                                    });
                                });
                            }),
                    )
                    .into_any_element()
            } else {
                h_flex()
                    .gap_3()
                    .child(
                        Button::new("notify")
                            .label("Notify")
                            .on_click(|_, window, cx| {
                                window.defer(cx, |window, cx| {
                                    window.push_notification(
                                        Notification::new()
                                            .message("Notification triggered")
                                            .autohide(true),
                                        cx,
                                    );
                                });
                            }),
                    )
                    .child(
                        Button::new("open-dialog")
                            .label("Open Dialog")
                            .primary()
                            .on_click(|_, window: &mut Window, cx| {
                                window.defer(cx, |window, cx| {
                                    window.open_dialog(cx, move |dialog, _, _| {
                                        dialog.title("Example Dialog").child(
                                            v_flex()
                                                .gap_2()
                                                .child("This is a dialog in GPUI Component.")
                                                .child(Button::new("ok").label("OK").primary()),
                                        )
                                    });
                                });
                            }),
                    )
                    .child(
                        Button::new("open-sheet")
                            .label("Open Sheet")
                            .outline()
                            .on_click(|_, window: &mut Window, cx| {
                                window.defer(cx, |window, cx| {
                                    window.open_sheet(cx, move |sheet, _, _| {
                                        sheet
                                            .title("Example Sheet")
                                            .child("This is a sheet drawer.")
                                    });
                                });
                            }),
                    )
                    .into_any_element()
            }),
        cx,
    )
    .into_any_element()
}
