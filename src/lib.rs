use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, SharedString, Window,
    WindowBounds, WindowOptions,
};

use log::{debug, error, LevelFilter};
use ohos_hilog_binding::log::Config;
use openharmony_ability::OpenHarmonyApp;

// On non-OHOS platforms, we don't need these imports

struct OhosHello {
    text: SharedString,
}

impl Render for OhosHello {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .bg(rgb(0x2c3e50))
            .size_full()
            .justify_center()
            .items_center()
            .child(
                div()
                    .text_2xl()
                    .text_color(rgb(0xecf0f1))
                    .font_weight(gpui::FontWeight::BOLD)
                    .child(format!("Hello from OpenHarmony! {}", &self.text)),
            )
            .child(
                div()
                    .text_lg()
                    .text_color(rgb(0xbdc3c7))
                    .child("GPUI is running on OHOS"),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .mt_4()
                    .child(
                        div()
                            .size_12()
                            .bg(rgb(0xe74c3c))
                            .rounded_full()
                            .border_2()
                            .border_color(rgb(0xffffff)),
                    )
                    .child(
                        div()
                            .size_12()
                            .bg(rgb(0x27ae60))
                            .rounded_full()
                            .border_2()
                            .border_color(rgb(0xffffff)),
                    )
                    .child(
                        div()
                            .size_12()
                            .bg(rgb(0x3498db))
                            .rounded_full()
                            .border_2()
                            .border_color(rgb(0xffffff)),
                    ),
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
            let info = inner_app.content_rect();
            let default_size = size(px(info.width as _), px(info.height as _));
            let bounds = Bounds::centered(None, default_size, cx);

            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|_| OhosHello {
                        text: "OHOS".into(),
                    })
                },
            )
            .unwrap();

            cx.activate(true);
        });
}
