use bdays::HolidayCalendar;
use chrono::NaiveDate;
use std::io::stdin;
fn main() {
    println!("This is the start of dayOff programming!");
    // User input from cmdline
    println!("What year are you asking for?");
    let mut user_year_input = String::new();
    stdin()
        .read_line(&mut user_year_input)
        .expect("Failed to read line.");
    let year: i32 = user_year_input
        .trim()
        .parse()
        .expect("Input not an integer.");
    let days_in_year: i32 = days_in_year(year);
    build_holiday_list(days_in_year, year);
}
// Build list of holidays weekends are given off already
fn build_holiday_list(days_in_year: i32, year: i32) {
    let cal = bdays::calendars::us::USSettlement;
    let mut vector: Vec<NaiveDate> = Vec::new();
    let mut iterator: i32 = 1;
    while iterator < days_in_year {
        let day = NaiveDate::from_yo(year, iterator.try_into().unwrap());
        if cal.is_holiday(day) {
            vector.push(day);
        }
        iterator += 1;
    }
    println!("{:?}", vector);
}
// Get the number of days for a given year
fn days_in_year(year: i32) -> i32 {
    let since = NaiveDate::signed_duration_since;
    let from_ymd = NaiveDate::from_ymd;
    let days = (since(from_ymd(year, 1, 1), from_ymd(year + 1, 1, 1))).num_days();
    return days.abs().try_into().unwrap();
}
