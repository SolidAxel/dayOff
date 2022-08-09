use bdays::HolidayCalendar;
use chrono::NaiveDate;
use std::io::{stdin, Read};
fn main() {
    println!("This is the start of dayOff programming!");
    // User input from cmdline
    let weekends_off: String;
    let year: i32;
    let days: i32;
    (year, days, weekends_off) = get_user_input();
    if weekends_off == "y" {
        build_holiday_list(days, year);
    }
}
// Get user input for years and weekends off question
// Unimplemented for now
// TODO: Get user input for days given off by workplace
fn get_user_input() -> (i32, i32, String) {
    let mut user_year_input: String = String::new();
    let mut user_weekends_off: String = String::new();
    let mut user_holidays_given_off: String = String::new();
    let mut holidays_off: String;
    let mut weekends_off: String;
    let mut days: i32;
    let mut year: i32;
    loop {
        user_year_input.clear();
        println!("What year are you asking for?");
        stdin()
            .read_line(&mut user_year_input)
            .expect("Failed to read line.");
        year = match user_year_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        days = days_in_year(year);
        if year >= std::i32::MIN {
            break;
        }
    }
    loop {
        user_weekends_off.clear();
        println!("Does your workplace give you weekends off? (y/n)");
        stdin()
            .read_line(&mut user_weekends_off)
            .expect("Failed to read line.");
        weekends_off = user_weekends_off
            .trim()
            .parse()
            .expect("Input is not a string.");
        if weekends_off.to_ascii_lowercase() == "n" || weekends_off.to_ascii_lowercase() == "y" {
            break;
        }
        println!("Invalid answer to \"weekends off\" question.");
    }
    loop {
        user_holidays_given_off.clear();
        println!(
            "What holidays does your workplace give you off? Please enter in the following format:"
        );
        println!("YYYY-m-dd");
        stdin()
            .read_line(&mut user_holidays_given_off)
            .expect("Failed to read line.");
        holidays_off = user_holidays_given_off
            .trim()
            .parse()
            .expect("Input is not as expected.");
        if holidays_off != "" {
            break;
        }
        println!("Empty answer to \"holidays off\" question.");
    }
    (year, days, weekends_off)
}
// Build list of holidays if weekends are given off already
// Unused for now
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
