use web_sys::Event;
use yew::prelude::*;

use crate::utils::{TimeLevel, to_full_time};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub cb: Callback<Event>,
    pub time_level: TimeLevel,
    pub value_selected: usize,
    pub values: &'static [usize],
}

#[function_component(Selector)]
pub fn selector(props: &Props) -> Html {
    let Props {
        values,
        time_level,
        cb,
        value_selected,
    } = props.clone();

    html! {

        <div class="field">
            <label class="label center-grid">{&time_level.to_string()}</label>
            <div class="control">
                <div class="select">
                    <select onchange={cb}>
                        { for values.iter().map(|option| {
                            html! { <option selected={value_selected == *option} value={to_full_time(*option, time_level)}>{*option}</option> }
                        }) }
                    </select>
                </div>
            </div>

        </div>
    }
}
