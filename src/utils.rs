use std::fmt::{self, Display, Formatter};

use gloo::{console::error, utils as gloo_utils};
use wasm_bindgen::JsCast as _;
use web_sys::HtmlMediaElement;

pub const ALL_SECONDS: &[usize] = &make_n::<0, 60>();
pub const ALL_MINUTES: &[usize] = ALL_SECONDS;
pub const ALL_HOURS: &[usize] = &make_n::<0, 24>();

pub const ADD_SECONDS: &[usize] = &[15, 30, 45];
pub const ADD_MINUTES: &[usize] = &[1, 5, 10, 15, 30, 45];
pub const ADD_HOURS: &[usize] = &[1, 2];

pub const LINK_MINUTES_1: &[usize] = &make_n::<1, 16>();
pub const LINK_MINUTES_2: &[usize] = &make_n::<16, 31>();
pub const LINK_HOURS: &[usize] = &make_n::<1, 11>();

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

const fn make_n<const S: usize, const N: usize>() -> [usize; N] {
    let mut arr = [S; N];
    let lower = S;
    let upper = S + N;
    let mut i = 0;
    while (i + lower) < upper {
        arr[i] = i;
        i += 1;
    }
    arr
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
