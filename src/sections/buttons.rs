use gpui::{AnyElement, IntoElement, ParentElement, Styled, Window};
use gpui_component::{
    WindowExt as _,
    button::{Button, ButtonGroup, ButtonVariant, ButtonVariants},
    notification::Notification,
    v_flex, h_flex,
};

use crate::ComponentGallery;

pub fn render(
    view: &mut ComponentGallery,
    _window: &mut Window,
    cx: &mut gpui::Context<ComponentGallery>,
) -> AnyElement {
    view.card(
        "Buttons & Actions",
        v_flex()
            .gap_3()
            .child(
                h_flex()
                    .gap_2()
                    .child(Button::new("primary").label("Primary").primary())
                    .child(
                        Button::new("secondary")
                            .label("Secondary")
                            .with_variant(ButtonVariant::Secondary),
                    )
                    .child(Button::new("outline").label("Outline").outline())
                    .child(Button::new("ghost").label("Ghost").ghost()),
            )
            .child(
                ButtonGroup::new("actions")
                    .outline()
                    .compact()
                    .child(Button::new("left").label("Left"))
                    .child(Button::new("center").label("Center"))
                    .child(Button::new("right").label("Right")),
            )
            .child(
                h_flex()
                    .gap_3()
                    .child(
                        Button::new("notify")
                            .label("Notify")
                            .on_click(|_, window, cx| {
                                window.push_notification(
                                    Notification::new()
                                        .message("Notification triggered")
                                        .autohide(true),
                                    cx,
                                );
                            }),
                    )
                    .child(
                        Button::new("open-dialog")
                            .label("Open Dialog")
                            .primary()
                            .on_click(|_, window: &mut Window, cx| {
                                window.open_dialog(cx, move |dialog, _, _| {
                                    dialog.title("Example Dialog").child(
                                        v_flex()
                                            .gap_2()
                                            .child("This is a dialog in GPUI Component.")
                                            .child(
                                                Button::new("ok")
                                                    .label("OK")
                                                    .primary(),
                                            ),
                                    )
                                });
                            }),
                    )
                    .child(
                        Button::new("open-sheet")
                            .label("Open Sheet")
                            .outline()
                            .on_click(|_, window: &mut Window, cx| {
                                window.open_sheet(cx, move |sheet, _, _| {
                                    sheet
                                        .title("Example Sheet")
                                        .child("This is a sheet drawer.")
                                });
                            }),
                    ),
            ),
        cx,
    )
    .into_any_element()
}
