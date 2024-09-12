mod countdown;
mod selects;
mod stopwatch;
mod time_links;
mod utils;

use gloo::utils as gloo_utils;

fn main() {
    let document = gloo_utils::document();

    let stopwatch_element = document.query_selector(".stopwatch-app").unwrap().unwrap();
    yew::start_app_in_element::<stopwatch::StopWatch>(stopwatch_element);

    let timer_element = document.query_selector(".timer-app").unwrap().unwrap();
    yew::start_app_in_element::<countdown::Timer>(timer_element);
}
