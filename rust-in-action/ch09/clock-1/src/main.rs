use chrono::{DateTime, Local};
use clap::{App, Arg};

// Acts as a namespace at this stage, providing extensibility later on.
struct Clock; // zero-sized type -> no memory usage -> compile time only

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    fn set() -> ! {
        unimplemented!()
    }
}

fn main() {
    let app = App::new("clock")
        .version("0.1.1")
        .about("Gets and sets (aspirationally) the time.")
        .arg(
            Arg::with_name("action")
                .takes_value(true)
                .possible_values(&["get", "set"])
                .default_value("get"),
        )
        .arg(
            Arg::with_name("std")
                .short("s")
                .long("use-standard")
                .takes_value(true)
                .possible_values(&["rfc2822", "rfc3339", "timestamp"])
                .default_value("rfc3339"),
        )
        .arg(
            Arg::with_name("datetime")
                .help("When <action> is 'set', apply <datetime>. Otherwise, ignore."),
        );

    let args = app.get_matches();

    // Default values set in clap app -> safe to unwrap
    let action = args.value_of("action").unwrap();
    let std = args.value_of("std").unwrap();

    // Coming soon...
    if action == "set" {
        unimplemented!()
    }

    let now = Clock::get();
    match std {
        "timestamp" => println!("{}", now.timestamp()),
        "rfc2822" => println!("{}", now.to_rfc2822()),
        "rfc3339" => println!("{}", now.to_rfc3339()),
        _ => unreachable!(), // Clap app restricts possible values
    }
}
