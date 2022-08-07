// use bdays::HolidayCalendar;
use chrono::NaiveDate;
use std::io::stdin;
fn main() {
    // let cal = bdays::calendars::WeekendsOnly;
    println!("This is the start of dayOff programming!");
    println!("Please enter the year you want number of days for: ");
    let mut user_year_input = String::new();
    stdin()
        .read_line(&mut user_year_input)
        .expect("Failed to read line.");
    let year: i32 = user_year_input
        .trim()
        .parse()
        .expect("Input not an integer");
    println!("{:?}", days_in_year(year))
}

fn days_in_year(year: i32) -> i64 {
    let since = NaiveDate::signed_duration_since;
    let from_ymd = NaiveDate::from_ymd;
    let days = (since(from_ymd(year, 1, 1), from_ymd(year + 1, 1, 1))).num_days();
    return days.abs();
}
