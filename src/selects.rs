use web_sys::Event;
use yew::prelude::*;

use crate::utils::{to_full_time, TimeLevel};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub values: &'static [usize],
    pub time_level: TimeLevel,
    pub cb: Callback<Event>,
    pub value_selected: usize,
}

#[function_component(Selector)]
pub fn selector(props: &Props) -> Html {

    let Props { values, time_level, cb, value_selected } = props.clone();

    html! { 

        <div class="field">
            <label class="label center-grid">{&time_level.to_string()}</label>
            <div class="control">
                <div class="select">
                    <select onchange={cb}> 
                        { for values.iter().map(|v| {
                            html! { <option selected={value_selected == *v} value={to_full_time(*v, time_level)}>{v}</option> }
                        }) }
                    </select>
                </div>
            </div>

        </div>
    }
}