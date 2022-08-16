use bdays::HolidayCalendar;
use chrono::{Datelike, NaiveDate, Weekday};
use std::io::stdin;
fn main() {
    println!("This is the start of dayOff programming!");
    // User input from cmdline
    let weekends_off: String;
    let year: i32;
    let days: i32;
    let vector: Vec<NaiveDate>;
    (year, days, weekends_off, vector) = get_user_input();
    if weekends_off == "y" {
        build_holiday_list(days, year);
    }
    let var = day_of_week_count(vector);
    println!("{:?}", var);
}
// Get user input for years and weekends off question
// Unimplemented for now
// TODO: Get user input for days given off by workplace
fn get_user_input() -> (i32, i32, String, Vec<NaiveDate>) {
    let mut user_year_input: String = String::new();
    let mut user_weekends_off: String = String::new();
    let mut user_holidays_given_off: String = String::new();
    let mut holidays_off: String;
    let mut weekends_off: String;
    let mut vector: Vec<NaiveDate> = Vec::new();
    let days: i32;
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
        if year >= std::i32::MIN {
            days = days_in_year(year);
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
            "What holidays does your workplace give you off? Please enter in the following format with only spaces in between dates:"
        );
        println!("m-d-YYYY");
        stdin()
            .read_line(&mut user_holidays_given_off)
            .expect("Failed to read line.");
        holidays_off = user_holidays_given_off
            .trim()
            .parse()
            .expect("Input is not as expected.");
        // Check that input actually matches format requested and add to vector.
        if holidays_off != "" {
            parse_dates(holidays_off, &mut vector);
            break;
        }
        println!("Empty answer to \"holidays off\" question.");
    }
    (year, days, weekends_off, vector)
}
// Figure out how to handle panics; maybe change to read in one by one and if "done" entered then stop asking for input
fn parse_dates(holidays_off: String, vector: &mut Vec<NaiveDate>) -> &Vec<NaiveDate> {
    // let mut vector: Vec<NaiveDate> = Vec::new();
    let null_date: NaiveDate = NaiveDate::from_ymd(1, 1, 1);
    for s in holidays_off.split_whitespace() {
        let date = NaiveDate::parse_from_str(s, "%m-%d-%Y").expect("Couldn't parse date");
        if date != null_date {
            vector.push(date)
        }
    }
    // println!("{:?}", vector);
    vector
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
fn day_of_week_count(vector: Vec<NaiveDate>) -> Vec<(Weekday, u8)> {
    let mut day_vector: Vec<(Weekday, u8)> = Vec::new();
    day_vector.insert(0, (Weekday::Mon, 0));
    day_vector.insert(1, (Weekday::Tue, 0));
    day_vector.insert(2, (Weekday::Wed, 0));
    day_vector.insert(3, (Weekday::Thu, 0));
    day_vector.insert(4, (Weekday::Fri, 0));
    day_vector.insert(5, (Weekday::Sat, 0));
    day_vector.insert(6, (Weekday::Sun, 0));

    for date in vector {
        // *map.entry(date.weekday()).or_insert(0) += 1;
        for day in &mut day_vector {
            if date.weekday() == day.0 {
                day.1 += 1;
            }
        }
    }
    day_vector
}
