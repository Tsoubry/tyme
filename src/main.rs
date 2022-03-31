
mod stopwatch;
mod countdown;
mod utils;
mod selects;
mod time_links;

use gloo::utils as gloo_utils;

fn main() {
    let document = gloo_utils::document();

    let stopwatch_element = document.query_selector(".stopwatch-app").unwrap().unwrap();
    yew::start_app_in_element::<stopwatch::StopWatch>(stopwatch_element);

    let timer_element = document.query_selector(".timer-app").unwrap().unwrap();
    yew::start_app_in_element::<countdown::Timer>(timer_element);
}
