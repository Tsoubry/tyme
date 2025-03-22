mod countdown;
mod selects;
mod stopwatch;
mod time_links;
mod utils;

use gloo::utils as gloo_utils;

fn main() {
    let document = gloo_utils::document();

    let stopwatch_element = document.query_selector(".stopwatch-app").unwrap().unwrap();
    yew::Renderer::<stopwatch::StopWatch>::with_root(stopwatch_element).render();

    let timer_element = document.query_selector(".timer-app").unwrap().unwrap();
    yew::Renderer::<countdown::Timer>::with_root(timer_element).render();
}
