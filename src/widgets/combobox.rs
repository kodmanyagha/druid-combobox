use druid::Widget;

pub struct ComboBoxItem<V> {
    key: u32,
    val: V,
}

pub struct ComboBox<T> {
    data: Vec<ComboBoxItem<T>>,
    selected_index: Option<u32>,
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum ComboBoxError {
    #[error("That key not found: {0}")]
    KeyNotExist(u32),
}

impl<T> ComboBox<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        ComboBox {
            data: Vec::new(),
            selected_index: None,
        }
    }

    pub fn get_selected_index(&self) -> Option<u32> {
        self.selected_index
    }

    pub fn get_selected_val(&self) -> Option<T> {
        if self.selected_index.is_none() {
            return None;
        }

        Some(
            self.data
                .iter()
                .find(|item| item.key == self.selected_index.unwrap())
                .unwrap()
                .val
                .clone(),
        )
    }

    pub fn set_selected_index(&mut self, key: u32) -> anyhow::Result<(), ComboBoxError> {
        self.data
            .iter()
            .find(|item| item.key == key)
            .ok_or(ComboBoxError::KeyNotExist(key))?;

        self.selected_index = Some(key);

        // TODO Handle view updates.

        Ok(())
    }
}

impl<T> Widget<T> for ComboBox<T> {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut T,
        env: &druid::Env,
    ) {
        todo!()
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &T,
        env: &druid::Env,
    ) {
        todo!()
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &T, data: &T, env: &druid::Env) {
        todo!()
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &T,
        env: &druid::Env,
    ) -> druid::Size {
        todo!()
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &druid::Env) {
        todo!()
    }
}
