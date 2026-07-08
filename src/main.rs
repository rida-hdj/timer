use std::io;
use std::io::Write;
use std::thread;
use std::time;

fn main() {
    timer();
}

fn timer() -> String {
    let mut seconds = 0;
    let mut minuts = 0;
    let mut hours = 0;
    loop {
        if hours == 24 {
            seconds = 0;
            minuts = 0;
            hours = 0;
        }

        if seconds == 60 {
            minuts += 1;
            seconds = 0;
        }

        if minuts == 60 {
            hours += 1;
            minuts = 0
        }

        print!("\r{hours:02}:{minuts:02}:{seconds:02}");
        io::stdout().flush().unwrap();

        let second = time::Duration::from_secs(1);
        thread::sleep(second);

        seconds += 1;
    }
}
