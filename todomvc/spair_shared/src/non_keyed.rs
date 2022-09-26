use spair::prelude::*;

pub const KEY: &str = "todos-spair-non-keyed";

pub fn render_list(e: spair::Element<super::App>) {
    let state = e.state();
    e.static_attributes().class("todo-list").list_clone(
        state.data.get_filtered_entries().map(super::RenderEntry),
    );
}
