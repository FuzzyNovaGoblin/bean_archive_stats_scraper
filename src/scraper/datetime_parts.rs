use chrono::{DateTime, Datelike, Duration, Local, LocalResult, TimeZone, Timelike};
use std::fmt::Display;

#[derive(Eq, Hash, PartialEq, Clone, Debug, serde::Deserialize)]
pub struct DateTimeParts {
    pub hour: u32,
    pub minute: u32,
}

impl DateTimeParts {

    pub fn get_target_time(&self) -> LocalResult<DateTime<Local>> {
        let day: DateTime<Local> = {
            let now = Local::now();
            if now.hour() > self.hour {
                now + Duration::days(1)
            } else {
                now
            }
        };

        Local.with_ymd_and_hms(
            day.year(),
            day.month(),
            day.day(),
            self.hour,
            self.minute,
            0,
        )
    }

    // pub fn discord_display(&self) -> String {

    //     let ret_str = format!(
    //         "{:0>2}:{:0>2}",
    //         get_str(self.hour, 2),
    //         get_str(self.minute, 2),
    //     );

    //     if let Some(s) = self.second {
    //         format!("{} {:0>2}", ret_str, s)
    //     } else {
    //         ret_str
    //     }
    // }
}

impl Display for DateTimeParts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hour, self.minute,)?;

        Ok(())
    }
}
