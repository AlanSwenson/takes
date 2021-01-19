use chrono::{DateTime, FixedOffset, Utc};
use std::fmt;
use std::io::{stdin, stdout, Read, Write};
use text_io::read;

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

struct NoteEvent {
    time_stamp: String,
    note: String,
    seconds: u16,
}

impl fmt::Display for NoteEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.note, self.time_stamp, self.seconds)
    }
}

fn main() {
    let est_tz = FixedOffset::west(5 * 3600);
    let mut session_name = String::new();
    let mut vec = Vec::new();
    println!("Title of Recording :");
    stdin().read_line(&mut session_name).unwrap();
    println!("Hello , {}", session_name);
    // wait for enter key
    pause();

    loop {
        // record time stamp
        let now: DateTime<Utc> = Utc::now();
        let time_stamp = now.with_timezone(&est_tz).format("%a %b %e %T");
        println!("Timestamp is: {}", time_stamp);
        // record note
        let mut note = String::new();
        println!("Note:");
        stdin().read_line(&mut note).unwrap();
        // ask for seconds
        println!("Seconds:");
        let seconds: u16 = read!();
        let current_note_event = NoteEvent {
            time_stamp: time_stamp.to_string(),
            note: note,
            seconds: seconds,
        };
        // wait for enter key
        vec.push(current_note_event);
        for x in &vec {
            println!("{}", x);
        }
        pause();
        // break if a certain shortcut is pressed
    }
}
