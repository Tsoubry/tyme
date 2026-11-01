use std::fmt::{self, Display, Formatter};

use gloo::{console::error, utils as gloo_utils};
use wasm_bindgen::JsCast as _;
use web_sys::HtmlMediaElement;

pub const ALL_SECONDS: &[usize] = &[
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
];
pub const ALL_MINUTES: &[usize] = ALL_SECONDS;
pub const ALL_HOURS: &[usize] = &[
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
];

pub const ADD_SECONDS: &[usize] = &[15, 30, 45];
pub const ADD_MINUTES: &[usize] = &[1, 5, 10, 15, 30, 45];
pub const ADD_HOURS: &[usize] = &[1, 2];

pub const LINK_MINUTES_1: &[usize] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
pub const LINK_MINUTES_2: &[usize] = &[16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30];
pub const LINK_HOURS: &[usize] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

#[derive(Clone, PartialEq, Copy)]
pub enum TimeLevel {
    Hour,
    Minute,
    Second,
}

impl Display for TimeLevel {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TimeLevel::Hour => write!(formatter, "Hours"),
            TimeLevel::Minute => write!(formatter, "Minutes"),
            TimeLevel::Second => write!(formatter, "Seconds"),
        }
    }
}

fn alarm_element() -> Option<HtmlMediaElement> {
    gloo_utils::document()
        .get_element_by_id("alarm-sound")?
        .dyn_into::<HtmlMediaElement>()
        .ok()
}

pub fn format_time(seconds: usize) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    format!("{hours:02}:{minutes:02}:{secs:02}")
}

pub fn from_full_time(time: &str) -> Option<(usize, TimeLevel)> {
    let mut time_parts = time.split(' ');

    let value = time_parts.next()?.parse::<usize>().ok()?;
    let unit = time_parts.next()?;

    if unit.contains("second") {
        Some((value, TimeLevel::Second))
    } else if unit.contains("minute") {
        Some((value, TimeLevel::Minute))
    } else if unit.contains("hour") {
        Some((value, TimeLevel::Hour))
    } else {
        None
    }
}

pub fn play_sound() {
    if let Some(audio) = alarm_element() {
        drop(audio.play().ok());
    }
}

pub fn stop_sound() {
    if let Some(audio) = alarm_element()
        && let Err(err) = audio.pause()
    {
        error!("Failed to pause alarm", err);
    }
}

pub fn to_full_time(value: usize, time_level: TimeLevel) -> String {
    match time_level {
        TimeLevel::Hour if value == 1 => format!("{value} hour"),
        TimeLevel::Hour => format!("{value} hours"),
        TimeLevel::Minute if value == 1 => format!("{value} minute"),
        TimeLevel::Minute => format!("{value} minutes"),
        TimeLevel::Second if value == 1 => format!("{value} second"),
        TimeLevel::Second => format!("{value} seconds"),
    }
}
