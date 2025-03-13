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
