pub mod data;
pub mod widgets;

use data::AppData;
use druid::{
    widget::{Button, Flex, Label},
    AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc,
};
use widgets::combobox;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder());
    let data = AppData {
        counter: 0,
        title: "App here".to_string(),
    };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
}

fn ui_builder() -> impl Widget<AppData> {
    let text = LocalizedString::new("hello-counter")
        .with_arg("count", |data: &AppData, _env| data.counter.into());
    let label = Label::new(text).padding(5.0).center();

    let button = Button::new("increment")
        .on_click(|_ctx, data: &mut AppData, _env| data.counter += 1)
        .padding(5.0);

    let combobox_1 = combobox::ComboBox::<String>::new();

    Flex::column()
        .with_child(label)
        .with_child(button)
        .with_child(combobox_1)
}
