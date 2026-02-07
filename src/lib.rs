use std::rc::Rc;
use std::time::Duration;

use gpui::{
    div, prelude::*, px, size, App, Application, Bounds, Context, Entity, SharedString, Window,
    WindowBounds, WindowOptions,
};
use gpui_component::{
    ActiveTheme as _, ElementExt as _, StyledExt as _, Root,
    scroll::ScrollableElement as _,
    color_picker::{ColorPickerEvent, ColorPickerState},
    date_picker::DatePickerState,
    input::InputState,
    list::ListState,
    select::SelectState,
    slider::{SliderEvent, SliderState},
    table::TableState,
    tree::TreeState,
    v_flex,
    VirtualListScrollHandle,
};

use log::LevelFilter;
use ohos_hilog_binding::log::Config;
use openharmony_ability::OpenHarmonyApp;

mod sections;

// On non-OHOS platforms, we don't need these imports

pub(crate) struct ComponentGallery {
    pub(crate) input_state: Entity<InputState>,
    pub(crate) textarea_state: Entity<InputState>,
    pub(crate) select_state: Entity<SelectState<Vec<SharedString>>>,
    pub(crate) date_picker: Entity<DatePickerState>,
    pub(crate) color_picker: Entity<ColorPickerState>,
    pub(crate) checkbox_checked: bool,
    pub(crate) switch_on: bool,
    pub(crate) radio_checked: bool,
    pub(crate) rating_value: usize,
    pub(crate) stepper_step: usize,
    pub(crate) pagination_page: usize,
    pub(crate) tab_index: usize,
    pub(crate) collapsible_open: bool,
    pub(crate) slider_state: Entity<SliderState>,
    pub(crate) slider_value: f32,
    pub(crate) progress_value: f32,
    pub(crate) form_name: Entity<InputState>,
    pub(crate) form_email: Entity<InputState>,
    pub(crate) form_role: Entity<SelectState<Vec<SharedString>>>,
    pub(crate) form_notes: Entity<InputState>,
    pub(crate) form_subscribe: bool,
    pub(crate) list_state: Entity<ListState<sections::advanced::SimpleListDelegate>>,
    pub(crate) table_state: Entity<TableState<sections::advanced::SimpleTableDelegate>>,
    pub(crate) tree_state: Entity<TreeState>,
    pub(crate) virtual_items: Vec<SharedString>,
    pub(crate) virtual_sizes: Rc<Vec<gpui::Size<gpui::Pixels>>>,
    pub(crate) virtual_scroll: VirtualListScrollHandle,
    pub(crate) sidebar_collapsed: bool,
    pub(crate) sidebar_side_right: bool,
    pub(crate) menu_message: SharedString,
    pub(crate) markdown_ready: bool,
    _subscriptions: Vec<gpui::Subscription>,
}

impl ComponentGallery {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.set_global::<sections::advanced::DemoSettings>(
            sections::advanced::DemoSettings::default(),
        );

        let input_state = cx.new(|cx| InputState::new(window, cx).placeholder("Type here..."));
        let textarea_state = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .rows(4)
                .placeholder("Write something...")
        });
        let select_state = cx.new(|cx| {
            SelectState::new(
                vec![
                    SharedString::from("Option A"),
                    SharedString::from("Option B"),
                    SharedString::from("Option C"),
                ],
                None,
                window,
                cx,
            )
            .searchable(true)
        });
        let date_picker = cx.new(|cx| DatePickerState::new(window, cx));
        let color_picker = cx.new(|cx| ColorPickerState::new(window, cx));
        let slider_state = cx.new(|_| {
            SliderState::new()
                .min(0.)
                .max(100.)
                .default_value(35.)
                .step(1.)
        });

        let form_name = cx.new(|cx| InputState::new(window, cx).placeholder("Name"));
        let form_email = cx.new(|cx| InputState::new(window, cx).placeholder("Email"));
        let form_role = cx.new(|cx| {
            SelectState::new(
                vec![
                    SharedString::from("Admin"),
                    SharedString::from("Editor"),
                    SharedString::from("Viewer"),
                ],
                None,
                window,
                cx,
            )
        });
        let form_notes = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .rows(3)
                .placeholder("Notes")
        });

        let list_state = cx.new(|cx| {
            ListState::new(sections::advanced::SimpleListDelegate::new(), window, cx)
        });
        let table_state = cx.new(|cx| {
            TableState::new(sections::advanced::SimpleTableDelegate::new(), window, cx)
        });
        let tree_state = cx.new(|cx| {
            TreeState::new(cx).items(sections::advanced::sample_tree_items())
        });

        let virtual_items = (1..=40)
            .map(|ix| SharedString::from(format!("Row {}", ix)))
            .collect::<Vec<_>>();
        let virtual_sizes = Rc::new(vec![size(px(1.), px(32.)); virtual_items.len()]);
        let virtual_scroll = VirtualListScrollHandle::new();

        let mut _subscriptions = vec![cx.subscribe(&slider_state, |this, _, ev, cx| {
            if let SliderEvent::Change(value) = ev {
                this.slider_value = value.start();
                this.progress_value = (this.slider_value / 100.0).clamp(0.0, 1.0) * 100.0;
                cx.notify();
            }
        })];

        _subscriptions.push(cx.subscribe(&color_picker, |this, _, ev, cx| {
            if let ColorPickerEvent::Change(_) = ev {
                cx.notify();
                let _ = this;
            }
        }));

        let mut view = Self {
            input_state,
            textarea_state,
            select_state,
            date_picker,
            color_picker,
            checkbox_checked: true,
            switch_on: false,
            radio_checked: true,
            rating_value: 3,
            stepper_step: 1,
            pagination_page: 2,
            tab_index: 0,
            collapsible_open: false,
            slider_state,
            slider_value: 35.0,
            progress_value: 35.0,
            form_name,
            form_email,
            form_role,
            form_notes,
            form_subscribe: true,
            list_state,
            table_state,
            tree_state,
            virtual_items,
            virtual_sizes,
            virtual_scroll,
            sidebar_collapsed: false,
            sidebar_side_right: false,
            menu_message: "Idle".into(),
            markdown_ready: !cfg!(gles),
            _subscriptions,
        };

        if cfg!(gles) {
            cx.spawn(async move |this, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(300))
                    .await;
                let _ = this.update(cx, |view, cx| {
                    view.markdown_ready = true;
                    cx.notify();
                });
            })
            .detach();
        }

        view
    }

    pub(crate) fn card<'a>(
        &self,
        title: impl Into<SharedString>,
        content: impl IntoElement + 'a,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + 'a {
        let title = title.into();
        v_flex()
            .gap_3()
            .p_4()
            .w_full()
            .bg(cx.theme().muted)
            .rounded_lg()
            .border_1()
            .border_color(cx.theme().border)
            .child(
                div()
                    .text_sm()
                    .font_semibold()
                    .text_color(cx.theme().muted_foreground)
                    .child(title),
            )
            .child(content)
    }
}

impl Render for ComponentGallery {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_compact = window.bounds().size.width <= px(680.);

        let content = v_flex()
            .w_full()
            .mx_auto()
            .max_w(px(960.))
            .overflow_x_hidden()
            .when(is_compact, |this| this.px_3().py_4().gap_3())
            .when(!is_compact, |this| this.px_6().py_6().gap_4())
            .child(sections::header::render(self, window, cx))
            .child(sections::buttons::render(self, window, cx))
            .child(sections::forms::render(self, window, cx))
            .child(sections::feedback::render(self, window, cx))
            .child(sections::controls::render(self, window, cx))
            .child(sections::navigation::render(self, window, cx))
            .child(sections::disclosure::render(self, window, cx))
            .child(sections::data_display::render(self, window, cx))
            .child(sections::overlays::render(self, window, cx))
            .child(sections::advanced::render(self, window, cx));

        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .child(
                v_flex()
                    .size_full()
                    .flex_1()
                    .overflow_y_scrollbar()
                    .child(content),
            )
    }
}

#[openharmony_ability_derive::ability]
pub fn openharmony_app(app: OpenHarmonyApp) {
    ohos_hilog_binding::log::init_once(Config::default().with_max_level(LevelFilter::Debug));

    let inner_app = app.clone();
    // Initialize and run GPUI application
    // The event loop is automatically integrated by the platform
    Application::new()
        .with_ohos_app(app.clone())
        .run(move |cx: &mut App| {
            gpui_component::init(cx);
            let info = inner_app.content_rect();
            let default_size = size(px(info.width as _), px(info.height as _));
            let bounds = Bounds::centered(None, default_size, cx);

            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|cx| ComponentGallery::new(window, cx));
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )
            .unwrap();
            cx.activate(true);
        });
}
