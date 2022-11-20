use spair::prelude::*;

use todomvc_shared::{Todos, Filter, TodoEntry};

#[cfg(feature = "keyed-app")]
mod keyed;
#[cfg(feature = "non-keyed-app")]
mod non_keyed;
mod router;

#[cfg(feature = "keyed-app")]
use keyed::*;
#[cfg(feature = "non-keyed-app")]
use non_keyed::*;


pub struct App {
    data: Todos,
    editing_id: Option<uuid::Uuid>,
    new_description: String,
}

impl App {
    fn save_data(&self) {
        self.data.save_todos(KEY).unwrap_throw();
    }
    fn set_filter(&mut self, filter: Filter) {
        self.data.filter = filter;

        self.save_data();
    }

    fn set_completed_for_all(&mut self, completed: bool) {
        self.data.set_completed_for_all(completed);

        self.save_data();
    }

    fn toggle_completion(&mut self, id: uuid::Uuid) {
        if let Some(e) = self.data.get_entry_by_id_mut(&id) {
            e.toggle_completion();
        }

        self.save_data()
    }

    fn remove_by_id(&mut self, id: uuid::Uuid) {
        self.data.remove_by_id(&id);

        self.save_data();
    }

    fn clear_completed(&mut self) {
        self.data.clear_completed();

        self.save_data();
    }

    fn new_description(&mut self, new_description: String) {
        self.new_description = new_description;
    }

    fn create_new_todo(&mut self) {
        self.data.new_entry(self.new_description.clone());
        self.new_description.clear();

        self.save_data();
    }

    fn start_editing(&mut self, id: uuid::Uuid) {
        self.editing_id = Some(id);
    }

    fn end_editing(&mut self, new_description: Option<String>) {
        let id = match self.editing_id {
            Some(id) => id,
            None => return,
        };
        match new_description {
            Some(new_description) => {
                self.data
                    .entries
                    .iter_mut()
                    .find(|item| item.id == id)
                    .expect_throw("Why editing item with an invalid id?")
                    .description = new_description;
                self.save_data();
            }
            None => self.remove_by_id(id),
        }
        self.editing_id = None;
    }

    fn cancel_editing(&mut self) {
        self.editing_id = None;
    }
}

impl spair::Component for App {
    type Routes = router::Routes;
    fn render(&self, e: spair::Element<Self>) {
        e.section(|s| {
            s.static_attributes()
                .class("todoapp")
                .rupdate(Header)
                .rupdate(Main)
                .rupdate(Footer);
        })
        .rupdate(Info);
    }
}

struct Header;
impl spair::Render<App> for Header {
    fn render(self, nodes: spair::Nodes<App>) {
        let comp = nodes.comp();
        let state = nodes.state();
        nodes.header(|h| {
            h.static_attributes()
                .class("header")
                .static_nodes()
                .h1(|h| h.rupdate("Spair Todos").done())
                .update_nodes()
                .input(|i| {
                    i.value(&state.new_description)
                        .static_attributes()
                        .class("new-todo")
                        .focus(true)
                        .placeholder("What needs to be done?")
                        .on_input(comp.handler_arg_mut(|state, arg: spair::InputEvent| {
                            if let Some(input) = arg.current_target_as_input_element() {
                                state.new_description(input.value());
                            }
                        }))
                        .on_key_press(comp.handler_arg_mut(|state, arg: spair::KeyboardEvent| {
                            // `.key_code()` is deprecated, so we use code instead
                            // https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/keyCode
                            if arg.raw().code().as_str() == "Enter" {
                                state.create_new_todo();
                            }
                        }));
                });
        });
    }
}

struct Main;
impl spair::Render<App> for Main {
    fn render(self, nodes: spair::Nodes<App>) {
        let comp = nodes.comp();
        let state = nodes.state();
        let todo_count = state.data.entry_count();
        let all_completed = state.data.is_all_completed();
        nodes.section(|s| {
            s.class_if(todo_count == 0, "hidden")
                .static_attributes()
                .class("main")
                .input(move |i| {
                    i.checked(all_completed)
                        .on_change(comp.handler_mut(move |state| state.set_completed_for_all(!all_completed)))
                        .static_attributes()
                        .id("toggle-all")
                        .class("toggle-all")
                        .r#type(spair::InputType::CheckBox);
                })
                .static_nodes()
                .label(|l| {
                    l.static_attributes()
                        .r#for("toggle-all")
                        .static_nodes()
                        .rstatic("Mark all as complete");
                })
                .update_nodes()
                .ul(render_list);
        });
    }
}

struct Footer;
impl spair::Render<App> for Footer {
    fn render(self, nodes: spair::Nodes<App>) {
        let comp = nodes.comp();
        let state = nodes.state();
        let item_count = state.data.entry_count();
        let item_left = item_count - state.data.completed_count();
        let some_completed = item_left < item_count;
        nodes.footer(|f| {
            f.class_if(item_count == 0, "hidden")
                .static_attributes()
                .class("footer")
                .update_nodes()
                .span(|s| {
                    s.static_attributes()
                        .class("todo-count")
                        .strong(|s| s.rupdate(item_left).done())
                        .rupdate(if item_left == 1 {
                            " item left"
                        } else {
                            " items left"
                        });
                })
                .ul(|u| {
                    u.static_attributes()
                        .class("filters")
                        .rupdate(FilterView {
                            current_filter: state.data.filter,
                            filter: Filter::All,
                        })
                        .rupdate(FilterView {
                            current_filter: state.data.filter,
                            filter: Filter::Active,
                        })
                        .rupdate(FilterView {
                            current_filter: state.data.filter,
                            filter: Filter::Completed,
                        });
                })
                .button(|b| {
                    b.class_if(!some_completed, "hidden")
                        .static_attributes()
                        .class("clear-completed")
                        .on_click(comp.handler_mut(App::clear_completed))
                        .rstatic("Clear completed");
                });
        });
    }
}

struct FilterView {
    current_filter: Filter,
    filter: Filter,
}

impl spair::Render<App> for FilterView {
    fn render(self, nodes: spair::Nodes<App>) {
        nodes.li(|l| {
            l.a(|a| {
                a.class_if(self.current_filter == self.filter, "selected")
                    .static_attributes()
                    .href(&router::Routes(self.filter))
                    .static_nodes()
                    .rstatic(&self.filter.to_string());
            });
        });
    }
}

struct Info;
impl spair::Render<App> for Info {
    fn render(self, nodes: spair::Nodes<App>) {
        nodes.footer(|f| {
            f.static_attributes()
                .class("info")
                .static_nodes()
                .p(|p| p.rstatic("Double-click to edit a todo").done())
                .p(|p| p.rstatic("Created by 'aclueless'").done())
                .p(|p| {
                    p.rstatic("Part of ").a(|a| {
                        a.static_attributes()
                            .href_str("http://todomvc.com")
                            .rstatic("TodoMVC");
                    });
                });
        });
    }
}

struct RenderEntry<'a>(&'a TodoEntry);
impl<'a> spair::ElementRender<App> for RenderEntry<'a> {
    const ELEMENT_TAG: &'static str = "li";
    fn render(self, li: spair::Element<App>) {
        let comp = li.comp();
        let state = li.state();
        let id = self.0.id;
        let is_editing_me = state.editing_id == Some(id);
        li.class_if(self.0.completed, "completed")
            .class_if(is_editing_me, "editing")
            .div(move |d| {
                d.static_attributes()
                    .class("view")
                    .input(|i| {
                        i.on_change(comp.handler_mut(move |state| state.toggle_completion(id)))
                            .checked(self.0.completed)
                            .static_attributes()
                            .class("toggle")
                            .r#type(spair::InputType::CheckBox);
                    })
                    .label(|l| {
                        l.on_double_click(comp.handler_mut(move |state| state.start_editing(id)))
                            .rupdate(&self.0.description);
                    })
                    .button(|b| {
                        b.on_click(comp.handler_mut(move |state| state.remove_by_id(id)))
                            .static_attributes()
                            .class("destroy");
                    });
            })
            .match_if(|mi| match is_editing_me {
                true => spair::set_arm!(mi)
                    .rupdate(EditingInput(&self.0.description))
                    .done(),
                false => spair::set_arm!(mi).done(),
            });
    }
}

struct EditingInput<'a>(&'a String);
impl<'a> spair::Render<App> for EditingInput<'a> {
    fn render(self, nodes: spair::Nodes<App>) {
        let comp = nodes.comp();
        nodes.input(|i| {
            i.focus(true)
                .value(self.0)
                .static_attributes()
                .class("edit")
                .on_blur(comp.handler_arg_mut(|state, arg: spair::FocusEvent| {
                    state.end_editing(get_value(arg.current_target_as()))
                }))
                .on_key_down(comp.handler_arg_mut(|state, arg: spair::KeyboardEvent| {
                    match arg.raw().code().as_str() {
                        "Escape" => state.cancel_editing(),
                        "Enter" => state.end_editing(get_value(arg.current_target_as())),
                        _ => {}
                    }
                }));
        });
    }
}

fn get_value(i: Option<spair::web_sys::HtmlInputElement>) -> Option<String> {
    i.and_then(|i| {
        let text = i.value();
        let text = text.trim();
        match text.is_empty() {
            true => None,
            false => Some(text.to_string()),
        }
    })
}

impl spair::Application for App {
    fn init(_comp: &spair::Comp<Self>) -> Self {
        Self {
            data: Todos::load_todos(KEY),
            editing_id: None,
            new_description: String::new(),
        }
    }

    fn init_router(comp: &spair::Comp<Self>) -> Option<router::Router> {
        Some(router::Router(comp.clone()))
    }
}
