use gloo::{console::log, timers::callback::Interval};
use wasm_bindgen::JsCast as _;
use web_sys::{EventTarget, HtmlInputElement, HtmlLinkElement, MouseEvent};
use yew::{Component, Context, Html, events::Event, html};

use crate::{
    selects::Selector,
    time_links::{AddTime, SetTime},
    utils::{
        ADD_HOURS, ADD_MINUTES, ADD_SECONDS, ALL_HOURS, ALL_MINUTES, ALL_SECONDS, LINK_HOURS, LINK_MINUTES_1,
        LINK_MINUTES_2, TimeLevel, format_time, from_full_time, play_sound, stop_sound,
    },
};

pub enum Msg {
    AddTime(usize, TimeLevel),
    PauseTimer,
    ResetTimer,
    SetSelectorTimer(usize, TimeLevel),
    ShortCutTimer(usize, TimeLevel),
    StartTimer,
    UpdateTime,
}

pub struct Timer {
    finished: bool,
    hour_selector: usize,
    interval: Option<Interval>,
    minute_selector: usize,
    progress: usize,
    reset_clicked: bool,
    second_selector: usize,
    seconds: usize,
    start_seconds: usize,
}

impl Timer {
    fn reset(&mut self) {
        self.start_seconds = 0;
        self.seconds = 0;
        self.finished = false;
        self.second_selector = 0;
        self.minute_selector = 0;
        self.hour_selector = 0;
        self.progress = 100;
    }

    fn set_timer_0(&mut self) {
        self.start_seconds = 0;
        self.seconds = 0;
        self.finished = false;
    }

    fn set_total_seconds(&mut self) {
        let total_seconds = self.hour_selector * 3600 + self.minute_selector * 60 + self.second_selector;
        self.start_seconds = total_seconds;
        self.seconds = total_seconds;
    }
}

impl Component for Timer {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Timer {
            finished: false,
            hour_selector: 0,
            interval: None,
            minute_selector: 0,
            progress: 100,
            reset_clicked: false,
            second_selector: 0,
            seconds: 0,
            start_seconds: 0,
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
                    self.interval = Some(clock_handle);
                }
            }
            Msg::UpdateTime => {
                self.seconds -= 1;
                self.progress = (self.seconds * 100 + self.start_seconds / 2) / self.start_seconds;
                if self.seconds == 0 {
                    drop(self.interval.take());
                    self.finished = true;
                }
            }
            Msg::PauseTimer => {
                drop(self.interval.take());
            }
            Msg::ResetTimer => {
                stop_sound();

                if self.reset_clicked {
                    self.reset();
                    self.reset_clicked = false;
                } else {
                    self.seconds = self.start_seconds;
                    self.interval = None;
                    self.finished = false;
                    self.progress = 100;
                    self.reset_clicked = true;
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let is_running = self.interval.is_some();
        let no_seconds = self.seconds == 0;

        let on_change = ctx.link().batch_callback(|event: Event| {
            let target: EventTarget = event.target()?;

            let time_string = target.unchecked_into::<HtmlInputElement>().value();
            log!("selecting", &time_string);

            let (value, time_level) = from_full_time(&time_string).unwrap_or((0, TimeLevel::Second));

            Some(Msg::SetSelectorTimer(value, time_level))
        });

        let on_click = ctx.link().batch_callback(|event: MouseEvent| {
            let target: EventTarget = event.target()?;

            let time_string = target.unchecked_into::<HtmlLinkElement>().id();
            log!("selecting", &time_string);

            let (value, time_level) = from_full_time(&time_string).unwrap_or((0, TimeLevel::Second));

            Some(Msg::ShortCutTimer(value, time_level))
        });

        let on_click_add = ctx.link().batch_callback(|event: MouseEvent| {
            let target: EventTarget = event.target()?;

            let time_string = target.unchecked_into::<HtmlLinkElement>().id();
            log!("adding", &time_string);

            let (value, time_level) = from_full_time(&time_string).unwrap_or((0, TimeLevel::Second));

            Some(Msg::AddTime(value, time_level))
        });

        if self.finished {
            play_sound();
        }

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
