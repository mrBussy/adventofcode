use super::Settings;

pub mod day1;
pub mod day2;
pub mod day3;

/// Runn all challenges
/// Printing header in a color scheme:
///Color	HEX	RGB
/// #ff0000	rgb(255, 0, 0)
// #ff8000	rgb(255, 128, 0)
// #ffff00	rgb(255, 255, 0)
// #80ff00	rgb(128, 255, 0)
// #00ff00	rgb(0, 255, 0)
// #00ff80	rgb(0, 255, 128)
// #00ffff	rgb(0, 255, 255)
// #0080ff	rgb(0, 128, 255)
// #0000ff	rgb(0, 0, 255)
// #8000ff	rgb(128, 0, 255)
// #ff00ff	rgb(255, 0, 255)
// #ff0080	rgb(255, 0, 128)
pub fn run(settings: Settings) {
    day1::run(settings);
    day2::run(settings);
    day3::run(settings);
}
