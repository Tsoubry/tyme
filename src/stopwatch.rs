use gloo::timers::callback::Interval;
use yew::{html, Component, Context, Html};

use crate::utils::format_time;

pub enum Msg {
    StartTimer,
    UpdateTime,
    PauseTimer,
    ResetTimer,
}

pub struct StopWatch {
    seconds: usize,
    timer: Option<Interval>,
}

impl Component for StopWatch {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        StopWatch {
            seconds: 0,
            timer: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartTimer => {
                let clock_handle = {
                    let link = ctx.link().clone();
                    Interval::new(1_000, move || link.send_message(Msg::UpdateTime))
                };
                self.timer = Some(clock_handle);
            }
            Msg::UpdateTime => {
                self.seconds += 1;
            }
            Msg::PauseTimer => {
                self.timer.take();
            }
            Msg::ResetTimer => {
                self.seconds = 0;
                self.timer = None;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let is_running = self.timer.is_some();
        html! {
            <>
                <div class="box center-grid">
                    <h1 class="title is-2 has-text-success-dark">{ "Stopwatch" }</h1>

                    <div id="time" class="block">
                        <h1 class="title is-1">{ format_time(self.seconds) } </h1>
                    </div>

                    <div class="columns">

                        <div id="buttons" class="buttons are-large">
                            <div class="column center-grid">
                                <button class="button is-info is-rounded" disabled={is_running} onclick={ctx.link().callback(|_| Msg::StartTimer)}>
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
                </div>
            </>
        }
    }
}
