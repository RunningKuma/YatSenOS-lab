#[macro_use]
mod macros;

pub mod logger;
pub use macros::*;

pub const fn get_ascii_header() -> &'static str {
    concat!(
        "\x1B[31m",
        r"__  __      __  _____            ____  _____", "\n",
        "\x1B[38;5;208m", 
        r"\ \/ /___ _/ /_/ ___/___  ____  / __ \/ ___/", "\n",
        "\x1B[33m", 
        r" \  / __ `/ __/\__ \/ _ \/ __ \/ / / /\__ \ ", "\n",
        "\x1B[32m", 
        r" / / /_/ / /_ ___/ /  __/ / / / /_/ /___/ / ", "\n",
        "\x1B[34m",
        r"/_/\__,_/\__//____/\___/_/ /_/\____//____/  ", "\n\n",
        "\x1B[35m",
        r"          RunningKuma - 22307058       v",
        env!("CARGO_PKG_VERSION"),
        "\x1B[0m"
    )
}

const SHORT_UNITS: [&str; 4] = ["B", "K", "M", "G"];
const UNITS: [&str; 4] = ["B", "KiB", "MiB", "GiB"];

pub fn humanized_size(size: u64) -> (f32, &'static str) {
    humanized_size_impl(size, false)
}

pub fn humanized_size_short(size: u64) -> (f32, &'static str) {
    humanized_size_impl(size, true)
}

#[inline]
pub fn humanized_size_impl(size: u64, short: bool) -> (f32, &'static str) {
    let bytes = size as f32;

    let units = if short { &SHORT_UNITS } else { &UNITS };

    let mut unit = 0;
    let mut bytes = bytes;

    while bytes >= 1024f32 && unit < units.len() {
        bytes /= 1024f32;
        unit += 1;
    }

    (bytes, units[unit])
}
