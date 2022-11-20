use spair::prelude::*;

pub const KEY: &str = "todos-spair-keyed";

impl<'a> spair::Keyed for super::RenderEntry<'a> {
    type Key = uuid::Uuid;
    fn key(&self) -> &Self::Key {
        &self.0.id
    }
}

pub fn render_list(e: spair::Element<super::App>) {
    let state = e.state();
    e.static_attributes().class("todo-list").keyed_list_clone(
        state.data.get_filtered_entries().map(super::RenderEntry),
    );
}
