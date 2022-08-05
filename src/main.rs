use std::{time::*, fmt::{Display}, io::{Write, BufWriter}};
use clap::Parser;

const CHARACTERS: [[&[&str]; 11]; 3] = [
[
    &["███", "█ █", "█ █", "█ █", "███"],   // 0                                
    &[" █ ", "██ ", " █ ", " █ ", "███"],   // 1  
    &["███", "  █", "███", "█  ", "███"],   // 2
    &["███", "  █", "███", "  █", "███"],   // 3
    &["█ █", "█ █", "███", "  █", "  █"],   // 4
    &["███", "█  ", "███", "  █", "███"],   // 5
    &["███", "█  ", "███", "█ █", "███"],   // 6
    &["███", "  █", "  █", "  █", "  █"],   // 7
    &["███", "█ █", "███", "█ █", "███"],   // 8
    &["███", "█ █", "███", "  █", "███"],   // 9
    &[" ", "█", " ", "█", " "]              // :
],
[
    &[r"   ___   ", r"  / _ \  ", r" | | | | ", r" | | | | ", r" | |_| | ", r"  \___/  "],
    &[r"  __     ", r" /_ |    ", r"  | |    ", r"  | |    ", r"  | |    ", r"  |_|    "],
    &[r"  ___    ", r" |__ \   ", r"    ) |  ", r"   / /   ", r"  / /_   ", r" |____|  "],
    &[r"  ____   ", r" |___ \  ", r"   __) | ", r"  |__ <  ", r"  ___) | ", r" |____/  "],
    &[r"  _  _   ", r" | || |  ", r" | || |_ ", r" |__   _|", r"    | |  ", r"    |_|  "],
    &[r"  _____  ", r" | ____| ", r" | |__   ", r" |___ \  ", r"  ___) | ", r" |____/  "],
    &[r"    __   ", r"   / /   ", r"  / /_   ", r" | '_ \  ", r" | (_) | ", r"  \___/  "],
    &[r"  ______ ", r" |____  |", r"     / / ", r"    / /  ", r"   / /   ", r"  /_/    "],
    &[r"   ___   ", r"  / _ \  ", r" | (_) | ", r"  > _ <  ", r" | (_) | ", r"  \___/  "],
    &[r"   ___   ", r"  / _ \  ", r" | (_) | ", r"  \__, | ", r"    / /  ", r"   /_/   "],
    &[r"         ", r"  _      ", r" (_)     ", r"         ", r"  _      ", r" (_)     "]
],
[
    &["╭─────╮", "│ ╭─╮ │", "│ │ │ │", "│ │ │ │", "│ ╰─╯ │", "╰─────╯"],
    &["  ╭─╮  ", "╭─╯ │  ", "╰─╮ │  ", "  │ │  ", "╭─╯ ╰─╮", "╰─────╯"],
    &["╭─────╮", "╰───╮ │", "╭───╯ │", "│ ╭───╯", "│ ╰───╮", "╰─────╯"],
    &["╭─────╮", "╰───╮ │", "╭───╯ │", "╰───╮ │", "╭───╯ │", "╰─────╯"],
    &["╭─╮ ╭─╮", "│ │ │ │", "│ ╰─╯ │", "╰───╮ │", "    │ │", "    ╰─╯"],
    &["╭─────╮", "│ ╭───╯", "│ ╰───╮", "╰───╮ │", "╭───╯ │", "╰─────╯"],
    &["╭─────╮", "│ ╭───╯", "│ ╰───╮", "│ ╭─╮ │", "│ ╰─╯ │", "╰─────╯"],
    &["╭─────╮", "╰───╮ │", "    │ │", "    │ │", "    │ │", "    ╰─╯"],
    &["╭─────╮", "│ ╭─╮ │", "│ ╰─╯ │", "│ ╭─╮ │", "│ ╰─╯ │", "╰─────╯"],
    &["╭─────╮", "│ ╭─╮ │", "│ ╰─╯ │", "╰───╮ │", "╭───╯ │", "╰─────╯"],
    &["       ", "  ╭─╮  ", "  ╰─╯  ", "  ╭─╮  ", "  ╰─╯  ", "       "]
]
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
    center: bool,

    /// Which font to use
    #[clap(short, long, value_parser, default_value_t = 0)]
    font: usize
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

fn generate_digit_art(number: i8, font: usize) -> Vec<String> {
    assert!((0i8..10i8).contains(&number), "Decimal digit not in 0..10");

    let mut out: Vec<String> = Vec::with_capacity(CHARACTERS[font][0].len()); 

    let number_art = CHARACTERS[font].get((number % 10).try_into().unwrap_or(0usize)).unwrap();
    
    for i in 0..out.capacity() {
        let number_art_string = String::from(*(*number_art).get(i).unwrap_or(&"")).clone();

        out.push(number_art_string);
    }

    out
}

fn print_time_art(time: &Time, center: bool, font: usize) -> () {
    assert!(font < CHARACTERS.len(), "Unknown character set");

    let temp = format!("{:02}", time.hours);
    
    let mut out: Vec<String> = vec![];
    out.resize(CHARACTERS[font][0].len(), "".to_owned());  

    for c in temp.chars() {
        let t = generate_digit_art(i8::try_from(c.to_digit(10).unwrap_or(0)).unwrap(), font);
        out = out.iter().zip(t.iter()).map(|(x, y)| format!("{} {}", x, y)).collect();
    }

    out = out.iter().zip(CHARACTERS[font].get(10).unwrap().iter()).map(|(x, y)| format!("{} {}", x, y)).collect();

    let temp = format!("{:02}", time.minutes);

    for c in temp.chars() {
        let t = generate_digit_art(i8::try_from(c.to_digit(10).unwrap_or(0)).unwrap(), font);  
        out = out.iter().zip(t.iter()).map(|(x, y)| format!("{} {}", x, y)).collect();
    }

    out = out.iter().zip(CHARACTERS[font].get(10).unwrap().iter()).map(|(x, y)| format!("{} {}", x, y)).collect();

    let temp = format!("{:02}", time.seconds);

    for c in temp.chars() {
        let t = generate_digit_art(i8::try_from(c.to_digit(10).unwrap_or(0)).unwrap(), font);  
        out = out.iter().zip(t.iter()).map(|(x, y)| format!("{} {}", x, y)).collect();
    }

    let length = out.get(0).unwrap().chars().count();
    let mut final_out = String::new();
    if center == true {
        let term_size = termsize::get().unwrap();        

        final_out += "\n".repeat((term_size.rows / 2 - u16::try_from(out.len() / 2).unwrap()).try_into().unwrap()).as_str();
       
        for s in out {
            final_out += format!("{:>x$}\n", s, x = (term_size.cols / 2 + u16::try_from(length / 2).unwrap()).try_into().unwrap()).as_str();
        }
    } else {
        for s in out {
            final_out += format!("{}\n", s).as_str();
        }
    }
   
    let mut writer = BufWriter::new(std::io::stdout());
    let _ = writer.write_all(final_out.as_bytes());
}

fn main() {
    let _ = ctrlc::set_handler(move || {
        print!("{esc}[?25h", esc = 27 as char);
        std::process::exit(0);
    });
    
    let args = Args::parse(); 

    print!("{esc}[?25l{esc}[2J{esc}[1;1H", esc = 27 as char);

    let mut old_seconds = 0;
    loop {
        let time = get_current_time(args.timezone);
        
        if time.seconds != old_seconds {
            print!("{esc}[1;1H", esc = 27 as char);
            print_time_art(&time, args.center, args.font);
        }

        old_seconds = time.seconds; 
        std::thread::sleep(Duration::from_millis(50));
    }
}
