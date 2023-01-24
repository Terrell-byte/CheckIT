use chrono::{Datelike, Local};
use crate::{init_view::View};


pub fn format_date_string(_view: &mut View) -> String {

    if _view.date.year() == Local::now().year() && _view.date.month() == Local::now().month() && _view.date.day() == Local::now().day() {
        return "Today".to_string();
    }

    let day_of_week = match _view.date.weekday() {
        chrono::Weekday::Mon => "Monday",
        chrono::Weekday::Tue => "Tuesday",
        chrono::Weekday::Wed => "Wednesday",
        chrono::Weekday::Thu => "Thursday",
        chrono::Weekday::Fri => "Friday",
        chrono::Weekday::Sat => "Saturday",
        chrono::Weekday::Sun => "Sunday",
    };

    let month = match _view.date.month() {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "",
    };

    format!("{} {}, {}", day_of_week, _view.date.day(), month)
}
