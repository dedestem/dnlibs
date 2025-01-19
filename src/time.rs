use std::time::{SystemTime, UNIX_EPOCH};

// Function to determine if a year is a leap year
pub fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

// Function to get the system's timezone offset
fn get_timezone_offset() -> i64 {
    let local_time = SystemTime::now();
    let utc_time = local_time.duration_since(UNIX_EPOCH).unwrap();
    let local_offset = local_time.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

    // Calculate the time difference between UTC and local time and return
    local_offset - utc_time.as_secs() as i64
}

pub fn get_time() -> String {
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    let seconds = duration_since_epoch.as_secs() as i64;

    // Get timezone offset
    let timezone_offset = get_timezone_offset();

    // Adjust for the time zone offset
    let local_seconds = seconds + timezone_offset;

    // Calculate hours, minutes, and seconds
    let seconds_in_minute = 60;
    let minutes_in_hour = 60;
    let hours_in_day = 24;

    let hours = (local_seconds / (seconds_in_minute * minutes_in_hour)) % hours_in_day;
    let minutes = (local_seconds / seconds_in_minute) % minutes_in_hour;
    let seconds = local_seconds % seconds_in_minute;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

pub fn get_date() -> String {
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    let seconds = duration_since_epoch.as_secs() as i64;

    // Get timezone offset
    let timezone_offset = get_timezone_offset();

    // Adjust for the time zone offset
    let local_seconds = seconds + timezone_offset;

    // Calculate total days since UNIX epoch
    let seconds_in_day = 86400;
    let mut days_since_epoch = local_seconds / seconds_in_day;

    // Calculate the year
    let mut year = 1970;
    let mut days_in_year = 365;

    while days_since_epoch >= days_in_year {
        if is_leap_year(year) {
            days_in_year = 366;
        } else {
            days_in_year = 365;
        }
        if days_since_epoch < days_in_year {
            break;
        }
        days_since_epoch -= days_in_year;
        year += 1;
    }

    // Calculate the month
    let mut month = 0;
    let days_in_month = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31] // Leap year months
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31] // Regular months
    };

    while days_since_epoch >= days_in_month[month as usize] {
        days_since_epoch -= days_in_month[month as usize];
        month += 1;
    }

    // The month is 1-based, so add 1
    let day = days_since_epoch + 1;

    format!("{:04}-{:02}-{:02}", year, month + 1, day)
}

pub fn get_time_and_date() -> String {
    let time = get_time();
    let date = get_date();
    format!("{} {}", date, time)
}