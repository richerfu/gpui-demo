use gpui::{div, px, AnyElement, IntoElement, ParentElement, Styled, Window};
use gpui_component::{
    checkbox::Checkbox,
    clipboard::Clipboard,
    color_picker::ColorPicker,
    date_picker::DatePicker,
    form::{field, v_form},
    h_flex,
    input::Input,
    radio::Radio,
    select::Select,
    switch::Switch,
    v_flex, ActiveTheme as _, Sizable, WindowExt as _,
};

use crate::ComponentGallery;

pub fn render(
    view: &mut ComponentGallery,
    window: &mut Window,
    cx: &mut gpui::Context<ComponentGallery>,
) -> AnyElement {
    let is_compact = window.bounds().size.width <= px(680.);
    view.card(
        "Form Inputs",
        v_flex()
            .gap_3()
            .child(Input::new(&view.input_state).cleanable(true))
            .child(Input::new(&view.textarea_state).h(gpui::px(120.)))
            .child(
                Select::new(&view.select_state)
                    .placeholder("Select an option")
                    .search_placeholder("Search options")
                    .cleanable(true),
            )
            .child(if is_compact {
                v_flex()
                    .gap_2()
                    .child(
                        Checkbox::new("agree")
                            .label("Agree")
                            .checked(view.checkbox_checked)
                            .on_click(cx.listener(|this, checked, _, cx| {
                                this.checkbox_checked = *checked;
                                cx.notify();
                            })),
                    )
                    .child(
                        Switch::new("switch")
                            .label("Enable")
                            .checked(view.switch_on)
                            .on_click(cx.listener(|this, checked, _, cx| {
                                this.switch_on = *checked;
                                cx.notify();
                            })),
                    )
                    .into_any_element()
            } else {
                h_flex()
                    .gap_3()
                    .items_center()
                    .child(
                        Checkbox::new("agree")
                            .label("Agree")
                            .checked(view.checkbox_checked)
                            .on_click(cx.listener(|this, checked, _, cx| {
                                this.checkbox_checked = *checked;
                                cx.notify();
                            })),
                    )
                    .child(
                        Switch::new("switch")
                            .label("Enable")
                            .checked(view.switch_on)
                            .on_click(cx.listener(|this, checked, _, cx| {
                                this.switch_on = *checked;
                                cx.notify();
                            })),
                    )
                    .into_any_element()
            })
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        Radio::new("radio1")
                            .label("Option 1")
                            .checked(view.radio_checked)
                            .on_click(cx.listener(|this, v, _, cx| {
                                this.radio_checked = *v;
                                cx.notify();
                            })),
                    )
                    .child(
                        Radio::new("radio2")
                            .label("Option 2")
                            .checked(!view.radio_checked)
                            .on_click(cx.listener(|this, v: &bool, _, cx| {
                                this.radio_checked = !*v;
                                cx.notify();
                            })),
                    ),
            )
            .child(if is_compact {
                v_flex()
                    .gap_2()
                    .child(DatePicker::new(&view.date_picker))
                    .child(ColorPicker::new(&view.color_picker).small())
                    .into_any_element()
            } else {
                h_flex()
                    .gap_3()
                    .child(DatePicker::new(&view.date_picker))
                    .child(ColorPicker::new(&view.color_picker).small())
                    .into_any_element()
            })
            .child(
                Input::new(&view.input_state).suffix(
                    Clipboard::new("clipboard")
                        .value_fn({
                            let state = view.input_state.clone();
                            move |_, cx| state.read(cx).value()
                        })
                        .on_copied(|value, window, cx| {
                            window.push_notification(format!("Copied: {}", value), cx)
                        }),
                ),
            )
            .child(
                v_form()
                    .label_width(gpui::px(120.))
                    .child(field().label("Name").child(Input::new(&view.form_name)))
                    .child(field().label("Email").child(Input::new(&view.form_email)))
                    .child(field().label("Role").child(Select::new(&view.form_role)))
                    .child(
                        field().label("Subscribe").child(
                            Switch::new("subscribe")
                                .checked(view.form_subscribe)
                                .label("Email updates")
                                .on_click(cx.listener(|this, checked, _, cx| {
                                    this.form_subscribe = *checked;
                                    cx.notify();
                                })),
                        ),
                    )
                    .child(
                        field()
                            .label("Notes")
                            .child(Input::new(&view.form_notes).h(gpui::px(90.))),
                    ),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child(format!(
                        "Input: {} | Selected: {}",
                        view.input_state.read(cx).value(),
                        view.select_state
                            .read(cx)
                            .selected_value()
                            .cloned()
                            .unwrap_or_else(|| "(none)".into())
                    )),
            ),
        cx,
    )
    .into_any_element()
}
