use std::rc::Rc;

use gpui::{
    AnyElement, IntoElement, ParentElement, SharedString, Styled, Window, div, px, size, App,
    Global,
};
use gpui_component::{
    ActiveTheme as _, StyledExt as _,
    IconName, Side,
    button::Button,
    chart::{AreaChart, BarChart, LineChart, PieChart},
    h_flex,
    list::{List, ListDelegate, ListItem, ListState},
    menu::{ContextMenuExt, DropdownMenu as _, PopupMenuItem},
    resizable::{h_resizable, resizable_panel},
    scroll::{ScrollableElement as _, ScrollbarAxis},
    setting::{SettingField, SettingGroup, SettingItem, SettingPage, Settings},
    sidebar::{Sidebar, SidebarFooter, SidebarHeader, SidebarMenu, SidebarMenuItem, SidebarToggleButton},
    table::{Column, Table, TableDelegate, TableState},
    tree::{TreeItem, TreeState, tree},
    v_flex,
    v_virtual_list,
};

use crate::ComponentGallery;

#[derive(Clone)]
pub struct SimpleListDelegate {
    items: Vec<SharedString>,
    selected: Option<usize>,
}

impl SimpleListDelegate {
    pub fn new() -> Self {
        Self {
            items: (1..=12)
                .map(|ix| SharedString::from(format!("List Item {}", ix)))
                .collect(),
            selected: None,
        }
    }
}

impl ListDelegate for SimpleListDelegate {
    type Item = ListItem;

    fn items_count(&self, _section: usize, _cx: &gpui::App) -> usize {
        self.items.len()
    }

    fn render_item(
        &mut self,
        ix: gpui_component::IndexPath,
        _window: &mut Window,
        _cx: &mut gpui::Context<ListState<Self>>,
    ) -> Option<Self::Item> {
        let label = self.items.get(ix.row)?.clone();
        Some(
            ListItem::new(ix)
                .selected(self.selected == Some(ix.row))
                .child(label),
        )
    }

    fn set_selected_index(
        &mut self,
        ix: Option<gpui_component::IndexPath>,
        _window: &mut Window,
        _cx: &mut gpui::Context<ListState<Self>>,
    ) {
        self.selected = ix.map(|ix| ix.row);
    }
}

#[derive(Clone)]
pub struct SimpleTableDelegate {
    columns: Vec<Column>,
    rows: Vec<[SharedString; 3]>,
}

#[derive(Default)]
pub struct DemoSettings {
    notifications: bool,
    username: SharedString,
}

impl Global for DemoSettings {}

impl DemoSettings {
    pub fn global(cx: &App) -> &DemoSettings {
        cx.global::<DemoSettings>()
    }

    pub fn global_mut(cx: &mut App) -> &mut DemoSettings {
        cx.global_mut::<DemoSettings>()
    }
}

impl SimpleTableDelegate {
    pub fn new() -> Self {
        let columns = vec![
            Column::new("id", "ID").width(px(60.)),
            Column::new("name", "Name").width(px(140.)),
            Column::new("status", "Status").width(px(120.)),
        ];
        let rows = (1..=8)
            .map(|ix| {
                [
                    SharedString::from(ix.to_string()),
                    SharedString::from(format!("Item {}", ix)),
                    SharedString::from(if ix % 2 == 0 { "Active" } else { "Pending" }),
                ]
            })
            .collect();

        Self { columns, rows }
    }
}

impl TableDelegate for SimpleTableDelegate {
    fn columns_count(&self, _cx: &gpui::App) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, _cx: &gpui::App) -> usize {
        self.rows.len()
    }

    fn column(&self, col_ix: usize, _cx: &gpui::App) -> Column {
        self.columns[col_ix].clone()
    }

    fn render_td(
        &mut self,
        row_ix: usize,
        col_ix: usize,
        _window: &mut Window,
        _cx: &mut gpui::Context<TableState<Self>>,
    ) -> impl IntoElement {
        self.rows[row_ix][col_ix].clone()
    }
}

#[derive(Clone)]
struct DailyMetric {
    day: SharedString,
    desktop: f64,
    mobile: f64,
}

fn chart_data() -> Vec<DailyMetric> {
    vec![
        DailyMetric { day: "Mon".into(), desktop: 120., mobile: 80. },
        DailyMetric { day: "Tue".into(), desktop: 160., mobile: 95. },
        DailyMetric { day: "Wed".into(), desktop: 140., mobile: 88. },
        DailyMetric { day: "Thu".into(), desktop: 190., mobile: 120. },
        DailyMetric { day: "Fri".into(), desktop: 170., mobile: 110. },
        DailyMetric { day: "Sat".into(), desktop: 130., mobile: 70. },
    ]
}

pub fn sample_tree_items() -> Vec<TreeItem> {
    vec![
        TreeItem::new("src", "src")
            .children(vec![
                TreeItem::new("lib", "lib.rs"),
                TreeItem::new("main", "main.rs"),
            ]),
        TreeItem::new("assets", "assets")
            .children(vec![TreeItem::new("logo", "logo.png")]),
        TreeItem::new("readme", "README.md"),
    ]
}

pub fn render(
    view: &mut ComponentGallery,
    window: &mut Window,
    cx: &mut gpui::Context<ComponentGallery>,
) -> AnyElement {
    let is_compact = window.bounds().size.width <= px(680.);
    let view_entity = cx.entity();
    v_flex()
        .gap_4()
        .child(view.card(
            "Menus & Context",
            v_flex()
                .gap_3()
                .child(
                    Button::new("menu")
                        .label("Dropdown Menu")
                        .outline()
                        .dropdown_menu(move |menu, window, cx| {
                            menu.item(
                                PopupMenuItem::new("Copy")
                                    .on_click(window.listener_for(&view_entity, |this, _, _, cx| {
                                        this.menu_message = "Copied".into();
                                        cx.notify();
                                    })),
                            )
                            .item(
                                PopupMenuItem::new("Refresh")
                                    .on_click(window.listener_for(&view_entity, |this, _, _, cx| {
                                        this.menu_message = "Refreshed".into();
                                        cx.notify();
                                    })),
                            )
                        }),
                )
                .child(
                    div()
                        .p_3()
                        .border_1()
                        .border_dashed()
                        .border_color(cx.theme().border)
                        .rounded_lg()
                        .child("Right click for context menu")
                        .context_menu(|this, _window, _cx| {
                            this.link("Docs", "https://gpui.rs")
                                .separator()
                                .item(PopupMenuItem::new("Inspect"))
                                .item(PopupMenuItem::new("Disable"))
                        }),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(cx.theme().muted_foreground)
                        .child(format!("Menu: {}", view.menu_message)),
                ),
            cx,
        ))
        .child(view.card(
            "List",
            List::new(&view.list_state)
                .p(px(8.))
                .border_1()
                .border_color(cx.theme().border)
                .rounded(cx.theme().radius),
            cx,
        ))
        .child(view.card(
            "Table",
            Table::new(&view.table_state).stripe(true),
            cx,
        ))
        .child(view.card(
            "Tree",
            tree(&view.tree_state, |ix, entry, selected, _window, _cx| {
                ListItem::new(ix)
                    .selected(selected)
                    .pl(px(12.) * entry.depth() as f32)
                    .child(entry.item().label.clone())
            })
            .border_1()
            .border_color(cx.theme().border)
            .rounded(cx.theme().radius)
            .p_2(),
            cx,
        ))
        .child(view.card(
            "Virtual List",
            div()
                .border_1()
                .border_color(cx.theme().border)
                .rounded(cx.theme().radius)
                .h(px(180.))
                .child(
                    v_virtual_list(
                        cx.entity(),
                        "virtual-list",
                        view.virtual_sizes.clone(),
                        move |story, visible_range: std::ops::Range<usize>, _, _| {
                            visible_range
                                .map(|ix| {
                                    div()
                                        .h(px(32.))
                                        .px_2()
                                        .items_center()
                                        .child(story.virtual_items[ix].clone())
                                })
                                .collect()
                        },
                    )
                    .track_scroll(&view.virtual_scroll),
                )
                .scrollbar(&view.virtual_scroll, ScrollbarAxis::Vertical),
            cx,
        ))
        .child(view.card(
            "Resizable",
            div()
                .h(px(180.))
                .border_1()
                .border_color(cx.theme().border)
                .child(if is_compact {
                    v_flex()
                        .gap_2()
                        .p_2()
                        .child(div().child("Left"))
                        .child(div().child("Center"))
                        .child(div().child("Right"))
                        .into_any_element()
                } else {
                    h_resizable("resizable")
                        .child(resizable_panel().size(px(120.)).child(div().child("Left")))
                        .child(resizable_panel().child(div().child("Center")))
                        .child(resizable_panel().size(px(120.)).child(div().child("Right")))
                        .into_any_element()
                }),
            cx,
        ))
        .child(view.card(
            "Sidebar",
            if is_compact {
                v_flex()
                    .gap_3()
                    .child(
                        Sidebar::new("sidebar")
                            .collapsed(view.sidebar_collapsed)
                            .side(if view.sidebar_side_right { Side::Right } else { Side::Left })
                            .header(
                                SidebarHeader::new()
                                    .child("Workspace")
                                    .text_sm(),
                            )
                            .child(
                                SidebarMenu::new()
                                    .child(
                                        SidebarMenuItem::new("Overview")
                                            .icon(IconName::LayoutDashboard)
                                            .active(true),
                                    )
                                    .child(
                                        SidebarMenuItem::new("Components")
                                            .icon(IconName::Frame)
                                            .children([
                                                SidebarMenuItem::new("Buttons"),
                                                SidebarMenuItem::new("Forms"),
                                            ]),
                                    )
                                    .child(
                                        SidebarMenuItem::new("Settings")
                                            .icon(IconName::Settings)
                                            .disable(true),
                                    ),
                            )
                            .footer(
                                SidebarFooter::new()
                                    .child("v0.1")
                                    .child(
                                        SidebarToggleButton::new()
                                            .side(if view.sidebar_side_right { Side::Right } else { Side::Left })
                                            .collapsed(view.sidebar_collapsed)
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.sidebar_collapsed = !this.sidebar_collapsed;
                                                cx.notify();
                                            })),
                                    ),
                            ),
                    )
                    .child(
                        v_flex()
                            .gap_2()
                            .child(
                                Button::new("toggle-side")
                                    .label("Toggle Side")
                                    .outline()
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.sidebar_side_right = !this.sidebar_side_right;
                                        cx.notify();
                                    })),
                            )
                            .child(
                                Button::new("toggle-collapse")
                                    .label("Toggle Collapse")
                                    .outline()
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.sidebar_collapsed = !this.sidebar_collapsed;
                                        cx.notify();
                                    })),
                            ),
                    )
                    .into_any_element()
            } else {
                h_flex()
                    .gap_3()
                    .child(
                        Sidebar::new("sidebar")
                            .collapsed(view.sidebar_collapsed)
                            .side(if view.sidebar_side_right { Side::Right } else { Side::Left })
                            .header(
                                SidebarHeader::new()
                                    .child("Workspace")
                                    .text_sm(),
                            )
                            .child(
                                SidebarMenu::new()
                                    .child(
                                        SidebarMenuItem::new("Overview")
                                            .icon(IconName::LayoutDashboard)
                                            .active(true),
                                    )
                                    .child(
                                        SidebarMenuItem::new("Components")
                                            .icon(IconName::Frame)
                                            .children([
                                                SidebarMenuItem::new("Buttons"),
                                                SidebarMenuItem::new("Forms"),
                                            ]),
                                    )
                                    .child(
                                        SidebarMenuItem::new("Settings")
                                            .icon(IconName::Settings)
                                            .disable(true),
                                    ),
                            )
                            .footer(
                                SidebarFooter::new()
                                    .child("v0.1")
                                    .child(
                                        SidebarToggleButton::new()
                                            .side(if view.sidebar_side_right { Side::Right } else { Side::Left })
                                            .collapsed(view.sidebar_collapsed)
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.sidebar_collapsed = !this.sidebar_collapsed;
                                                cx.notify();
                                            })),
                                    ),
                            ),
                    )
                    .child(
                        v_flex()
                            .gap_2()
                            .child(
                                Button::new("toggle-side")
                                    .label("Toggle Side")
                                    .outline()
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.sidebar_side_right = !this.sidebar_side_right;
                                        cx.notify();
                                    })),
                            )
                            .child(
                                Button::new("toggle-collapse")
                                    .label("Toggle Collapse")
                                    .outline()
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.sidebar_collapsed = !this.sidebar_collapsed;
                                        cx.notify();
                                    })),
                            ),
                    )
                    .into_any_element()
            },
            cx,
        ))
        .child(view.card(
            "Charts",
            {
                let data = chart_data();
                v_flex()
                    .gap_3()
                    .child(
                        AreaChart::new(data.clone())
                            .x(|d| d.day.clone())
                            .y(|d| d.desktop)
                            .y(|d| d.mobile),
                    )
                    .child(
                        BarChart::new(data.clone())
                            .x(|d| d.day.clone())
                            .y(|d| d.desktop),
                    )
                    .child(
                        LineChart::new(data.clone())
                            .x(|d| d.day.clone())
                            .y(|d| d.mobile),
                    )
                    .child(
                        PieChart::new(data.clone())
                            .value(|d| d.desktop as f32),
                    )
            },
            cx,
        ))
        .child(view.card(
            "Settings",
            Settings::new("demo-settings").pages(vec![
                SettingPage::new("General")
                    .default_open(true)
                    .group(
                        SettingGroup::new()
                            .title("Preferences")
                            .items([
                                SettingItem::new(
                                    "Notifications",
                                    SettingField::switch(
                                        |cx| DemoSettings::global(cx).notifications,
                                        |v, cx| DemoSettings::global_mut(cx).notifications = v,
                                    ),
                                )
                                .description("Receive push notifications"),
                                SettingItem::new(
                                    "Username",
                                    SettingField::input(
                                        |cx| DemoSettings::global(cx).username.clone(),
                                        |v, cx| DemoSettings::global_mut(cx).username = v,
                                    ),
                                )
                                .description("Public display name"),
                            ]),
                    ),
            ]),
            cx,
        ))
        .into_any_element()
}
