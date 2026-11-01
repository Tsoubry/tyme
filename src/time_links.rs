use web_sys::MouseEvent;
use yew::prelude::*;

use crate::utils::{TimeLevel, to_full_time};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub cb: Callback<MouseEvent>,
    pub time_level: TimeLevel,
    pub values: &'static [usize],
}

#[function_component(SetTime)]
pub fn set_time(props: &Props) -> Html {
    let Props { values, time_level, cb } = props.clone();

    html! {
                    { for values.iter().map(|value| {

                        let full_time = to_full_time(*value, time_level);

                        html! {
                            <p>
                                <span class="tag is-link is-light">
                                    <a onclick={cb.clone()} id={full_time.clone()}>{full_time}</a>
                                </span>
                            </p>
                        }
                    }) }

    }
}

#[function_component(AddTime)]
pub fn add_time(props: &Props) -> Html {
    let Props { values, time_level, cb } = props.clone();

    html! {

        { for values.iter().map(|value| {

            let full_time = to_full_time(*value, time_level);

            html! {
                <p>
                    <span class="tag is-link is-light">
                        <a onclick={cb.clone()} id={full_time.clone()}>{format!("+{full_time}")}</a>
                    </span>
                </p>
             }
        }) }

    }
}
