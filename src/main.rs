use std::{time::*, fmt::{Display}};
use clap::Parser;

const CHARACTERS: [&[&str]; 11] = [
                                &["▉▉▉", "▉ ▉", "▉ ▉", "▉ ▉", "▉▉▉"],   // 0                                
                                &["  ▉", "  ▉", "  ▉", "  ▉", "  ▉"],   // 1  
                                &["▉▉▉", "  ▉", "▉▉▉", "▉  ", "▉▉▉"],   // 2
                                &["▉▉▉", "  ▉", "▉▉▉", "  ▉", "▉▉▉"],   // 3
                                &["▉ ▉", "▉ ▉", "▉▉▉", "  ▉", "  ▉"],   // 4
                                &["▉▉▉", "▉  ", "▉▉▉", "  ▉", "▉▉▉"],   // 5
                                &["▉▉▉", "▉  ", "▉▉▉", "▉ ▉", "▉▉▉"],   // 6
                                &["▉▉▉", "  ▉", "  ▉", "  ▉", "  ▉"],   // 7
                                &["▉▉▉", "▉ ▉", "▉▉▉", "▉ ▉", "▉▉▉"],   // 8
                                &["▉▉▉", "▉ ▉", "▉▉▉", "  ▉", "▉▉▉"],   // 9
                                &[" ", "▉", " ", "▉", " "]              // :
                            ];

struct Time {
    seconds: i8,
    minutes: i8,
    hours: i8
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:02}:{:02}:{:02}", self.hours, self.minutes, self.seconds).as_str())
    } }


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Timezone offset based on UTC+0
    #[clap(short, long, value_parser, default_value_t = 0)]
    timezone: i8,

    /// Should the clock be centered
    #[clap(short, long)]
    center: bool
}

fn get_current_time(timezone: i8) -> Time {
    let now = UNIX_EPOCH.elapsed().unwrap().as_secs();

    let seconds = now % 60;
    let minutes = (now / 60) % 60;
    let hours = if timezone < 0 { 
        ((now / 3600 - u64::try_from(-1i8 * timezone).unwrap_or(0u64))) % 24
    } else {
        ((now / 3600 + u64::try_from(timezone).unwrap_or(0u64))) % 24
    };

    Time {
        seconds: i8::try_from(seconds).unwrap_or(0i8),
        minutes: i8::try_from(minutes).unwrap_or(0i8),
        hours: i8::try_from(hours).unwrap_or(0i8)
    }
}

fn generate_digit_art(number: i8) -> [String; 5] {
    assert!((0i8..10i8).contains(&number), "Decimal digit not in 0..10");
    
    let mut out: [String; 5] = ["".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned()];
    
    let number_art = CHARACTERS.get((number % 10).try_into().unwrap_or(0usize)).unwrap();
    
    for i in 0..out.len() {
        let number_art_string = String::from(*(*number_art).get(i).unwrap()).clone();

        out.get_mut(i).unwrap().clone_from(&number_art_string);
    }

    out
}

fn print_time_art(time: Time, center: bool) -> () {
    let temp = format!("{:02}", time.hours);
    
    let mut out: Vec<String> = vec!["".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned()];  

    for c in temp.chars() {
        let t = generate_digit_art(i8::try_from(c.to_digit(10).unwrap_or(0)).unwrap());  
        out = out.iter().zip(t.iter()).map(|(x, y)| format!("{} {}", x, y)).collect();
    }

    out = out.iter().zip(CHARACTERS.get(10).unwrap().iter()).map(|(x, y)| format!("{} {}", x, y)).collect();

    let temp = format!("{:02}", time.minutes);

    for c in temp.chars() {
        let t = generate_digit_art(i8::try_from(c.to_digit(10).unwrap_or(0)).unwrap());  
        out = out.iter().zip(t.iter()).map(|(x, y)| format!("{} {}", x, y)).collect();
    }

    out = out.iter().zip(CHARACTERS.get(10).unwrap().iter()).map(|(x, y)| format!("{} {}", x, y)).collect();

    let temp = format!("{:02}", time.seconds);

    for c in temp.chars() {
        let t = generate_digit_art(i8::try_from(c.to_digit(10).unwrap_or(0)).unwrap());  
        out = out.iter().zip(t.iter()).map(|(x, y)| format!("{} {}", x, y)).collect();
    }

    let length = out.get(0).unwrap().chars().count();

    if center == true {
        let term_size = termsize::get().unwrap();        

        let mut final_out = String::new();

        // -2 is a magic number half of font size floored
        final_out += "\n".repeat((term_size.rows / 2 - 2).try_into().unwrap()).as_str();
       
        for s in out {
            final_out += format!("{:>x$}\n", s, x = (term_size.cols / 2 + u16::try_from(length / 2).unwrap()).try_into().unwrap()).as_str();
        }
        
        print!("{}", final_out);
    } else {
        let mut final_out = String::new();
        for s in out {
            final_out += format!("{}\n", s).as_str();
        }
        
        print!("{}", final_out);
    }
        
}

fn main() {
    let args = Args::parse(); 

    let now = SystemTime::UNIX_EPOCH;

    std::thread::sleep(Duration::from_millis((now.elapsed().unwrap().as_millis() % 1000).try_into().unwrap()));

    loop {
        let time = get_current_time(args.timezone);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        print_time_art(time, args.center);
        std::thread::sleep(Duration::from_secs(1));
    }
}
