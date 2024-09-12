use std::fmt::Display;

use gloo::utils as gloo_utils;
use wasm_bindgen::JsCast;
use web_sys::HtmlMediaElement;

pub const ALL_SECONDS: &[usize] = &[
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
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

pub fn format_time(seconds: usize) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

#[derive(Clone, PartialEq, Copy)]
pub enum TimeLevel {
    Hour,
    Minute,
    Second,
}

impl Display for TimeLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeLevel::Hour => write!(f, "Hours"),
            TimeLevel::Minute => write!(f, "Minutes"),
            TimeLevel::Second => write!(f, "Seconds"),
        }
    }
}

pub fn to_full_time(value: usize, time_level: TimeLevel) -> String {
    match time_level {
        TimeLevel::Hour if value == 1 => format!("{} hour", value),
        TimeLevel::Hour => format!("{} hours", value),
        TimeLevel::Minute if value == 1 => format!("{} minute", value),
        TimeLevel::Minute => format!("{} minutes", value),
        TimeLevel::Second if value == 1 => format!("{} second", value),
        TimeLevel::Second => format!("{} seconds", value),
    }
}

pub fn from_full_time(time: &str) -> Option<(usize, TimeLevel)> {
    let mut time_parts = time.split(' ');

    let value = time_parts.next().unwrap().parse::<usize>().ok();
    let time_level = time_parts.next();

    match (value, time_level) {
        (None, _) => None,
        (Some(value), Some(l)) if l.contains("second") => Some((value, TimeLevel::Second)),
        (Some(value), Some(l)) if l.contains("minute") => Some((value, TimeLevel::Minute)),
        (Some(value), Some(l)) if l.contains("hour") => Some((value, TimeLevel::Hour)),
        _ => None,
    }
}

pub fn play_sound() {
    let document = gloo_utils::document();
    let audio_element = document.get_element_by_id("alarm-sound").unwrap();

    let audio = audio_element.dyn_into::<HtmlMediaElement>().unwrap();
    audio.play().ok();
}

pub fn stop_sound() {
    let document = gloo_utils::document();
    let audio_element = document.get_element_by_id("alarm-sound").unwrap();

    let audio = audio_element.dyn_into::<HtmlMediaElement>().unwrap();
    audio.pause().ok();
}
