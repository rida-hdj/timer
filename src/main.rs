// Copyright © 2026 rida-hdj <https://github.com/rida-hdj>

use ctrlc;
use rusty_audio::Audio;
use std::io::{self, Write};
use std::{thread, time};

fn main() {
    ctrlc_handler();
    clear_terminal();
    println!("Welcome to the timer");
    loop {
        clear_terminal();
        timer();
        audio();
    }
}

fn take_time() -> i32 {
    loop {
        print!("Enter the duration in minuts: ");
        io::stdout().flush().unwrap();

        let mut user_duration_of_time = String::new();
        io::stdin().read_line(&mut user_duration_of_time).unwrap();
        match user_duration_of_time.trim().parse::<i32>() {
            Ok(period) => {
                clear_terminal();
                return period;
            }
            Err(__) => println!("invalid input, try again :)"),
        }
    }
}
// todo: fix take input while the alarm sound in running

fn timer() {
    let mut seconds = take_time() * 60;
    if seconds <= 86400 {
        loop {
            clear_terminal();
            let hours = seconds / 3600;
            let minuts = (seconds % 3600) / 60;
            let secs = seconds % 60;

            print!("\r{hours:02}:{minuts:02}:{secs:02}");
            io::stdout().flush().unwrap();

            if seconds == 0 {
                break;
            }

            seconds -= 1;

            thread::sleep(time::Duration::from_secs(1));
        }
        clear_terminal();
        print!("\rdone");
        io::stdout().flush().unwrap();
    } else {
        print!("this is more than day :(");
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_secs(5));
        clear_terminal();
        timer();
    }
}

fn audio() {
    let mut audio = Audio::new();
    audio.add("alarm", "alarm.mp3"); // Load the sound, give it a name
    audio.play("alarm"); // Execution continues while playback occurs in another thread.
    audio.wait(); // Block until sounds finish playing
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn ctrlc_handler() {
    ctrlc::set_handler(|| {
        println!(
            "
thanks for using timer"
        );
        std::process::exit(0)
    })
    .expect("can't exit");
}
