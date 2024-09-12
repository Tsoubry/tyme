use gloo::timers::callback::Interval;

use gloo::console::log;

use wasm_bindgen::JsCast;

use web_sys::{EventTarget, HtmlInputElement, HtmlLinkElement, MouseEvent};
use yew::events::Event;
use yew::{html, Component, Context, Html};

use crate::utils::{format_time, from_full_time, play_sound, stop_sound, TimeLevel};
use crate::utils::{
    ADD_HOURS, ADD_MINUTES, ADD_SECONDS, ALL_HOURS, ALL_MINUTES, ALL_SECONDS, LINK_HOURS,
    LINK_MINUTES_1, LINK_MINUTES_2,
};

use crate::selects::Selector;
use crate::time_links::{AddTime, SetTime};

pub enum Msg {
    SetSelectorTimer(usize, TimeLevel),
    ShortCutTimer(usize, TimeLevel),
    AddTime(usize, TimeLevel),
    StartTimer,
    UpdateTime,
    PauseTimer,
    ResetTimer,
}

pub struct Timer {
    start_seconds: usize,
    seconds: usize,
    timer: Option<Interval>,
    finished: bool,
    second_selector: usize,
    minute_selector: usize,
    hour_selector: usize,
    progress: usize,
    reset_clicked: bool,
}

impl Timer {
    fn set_timer_0(&mut self) {
        self.start_seconds = 0;
        self.seconds = 0;
        self.finished = false;
    }

    fn reset(&mut self) {
        self.start_seconds = 0;
        self.seconds = 0;
        self.finished = false;
        self.second_selector = 0;
        self.minute_selector = 0;
        self.hour_selector = 0;
        self.progress = 100;
    }

    fn set_total_seconds(&mut self) {
        let total_seconds =
            self.hour_selector * 3600 + self.minute_selector * 60 + self.second_selector;
        self.start_seconds = total_seconds;
        self.seconds = total_seconds;
    }
}

impl Component for Timer {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Timer {
            start_seconds: 0,
            seconds: 0,
            timer: None,
            finished: false,
            second_selector: 0,
            minute_selector: 0,
            hour_selector: 0,
            progress: 100,
            reset_clicked: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetSelectorTimer(value, time_level) => {
                self.set_timer_0();
                match time_level {
                    TimeLevel::Second => self.second_selector = value,
                    TimeLevel::Minute => self.minute_selector = value,
                    TimeLevel::Hour => self.hour_selector = value,
                }
                self.set_total_seconds();
            }
            Msg::ShortCutTimer(value, time_level) => {
                self.reset();
                match time_level {
                    TimeLevel::Second => self.second_selector = value,
                    TimeLevel::Minute => self.minute_selector = value,
                    TimeLevel::Hour => self.hour_selector = value,
                }
                self.set_total_seconds();
            }
            Msg::AddTime(value, time_level) => {
                self.set_timer_0();
                match time_level {
                    TimeLevel::Second => self.second_selector += value,
                    TimeLevel::Minute => self.minute_selector += value,
                    TimeLevel::Hour => self.hour_selector += value,
                }
                self.set_total_seconds();
            }
            Msg::StartTimer => {
                if self.seconds > 0 {
                    let clock_handle = {
                        let link = ctx.link().clone();
                        Interval::new(1000, move || link.send_message(Msg::UpdateTime))
                    };
                    self.timer = Some(clock_handle);
                }
            }
            Msg::UpdateTime => {
                self.seconds -= 1;
                self.progress =
                    (self.seconds as f64 / self.start_seconds as f64 * 100.0).round() as usize;
                if self.seconds == 0 {
                    self.timer.take();
                    self.finished = true;
                }
            }
            Msg::PauseTimer => {
                self.timer.take();
            }
            Msg::ResetTimer => {
                stop_sound();

                if self.reset_clicked {
                    self.reset();
                    self.reset_clicked = false;
                } else {
                    self.seconds = self.start_seconds;
                    self.timer = None;
                    self.finished = false;
                    self.progress = 100;
                    self.reset_clicked = true;
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let is_running = self.timer.is_some();
        let no_seconds = self.seconds == 0;

        let on_change = ctx.link().callback(|e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");

            let time_string = target.unchecked_into::<HtmlInputElement>().value();
            log!("selecting", &time_string);

            let (value, time_level) =
                from_full_time(&time_string).unwrap_or((0, TimeLevel::Second));

            Msg::SetSelectorTimer(value, time_level)
        });

        let on_click = ctx.link().callback(|e: MouseEvent| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");

            let time_string = target.unchecked_into::<HtmlLinkElement>().id();
            log!("selecting", &time_string);

            let (value, time_level) =
                from_full_time(&time_string).unwrap_or((0, TimeLevel::Second));

            Msg::ShortCutTimer(value, time_level)
        });

        let on_click_add = ctx.link().callback(|e: MouseEvent| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");

            let time_string = target.unchecked_into::<HtmlLinkElement>().id();
            log!("adding", &time_string);

            let (value, time_level) =
                from_full_time(&time_string).unwrap_or((0, TimeLevel::Second));

            Msg::AddTime(value, time_level)
        });

        if self.finished {
            play_sound()
        };

        html! {
            <>
                <div class="box center-grid">
                    <h1 class="title is-2 has-text-success-dark">{ "Timer" }</h1>
                    <div id="time" class="block">
                        <h1 class="title is-1">{ format_time(self.seconds) } </h1>
                    </div>

                    <progress class="progress is-small is-link" value={self.progress.to_string()} max="100">{format!("{}%", self.progress)}</progress>

                    <div class="columns">

                        <div id="buttons" class="buttons are-large">
                            <div class="column center-grid">
                                <button class="button is-info is-rounded" disabled={is_running || no_seconds} onclick={ctx.link().callback(|_| Msg::StartTimer)}>
                                    { "Start" }
                                </button>
                            </div>

                            <div class="column center-grid">
                                <button class="button is-link is-rounded" disabled={!is_running} onclick={ctx.link().callback(|_| Msg::PauseTimer)}>
                                    { "Pause" }
                                </button>
                            </div>

                            <div class="column center-grid">
                                <button class="button is-warning is-rounded" onclick={ctx.link().callback(|_| Msg::ResetTimer)}>
                                    { "Reset" }
                                </button>
                            </div>
                        </div>

                    </div>

                    <div id="selectors">
                        <div class="field is-horizontal">

                        <Selector values={ALL_HOURS} time_level={TimeLevel::Hour} cb={on_change.clone()} value_selected={self.hour_selector} />
                        <Selector values={ALL_MINUTES} time_level={TimeLevel::Minute} cb={on_change.clone()} value_selected={self.minute_selector} />
                        <Selector values={ALL_SECONDS} time_level={TimeLevel::Second} cb={on_change} value_selected={self.second_selector} />

                        </div>
                    </div>

                    <div class="columns">
                        <div class="column has-text-centered">
                            <SetTime values={LINK_MINUTES_1} time_level={TimeLevel::Minute} cb={on_click.clone()} />

                        </div>
                        <div class="column has-text-centered">
                            <SetTime values={LINK_MINUTES_2} time_level={TimeLevel::Minute} cb={on_click.clone()} />

                        </div>
                        <div class="column has-text-centered">
                            <SetTime values={LINK_HOURS} time_level={TimeLevel::Hour} cb={on_click} />
                        </div>

                        <div class="column has-text-centered">
                            <AddTime values={ADD_SECONDS} time_level={TimeLevel::Second} cb={on_click_add.clone()} />
                            <AddTime values={ADD_MINUTES} time_level={TimeLevel::Minute} cb={on_click_add.clone()} />
                            <AddTime values={ADD_HOURS} time_level={TimeLevel::Hour} cb={on_click_add} />
                        </div>
                    </div>

                </div>

            </>
        }
    }
}
