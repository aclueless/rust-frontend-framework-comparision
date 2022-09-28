use crate::app::App;

mod util;
mod todo;
mod app;

pub fn main() {
    dominator::append_dom(&dominator::get_id("app"), App::render(App::deserialize()));
}
