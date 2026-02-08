use gpui::prelude::FluentBuilder as _;
use gpui::{px, AnyElement, IntoElement, ParentElement, Styled, Window};
use gpui_component::{
    breadcrumb::{Breadcrumb, BreadcrumbItem},
    pagination::Pagination,
    tab::{Tab, TabBar},
    v_flex,
};

use crate::ComponentGallery;

pub fn render(
    view: &mut ComponentGallery,
    window: &mut Window,
    cx: &mut gpui::Context<ComponentGallery>,
) -> AnyElement {
    let is_compact = window.bounds().size.width <= px(680.);
    view.card(
        "Navigation",
        v_flex()
            .gap_3()
            .child(
                Breadcrumb::new()
                    .child("Home")
                    .child("Components")
                    .child(BreadcrumbItem::new("Gallery")),
            )
            .child(
                TabBar::new("tabs")
                    .selected_index(view.tab_index)
                    .when(is_compact, |this| this.w_full())
                    .on_click(cx.listener(|this, ix, _, cx| {
                        this.tab_index = *ix;
                        cx.notify();
                    }))
                    .child(Tab::new().label("Overview"))
                    .child(Tab::new().label("Details"))
                    .child(Tab::new().label("Stats")),
            )
            .child(
                Pagination::new("pagination")
                    .current_page(view.pagination_page)
                    .total_pages(8)
                    .when(is_compact, |this| this.w_full())
                    .on_click({
                        let entity = cx.entity();
                        move |page, _, cx| {
                            entity.update(cx, |this, cx| {
                                this.pagination_page = *page;
                                cx.notify();
                            });
                        }
                    }),
            ),
        cx,
    )
    .into_any_element()
}
