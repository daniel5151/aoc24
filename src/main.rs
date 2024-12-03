#![allow(
    clippy::collapsible_else_if,
    clippy::type_complexity,
    clippy::unreadable_literal
)]
#![feature(array_windows)]

#[macro_use]
pub mod prelude;
pub mod util;

/// Catch-all error type (works with anything that implements std::error::Error)
pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

// Utility macro to make adding new days a breeze
macro_rules! days {
    ($($day:ident),* $(,)*) => {
        $(mod $day;)*

        fn route_day(day: &str, question: &str, input: &str, other_args: &[&str]) -> DynResult<()> {
            let day = format!("day{}", day);

            match day.as_str() {
                $(stringify!($day) => match question {
                    "1" => println!("Answer: {:?}", $day::q1(input, other_args)?),
                    "2" => println!("Answer: {:?}", $day::q2(input, other_args)?),
                    _ => return Err("Unknown question".into()),
                })*
                _ => return Err("Unknown day".into()),
            }

            Ok(())
        }
    };
}

days! {
    day1,
    day2,
    day3,
}

fn main() -> DynResult<()> {
    let args = std::env::args().collect::<Vec<String>>();
    let args = args.iter().map(String::as_str).collect::<Vec<&str>>();

    let (day, question) = match (args.get(1), args.get(2)) {
        (None, _) | (_, None) => return Err("Must specify day and question (e.g: 3 1)".into()),
        (Some(d), Some(q)) => (d, q),
    };

    let input_path = format!("./inputs/{}.txt", day);
    let input_path = std::path::Path::new(&input_path);

    let mut input = std::fs::read_to_string(input_path)
        .map_err(|e| format!("Could not open {}: {}", input_path.to_string_lossy(), e))?;
    input.truncate(input.trim_end().len());

    route_day(day, question, &input, &args[3..])
}
