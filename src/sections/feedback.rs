use gpui::{AnyElement, IntoElement, ParentElement, Styled, Window, px};
use gpui_component::{
    StyledExt as _,
    alert::Alert,
    progress::{Progress, ProgressCircle},
    skeleton::Skeleton,
    spinner::Spinner,
    v_flex, h_flex,
};

use crate::ComponentGallery;

pub fn render(
    view: &mut ComponentGallery,
    window: &mut Window,
    cx: &mut gpui::Context<ComponentGallery>,
) -> AnyElement {
    let is_compact = window.bounds().size.width <= px(680.);
    view.card(
        "Status & Feedback",
        v_flex()
            .gap_3()
            .child(Alert::info("alert", "This is an info alert.").title("Heads up"))
            .child(
                if is_compact {
                    v_flex()
                        .gap_2()
                        .child(Progress::new("progress").value(view.progress_value))
                        .child(
                            ProgressCircle::new("progress-circle").value(
                                view.progress_value,
                            ),
                        )
                        .into_any_element()
                } else {
                    h_flex()
                        .gap_3()
                        .items_center()
                        .child(Progress::new("progress").value(view.progress_value))
                        .child(
                            ProgressCircle::new("progress-circle").value(
                                view.progress_value,
                            ),
                        )
                        .into_any_element()
                },
            )
            .child(
                if is_compact {
                    v_flex()
                        .gap_2()
                        .items_center()
                        .child(Spinner::new())
                        .child(Skeleton::new().w(gpui::px(140.)).h(gpui::px(12.)))
                        .child(Skeleton::new().w(gpui::px(64.)).h(gpui::px(36.)))
                        .into_any_element()
                } else {
                    h_flex()
                        .gap_3()
                        .items_center()
                        .child(Spinner::new())
                        .child(Skeleton::new().w(gpui::px(140.)).h(gpui::px(12.)))
                        .child(Skeleton::new().w(gpui::px(64.)).h(gpui::px(36.)))
                        .into_any_element()
                },
            ),
        cx,
    )
    .into_any_element()
}
