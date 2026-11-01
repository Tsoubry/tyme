mod countdown;
mod selects;
mod stopwatch;
mod time_links;
mod utils;

use gloo::{console::error, utils as gloo_utils};

fn main() {
    let document = gloo_utils::document();

    if let Ok(Some(stopwatch_element)) = document.query_selector(".stopwatch-app") {
        drop(yew::Renderer::<stopwatch::StopWatch>::with_root(stopwatch_element).render());
    } else {
        error!("Missing .stopwatch-app element");
    }

    if let Ok(Some(timer_element)) = document.query_selector(".timer-app") {
        drop(yew::Renderer::<countdown::Timer>::with_root(timer_element).render());
    } else {
        error!("Missing .timer-app element");
    }
}
