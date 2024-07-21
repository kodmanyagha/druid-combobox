use druid::Data;

#[derive(Clone, Data)]
pub struct AppData {
    pub counter: i32,
    pub title: String,
}
