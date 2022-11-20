use leptos::*;
pub use todomvc_leptos::*;
fn main() {
    mount_to_body(|cx| view! { cx,  <TodoMVC/> })
}
