use druid::{Data, Widget, WidgetExt};

pub struct ComboBoxItem {
    key: u32,
    label: String,
}

pub struct ComboBox {
    items: Vec<ComboBoxItem>,
    selected_key: Option<u32>,
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum ComboBoxError {
    #[error("That key not found: {0}")]
    KeyNotExist(u32),
}

impl<T: Data> Widget<T> for ComboBox {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut T,
        env: &druid::Env,
    ) {
        println!("event invoked");
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &T,
        env: &druid::Env,
    ) {
        println!("lifecycle invoked");
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &T, data: &T, env: &druid::Env) {
        println!("update invoked");
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &T,
        env: &druid::Env,
    ) -> druid::Size {
        println!("layout invoked");

        druid::Size {
            width: 0_f64,
            height: 0_f64,
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &druid::Env) {
        println!("paint invoked");
    }
}

impl ComboBox {
    pub fn new() -> Self {
        ComboBox {
            items: Vec::new(),
            selected_key: None,
        }
    }

    pub fn get_selected_key(&self) -> Option<u32> {
        self.selected_key
    }

    pub fn get_selected_label(&self) -> Option<String> {
        self.selected_key?;

        Some(
            self.items
                .iter()
                .find(|item| item.key == self.selected_key.unwrap())
                .unwrap()
                .label
                .clone(),
        )
    }

    pub fn set_selected_index(&mut self, key: u32) -> anyhow::Result<(), ComboBoxError> {
        self.items
            .iter()
            .find(|item| item.key == key)
            .ok_or(ComboBoxError::KeyNotExist(key))?;

        self.selected_key = Some(key);

        Ok(())
    }

    pub fn add_item(mut self, key: u32, label: String) -> Self {
        self.items.push(ComboBoxItem { key, label });
        self
    }

    pub fn add_items(mut self, items: Vec<ComboBoxItem>) -> Self {
        items.into_iter().for_each(|item| self.items.push(item));
        self
    }
}

impl Default for ComboBox {
    fn default() -> Self {
        Self::new()
    }
}
