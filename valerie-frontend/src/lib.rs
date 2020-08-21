use valerie::prelude::components::*;
use valerie::prelude::*;

fn ui() -> Node {
    let input = StateMutex::new(String::new());
    div!(
        h1!("TikTok Loader").class("welcome"),
        input!("Text").bind(input.clone())
            .placeholder("Search user here..").class("search-box"),
        button!(span!("search").class("material-icons")).class("mag-glass").on_event("click", input, |x, e| {

        })
            
    ).into()
}

#[valerie(start)]
pub fn run() {
    App::render_single(ui());
}