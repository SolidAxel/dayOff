// use bdays::HolidayCalendar;
use chrono::NaiveDate;
fn main() {
    // let cal = bdays::calendars::WeekendsOnly;
    println!("This is the start of dayOff programming!");
    println!("{:?}", days_in_year(2024))
}
fn days_in_year(year: i32) -> i64 {
    let since = NaiveDate::signed_duration_since;
    let from_ymd = NaiveDate::from_ymd;
    let days = (since(from_ymd(year, 1, 1), from_ymd(year + 1, 1, 1))).num_days();
    return days.abs();
}
