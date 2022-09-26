use todomvc_shared::Filter;

pub use super::App;

pub struct Router(pub spair::Comp<App>);
impl spair::Router for Router {
    fn routing(&self, location: spair::web_sys::Location) {
        let filter = match location.hash().unwrap_or_else(|_| String::new()).as_str() {
            "#completed" => Filter::Completed,
            "#active" => Filter::Active,
            _ => Filter::All,
        };
        self.0.callback_arg_mut(App::set_filter).emit(filter);
    }
}

pub struct Routes(pub Filter);
impl spair::Routes for Routes {
    type Router = Router;
    fn url(&self) -> String {
        self.0.as_href().to_string()
    }
}
