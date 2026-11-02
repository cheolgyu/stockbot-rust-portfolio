use chrono::{DateTime, Duration};
use chrono_tz::Tz;

pub fn get_run_time_sec(start: DateTime<Tz>, end: DateTime<Tz>) -> Duration {
    let dt: Duration = end - start;
    let sec = dt.num_seconds();

    let remove_milliseconds = Duration::seconds(sec);
    remove_milliseconds
}
