use chrono::{NaiveDate};
use chrono::Datelike;

fn is_leap_year(y :u32)->bool{
    (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
}

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if is_leap_year(year){
        return None;
    }
    // 365/2 = 183  => year/07/02
    let date = NaiveDate::from_yo_opt(year as i32, 183)?;
    Some(date.weekday())

}