use std::{time::*, fmt::{Display}};
use clap::Parser;

struct Time {
    seconds: i32,
    minutes: i32,
    hours: i32,
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:02}:{:02}:{:02}", self.hours, self.minutes, self.seconds).as_str())
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Timezone offset based on UTC+0
    #[clap(short, long, value_parser, default_value_t = 0)]
    timezone: u8
}

fn get_current_time(timezone: i64) -> Time {
    let now = UNIX_EPOCH.elapsed().unwrap().as_secs();

    let seconds = now % 60;
    let minutes = (now / 60) % 60;
    let hours = (now / 3600 + timezone as u64) % 24;

    Time {
        seconds: seconds as i32,
        minutes: minutes as i32,
        hours: hours as i32,
    }
}

fn main() {
    let args = Args::parse(); 

    let time = get_current_time(args.timezone as i64);

    println!("{}", time);
}
