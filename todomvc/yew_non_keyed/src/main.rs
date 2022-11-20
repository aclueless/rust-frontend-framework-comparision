use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement as InputElement;
use strum::IntoEnumIterator;

use yew::events::{FocusEvent, KeyboardEvent};
use yew::html::Scope;
use yew::{classes, html, Classes, Component, Context, Html, NodeRef, TargetCast};

use todomvc_shared::{TodoEntry, Filter, Todos};

const KEY: &str = "yew.todomvc.self";

pub enum Msg {
    Add(String),
    Edit((uuid::Uuid, String)),
    Remove(uuid::Uuid),
    SetFilter(Filter),
    ToggleAll,
    ToggleEdit(uuid::Uuid),
    Toggle(uuid::Uuid),
    ClearCompleted,
    Focus,
}

pub struct App {
    data: Todos,
    edit_value: String,
    editing_id: Option<uuid::Uuid>,

    focus_ref: NodeRef,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let focus_ref = NodeRef::default();
        Self {
            data: Todos::load_todos(KEY),
            edit_value: "".into(),
            editing_id: None,
            focus_ref
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add(description) => {
                self.data.new_entry(description);
            }
            Msg::Edit((id, edit_value)) => {
                if edit_value.is_empty() {
                    self.data.remove_by_id(&id);
                } else if let Some(e) = self.data.get_entry_by_id_mut(&id) {
                    e.description = edit_value;
                }
                self.edit_value = "".to_string();
            }
            Msg::Remove(id) => {
                self.data.remove_by_id(&id);
            }
            Msg::SetFilter(filter) => {
                self.data.filter = filter;
            }
            Msg::ToggleEdit(id) => {
                self.edit_value = self.data.get_entry_by_id_mut(&id).unwrap_throw().description.clone();
                self.editing_id = Some(id);
            }
            Msg::ToggleAll => {
                let status = !self.data.is_all_completed();
                self.data.set_completed_for_all(status);
            }
            Msg::Toggle(id) => {
                self.data.get_entry_by_id_mut(&id).unwrap_throw().toggle_completion();
            }
            Msg::ClearCompleted => {
                self.data.clear_completed();
            }
            Msg::Focus => {
                if let Some(input) = self.focus_ref.cast::<InputElement>() {
                    input.focus().unwrap();
                }
            }
        }
        self.data.save_todos(KEY).unwrap_throw();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let hidden_class = if self.data.entries.is_empty() {
            "hidden"
        } else {
            ""
        };
        html! {
            <div class="todomvc-wrapper">
                <section class="todoapp">
                    <header class="header">
                        <h1>{ "todos" }</h1>
                        { self.view_input(ctx.link()) }
                    </header>
                    <section class={classes!("main", hidden_class)}>
                        <input
                            type="checkbox"
                            class="toggle-all"
                            id="toggle-all"
                            checked={self.data.is_all_completed()}
                            onclick={ctx.link().callback(|_| Msg::ToggleAll)}
                        />
                        <label for="toggle-all" />
                        <ul class="todo-list">
                            { for self.data.get_filtered_entries().map(|e| self.view_entry(e, ctx.link())) }
                        </ul>
                    </section>
                    <footer class={classes!("footer", hidden_class)}>
                        <span class="todo-count">
                            <strong>{ self.data.entry_count() }</strong>
                            { " item(s) left" }
                        </span>
                        <ul class="filters">
                            { for Filter::iter().map(|flt| self.view_filter(flt, ctx.link())) }
                        </ul>
                        <button class="clear-completed" onclick={ctx.link().callback(|_| Msg::ClearCompleted)}>
                            { format!("Clear completed ({})", self.data.completed_count()) }
                        </button>
                    </footer>
                </section>
                <footer class="info">
                    <p>{ "Double-click to edit a todo" }</p>
                    <p>{ "Written by " }<a href="https://github.com/DenisKolodin/" target="_blank">{ "Denis Kolodin" }</a></p>
                    <p>{ "Part of " }<a href="http://todomvc.com/" target="_blank">{ "TodoMVC" }</a></p>
                </footer>
            </div>
        }
    }
}

impl App {
    fn view_filter(&self, filter: Filter, link: &Scope<Self>) -> Html {
        let cls = if self.data.filter == filter {
            "selected"
        } else {
            "not-selected"
        };
        html! {
            <li>
                <a class={cls}
                   href={filter.as_href()}
                   onclick={link.callback(move |_| Msg::SetFilter(filter))}
                >
                    { filter }
                </a>
            </li>
        }
    }

    fn view_input(&self, link: &Scope<Self>) -> Html {
        let onkeypress = link.batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: InputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                Some(Msg::Add(value))
            } else {
                None
            }
        });
        html! {
            // You can use standard Rust comments. One line:
            // <li></li>
            <input
                class="new-todo"
                placeholder="What needs to be done?"
                {onkeypress}
            />
            /* Or multiline:
            <ul>
                <li></li>
            </ul>
            */
        }
    }

    fn view_entry(&self, entry: &TodoEntry, link: &Scope<Self>) -> Html {
        let mut class = Classes::from("todo");
        if self.editing_id == Some(entry.id) {
            class.push(" editing");
        }
        if entry.completed {
            class.push(" completed");
        }
        let id = entry.id;
        html! {
            <li {class}>
                <div class="view">
                    <input
                        type="checkbox"
                        class="toggle"
                        checked={entry.completed}
                        onclick={link.callback(move |_| Msg::Toggle(id))}
                    />
                    <label ondblclick={link.callback(move |_| Msg::ToggleEdit(id))}>{ &entry.description }</label>
                    <button class="destroy" onclick={link.callback(move |_| Msg::Remove(id))} />
                </div>
                { self.view_entry_edit_input(entry, link) }
            </li>
        }
    }

    fn view_entry_edit_input(&self, entry: &TodoEntry, link: &Scope<Self>) -> Html {
        let id = entry.id;
        let edit = move |input: InputElement| {
            let value = input.value();
            input.set_value("");
            Msg::Edit((id, value))
        };

        let onblur = link.callback(move |e: FocusEvent| edit(e.target_unchecked_into()));

        let onkeypress = link.batch_callback(move |e: KeyboardEvent| {
            (e.key() == "Enter").then(|| edit(e.target_unchecked_into()))
        });

        if self.editing_id == Some(entry.id) {
            html! {
                <input
                    class="edit"
                    type="text"
                    ref={self.focus_ref.clone()}
                    value={self.edit_value.clone()}
                    onmouseover={link.callback(|_| Msg::Focus)}
                    {onblur}
                    {onkeypress}
                />
            }
        } else {
            html! { <input type="hidden" /> }
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
