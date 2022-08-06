use bdays::HolidayCalendar;
use chrono::NaiveDate;
fn main() {
    let cal = bdays::calendars::WeekendsOnly;
    println!("This is the start of dayOff programming!");
    let d0 = NaiveDate::from_ymd(2022, 2, 1);
    let check = cal.is_holiday(d0);
    assert_eq!(check, false);
    println!("{}", check);
}
