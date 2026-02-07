use gpui::{AnyElement, IntoElement, ParentElement, Styled, Window, px};
use gpui_component::{
    StyledExt as _,
    avatar::Avatar,
    badge::Badge,
    description_list::DescriptionList,
    divider::Divider,
    group_box::GroupBox,
    tag::Tag,
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
        "Data Display",
        v_flex()
            .gap_3()
            .child(
                DescriptionList::new()
                    .columns(2)
                    .item("Name", "GPUI Component", 1)
                    .item("Platform", "OpenHarmony", 1)
                    .item("Theme", "shadcn", 1)
                    .item("Version", "0.1", 1),
            )
            .child(
                GroupBox::new()
                    .title("Summary")
                    .child(
                        v_flex()
                            .gap_2()
                            .child("Compact card-like grouping")
                            .child(Divider::horizontal())
                            .child(
                                if is_compact {
                                    v_flex()
                                        .gap_2()
                                        .child(Tag::new().child("New"))
                                        .child(Badge::new().count(3))
                                        .child(Avatar::new().name("GP"))
                                        .into_any_element()
                                } else {
                                    h_flex()
                                        .gap_2()
                                        .child(Tag::new().child("New"))
                                        .child(Badge::new().count(3))
                                        .child(Avatar::new().name("GP"))
                                        .into_any_element()
                                },
                            ),
                    ),
            ),
        cx,
    )
    .into_any_element()
}
